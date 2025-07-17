use crate::error::{Error, Result};
use crate::parser::*;
use inquire::validator::Validation;
use inquire::*;
use std::collections::HashSet;
use std::fs::{self};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
/**
 * Enables browsing for files in the terminal
 *
 *
 */

pub fn pick_folder(start_dir: &Path) -> Result<PathBuf> {
    let read_dir = fs::read_dir(start_dir).unwrap();
    let mut options: Vec<String> = vec!["..".to_string(), "[Choose current folder]".to_string()];

    //look at jeremy chone best practice for into() type
    let mut displayable_options = read_dir
        .filter_map(|result| result.ok())
        .map(|dir_entry| dir_entry.file_name().display().to_string())
        .collect::<Vec<String>>();

    options.append(&mut displayable_options);
    let selection = Select::new("Choose an output folder!", options).prompt();
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
                Ok(start_dir.to_path_buf())
            } else {
                let selected_path = start_dir.join(&choice);
                if selected_path.is_dir() {
                    //clear line if dir was selected
                    print!("\x1B[1A\x1B[2K");
                    io::stdout().flush().unwrap();
                    return pick_folder(&selected_path);
                } else {
                    //file was picked which is not possible
                    print!("\x1B[1A\x1B[2K");
                    io::stdout().flush().unwrap();
                    return pick_folder(start_dir);
                }
            }
        }

        Err(err) => return Err(Error::InquireError(err)),
    }
}

pub fn pick_file(start_dir: &Path) -> Option<PathBuf> {
    let paths = fs::read_dir(start_dir).unwrap();
    let mut file_names = paths
        .filter_map(|path| path.ok())
        .map(|dir| dir.file_name().display().to_string())
        .collect::<Vec<String>>();

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

pub fn select_pages() -> Result<HashSet<u32>> {
    // ToDO Check if pages are out of bounds for doc
    // with validation function
    let mut page_string = CustomType::new("Please enter pages!")
        .with_validator(
            |string: &String| match parse_page_string(&mut string.clone()) {
                Ok(_) => Ok(Validation::Valid),

                Err(err) => Ok(Validation::Invalid(err.into())),
            },
        )
        .with_help_message("Like 1,2,3,4-5")
        .prompt()?;
    let pages = parse_page_string(&mut page_string)?;
    Ok(pages)
}

pub fn select_output_name() -> Result<String>{

    let string = CustomType::new("Enter file name!").with_validator(|string: &String| {
                //ToDO check if ends on pdf
                Ok(Validation::Valid)
            }).with_help_message("Must end in .pdf or .PDF").prompt()?;
    Ok(string)

}
