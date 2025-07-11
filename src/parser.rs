use std::{process, u32};
use std::collections::HashSet;

/**
 * 
 *  Parse the pages that are pass by the user
 *  Pattern for pages is are comma seperated numbers for single pages and dash separation within a range like so : 4, 5, 6, 7-12 
 * 
 * */

pub fn parse_page_string(page_string: &mut String) -> HashSet<u32> {

    page_string.retain(|c| !c.is_whitespace());
    let sub_strings = page_string.split(',');
    let mut pages: HashSet<u32> = HashSet::new();

    for s in sub_strings {
        let str = s.to_string();

        if str.contains('-') {
            let range = str.split_once('-');
            match range {

                Some(tuple) => {
                    let r1 = tuple.0.to_string().parse::<u32>().unwrap_or_else(|err| {
                        eprintln!("Invalid Input: {}\nError: {err}", tuple.0);
                        process::exit(0);
                    }); 
                    let r2 =  tuple.1.to_string().parse::<u32>().unwrap_or_else(|err| {
                        eprintln!("Invalid Input: {}\nError: {err}", tuple.1);
                        process::exit(0);
                    }); 
                    if r1 <= r2 {
                        for p in r1..=r2 {
                            pages.insert(p);
                        }
                    } else {
                        eprintln!("Please try again\nInvalid range from {r1} to {r2}");
                        process::exit(0);
                    }

                }
                None =>{
                    eprintln!("Failed to parse range");
                }
            }
    
        } else {
            insert_parsed_pages(&mut pages, &str);

        };
    }

    pages

}   

fn insert_parsed_pages(pages: &mut HashSet<u32>, str: &String) {

    let page = str.parse::<u32>();
            match page {
                Ok(page) => {
                    pages.insert(page);
                }

                Err(err) => {
                        eprintln!("Failed to parse page\n {err}");
                }
            }

}