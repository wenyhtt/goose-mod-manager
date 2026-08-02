use reqwest::blocking::Client;
use scraper::{Html, Selector};
use serde_json;
use std::fs;
use std::io::Write;
use std::path::Path;

#[path = "../models.rs"]
pub mod models;
use models::ModEntry;

fn main() {
    let client = Client::new();
    let base_url = "https://www.gta5-mods.com";
    let url = format!("{}/vehicles", base_url);

    println!("Fetching {}", url);
    let res = client
        .get(&url)
        .send()
        .expect("Failed to fetch")
        .text()
        .expect("Failed to read text");
    let document = Html::parse_document(&res);

    let obj_selector = Selector::parse(".file-list-obj").unwrap();
    let a_selector = Selector::parse("a").unwrap();
    let img_selector = Selector::parse("img").unwrap();
    let li_selector = Selector::parse("li").unwrap();

    let mut mods = Vec::new();

    fs::create_dir_all("assets/thumbnails").expect("Failed to create thumbnails directory");

    for element in document.select(&obj_selector) {
        if let Some(a_elem) = element.select(&a_selector).next() {
            let href = a_elem.value().attr("href").unwrap_or("");
            let title = a_elem.value().attr("title").unwrap_or("");

            let full_url = format!("{}{}", base_url, href);

            let img_src = if let Some(img_elem) = element.select(&img_selector).next() {
                img_elem.value().attr("src").unwrap_or("").to_string()
            } else {
                String::new()
            };

            let categories = element
                .select(&li_selector)
                .map(|li| li.text().collect::<String>())
                .collect::<Vec<_>>()
                .join(", ");
            let category = if categories.is_empty() {
                "Script".to_string()
            } else {
                categories
            };

            // Download thumbnail
            let mut thumbnail_path = None;
            if !img_src.is_empty() {
                let file_name = img_src.split('/').last().unwrap_or("thumb.jpg");
                let local_path = format!("assets/thumbnails/{}", file_name);
                let path = Path::new(&local_path);

                if !path.exists() {
                    println!("Downloading thumbnail: {}", img_src);
                    if let Ok(mut resp) = client.get(&img_src).send() {
                        if let Ok(mut file) = fs::File::create(&local_path) {
                            let _ = std::io::copy(&mut resp, &mut file);
                        }
                    }
                }
                thumbnail_path = Some(local_path);
            }

            let mod_entry = ModEntry::new(
                title,
                &category,
                "Unknown Size",
                &full_url,
                thumbnail_path.as_deref(),
            );
            mods.push(mod_entry);
        }
    }

    let json_data = serde_json::to_string_pretty(&mods).expect("Failed to serialize");
    let mut file = fs::File::create("mods.json").expect("Failed to create mods.json");
    file.write_all(json_data.as_bytes())
        .expect("Failed to write to mods.json");

    println!("Scraping complete. Saved {} mods to mods.json", mods.len());
}
