use std::ffi::OsString;
/**
 * Enables browsing for files in the terminal
 * 
 * 
 */
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use inquire::*;
use std::fs::{self, DirEntry};

pub fn pick_folder(start_dir: &Path) -> Option<PathBuf> {

    let read_dir = fs::read_dir(start_dir).unwrap();  
    let mut options: Vec<String> = vec!["..".to_string(), "[Choose current folder]".to_string()];
    let mut displayable_options = read_dir
        .filter_map(|result| result.ok())
        .map(|dir_entry| dir_entry.file_name().into_string())
        .filter_map(|result| result.ok())
        .collect::<Vec<String>>();
    
    options.append(&mut displayable_options);
    let selection = Select::new("Choose a folder!", options).prompt();
    match selection {

        Ok(choice) => {

            if choice == ".." {
                print!("\x1B[1A\x1B[2K");
                io::stdout().flush().unwrap();

                if let Some(parent) = start_dir.parent() {
                    return pick_folder(parent);

                } else {
                    return pick_folder(start_dir); // Already at root
                }
            } else if choice == "[Choose current folder]" {
                Some(start_dir.to_path_buf())

            }else {
                let selected_path = start_dir.join(&choice);
                if selected_path.is_dir() {
                    //clear line if dir was selected
                    print!("\x1B[1A\x1B[2K"); 
                    io::stdout().flush().unwrap();
                    return pick_folder(&selected_path);

                }else {
                    //file was picked which is not possible
                    print!("\x1B[1A\x1B[2K"); 
                    io::stdout().flush().unwrap();
                    return pick_folder(start_dir);
                }
                
            }
        }

        Err(err) => {
            None
        }

    }
    


}

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