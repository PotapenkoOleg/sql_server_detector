mod clap_parser;
mod sql_server_keywords;

use crate::clap_parser::Args;
use crate::sql_server_keywords::agg_func::get_sql_server_agg_func;
use crate::sql_server_keywords::analytic_func::get_sql_server_analytic_func;
use crate::sql_server_keywords::bit_manipulation_func::get_sql_server_bit_manipulation_func;
use crate::sql_server_keywords::collation_func::get_sql_collation_func;
use crate::sql_server_keywords::configuration_func::get_sql_configuration_func;
use crate::sql_server_keywords::conversion_func::get_sql_server_conversion_func;
use crate::sql_server_keywords::crypto_func::get_sql_server_crypto_func;
use crate::sql_server_keywords::cursor_func::get_sql_server_cursor_func;
use crate::sql_server_keywords::data_type_func::get_sql_server_data_type_func;
use crate::sql_server_keywords::date_time_func::get_sql_server_date_time_func;
use crate::sql_server_keywords::fuzzy_string_match_func::get_sql_server_fuzzy_string_match_func;
use crate::sql_server_keywords::graph_func::get_sql_server_graph_func;
use crate::sql_server_keywords::json_func::get_sql_server_json_func;
use crate::sql_server_keywords::keywords::get_sql_server_keywords;
use crate::sql_server_keywords::logical_func::get_sql_server_logical_func;
use crate::sql_server_keywords::math_func::get_sql_server_math_func;
use crate::sql_server_keywords::metadata_func::get_sql_server_metadata_func;
use crate::sql_server_keywords::ranking_func::get_sql_server_ranking_func;
use crate::sql_server_keywords::regex_func::get_sql_server_regex_func;
use crate::sql_server_keywords::replication_func::get_sql_server_replication_func;
use crate::sql_server_keywords::security_func::get_sql_server_security_func;
use crate::sql_server_keywords::string_func::get_sql_server_string_func;
use crate::sql_server_keywords::system_func::get_sql_server_system_func;
use crate::sql_server_keywords::system_statistical_func::get_sql_server_system_statistical_func;
use crate::sql_server_keywords::text_image_func::get_sql_server_text_image_func;
use crate::sql_server_keywords::trigger_func::get_sql_server_trigger_func;
use crate::sql_server_keywords::vector_func::get_sql_server_vector_func;
use anyhow::Context;
use clap::Parser;
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use tokio::io;

use tokio::task::JoinSet;
use tokio::time::Instant;

async fn ensure_directory_exists_and_empty(dir: &PathBuf) -> anyhow::Result<()> {
    if !dir.exists() {
        fs::create_dir_all(&dir)
            .await
            .with_context(|| "Failed to create directory".to_string())?;
    } else {
        let mut entries = fs::read_dir(dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_file() {
                fs::remove_file(path).await?;
            }
        }
    }
    Ok(())
}

async fn list_files(dir: &PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut entries = fs::read_dir(dir).await?;
    let mut files = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.is_file() {
            files.push(path);
        }
    }
    Ok(files)
}

async fn read_file(file: &PathBuf) -> anyhow::Result<String> {
    let content = tokio::fs::read_to_string(file)
        .await
        .with_context(|| format!("Failed to read file: {}", file.to_str().unwrap()))?;
    Ok(content)
}

async fn process_file_content(
    file: &PathBuf,
    content: &str,
    all_keywords: Arc<HashSet<String>>,
) -> anyhow::Result<()> {
    let file_name = file.file_name().unwrap().to_str().unwrap();
    let invalid_lines: Vec<(usize, String)> = content
        .lines()
        .map(|line| line.to_uppercase())
        .enumerate()
        .filter(|(line_number, line)| {
            check_line_for_sql_server_keywords(file_name, line_number, line, all_keywords.clone())
        })
        .collect();

    if !invalid_lines.is_empty() {
        write_lines_to_file(file, &invalid_lines).await?;
    }
    Ok(())
}

