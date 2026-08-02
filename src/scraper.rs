use scraper::{Html, Selector};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use crate::models::ModEntry;

fn project_root() -> PathBuf {
    let manifest = std::env::var("CARGO_MANIFEST_DIR")
        .unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_string());
    PathBuf::from(manifest)
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let base_url = "https://www.gta5-mods.com";
    let url = format!("{}/vehicles", base_url);

    println!("Fetching {}", url);
    let res = client.get(&url).send()?.text()?;
    let document = Html::parse_document(&res);

    let obj_selector = Selector::parse(".file-list-obj").unwrap();
    let a_selector = Selector::parse("a").unwrap();
    let img_selector = Selector::parse("img").unwrap();
    let li_selector = Selector::parse("li").unwrap();

    let mut mods = Vec::new();
    let root = project_root();
    let thumb_dir = root.join("assets/thumbnails");
    fs::create_dir_all(&thumb_dir)?;

    for element in document.select(&obj_selector) {
        if let Some(a_elem) = element.select(&a_selector).next() {
            let href = a_elem.value().attr("href").unwrap_or("");
            let title = a_elem.value().attr("title").unwrap_or("");
            let full_url = format!("{}{}", base_url, href);

            let img_src = element
                .select(&img_selector)
                .next()
                .and_then(|img| img.value().attr("src"))
                .unwrap_or("")
                .to_string();

            let categories: Vec<String> = element
                .select(&li_selector)
                .map(|li| li.text().collect::<String>().trim().to_string())
                .filter(|s| !s.is_empty())
                .take(2)
                .collect();
            let category = if categories.is_empty() {
                "Script".to_string()
            } else if categories[0] == "Add-On" {
                categories.get(1).cloned().unwrap_or_else(|| categories[0].clone())
            } else {
                categories[0].clone()
            };

            let thumbnail_path = if img_src.is_empty() {
                None
            } else {
                let file_name = img_src.split('/').last().unwrap_or("thumb.jpg");
                let abs_path = thumb_dir.join(file_name);
                if !abs_path.exists() {
                    println!("Downloading thumbnail: {}", img_src);
                    if let Ok(mut resp) = client.get(&img_src).send() {
                        if let Ok(mut file) = fs::File::create(&abs_path) {
                            let _ = std::io::copy(&mut resp, &mut file);
                        }
                    }
                }
                Some(abs_path.to_string_lossy().to_string())
            };

            mods.push(ModEntry::new(
                title,
                &category,
                &full_url,
                thumbnail_path.as_deref(),
            ));
        }
    }

    let json_path = root.join("mods.json");
    let json_data = serde_json::to_string_pretty(&mods)?;
    let mut file = fs::File::create(&json_path)?;
    file.write_all(json_data.as_bytes())?;
    println!("Scraping complete. Saved {} mods to {}", mods.len(), json_path.display());
    Ok(())
}
