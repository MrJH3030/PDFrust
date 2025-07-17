use crate::error::{Error, Result};
use std::collections::HashSet;
use std::u32;

/**
 *
 *  Parse the pages that are passed by the user
 *  Pattern for pages are comma seperated numbers for single pages and dash separation within a range like so : 4, 5, 6, 7-12
 *
 *  ToDo rethink error handling with process::exit
 *
 * */

pub fn parse_page_string(page_string: &mut String) -> Result<HashSet<u32>> {
    page_string.retain(|c| !c.is_whitespace());
    let sub_strings = page_string.split(',');
    let mut pages: HashSet<u32> = HashSet::new();

    for s in sub_strings {
        let string = s.to_string();

        if string.contains('-') {
            let range = string.split_once('-');
            match range {
                Some(tuple) => {
                    let r1 = tuple.0.to_string().parse::<u32>().map_err(|err| {
                        Error::Custom(format!("{err}! Failed to parse: {}", tuple.0))
                    })?;
                    let r2 = tuple.1.to_string().parse::<u32>().map_err(|err| {
                        Error::Custom(format!("{err}! Failed to parse: {}", tuple.1))
                    })?;
                    if r1 <= r2 {
                        for p in r1..=r2 {
                            pages.insert(p);
                        }
                    } else {
                        return Err(Error::InvalidRangeError { start: r1, end: r2 });
                    }
                }
                None => {
                    return Err(Error::FailedToParseRange {
                        range_string: string,
                    });
                }
            }
        } else {
            //insert_parsed_pages(&mut pages, &string);
            let page = string.parse::<u32>()?;
            pages.insert(page);
        };
    }

    Ok(pages)
}
