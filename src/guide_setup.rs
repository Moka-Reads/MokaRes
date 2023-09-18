use crate::prompt;
use mdbook::config::HtmlConfig;
use mdbook::{Config, MDBook};
use mokareads_core::Result;
use std::path::Path;
fn setup_config(title: String, authors: Vec<String>, description: String) -> Result<Config> {
    let mut config = Config::default();
    config.book.authors = authors;
    config.book.title = Some(title);
    config.book.description = Some(description);

    let hconf = HtmlConfig {
        default_theme: Some("Rust".to_string()),
        git_repository_icon: Some("fa-github".to_string()),
        curly_quotes: true,
        mathjax_support: false,
        copy_fonts: true,
        no_section_label: false,
        ..Default::default()
    };

    config.set("output.html", hconf)?;

    Ok(config)
}

pub fn build() -> Result<()> {
    prompt!(
      title: "Title: ",
      authors: "Authors: ",
      description: "Description: ",
      root_path: "Root Path: "
    );
    let authors: Vec<String> = authors.split(',').map(|x| x.to_string()).collect();
    let path = Path::new(&root_path).join(&title);
    let config = setup_config(title, authors, description)?;
    MDBook::init(path)
        .create_gitignore(true)
        .with_config(config)
        .build()?;

    Ok(())
}
