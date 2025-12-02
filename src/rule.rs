use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub target_directory: String,
    pub scrape_url: String,
    pub season_selector_query: String,
    pub season_selector_skip: u32,
    pub episode_selector_query: String,
    pub episode_selector_skip: u32,
    pub episode_field_selectors: Vec<FieldSelectors>,
    pub file_name_template: String,
    pub file_name_check_template: String,
    pub has_season_directory: bool,
    pub season_directory_template: String,
    pub dry_run: bool
}

#[derive(Debug, Deserialize)]
pub struct FieldSelectors {
    pub title: String,
    pub selector_query: String
}