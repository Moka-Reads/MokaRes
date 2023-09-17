use crate::prompt;
use mdbook::config::HtmlConfig;
use mdbook::{Config, MDBook};
use mokareads_core::Result;
use std::path::Path;
fn setup_config(title: String, authors: &Vec<String>, description: String) -> Result<Config> {
    let mut config = Config::default();
    config.book.authors = authors.clone();
    config.book.title = Some(title);
    config.book.description = Some(description);

    let mut hconf = HtmlConfig::default();
    hconf.default_theme = Some("Rust".to_string());
    hconf.git_repository_icon = Some("fa-github".to_string());
    hconf.curly_quotes = true;
    hconf.mathjax_support = false;
    hconf.copy_fonts = true;
    hconf.no_section_label = false;

    hconf.playground.editable = false;
    hconf.playground.copyable = true;
    hconf.playground.copy_js = true;
    hconf.playground.line_numbers = false;
    hconf.playground.runnable = true;

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
    let authors = authors
        .split(',')
        .map(|x| x.to_string())
        .collect();
    let path = Path::new(&root_path).join(&title);
    let config = setup_config(title, &authors, description)?;
    MDBook::init(path)
        .create_gitignore(true)
        .with_config(config)
        .build()?;

    Ok(())
}
