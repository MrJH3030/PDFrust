mod args;
use std::path::PathBuf;

use clap::Parser;
use args::CliArgs;
use lopdf::{Document};




fn main() {
    
    let arguments = CliArgs::parse();
   //println!("{:?}",arguments);
   //let mut doc1 = Document::load("assets/dummy3.pdf").unwrap();
   //let mut doc2 = Document::load("assets/dummy.pdf").unwrap();
   // run the program with correct arguemtns at this point
   //needs error handling instead of unwrap()
   let mut docs = load_docs_from_paths(vec![arguments.file_path_1, arguments.file_path_2]);
   let mut doc = docs.get_mut(0).unwrap(); 
  
   

    delete_pages(&mut doc, &[2,3], "assets/page1.pdf".to_string())
}

fn load_docs_from_paths( paths: Vec<PathBuf>) -> Vec<Document>{
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

fn delete_pages(doc :&mut Document, pages :&[u32], output_path: String){
    doc.delete_pages(pages);
    doc.save(output_path).unwrap();
}