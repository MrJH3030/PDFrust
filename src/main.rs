mod args;
mod browser;
mod error;
mod parser;
mod pdf_util;

pub use self::error::{Error, Result};
use args::*;
use clap::Parser;
use inquire::Confirm;
use lopdf::Document;
use std::env;
use std::path::Path;
use std::io::{self, Write};

fn main() -> Result<()> {
    let arguments = CliArgs::parse();
    match &arguments.command {
        Commands::Merge {
            file_path_1,
            page_string_1,
            file_path_2,
            page_string_2,
            output_path,
            output_name,
        } => {
            let mut documents = pdf_util::load_docs_from_paths(vec![file_path_1, file_path_2]);

            match page_string_1 {
                Some(page_string_1) => {
                    let pages_1 = parser::parse_page_string(&mut page_string_1.clone())?;
                    let doc = &mut documents[0];
                    pdf_util::delete_pages_not_in(&pages_1, doc);
                }
                None => {}
            }

            match page_string_2 {
                Some(page_string_2) => {
                    let pages_2 = parser::parse_page_string(&mut page_string_2.clone())?;
                    let doc = &mut documents[1];
                    pdf_util::delete_pages_not_in(&pages_2, doc);
                }
                None => {}
            }

            //ToDo choose output path and let default be the folder from where th app was called
            match pdf_util::merge_docs(documents, output_path.clone(), output_name.clone()) {
                Ok(_) => {
                    println!("Successfully merged documents!");
                }
                Err(err) => {
                    eprintln!("{err}");
                }
            }
        }

        Commands::Browse {} => {        
           
            // make this better to have handle more files in the future
            //ToDo page string for all ? 
            let mut keep_adding: bool = true;
            let mut documents: Vec<Document> =  Vec::new();
            while keep_adding {

                let file = browser::pick_file(&Path::new(&env::current_dir()?)).unwrap();
                let mut  document = Document::load(file.display().to_string())?;
                let page_count = document.get_pages().len();    
                let pages = browser::select_pages(page_count)?;
                pdf_util::delete_pages_not_in(&pages, &mut document);
                documents.push(document);
                keep_adding = Confirm::new("Add another file?").with_default(true).prompt()?;
                print!("\x1B[1A\x1B[2K");
                io::stdout().flush().unwrap();           

            }

            let output_path = browser::pick_folder(&Path::new(&env::current_dir()?))?;
            let output_file_name = browser::select_output_name()?;

            match pdf_util::merge_docs(documents, Some(output_path), Some(output_file_name)) {
                Ok(()) => {
                    println!("Successfully merged documents!");
                }

                Err(err) => {
                    eprintln!("{err}");
                }
            }
        }

        Commands::Delete {} => {}
    }
    Ok(())
}
