//! MoKaRes
//! A Resource Manager for MoKa Reads
//! By Mustafif Khan
//! Under the GPLv2 License
mod guide_setup;
mod indexer;

use mokareads_core::resources::article::{Article, Metadata as ArticleMetadata};
use mokareads_core::resources::cheatsheet::{Cheatsheet, Language, Metadata as CheatsheetMetadata};
use mokareads_core::Result;
use std::path::Path;
use structopt::StructOpt;

#[macro_export]
macro_rules! prompt {
    ($($var:ident : $msg:expr), *) => {
        $(
            let $var: String = {
                let mut buf = String::new();
                let str: String = $msg.to_string();
                println!("{}", str);
                ::std::io::stdin().read_line(&mut buf).unwrap();
                buf.trim().to_owned()
            };
        )*
    };
}

#[derive(Debug, StructOpt)]
enum ResourceTypes {
    Cheatsheet,
    Article,
    Guide,
}
#[derive(Debug, StructOpt)]
#[structopt(about = "A Resources Manager for MoKa Reads")]
enum CLI {
    #[structopt(about = "Create a new resource (cheatsheet/article)")]
    New(ResourceTypes),
}

async fn new_cheatsheet() -> Result<()> {
    prompt!(
        title: "Title:",
        author: "Author:",
        level: "Level:",
        lang: "Language:",
        icon: {
            let language = Language::from_str(&lang.to_lowercase());
            format!("Icon: (suggestion {})", language.icon_suggestion())
        }
    );

    let mut icon = icon;

    if icon.is_empty() {
        let language = Language::from_str(&lang.to_lowercase());
        icon = language.icon_suggestion();
    }

    let level: u8 = level.parse().unwrap();
    let metadata = CheatsheetMetadata::new(&title, &author, level, &lang, &icon);
    let content = format!("## {}", &title);

    let cheatsheet = Cheatsheet::new(metadata, content);
    let file_name = format!("{}.md", &cheatsheet.slug);
    let md = cheatsheet.to_markdown();

    // check if file exists first
    let path = Path::new(&file_name);
    if path.exists() {
        println!("File already exists!");
        return Ok(());
    }

    tokio::fs::write(file_name, md).await?;

    Ok(())
}

async fn new_article() -> Result<()> {
    prompt!(
        title: "Title:",
        description: "Description:",
        author: "Author:",
        tags: "Tags:",
        icon: "Icon:"
    );

    let metadata = ArticleMetadata::new(&title, &description, &author, &icon, &tags);
    let content = format!("## {}", &title);

    let article = Article::new(metadata, content);
    let file_name = format!("{}.md", &article.slug);
    let md = article.to_markdown();

    let path = Path::new(&file_name);
    if path.exists() {
        println!("File already exists!");
        return Ok(());
    }

    tokio::fs::write(file_name, md).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = CLI::from_args();
    match cli {
        CLI::New(rt) => match rt {
            ResourceTypes::Cheatsheet => {
                new_cheatsheet().await?;
            }
            ResourceTypes::Article => {
                new_article().await?;
            }
            ResourceTypes::Guide => {
                guide_setup::build()?;
            }
        },
    }

    Ok(())
}
