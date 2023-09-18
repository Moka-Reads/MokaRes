use mokareads_core::resources::{article::Article, cheatsheet::Cheatsheet, guide::Guide, Parser};
use mokareads_core::Result;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;
use tokio::fs::{read_to_string, File};
use tokio::io::AsyncWriteExt;
use toml::{from_str, to_string_pretty};
use walkdir::WalkDir;

/// Indexer to read each directory and construct a README file
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Indexer {
    readme: PathBuf,
    article: PathBuf,
    cheatsheet: PathBuf,
    guide: PathBuf,
    readme_conf: ReadmeConf,
}

/// The configuration for the README including a header, subheader, and license info
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ReadmeConf {
    header: String,
    subheader: Option<String>,
    license_info: String,
}

/// Asynchronously reads a directory and gets the markdown file's contents and its path
async fn get_files(folder: &PathBuf) -> Result<Vec<(String, PathBuf)>> {
    let mut entries = Vec::new();

    for entry in WalkDir::new(folder) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
            let content = read_to_string(entry.path()).await?;
            entries.push((content, entry.path().to_path_buf()));
        }
    }

    Ok(entries)
}
/// Reads the directory names inside of a folder
fn get_dir_names(folder: &PathBuf) -> Result<Vec<String>> {
    let mut entries = Vec::new();

    for entry in WalkDir::new(folder)
        .max_depth(1)
        .into_iter()
        .skip(1)
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_dir() {
            if let Some(dir_name) = path.file_name().and_then(|os_str| os_str.to_str()) {
                entries.push(dir_name.to_string());
            }
        }
    }

    Ok(entries)
}
/// Creates a new indexer by creating a default configuration and saving it as `indexer.toml`
pub async fn new_indexer() -> Result<()> {
    let indexer = Indexer::default();
    let data = indexer.to_string();
    let mut file = File::create("indexer.toml").await?;
    file.write_all(data.as_bytes()).await?;
    Ok(())
}
/// Capitalizes the first letter of a string
fn capitalize_first(input: &str) -> String {
    let (first_char, rest) = input.split_at(1);
    let mut capitalized = String::with_capacity(input.len());
    capitalized.push_str(&first_char.to_ascii_uppercase());
    capitalized.push_str(rest);
    return capitalized;
}
impl Indexer {
    /// Reads `indexer.toml` and returns `Result<Self>`
    pub async fn read() -> Result<Self> {
        let indexer = read_to_string("indexer.toml").await?;
        Ok(from_str(&indexer).unwrap_or_default())
    }
    /// Gets the articles using `get_files()` with the `article` path
    async fn articles(&self) -> Result<Vec<(Article, PathBuf)>> {
        let raw = get_files(&self.article).await?;
        Ok(raw
            .into_iter()
            .map(|(data, path)| (Article::parse(&data), path))
            .collect())
    }
    /// Gets the cheatsheets using `get_files()` with the `cheatsheet` path
    async fn cheatsheets(&self) -> Result<Vec<(Cheatsheet, PathBuf)>> {
        let raw = get_files(&self.cheatsheet).await?;
        Ok(raw
            .into_iter()
            .map(|(data, path)| (Cheatsheet::parse(&data), path))
            .collect())
    }
    /// Gets the guides using the `get_files()` with the `guide` path
    async fn guides(&self) -> Result<Vec<Guide>> {
        let guides = get_dir_names(&self.guide)?;
        Ok(guides.into_iter().map(|x| Guide::new(&x)).collect())
    }
    /// Converts all the resources into markdown format
    async fn build_data(&self) -> Result<String> {
        let articles = self.articles().await?;
        let cheatsheets = self.cheatsheets().await?;
        let guides = self.guides().await?;

        let mut contents = Vec::new();
        contents.push(self.readme_conf.to_string());

        contents.push("## Articles  ".to_string());

        for (a, path) in articles {
            let s = format!("- [{}]({:?})", a.title(), path);
            contents.push(s)
        }

        contents.push("## Cheatsheets  ".to_string());

        for (c, path) in cheatsheets {
            let s = format!(
                "- **{}**: [{}]({:?})",
                c.lang(),
                capitalize_first(&c.title()),
                path
            );
            contents.push(s)
        }

        contents.push("## Guides  ".to_string());

        for g in guides {
            let s = format!("- [{}]({})", &g.repo_name, &g.addy);
            contents.push(s)
        }

        Ok(contents.join("\n"))
    }
    /// Builds the `README` file using the README configuration and the `readme` path
    pub async fn build_readme(&self) -> Result<()> {
        let mut readme_file = File::create(&self.readme).await?;
        let data = self.build_data().await?;

        readme_file.write_all(data.as_bytes()).await?;

        Ok(())
    }
}

impl Display for Indexer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let str = to_string_pretty(&self).unwrap();
        f.write_str(&str)
    }
}

impl Display for ReadmeConf {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut joined = Vec::new();
        let header = format!("# {}", self.header);
        joined.push(header);
        if let Some(subheader) = &self.subheader {
            joined.push(format!("## {}", subheader));
        }
        let license_info = format!("> {}", self.license_info);
        joined.push(license_info);
        f.write_str(&joined.join("\n"))
    }
}
