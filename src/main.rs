mod args;
mod pdf_util;

//use pdf_util;
use clap::Parser;
use args::*;
use std::collections::BTreeMap;
use lopdf::{Document, Object, ObjectId,  Bookmark};
use inquire::*;
use std::fs;
use std::env;
use std::io::{self, Write};
use std::path::{Path, PathBuf};


fn main() {
    
    let arguments = CliArgs::parse();
    match &arguments.command {
        args::Commands::Merge {file_path_1,pages_1, file_path_2, pages_2, output_path} => {
                let docs = pdf_util::load_docs_from_paths(vec![file_path_1 , file_path_2]);                
                match pdf_util::merge_docs(docs){
                    Ok(())=> {

                    },
                    Err(err)=>{

                        eprintln!("{err}");
                    }
                }
        }

        Commands::Browse {} => {
                println!("Browse called");
               // ToDa ask what pages
                let first_file = pick_file(&Path::new(&env::current_dir().unwrap())).unwrap();
                println!("You chose {:?} ", first_file);
                let second_file = pick_file(&Path::new(&env::current_dir().unwrap())).unwrap();
                println!("You chose {:?} ", first_file);
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

fn pick_file(start_dir: &Path) -> Option<PathBuf>{

    let mut options: Vec<String> = vec!["..".to_string()];
    let paths = fs::read_dir(start_dir).unwrap(); // error?
    
    //wtf
    let mut file_names =  paths.filter_map(|path| {
        path.ok().and_then(|p|{
            p.path().file_name().and_then(|n| n.to_str().map(|s| String::from(s)))
        })
    }).collect::<Vec<String>>();
    options.append(&mut file_names);

    // clear line when changing dir to avoid elongated terminal print 
    // take closer look again!
    print!("\x1B[1A\x1B[2K"); // move up and clear
    io::stdout().flush().unwrap();

    let selection = Select::new("Choose a file!", options.clone()).prompt();
    match selection {
        Ok(choice) => {
            if choice == ".." {
                if let Some(parent) = start_dir.parent() {
                    return pick_file(parent);
                } else {
                    return pick_file(start_dir); // Already at root
                }
            }

            let selected_path = start_dir.join(&choice);
            if selected_path.is_dir() {
                pick_file(&selected_path)
            } else if selected_path.is_file() {
                Some(selected_path)
            } else {
                None
            }
        }
        Err(_) => None,
    }


}





