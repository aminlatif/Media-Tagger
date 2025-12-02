use std::{collections::HashMap, fs};
use once_cell::sync::Lazy;
use regex::Regex;

static TOKEN_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\{\{(.+?)\}\}").unwrap());
static TOKEN_DECODE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\w)(\d+)\.(.+)").unwrap());
static TOKEN_DECODE_FORMAT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\w)(\d*)").unwrap());
static TOKEN_DECODE_FORMAT_CLEAN_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"[<>:"/\\|?\*\x00-\x1F]"#).unwrap());
static TOKEN_DECODE_FORMAT_TITLE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"[ ',\.\-_%]"#).unwrap());

pub fn rename_files(target_directory: &str, file_name_template: &str, file_name_check_template: &str) -> Result<(), Box<dyn std::error::Error>> {
    let csv_content = fs::read_to_string(target_directory.to_string() + "/.tagger/guide.csv")?;

    let mut rdr = csv::Reader::from_reader(csv_content.as_bytes());

    for result in rdr.records() {
        let record = result?;
        let fields: Vec<String> = record.iter().map(|s| s.to_string()).collect();
        let target_file_name =
            replace_file_name_template_tokens(String::from(file_name_template), fields.clone());
        let source_file_name_check = replace_file_name_template_tokens(
            String::from(file_name_check_template),
            fields.clone(),
        );
        println!(
            "target_file_name: {:?} from source_file_name_check: {:?}",
            target_file_name, source_file_name_check
        );

        let season_directory_name =
            replace_file_name_template_tokens(String::from("Season{{i1.p2}}"), fields);
        let season_directory_path = target_directory.to_string() + "\\" + &season_directory_name;
        if !std::path::Path::new(&season_directory_path).exists() {
            println!(
                "\x1b[31mSeason directory path \"{}\" does not exist.\x1b[0m",
                season_directory_path
            );
            continue;
        }

        println!("Searching in \"{}\"...", season_directory_name);

        let matches = find_file_by_partial_name(&season_directory_path, source_file_name_check);
        if matches.is_empty() {
            println!("\x1b[31mNo files found.\x1b[0m");
            continue;
        } else {
            println!("found files: ");

            if matches.len() > 1 {
                println!("\x1b[33mMultiple files found.\x1b[0m");
                continue;
            }

            for path in matches.clone() {
                println!("{}", path.display());
                // let original_file_name = path.file_name().unwrap().to_str().unwrap();
                let original_file_extension = path.extension().unwrap().to_str().unwrap();
                let original_file_without_extension = path.file_stem().unwrap().to_str().unwrap();
                println!("\x1b[34mOriginal file name: {} ({})\x1b[0m", original_file_without_extension, original_file_extension);
                let target_file_path = season_directory_path.to_string() + "\\" + &target_file_name + "." + original_file_extension;
                println!("\x1b[34mTaget file path: {}\x1b[0m", target_file_path);
                fs::rename(path.clone(), season_directory_path.to_string() + "\\" + &target_file_name + "." + original_file_extension)?;
            }
        }
    }

    Ok(())
}


fn find_file_by_partial_name(root: &str, partial_name: String) -> Vec<std::path::PathBuf> {
    walkdir::WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            e.file_name()
                .to_str()
                .map(|name| name.to_lowercase().contains(&partial_name.to_lowercase()))
                .unwrap_or(false)
        })
        .map(|e| e.into_path())
        .collect()
}

fn replace_file_name_template_tokens(template: String, record: Vec<String>) -> String {
    let mut file_name = template.clone();
    let mut tokens = Vec::new();

    for token in TOKEN_REGEX.captures_iter(&template) {
        let token_string = token[1].to_string();
        tokens.push(token_string);
    }

    for token in tokens {
        let value = get_record_value_for_token(token.clone(), record.clone());
        // println!("{token} -> {value}");
        file_name = file_name.replace(("{{".to_owned() + &token + "}}").as_str(), &value);
    }

    return file_name;
}

fn get_record_value_for_token(token: String, record: Vec<String>) -> String {
    let mut token_type = String::from("i");
    let mut token_index = 0;
    let mut token_format = String::from("p2");
    let mut token_format_hashmap: HashMap<String, u8> = HashMap::new();

    let capture = TOKEN_DECODE_REGEX.captures(&token).unwrap();
    token_type = capture[1].to_string();
    token_index = capture[2].parse().unwrap();
    token_format = capture[3].to_string();

    for format_capture in TOKEN_DECODE_FORMAT_REGEX.captures_iter(&token_format) {
        let format_type: String = format_capture[1].to_string();
        let format_value: u8 = format_capture[2].parse().unwrap_or(0);
        token_format_hashmap.insert(format_type, format_value);
    }

    match token_type.as_str() {
        "i" => {
            let mut number_padding: usize = 0;

            if token_format_hashmap.contains_key("p") {
                number_padding = token_format_hashmap["p"] as usize;
            }

            return format!(
                "{:0>width$}",
                record[token_index].clone(),
                width = number_padding
            );
        }
        "s" => {
            let mut string_value = record[token_index].clone();
            // println!("{:?}", string_value);
            if token_format_hashmap.contains_key("c") {
                string_value = TOKEN_DECODE_FORMAT_CLEAN_REGEX
                    .replace_all(&string_value, "")
                    .to_string();
            }
            if token_format_hashmap.contains_key("t") {
                string_value = to_pascal_case(&string_value);
                string_value = TOKEN_DECODE_FORMAT_TITLE_REGEX
                    .replace_all(&string_value, "")
                    .to_string();
            }
            return string_value;
        }
        _ => {}
    }

    return token;
}

fn to_pascal_case(input: &str) -> String {
    input
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => {
                    first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                }
                None => String::new(),
            }
        })
        .collect()
}
