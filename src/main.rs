mod rename_files;
mod scrape_data;

use scrape_data::get_html_content;
use scrape_data::scrape_data;

use rename_files::rename_files;

// const TARGET_DIRECTORY: &str = "F:\\Videos\\Series\\Futurama";
const TARGET_DIRECTORY: &str = "F:/Videos/Series/Dororo";

// const SCRAPE_URL : &str = "https://theinfosphere.org/Episode_Listing_(broadcast_order)";
const SCRAPE_URL: &str = "https://en.wikipedia.org/wiki/List_of_Dororo_(2019_TV_series)_episodes";

// const SEASON_SELECTOR_QUERY: &str = "table.overview";
const SEASON_SELECTOR_QUERY: &str = "table.wikiepisodetable";
const SEASON_SELECTOR_SKIP: i32 = 0;

// const EPISODE_SELECTOR_QUERY: &str = "tr.oCentre";
const EPISODE_SELECTOR_QUERY: &str = "tr.module-episode-list-row";
const EPISODE_SELECTOR_SKIP: i32 = 0;

const EPISODE_FIELD_SELECTORS: [[&str; 2]; 2] = [["title", "td.summary"], ["#", "th"]];

// const FILE_NAME_TEMPLATE: &str = "Futurama-S{{i1.p2}}E{{i2.p2}}-{{s3.ct}}";
const FILE_NAME_TEMPLATE: &str = "Dororo-{{i2.p2}}-{{s3.ct}}";

// const FILE_NAME_CHECK_TEMPLATE: &str = "Futurama_S{{i1.p2}}E{{i2.p2}}";
const FILE_NAME_CHECK_TEMPLATE: &str = "Dororo - {{i2.p2}}";

// const HAS_SEASON_DIRECTORY: bool = true;
const HAS_SEASON_DIRECTORY: bool = false;

const SEASON_DIRECTORY_TEMPLATE: &str = "Season{{i1.p2}}";

const DRY_RUN: bool = false;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.contains(&String::from("html")) {
        get_html_content(TARGET_DIRECTORY, SCRAPE_URL).await?;
    }

    if args.contains(&String::from("scrape")) {
        let episode_field_selectors: Vec<Vec<String>> = EPISODE_FIELD_SELECTORS
            .iter()
            .map(|row| row.iter().map(|s| s.to_string()).collect())
            .collect();
        scrape_data(
            TARGET_DIRECTORY,
            SEASON_SELECTOR_QUERY,
            SEASON_SELECTOR_SKIP,
            EPISODE_SELECTOR_QUERY,
            EPISODE_SELECTOR_SKIP,
            episode_field_selectors,
        )
        .await?;
    }

    if args.contains(&String::from("rename")) {
        rename_files(
            TARGET_DIRECTORY,
            FILE_NAME_TEMPLATE,
            FILE_NAME_CHECK_TEMPLATE,
            HAS_SEASON_DIRECTORY,
            SEASON_DIRECTORY_TEMPLATE,
            DRY_RUN
        )?;
    }

    Ok(())
}
