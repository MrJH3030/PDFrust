/**
 * Enables browsing for files in the terminal
 * 
 * 
 */
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use inquire::*;
use std::fs;


pub fn pick_file(start_dir: &Path) -> Option<PathBuf>{

    let paths = fs::read_dir(start_dir).unwrap();
    let mut file_names =  paths.filter_map(|path| {
        path.ok().and_then(|p|{
            p.path().file_name().and_then(|n| n.to_str().map(|s| String::from(s)))
        })
    }).collect::<Vec<String>>();

    // move up one dir option
    let mut options: Vec<String> = vec!["..".to_string()];
    options.append(&mut file_names);

    // Uses Inquire lib for terminal management
    let selection = Select::new("Choose a file!", options.clone()).prompt();
    match selection {
        Ok(choice) => {
            if choice == ".." {
                //clear line if a dir was selected
                print!("\x1B[1A\x1B[2K");
                io::stdout().flush().unwrap();

                if let Some(parent) = start_dir.parent() {
                    return pick_file(parent);

                } else {
                    return pick_file(start_dir); // Already at root
                }
            }

            let selected_path = start_dir.join(&choice);
            if selected_path.is_dir() {
                //clear line if dir was selected
                print!("\x1B[1A\x1B[2K"); 
                io::stdout().flush().unwrap();
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