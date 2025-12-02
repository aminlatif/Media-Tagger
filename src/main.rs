mod rule;
mod rename_files;
mod scrape_data;

use std::fs;

use scrape_data::get_html_content;
use scrape_data::scrape_data;

use rename_files::rename_files;

use crate::rule::Rule;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    let rule_file_path = args.get(1).unwrap();

    let rule = parse_yaml(rule_file_path)?;

    if args.contains(&String::from("html")) || args.contains(&String::from("all")) {
        get_html_content(&rule).await?;
    }

    if args.contains(&String::from("scrape")) || args.contains(&String::from("all")) {
        scrape_data(&rule).await?;
    }

    if args.contains(&String::from("rename")) || args.contains(&String::from("all")) {
        rename_files(&rule)?;
    }

    Ok(())
}


fn parse_yaml(rule_file_path: &str) -> Result<Rule, Box<dyn std::error::Error>> {
    let yaml_str = fs::read_to_string(rule_file_path)?;

    let rules: Rule = serde_yaml::from_str(&yaml_str)?;

    println!("{:#?}", rules);

    Ok(rules)
}