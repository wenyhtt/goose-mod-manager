use scraper::{ElementRef, Html, Selector};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;

use crate::models::ModEntry;

fn project_root() -> PathBuf {
    let manifest = std::env::var("CARGO_MANIFEST_DIR")
        .unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_string());
    PathBuf::from(manifest)
}

pub fn cache_dir() -> PathBuf {
    let home = || std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE"));
    let base = if cfg!(target_os = "windows") {
        std::env::var_os("LOCALAPPDATA")
            .or_else(home)
            .map(PathBuf::from)
    } else if cfg!(target_os = "macos") {
        home().map(|path| PathBuf::from(path).join("Library/Caches"))
    } else {
        std::env::var_os("XDG_CACHE_HOME")
            .or_else(|| home().map(|path| PathBuf::from(path).join(".cache").into_os_string()))
            .map(PathBuf::from)
    };
    base.unwrap_or_else(project_root)
        .join("goose-mod-manager")
}

fn load_cached_mods() -> HashMap<String, ModEntry> {
    fs::read_to_string(cache_dir().join("mods.json"))
        .or_else(|_| fs::read_to_string(project_root().join("mods.json")))
        .ok()
        .and_then(|data| serde_json::from_str::<Vec<ModEntry>>(&data).ok())
        .unwrap_or_default()
        .into_iter()
        .map(|mod_entry| (mod_entry.url.clone(), mod_entry))
        .collect()
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mods = run_page(1)?;
    save_mods(&mods)
}

pub fn run_page(page: usize) -> Result<Vec<ModEntry>, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;
    let base_url = "https://www.gta5-mods.com";
    let url = if page == 1 {
        format!("{}/vehicles", base_url)
    } else {
        format!("{}/vehicles/{}", base_url, page)
    };

    println!("Fetching {}", url);
    let res = client.get(&url).send()?.text()?;
    let document = Html::parse_document(&res);

    let obj_selector = Selector::parse(".file-list-obj").unwrap();
    let a_selector = Selector::parse("a").unwrap();
    let img_selector = Selector::parse("img").unwrap();
    let li_selector = Selector::parse("li").unwrap();
    let author_selector = Selector::parse("a[href*='/users/'], a[href*='/user/']").unwrap();
    let version_selector = Selector::parse("[class*='version'], [class*='Version']").unwrap();
    let downloads_selector = Selector::parse("[class*='download'], [class*='Download']").unwrap();

    let mut mods = Vec::new();
    let cached_mods = load_cached_mods();
    let thumb_dir = cache_dir().join("thumbnails");
    fs::create_dir_all(&thumb_dir)?;

    for element in document.select(&obj_selector) {
        if let Some(a_elem) = element.select(&a_selector).next() {
            let href = a_elem.value().attr("href").unwrap_or("");
            let title = a_elem.value().attr("title").unwrap_or("");
            let full_url = format!("{}{}", base_url, href);

            if let Some(cached) = cached_mods.get(&full_url) {
                if cached
                    .thumbnail_path
                    .as_ref()
                    .map(|path| fs::metadata(path).map(|m| m.len() > 0).unwrap_or(false))
                    .unwrap_or(false)
                    && !cached.author.is_empty()
                    && !cached.version.is_empty()
                    && !cached.downloads.is_empty()
                {
                    mods.push(cached.clone());
                    continue;
                }
            }

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
                .take(3)
                .collect();
            let category = categories
                .into_iter()
                .filter(|category| category != "Add-On")
                .collect::<Vec<_>>();
            let category = if category.is_empty() {
                "Script".to_string()
            } else {
                category.join(", ")
            };

            let author = element
                .select(&author_selector)
                .map(element_text)
                .find(|text| !text.is_empty())
                .unwrap_or_default();
            let version = element
                .select(&version_selector)
                .map(element_text)
                .find(|text| !text.is_empty())
                .unwrap_or_default();
            let downloads = extract_downloads(element, title, &downloads_selector);

            let thumbnail_path = if img_src.is_empty() {
                cached_mods
                    .get(&full_url)
                    .and_then(|mod_entry| mod_entry.thumbnail_path.clone())
            } else {
                let file_name = img_src.split('/').last().unwrap_or("thumb.jpg");
                let abs_path = thumb_dir.join(file_name);
                if !abs_path.exists() || fs::metadata(&abs_path).map(|m| m.len() == 0).unwrap_or(true) {
                    println!("Downloading thumbnail: {}", img_src);
                    if let Ok(bytes) = client
                        .get(&img_src)
                        .send()
                        .and_then(|response| response.error_for_status())
                        .and_then(|response| response.bytes())
                    {
                        if !bytes.is_empty() {
                            fs::write(&abs_path, &bytes)?;
                        }
                    }
                }
                Some(abs_path.to_string_lossy().to_string())
            };

            let mut mod_entry = cached_mods
                .get(&full_url)
                .cloned()
                .unwrap_or_else(|| ModEntry::new(title, &category, &full_url, "", "", "", None));
            if !author.is_empty() {
                mod_entry.author = author;
            }
            if !version.is_empty() {
                mod_entry.version = version;
            }
            if !downloads.is_empty() {
                mod_entry.downloads = downloads;
            }
            mod_entry.thumbnail_path = thumbnail_path;
            mods.push(mod_entry);
        }
    }

    Ok(mods)
}

