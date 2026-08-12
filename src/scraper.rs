use scraper::{ElementRef, Html, Selector};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;

use crate::models::{ModEntry, ModVersion};

pub struct ModDetails {
    pub likes: String,
    pub downloads: String,
    pub image_paths: Vec<String>,
}

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
    base.unwrap_or_else(project_root).join("goose-mod-manager")
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
                if !abs_path.exists()
                    || fs::metadata(&abs_path)
                        .map(|m| m.len() == 0)
                        .unwrap_or(true)
                {
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

pub fn run_details(url: &str) -> Result<ModDetails, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;
    let html = client.get(url).send()?.error_for_status()?.text()?;
    let document = Html::parse_document(&html);
    let detail = extract_details(&document);
    let slug = url
        .trim_end_matches('/')
        .rsplit('/')
        .next()
        .unwrap_or("mod")
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
        .collect::<String>();
    let image_dir = cache_dir().join("detail-images");
    fs::create_dir_all(&image_dir)?;
    let mut image_paths = Vec::new();

    for image_url in detail.image_urls {
        let file_name = image_url
            .split('?')
            .next()
            .unwrap_or(&image_url)
            .rsplit('/')
            .next()
            .unwrap_or("detail.jpg");
        let path = image_dir.join(format!("{slug}-{file_name}"));
        if !path.exists() || fs::metadata(&path).map(|m| m.len() == 0).unwrap_or(true) {
            if let Ok(bytes) = client
                .get(&image_url)
                .send()
                .and_then(|response| response.error_for_status())
                .and_then(|response| response.bytes())
            {
                if !bytes.is_empty() {
                    fs::write(&path, &bytes)?;
                }
            }
        }
        if path.exists() {
            image_paths.push(path.to_string_lossy().to_string());
        }
    }

    Ok(ModDetails {
        likes: detail.likes,
        downloads: detail.downloads,
        image_paths,
    })
}

pub fn fetch_versions(url: &str) -> Result<Vec<ModVersion>, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(15))
        .build()?;
    let html = client.get(url).send()?.error_for_status()?.text()?;
    Ok(extract_versions(&Html::parse_document(&html)))
}

pub fn download_version(version: &ModVersion) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(60))
        .build()?;
    let response = client
        .get(&version.download_url)
        .send()?
        .error_for_status()?;
    let filename = response
        .headers()
        .get(reqwest::header::CONTENT_DISPOSITION)
        .and_then(|value| value.to_str().ok())
        .and_then(filename_from_content_disposition)
        .unwrap_or_else(|| fallback_archive_name(&version.label));
    let downloads = cache_dir().join("downloads");
    fs::create_dir_all(&downloads)?;
    let path = downloads.join(filename);
    let temporary = path.with_extension("part");
    let bytes = response.bytes()?;
    if bytes.is_empty() {
        return Err("Downloaded archive was empty".into());
    }
    fs::write(&temporary, &bytes)?;
    fs::rename(&temporary, &path)?;
    Ok(path)
}

struct ExtractedDetails {
    likes: String,
    downloads: String,
    image_urls: Vec<String>,
}

fn extract_details(document: &Html) -> ExtractedDetails {
    let image_selector = Selector::parse("a.thumbnail.mfp-image[href]").unwrap();
    let likes_selector = Selector::parse(".num-likes").unwrap();
    let downloads_selector =
        Selector::parse(".file-downloads .num-downloads, .num-downloads").unwrap();

    ExtractedDetails {
        likes: document
            .select(&likes_selector)
            .map(element_text)
            .find(|text| is_count(text))
            .unwrap_or_default(),
        downloads: document
            .select(&downloads_selector)
            .map(element_text)
            .filter_map(|text| first_count(&text))
            .next()
            .unwrap_or_default(),
        image_urls: document
            .select(&image_selector)
            .filter_map(|a| a.value().attr("href"))
            .map(absolute_url)
            .collect(),
    }
}

