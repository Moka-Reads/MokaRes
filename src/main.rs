//! MoKaRes
//! A Resource Manager for MoKa Reads
//! By Mustafif Khan
//! Under the GPLv2 License

/// Setups the guide builder
mod guide_setup;
/// The indexer to build README
mod indexer;

use crate::indexer::{new_indexer, Indexer};
use mokareads_core::awesome_lists::AwesomeList;
use mokareads_core::resources::article::{Article, Metadata as ArticleMetadata};
use mokareads_core::resources::cheatsheet::{Cheatsheet, Language, Metadata as CheatsheetMetadata};
use mokareads_core::Result;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use std::path::Path;
use std::time::Duration;
use structopt::StructOpt;

/// Provides a way to do multiple user inputs in an easy macro
/// Given the syntax <variable ident>: <prompt>
/// Usage:
/// ```rust
/// prompt!{
///     var1: "Prompt for var 1: ",
///     var2: {
///         "A block for the prompt of var 2: "
///     }
/// }
/// ```
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

/// The different types of resources that a user can create
#[derive(Debug, StructOpt)]
enum ResourceTypes {
    Cheatsheet,
    Article,
    Guide,
    Indexer,
}

/// The CLI commands
#[derive(Debug, StructOpt)]
#[structopt(about = "A Resources Manager for MoKa Reads")]
enum Cli {
    #[structopt(about = "Create a new resource")]
    New(ResourceTypes),
    #[structopt(about = "Build the README from `./indexer.toml`")]
    BuildIndexer,
    #[structopt(about = "Generates a `awesome.json` given the amount of pages wanted")]
    GenerateAwesome{
        #[structopt(short, long)]
        pages: usize, 
        #[structopt(short, long)]
        attempts: Option<usize> // default 1
    }
}

/// Creates a new cheatsheet with prompts
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

/// Creates a new article with prompts
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
    let cli = Cli::from_args();
    match cli {
        Cli::New(rt) => match rt {
            ResourceTypes::Cheatsheet => {
                new_cheatsheet().await?;
            }
            ResourceTypes::Article => {
                new_article().await?;
            }
            ResourceTypes::Guide => {
                guide_setup::build()?;
            }
            ResourceTypes::Indexer => {
                new_indexer().await?;
            }
        },
        Cli::BuildIndexer => {
            let indexer = Indexer::read().await?;
            indexer.build_readme().await?;
        }
        Cli::GenerateAwesome { pages, attempts } => {
            let mut awesome_list = AwesomeList::default();
            let attempts = attempts.unwrap_or(1);

            for i in 1..=attempts{
                if let Ok(val) = AwesomeList::new(pages).await{
                    awesome_list = val;
                    break;
                }else {
                    if i == attempts{
                        println!("Ran out of attempts, unable to generate!");
                        println!("Try lowering the pages number...");
                        return Ok(());
                    }
                    // put the thread to sleep for 10 seconds so we aren't 
                    // constantly sending github requests and potentially DDOSing
                    std::thread::sleep(Duration::from_secs(10));
                    continue;
                }
            }
            
            let data = serde_json::to_string_pretty(&awesome_list).unwrap();
            let mut file = File::create("awesome.json").await?;
            file.write_all(data.as_bytes()).await?;
            println!("Successfully generated `awesome.json`");
        }
    }

    Ok(())
}
