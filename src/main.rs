mod args;
mod pdf_util;
mod browser;
mod parser;

//use pdf_util;
use clap::Parser;
use args::*;
use std::env;
use std::path::{Path, PathBuf};


fn main() {
    
    let mut arguments = CliArgs::parse();
    match &arguments.command {

        Commands::Merge {file_path_1, pages_1, file_path_2, pages_2, output_path} => {

                let docs = pdf_util::load_docs_from_paths(vec![file_path_1 , file_path_2]);
                match pages_1 {
                    Some( pages_1) =>{
                       // mutability issue parser::parse_page_string(pages_1);
                    }
                    None => {

                    }
                    
                }

                match pdf_util::merge_docs(docs){
                    Ok(())=> {

                    },
                    Err(err)=>{
                        eprintln!("{err}");
                    }
                }
        }

        Commands::Browse {} => {
               // ToDo ask what pages
                let first_file = browser::pick_file(&Path::new(&env::current_dir().unwrap())).unwrap();
                let second_file = browser::pick_file(&Path::new(&env::current_dir().unwrap())).unwrap();        
                let documents = pdf_util::load_docs_from_paths(vec![&first_file, &second_file]);
                match pdf_util::merge_docs(documents){

                    Ok(())=>{
                        println!("Successfully merged documents!");
                    }

                    Err(err)=>{
                        eprintln!("{err}");
                    }
                }
        }

        Commands::Delete {  } => {

        }

    }
}