fn extract_versions(document: &Html) -> Vec<ModVersion> {
    let container_selector = Selector::parse(".file-version-container").unwrap();
    let link_selector = Selector::parse("a[href*='/download/']").unwrap();
    let downloads_selector = Selector::parse(".num-downloads").unwrap();
    let size_selector = Selector::parse(".file-size").unwrap();
    let date_selector = Selector::parse(".file-date").unwrap();

    let mut seen = HashSet::new();

    document
        .select(&container_selector)
        .filter_map(|entry| {
            let download_url = entry
                .select(&link_selector)
                .find_map(|link| link.value().attr("href"))
                .map(absolute_url)?;
            if !seen.insert(download_url.clone()) {
                return None;
            }
            let label = extract_version_label(entry)?;
            let text = element_text(entry);
            Some(ModVersion {
                label,
                size: entry
                    .select(&size_selector)
                    .next()
                    .map(element_text)
                    .and_then(clean_size)
                    .or_else(|| extract_size(&text))
                    .unwrap_or_default(),
                downloads: entry
                    .select(&downloads_selector)
                    .next()
                    .map(element_text)
                    .and_then(|text| first_count(&text))
                    .or_else(|| first_count(&text))
                    .unwrap_or_default(),
                published_at: entry
                    .select(&date_selector)
                    .next()
                    .map(element_text)
                    .or_else(|| extract_date(&text))
                    .unwrap_or_default(),
                download_url,
            })
        })
        .collect()
}

fn clean_size(text: String) -> Option<String> {
    let text = text.trim().trim_start_matches(',').trim();
    (!text.is_empty()).then(|| text.to_string())
}

fn extract_version_label(entry: ElementRef<'_>) -> Option<String> {
    let mut label = String::new();
    for text in entry
        .text()
        .map(|text| text.replace('\u{a0}', " "))
        .map(|text| text.trim().to_string())
        .filter(|text| !text.is_empty())
    {
        if label.is_empty() {
            label = text;
        } else if text.starts_with('(') {
            label.push(' ');
            label.push_str(&text);
        } else {
            break;
        }
    }
    (!label.is_empty()).then_some(label)
}

fn extract_size(text: &str) -> Option<String> {
    text.split_whitespace()
        .collect::<Vec<_>>()
        .windows(2)
        .find(|parts| matches!(parts[1], "B" | "KB" | "MB" | "GB"))
        .map(|parts| format!("{} {}", parts[0], parts[1]))
}

fn extract_date(text: &str) -> Option<String> {
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let words = text.split_whitespace().collect::<Vec<_>>();
    words
        .iter()
        .position(|word| months.contains(word))
        .and_then(|index| {
            words
                .get(index..index + 3)
                .map(|date| date.join(" ").trim_end_matches(',').to_string())
        })
}

fn filename_from_content_disposition(value: &str) -> Option<String> {
    value.split(';').find_map(|part| {
        part.trim()
            .strip_prefix("filename=")
            .map(|name| safe_filename(name.trim_matches('"')))
            .filter(|name| !name.is_empty())
    })
}

fn fallback_archive_name(label: &str) -> String {
    let label = safe_filename(label);
    format!("{}.zip", if label.is_empty() { "mod" } else { &label })
}

fn safe_filename(value: &str) -> String {
    value
        .chars()
        .filter(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '.' | '-' | '_')
        })
        .collect()
}

fn absolute_url(url: &str) -> String {
    if url.starts_with("http") {
        url.to_string()
    } else if url.starts_with("//") {
        format!("https:{url}")
    } else {
        format!("https://www.gta5-mods.com{url}")
    }
}

fn element_text(element: ElementRef<'_>) -> String {
    element.text().collect::<String>().trim().to_string()
}

fn extract_downloads(element: ElementRef<'_>, title: &str, selector: &Selector) -> String {
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
    let value =
        value.trim_matches(|character: char| !character.is_ascii_digit() && character != ',');
    !value.is_empty()
        && value.chars().any(|character| character.is_ascii_digit())
        && value
            .chars()
            .all(|character| character.is_ascii_digit() || character == ',')
}

