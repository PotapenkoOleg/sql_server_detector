mod clap_parser;

use crate::clap_parser::Args;
use anyhow::Context;
use clap::Parser;
use std::path::PathBuf;
use tokio::fs;
use tokio::io;

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

async fn process_file_content(file: &PathBuf, content: &str) -> anyhow::Result<()> {
    let invalid_lines: Vec<(usize, String)> = content
        .lines()
        .map(|line| line.to_uppercase())
        .enumerate()
        .filter(|(_, line)| line.contains("OBJECT"))
        .collect();

    if !invalid_lines.is_empty() {
        write_lines_to_file(file, &invalid_lines).await?;
    }
    Ok(())
}

async fn write_lines_to_file(file: &PathBuf, lines: &Vec<(usize, String)>) -> anyhow::Result<()> {
    let content = lines
        .iter()
        .map(|(line_num, line)| format!("{}: {}", line_num, line))
        .collect::<Vec<String>>()
        .join("\n");

    fs::write(file, content)
        .await
        .with_context(|| format!("Failed to write file: {}", file.to_str().unwrap()))?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let input_dir_name = &args.input_directory;
    let input_dir = PathBuf::from(input_dir_name);
    let mut output_dir = PathBuf::from(input_dir_name);
    output_dir.push("OUTPUT");
    ensure_directory_exists_and_empty(&output_dir).await.unwrap();
    let files = list_files(&input_dir).await;
    for file in files.unwrap() {
        let file_content = read_file(&file).await;
        process_file_content(
            &output_dir.join(file.file_name().unwrap()),
            &file_content.unwrap(),
        )
        .await
        .expect("TODO: panic message");
    }
}
