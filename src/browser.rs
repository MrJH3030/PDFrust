use crate::error::{Error, Result};
use crate::utils::strings::standard_string;
use crate::utils::strings::StandardString as StandardString;
use crate::{parser, utils};
use inquire::validator::Validation;
use inquire::*;
use std::collections::HashSet;
use std::fs::{self};
use std::path::{Path, PathBuf};
/**
 * Enables browsing for files in the terminal
 *
 *
 */

pub fn pick_folder(start_dir: &Path) -> Result<PathBuf> {
    let read_dir = fs::read_dir(start_dir).unwrap();
    let mut options: Vec<String> = vec![standard_string(StandardString::MoveUp).to_string(), standard_string(StandardString::ChooseCurrentFolder).to_string()];

    //look at jeremy chone best practice for into() type
    let mut displayable_options = read_dir
        .filter_map(|result| result.ok())
        .map(|dir_entry| dir_entry.file_name().display().to_string())
        .collect::<Vec<String>>();

    options.append(&mut displayable_options);
    let selection = Select::new(standard_string(StandardString::ChooseOutputFolder), options).prompt();
    match selection {
        Ok(choice) => {
            if choice == ".." {
                utils::clear_screen_line();
                // select parent folder
                if let Some(parent) = start_dir.parent() {
                    return pick_folder(parent);

                } else {
                    return pick_folder(start_dir); // Already at root
                }
            } else if choice == standard_string(StandardString::ChooseCurrentFolder) {
                Ok(start_dir.to_path_buf())
            } else {
                let selected_path = start_dir.join(&choice);
                if selected_path.is_dir() {
                    //clear line if dir was selected
                    utils::clear_screen_line();
                    return pick_folder(&selected_path);
                } else {
                    //file was picked which is not a valid option -> choose again
                    utils::clear_screen_line();
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
    let selection = Select::new(standard_string(StandardString::ChooseFile), options.clone()).prompt();
    match selection {
        Ok(choice) => {
            if choice == ".." {
                //clear line if a dir was selected
                utils::clear_screen_line();

                if let Some(parent) = start_dir.parent() {
                    return pick_file(parent);
                } else {
                    return pick_file(start_dir); // Already at root
                }
            }

            let selected_path = start_dir.join(&choice);
            if selected_path.is_dir() {
                utils::clear_screen_line();
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

pub fn select_pages( page_count: usize) -> Result<HashSet<u32>> {
    // ToDO Check if pages are out of bounds for doc
    // with validation function
    let mut page_string = CustomType::new(format!("Please select pages! [This document contains {page_count} page/s]").as_str())
        .with_validator(
            |string: &String| match parser::parse_page_string(&mut string.clone()) {
                Ok(_) => Ok(Validation::Valid),

                Err(err) => Ok(Validation::Invalid(err.into())),
            },
        )
        .with_help_message(standard_string(StandardString::PatternHelperMessage))
        .prompt()?;
    let pages = parser::parse_page_string(&mut page_string)?;
    Ok(pages)
}

pub fn select_output_name() -> Result<String>{

    let string = CustomType::new(standard_string(StandardString::EnterOutputFileName)).with_validator(|string: &String| {

                //ToDO check if ends on pdf
                if string.ends_with(standard_string(StandardString::PdfLowerCase)) || string.ends_with(standard_string(StandardString::PdfUpperCase)) {
                    return Ok(Validation::Valid);
                
                }
                return Ok(Validation::Invalid(standard_string(StandardString::InvalidFileName).into()));
            }).with_help_message(standard_string(StandardString::PdfHelperMessage)).prompt()?;
    Ok(string)

}

