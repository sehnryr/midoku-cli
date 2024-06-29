mod parsing;

use clap::Parser;
use midoku_bindings::exports::{Chapter, Filter, Manga, Page};
use midoku_bindings::Bindings;
use miniserde::json::{self, Object};
use proc_macros::{printit, timeit};

use crate::parsing::parse_value;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Path to the extension
    #[arg(short, long)]
    extension: String,

    /// Settings for the extension (json string)
    #[arg(short, long)]
    settings: Option<String>,
}

#[timeit]
#[printit]
fn initialize(bindings: &mut Bindings) -> Result<(), Box<dyn std::error::Error>> {
    bindings
        .initialize()
        .map_err(|_| "Failed to initialize".into())
}

#[timeit]
#[printit]
fn get_manga_list(
    bindings: &mut Bindings,
    filters: Vec<Filter>,
    page: u32,
) -> Result<(Vec<Manga>, bool), Box<dyn std::error::Error>> {
    bindings
        .get_manga_list(filters, page)
        .map_err(|_| "Failed to get manga list".into())
}

#[timeit]
#[printit]
fn get_manga_details(
    bindings: &mut Bindings,
    manga_id: String,
) -> Result<Manga, Box<dyn std::error::Error>> {
    bindings
        .get_manga_details(manga_id)
        .map_err(|_| "Failed to get manga details".into())
}

#[timeit]
#[printit]
fn get_chapter_list(
    bindings: &mut Bindings,
    manga_id: String,
) -> Result<Vec<Chapter>, Box<dyn std::error::Error>> {
    bindings
        .get_chapter_list(manga_id)
        .map_err(|_| "Failed to get chapter list".into())
}

#[timeit]
#[printit]
fn get_page_list(
    bindings: &mut Bindings,
    manga_id: String,
    chapter_id: String,
) -> Result<Vec<Page>, Box<dyn std::error::Error>> {
    bindings
        .get_page_list(manga_id, chapter_id)
        .map_err(|_| "Failed to get page list".into())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Load the extension
    let mut bindings = Bindings::from_file(args.extension)?;

    // Parse and set the settings
    if let Some(settings) = args.settings {
        let settings: Object = json::from_str(&settings)?;
        for (key, value) in settings {
            bindings.settings_mut().insert(key, parse_value(value)?);
        }
    }

    initialize(&mut bindings)?;

    let (_manga_list, _) = get_manga_list(&mut bindings, Vec::new(), 0)?;
    // let manga_id = _manga_list[0].id.clone();
    let manga_id = "74ad3ad0-41e2-4919-a3dd-e5061c3444da".to_string();
    let _manga_details = get_manga_details(&mut bindings, manga_id.clone())?;
    let chapters = get_chapter_list(&mut bindings, manga_id.clone())?;
    let _pages = get_page_list(&mut bindings, manga_id.clone(), chapters[0].id.clone())?;

    Ok(())
}