fn element_text(element: ElementRef<'_>) -> String {
    element.text().collect::<String>().trim().to_string()
}

fn extract_downloads(
    element: ElementRef<'_>,
    title: &str,
    selector: &Selector,
) -> String {
    if let Some(downloads) = element
        .select(selector)
        .map(element_text)
        .find(|text| is_count(text))
    {
        return downloads;
    }

    let text = element
        .text()
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    let before_title = text.split(title).next().unwrap_or(&text);

    before_title
        .split_whitespace()
        .find(|token| is_count(token))
        .unwrap_or_default()
        .to_string()
}

fn is_count(value: &str) -> bool {
    let value = value.trim_matches(|character: char| !character.is_ascii_digit() && character != ',');
    !value.is_empty()
        && value.chars().any(|character| character.is_ascii_digit())
        && value
            .chars()
            .all(|character| character.is_ascii_digit() || character == ',')
}

pub fn save_mods(mods: &[ModEntry]) -> Result<(), Box<dyn std::error::Error>> {
    let cache = cache_dir();
    fs::create_dir_all(&cache)?;
    let json_path = cache.join("mods.json");
    let json_data = serde_json::to_string_pretty(&mods)?;
    let mut file = fs::File::create(&json_path)?;
    file.write_all(json_data.as_bytes())?;
    println!("Scraping complete. Saved {} mods to {}", mods.len(), json_path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_author_and_version_from_listing() {
        let document = Html::parse_fragment(
            r#"<div class="file-list-obj"><a href="/users/tester">Tester</a><span class="file-version">V2.1</span><span class="downloads">226</span></div>"#,
        );
        let item = document
            .select(&Selector::parse(".file-list-obj").unwrap())
            .next()
            .unwrap();
        let author_selector = Selector::parse("a[href*='/users/'], a[href*='/user/']").unwrap();
        let version_selector = Selector::parse("[class*='version'], [class*='Version']").unwrap();

        assert_eq!(
            item.select(&author_selector).next().map(element_text),
            Some("Tester".to_string())
        );
        assert_eq!(
            item.select(&version_selector).next().map(element_text),
            Some("V2.1".to_string())
        );
        let downloads_selector = Selector::parse("[class*='download'], [class*='Download']").unwrap();
        assert_eq!(
            item.select(&downloads_selector).next().map(element_text),
            Some("226".to_string())
        );
        assert_eq!(extract_downloads(item, "Test Mod", &downloads_selector), "226");
    }
}