fn first_count(value: &str) -> Option<String> {
    value
        .split_whitespace()
        .map(|token| token.trim_matches(|c: char| !c.is_ascii_digit() && c != ','))
        .find(|token| is_count(token))
        .map(str::to_string)
}

pub fn save_mods(mods: &[ModEntry]) -> Result<(), Box<dyn std::error::Error>> {
    let cache = cache_dir();
    fs::create_dir_all(&cache)?;
    let json_path = cache.join("mods.json");
    let json_data = serde_json::to_string_pretty(&mods)?;
    let mut file = fs::File::create(&json_path)?;
    file.write_all(json_data.as_bytes())?;
    println!(
        "Scraping complete. Saved {} mods to {}",
        mods.len(),
        json_path.display()
    );
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
        let downloads_selector =
            Selector::parse("[class*='download'], [class*='Download']").unwrap();
        assert_eq!(
            item.select(&downloads_selector).next().map(element_text),
            Some("226".to_string())
        );
        assert_eq!(
            extract_downloads(item, "Test Mod", &downloads_selector),
            "226"
        );
    }

    #[test]
    fn extracts_detail_images_and_likes() {
        let document = Html::parse_fragment(
            r#"
            <a class="thumbnail mfp-image" href="https://img.gta5-mods.com/q95/images/mod/one.png"></a>
            <a class="thumbnail mfp-image" href="//img.gta5-mods.com/q95/images/mod/two.png"></a>
            <div class="file-downloads"><span class="num-downloads">3,864</span></div>
            <span class="num-likes">53</span>
            "#,
        );

        let details = extract_details(&document);

        assert_eq!(details.likes, "53");
        assert_eq!(details.downloads, "3,864");
        assert_eq!(
            details.image_urls,
            vec![
                "https://img.gta5-mods.com/q95/images/mod/one.png",
                "https://img.gta5-mods.com/q95/images/mod/two.png"
            ]
        );
    }

    #[test]
    fn extracts_all_versions_with_download_links() {
        let document = Html::parse_fragment(
            r#"
            <div class="well pull-left file-version-container">
              <div class="pull-left">
                <i class="fa fa-file"></i>&nbsp;1.0.13 <span>(current)</span>
                <p>
                  <span class="num-downloads">1,017,488 downloads <span class="file-size">, 9.14 MB</span></span>
                </p>
                <p>June 30, 2020</p>
              </div>
              <div class="pull-right">
                <a target="_blank" href="https://www.virustotal.com/example"><i class="fa fa-shield vt-version"></i></a>
                <a target="_blank" href="/tools/gta-v-launcher/download/94658"><i class="fa fa-download download-version"></i></a>
              </div>
            </div>
            <div class="well pull-left file-version-container">
              <div class="pull-left">
                <i class="fa fa-file"></i>&nbsp;1.0.13 <span>(current)</span>
                <p>
                  <span class="num-downloads">1,017,488 downloads <span class="file-size">, 9.14 MB</span></span>
                </p>
                <p>June 30, 2020</p>
              </div>
              <div class="pull-right">
                <a target="_blank" href="/tools/gta-v-launcher/download/94658"><i class="fa fa-download download-version"></i></a>
              </div>
            </div>
            "#,
        );

        assert_eq!(
            extract_versions(&document),
            vec![ModVersion {
                label: "1.0.13 (current)".to_string(),
                size: "9.14 MB".to_string(),
                downloads: "1,017,488".to_string(),
                published_at: "June 30, 2020".to_string(),
                download_url: "https://www.gta5-mods.com/tools/gta-v-launcher/download/94658"
                    .to_string(),
            }]
        );
    }

    #[test]
    fn archive_filenames_are_safe() {
        assert_eq!(
            filename_from_content_disposition("attachment; filename=mod.zip"),
            Some("mod.zip".to_string())
        );
        assert_eq!(fallback_archive_name("v2.0 / test"), "v2.0test.zip");
    }
}
