mod args;
mod pdf_util;
mod browser;
mod parser;

//use pdf_util;
use clap::Parser;
use args::*;
use std::env;
use std::path::{Path};


fn main() {
    
    let arguments = CliArgs::parse();
    match &arguments.command {

        Commands::Merge {file_path_1, page_string_1, file_path_2, page_string_2, output_path} => {

                let mut docs = pdf_util::load_docs_from_paths(vec![file_path_1 , file_path_2]);
                
                match page_string_1 {
                    Some( page_string_1) =>{
                        let pages_1 = parser::parse_page_string(&mut page_string_1.clone());
                        let doc = &mut docs[0];
                        pdf_util::delete_pages_not_in(&pages_1, doc);
                        
                    }
                    None => {} 
                }

                match page_string_2 {
                    Some( page_string_2) =>{
                        let pages_2 =  parser::parse_page_string(&mut page_string_2.clone());
                        let doc = &mut docs[1];
                        pdf_util::delete_pages_not_in(&pages_2, doc);
                    }
                    None => {} 
                }

                match pdf_util::merge_docs(docs){
                    Ok(())=> {
                        println!( "Success");

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
                
                // ToDo pic output folder
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







