use reqwest::Client;
use std::fs;

use crate::rule::Rule;

pub async fn scrape_data(
    rule: &Rule
    // target_directory: &str,
    // season_selector_query: &str,
    // season_selector_skip_init: i32,
    // episode_selector_query: &str,
    // episode_selector_skip_init: i32,
    // episode_field_selectors: Vec<Vec<String>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let tagger_directory_path = rule.target_directory.to_string() + "/.tagger";

    let guide_html_file_path = tagger_directory_path.clone() + "/guide.html";

    if !std::path::Path::new(&guide_html_file_path).exists() {
        panic!("guide.html file does not exist.");
    }

    let guide_html_content = fs::read_to_string(guide_html_file_path).unwrap();

    let document = scraper::Html::parse_document(&guide_html_content);

    let mut csv_content = String::new();

    csv_content.push_str("#,season,episode");

    for field in rule.episode_field_selectors.iter() {
        csv_content.push_str(",");
        csv_content.push_str(&field.title.to_string().as_str());
    }
    csv_content.push_str("\n");

    let season_selector = scraper::Selector::parse(rule.season_selector_query.as_str()).unwrap();
    let mut season_selector_skip = rule.season_selector_skip;

    let mut season = 1;
    let mut episode_cul = 1;

    for season_element in document.select(&season_selector) {
        if season_selector_skip > 0 {
            season_selector_skip = season_selector_skip - 1;
            continue;
        }

        let episode_selector = scraper::Selector::parse(rule.episode_selector_query.as_str()).unwrap();
        let mut episode_selector_skip = rule.episode_selector_skip;

        let mut episode = 1;

        for episode_element in season_element.select(&episode_selector) {
            if episode_selector_skip > 0 {
                episode_selector_skip = episode_selector_skip - 1;
                continue;
            }

            csv_content.push_str((episode_cul.to_string() + ",").as_str());
            csv_content.push_str((season.to_string() + ",").as_str());
            csv_content.push_str((episode.to_string()).as_str());

            for field in rule.episode_field_selectors.iter() {
                csv_content.push_str(",");
                let field_element = episode_element
                    .select(&scraper::Selector::parse(&field.selector_query.as_str()).unwrap())
                    .next()
                    .unwrap();
                let field_value = field_element.text().collect::<String>();
                let field_value = refine_field_value(field_value);
                csv_content.push_str(field_value.as_str());
            }
            csv_content.push_str("\n");

            episode_cul = episode_cul + 1;
            episode = episode + 1;
        }

        season = season + 1;
    }

    fs::write(
        tagger_directory_path.to_string() + "/guide.csv",
        &csv_content,
    )?;

    Ok(())
}

pub async fn get_html_content(
    rule: &Rule
    // target_directory: &str,
    // scrape_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .user_agent("MediaTgger/1.0 (contact: amin.latifkar@gmail.com)")
        .build()?;

    let response_text = client.get(rule.scrape_url.clone()).send().await?.text().await?;

    let tagger_directory_path = rule.target_directory.to_string() + "/.tagger";

    if !std::path::Path::new(&tagger_directory_path).exists() {
        fs::create_dir(&tagger_directory_path).unwrap();
    }

    fs::write(tagger_directory_path + "/guide.html", &response_text)?;

    Ok(())
}

pub fn refine_field_value(field_value: String) -> String {
    let field_value_array: Vec<String> = field_value.split("<br").map(|s| s.to_string()).collect();
    let field_value = field_value_array[0].clone();
    let field_value = field_value
        .trim()
        .replace("\n", " ")
        .replace(",", " ")
        .replace("\"", "")
        .replace("\'", "")
        .replace("  ", " ");
    return field_value;
}
