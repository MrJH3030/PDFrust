use std::u32;
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
            let range = str.split('-');
            if range.clone().count() != 2 {
                eprintln!("Invalid range detected");

            }else {

                for r in range {
                    insert_parsed_pages(&mut pages, &r.to_string());                    
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