fn check_line_for_sql_server_keywords(
    file_name: &str,
    line_number: &usize,
    line: &str,
    all_keywords: Arc<HashSet<String>>,
) -> bool {
    for keyword in all_keywords.iter() {
        if line.contains(keyword) {
            println!(
                "{} | {} | {} | {}",
                file_name,
                line_number,
                keyword,
                line.replace("|", "!").trim()
            );
            return true;
        }
    }

    false
}

async fn write_lines_to_file(file: &PathBuf, lines: &Vec<(usize, String)>) -> anyhow::Result<()> {
    let content = lines
        .iter()
        .map(|(line_num, line)| format!("{}: {}", line_num, line.trim()))
        .collect::<Vec<String>>()
        .join("\n");

    fs::write(file, content)
        .await
        .with_context(|| format!("Failed to write file: {}", file.to_str().unwrap()))?;
    Ok(())
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let now = Instant::now();
    let args = Args::parse();
    let input_dir_name = &args.input_directory;
    let input_dir = PathBuf::from(input_dir_name);
    let mut output_dir = PathBuf::from(input_dir_name);
    output_dir.push("OUTPUT");
    ensure_directory_exists_and_empty(&output_dir)
        .await
        .unwrap();
    let files = list_files(&input_dir).await;
    // region Keywords
    let mut all_keywords = HashSet::<String>::new();
    if let Some(keywords_file_name) = &args.keywords_file_name {
        let all_keywords_vec: Vec<String> = fs::read_to_string(keywords_file_name)
            .await
            .unwrap()
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.to_uppercase())
            .collect();
        all_keywords.extend(all_keywords_vec);
    }
    if args.load_sql_server_keywords {
        all_keywords.insert("SQLINES".to_string());
        all_keywords.insert("NOLOCK".to_string());
        all_keywords.extend(get_sql_server_keywords());
        all_keywords.extend(get_sql_server_agg_func());
        all_keywords.extend(get_sql_server_analytic_func());
        all_keywords.extend(get_sql_server_bit_manipulation_func());
        all_keywords.extend(get_sql_collation_func());
        all_keywords.extend(get_sql_configuration_func());
        all_keywords.extend(get_sql_server_conversion_func());
        all_keywords.extend(get_sql_server_crypto_func());
        all_keywords.extend(get_sql_server_cursor_func());
        all_keywords.extend(get_sql_server_data_type_func());
        all_keywords.extend(get_sql_server_date_time_func());
        all_keywords.extend(get_sql_server_fuzzy_string_match_func());
        all_keywords.extend(get_sql_server_graph_func());
        all_keywords.extend(get_sql_server_json_func());
        all_keywords.extend(get_sql_server_logical_func());
        all_keywords.extend(get_sql_server_math_func());
        all_keywords.extend(get_sql_server_metadata_func());
        all_keywords.extend(get_sql_server_ranking_func());
        all_keywords.extend(get_sql_server_regex_func());
        all_keywords.extend(get_sql_server_replication_func());
        all_keywords.extend(get_sql_server_security_func());
        all_keywords.extend(get_sql_server_string_func());
        all_keywords.extend(get_sql_server_system_func());
        all_keywords.extend(get_sql_server_system_statistical_func());
        all_keywords.extend(get_sql_server_text_image_func());
        all_keywords.extend(get_sql_server_trigger_func());
        all_keywords.extend(get_sql_server_vector_func());
    }
    // endregion
    let all_keywords = Arc::new(all_keywords);
    let mut set = JoinSet::new();
    for file in files.unwrap() {
        let output_file = output_dir.join(file.file_name().unwrap());
        let all_keywords = all_keywords.clone();
        set.spawn(async move {
            let file_content = read_file(&file).await;
            process_file_content(&output_file, &file_content.unwrap(), all_keywords).await
        });
    }
    let _ = set.join_all().await;
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
