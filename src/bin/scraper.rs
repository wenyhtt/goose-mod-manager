#[path = "../models.rs"]
mod models;
#[path = "../scraper.rs"]
mod scraper;

fn main() {
    if let Err(error) = scraper::run() {
        eprintln!("Scraping failed: {error}");
        std::process::exit(1);
    }
}
