mod args;


use clap::Parser;
use args::*;
use std::collections::BTreeMap;
use std::process::ChildStderr;
use lopdf::content::{Content, Operation};
use lopdf::{Document, Object, ObjectId, Stream, Bookmark};
use inquire::*;
use std::fs;
use std::env;
use std::io::{self, Write};
use std::path::{Path, PathBuf};





fn main() {
    
    let arguments = CliArgs::parse();
    match &arguments.command {
        args::Commands::Merge{file_path_1,pages_1, file_path_2, pages_2, output_path} => {
                let docs = load_docs_from_paths(vec![file_path_1 , file_path_2]);
                
                match merge_docs(docs){
                    Ok(())=> {

                    },
                    Err(err)=>{

                        eprintln!("{err}");
                    }
                }
        }
        Commands::Browse{} => {
                eprintln!("Browse called");
               // ToDa ask what pages
                let first_file = pick_file(&Path::new(&env::current_dir().unwrap())).unwrap();
                println!("You chose {:?} ", first_file);
                let second_file = pick_file(&Path::new(&env::current_dir().unwrap())).unwrap();
                println!("You chose {:?} ", first_file);
                let documents = load_docs_from_paths(vec![&first_file, &second_file]);
                match merge_docs(documents){
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
   
   
   //let mut doc = docs.get_mut(0).unwrap(); 
   //doc.add
  
   

    //delete_pages(&mut doc, &[2,3], "assets/page1.pdf".to_string())
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

fn load_docs_from_paths( paths: Vec<&PathBuf>) -> Vec<Document>{
    let mut documents: Vec<Document> = Vec::new();
    for path in paths{

        match Document::load(path.display().to_string()){
            Ok(doc) =>{
                documents.push(doc);
                
            }
            Err(err) =>{
                println!("Could not find path: {}\n-> {} ", path.display().to_string(), err);
                std::process::exit(0);
            }

        } 
        
    }
    return documents;
}

fn delete_pages_and_save(doc :&mut Document, pages :&[u32], output_path: String){
    doc.delete_pages(pages);
    doc.save(output_path).unwrap();
}

/*This code is basically the same as the merge example in lopdf docs */
fn merge_docs(documents: Vec<Document>) -> std::io::Result<()>{
    
    let mut max_id = 1;
    let mut pagenum = 1;
    // Collect all Documents Objects grouped by a map
    let mut documents_pages = BTreeMap::new();
    let mut documents_objects = BTreeMap::new();
    let mut document = Document::with_version("1.5");

    for mut doc in documents {
        let mut first = false;
        doc.renumber_objects_with(max_id);

        max_id = doc.max_id + 1;

        documents_pages.extend(
            doc
                    .get_pages()
                    .into_iter()
                    .map(|(_, object_id)| {
                        if !first {
                            let bookmark = Bookmark::new(String::from(format!("Page_{}", pagenum)), [0.0, 0.0, 1.0], 0, object_id);
                            document.add_bookmark(bookmark, None);
                            first = true;
                            pagenum += 1;
                        }

                        (
                            object_id,
                            doc.get_object(object_id).unwrap().to_owned(),
                        )
                    })
                    .collect::<BTreeMap<ObjectId, Object>>(),
        );
        documents_objects.extend(doc.objects);
    }

    // "Catalog" and "Pages" are mandatory.
    let mut catalog_object: Option<(ObjectId, Object)> = None;
    let mut pages_object: Option<(ObjectId, Object)> = None;

    // Process all objects except "Page" type
    for (object_id, object) in documents_objects.iter() {
        // We have to ignore "Page" (as are processed later), "Outlines" and "Outline" objects.
        // All other objects should be collected and inserted into the main Document.
        match object.type_name().unwrap_or(b"") {
            b"Catalog" => {
                // Collect a first "Catalog" object and use it for the future "Pages".
                catalog_object = Some((
                    if let Some((id, _)) = catalog_object {
                        id
                    } else {
                        *object_id
                    },
                    object.clone(),
                ));
            }
            b"Pages" => {
                // Collect and update a first "Pages" object and use it for the future "Catalog"
                // We have also to merge all dictionaries of the old and the new "Pages" object
                if let Ok(dictionary) = object.as_dict() {
                    let mut dictionary = dictionary.clone();
                    if let Some((_, ref object)) = pages_object {
                        if let Ok(old_dictionary) = object.as_dict() {
                            dictionary.extend(old_dictionary);
                        }
                    }

                    pages_object = Some((
                        if let Some((id, _)) = pages_object {
                            id
                        } else {
                            *object_id
                        },
                        Object::Dictionary(dictionary),
                    ));
                }
            }
            b"Page" => {}     // Ignored, processed later and separately
            b"Outlines" => {} // Ignored, not supported yet
            b"Outline" => {}  // Ignored, not supported yet
            _ => {
                document.objects.insert(*object_id, object.clone());
            }
        }
    }

    // If no "Pages" object found, abort.
    if pages_object.is_none() {
        println!("Pages root not found.");

        return Ok(());
    }

    // Iterate over all "Page" objects and collect into the parent "Pages" created before
    for (object_id, object) in documents_pages.iter() {
        if let Ok(dictionary) = object.as_dict() {
            let mut dictionary = dictionary.clone();
            dictionary.set("Parent", pages_object.as_ref().unwrap().0);

            document
                    .objects
                    .insert(*object_id, Object::Dictionary(dictionary));
        }
    }

    // If no "Catalog" found, abort.
    if catalog_object.is_none() {
        println!("Catalog root not found.");

        return Ok(());
    }

    let catalog_object = catalog_object.unwrap();
    let pages_object = pages_object.unwrap();

    // Build a new "Pages" with updated fields
    if let Ok(dictionary) = pages_object.1.as_dict() {
        let mut dictionary = dictionary.clone();

        // Set new pages count
        dictionary.set("Count", documents_pages.len() as u32);

        // Set new "Kids" list (collected from documents pages) for "Pages"
        dictionary.set(
            "Kids",
            documents_pages
                    .into_iter()
                    .map(|(object_id, _)| Object::Reference(object_id))
                    .collect::<Vec<_>>(),
        );

        document
                .objects
                .insert(pages_object.0, Object::Dictionary(dictionary));
    }

    // Build a new "Catalog" with updated fields
    if let Ok(dictionary) = catalog_object.1.as_dict() {
        let mut dictionary = dictionary.clone();
        dictionary.set("Pages", pages_object.0);
        dictionary.remove(b"Outlines"); // Outlines not supported in merged PDFs

        document
                .objects
                .insert(catalog_object.0, Object::Dictionary(dictionary));
    }

    document.trailer.set("Root", catalog_object.0);

    // Update the max internal ID as wasn't updated before due to direct objects insertion
    document.max_id = document.objects.len() as u32;

    // Reorder all new Document objects
    document.renumber_objects();

    // Set any Bookmarks to the First child if they are not set to a page
    document.adjust_zero_pages();

    // Set all bookmarks to the PDF Object tree then set the Outlines to the Bookmark content map.
    if let Some(n) = document.build_outline() {
        if let Ok(Object::Dictionary(dict)) = document.get_object_mut(catalog_object.0) {
            dict.set("Outlines", Object::Reference(n));
        }
    }

    document.compress();

    // Save the merged PDF.

    //if let Some() {
    //check output path
    document.save("assets/merged.pdf").unwrap();
 

    Ok(())


}