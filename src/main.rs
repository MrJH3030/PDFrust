mod args;
mod browser;
mod error;
mod parser;
mod pdf_util;

pub use self::error::{Error, Result};
use args::*;
use clap::Parser;
use std::env;
use std::path::Path;

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
            let first_file = browser::pick_file(&Path::new(&env::current_dir()?)).unwrap();            
            let pages_1 = browser::select_pages()?;        
        
            let second_file = browser::pick_file(&Path::new(&env::current_dir()?)).unwrap();
            let pages_2 = browser::select_pages()?;            

            let output_path = browser::pick_folder(&Path::new(&env::current_dir()?))?;
            let output_file_name = browser::select_output_name()?;
            let mut documents = pdf_util::load_docs_from_paths(vec![&first_file, &second_file]);

            
            pdf_util::delete_pages_not_in(&pages_1, &mut documents[0]);
            pdf_util::delete_pages_not_in(&pages_2, &mut documents[1]);

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
