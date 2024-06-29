use clap::Parser;
use midoku_bindings::exports::{Chapter, Filter, Manga, Number, Page, Value};
use midoku_bindings::Bindings;
use miniserde::json::{self, Object};

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

/// Parse a json value into a midoku value
fn parse_value(value: json::Value) -> Result<Value, Box<dyn std::error::Error>> {
    match value {
        json::Value::Bool(value) => Ok(Value::Bool(value)),
        json::Value::Number(value) => match value {
            json::Number::I64(value) => Ok(Value::Number(Number::S64(value))),
            json::Number::U64(value) => Ok(Value::Number(Number::U64(value))),
            json::Number::F64(value) => Ok(Value::Number(Number::F64(value))),
        },
        json::Value::String(value) => Ok(Value::String(value)),
        json::Value::Array(value) => {
            let mut parsed_value: Vec<String> = Vec::new();
            for value in value {
                match value {
                    json::Value::String(value) => {
                        parsed_value.push(value);
                    }
                    _ => return Err("Invalid value type in array".into()),
                }
            }
            Ok(Value::Array(parsed_value))
        }
        json::Value::Object(value) => {
            let mut parsed_value: Vec<(String, String)> = Vec::new();
            for (key, value) in value {
                match value {
                    json::Value::String(value) => {
                        parsed_value.push((key, value));
                    }
                    _ => return Err("Invalid value type in object".into()),
                }
            }
            Ok(Value::Map(parsed_value))
        }
        _ => Err("Invalid value type".into()),
    }
}

fn initialize(bindings: &mut Bindings) -> Result<(), Box<dyn std::error::Error>> {
    bindings
        .initialize()
        .map_err(|_| "Failed to initialize".into())
}

fn get_manga_list(
    bindings: &mut Bindings,
    filters: Vec<Filter>,
    page: u32,
) -> Result<(Vec<Manga>, bool), Box<dyn std::error::Error>> {
    bindings
        .get_manga_list(filters, page)
        .map_err(|_| "Failed to get manga list".into())
}

fn get_manga_details(
    bindings: &mut Bindings,
    manga_id: String,
) -> Result<Manga, Box<dyn std::error::Error>> {
    bindings
        .get_manga_details(manga_id)
        .map_err(|_| "Failed to get manga details".into())
}

fn get_chapter_list(
    bindings: &mut Bindings,
    manga_id: String,
) -> Result<Vec<Chapter>, Box<dyn std::error::Error>> {
    bindings
        .get_chapter_list(manga_id)
        .map_err(|_| "Failed to get chapter list".into())
}

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
