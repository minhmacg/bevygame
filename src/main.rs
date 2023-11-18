pub mod data_input;
pub mod data_output;
use std::error::Error;
use data_input::{Messages, MessItems};
use std::io::{prelude,BufReader, Read};
use std::fs::{File,read_to_string, self};
use std::path::Path;
use csv::Writer;
use serde_json::{Value, Number};

fn remove_duplicate(messvec: &mut Messages,
                    vec_dup: &Vec<(usize,usize,Number,String)> ) {
    let mut index: Vec<usize> = Vec::new();
    for i in vec_dup.iter() {
        index.push(i.1)
    }
    for i in 0..vec_dup.len() {
        messvec.messages[vec_dup[i].0].content = Some(vec_dup[i].3.clone())
    }
    let mut rs: Vec<MessItems> = Vec::new();
    for (i,v) in messvec.messages.iter().enumerate(){
            if !index.contains(&i) {
                rs.push(v.clone())
            }
    }
    messvec.messages = rs
}
fn read_line_to_vec(path: &Path) -> Vec<String>{
    read_to_string(path)
        .unwrap().replace(","," ").lines().map(String::from).collect() 
}

fn read_json_to_struct(path: &str) -> Result<Messages, Box<dyn Error>> {
    let file = File::open(path)?;
    let rs: Messages = serde_json::from_reader(file)?;
    Ok(rs)
}

fn main() -> Result<(), Box<dyn Error>>{
    
    for dir in fs::read_dir("./jsonbeat/output").unwrap() {
        println!("{:#?}", dir.unwrap().path())
    }
    let linkjson = "./jsonbeat/p2.json";
    let json = read_json_to_struct(&linkjson).unwrap();
    let linktxt = Path::new("./links/p2.txt");
    let txt = read_line_to_vec(&linktxt);
    println!("{:#?}", read_line_to_vec(&linktxt));
    println!("{:#?}", json);
    for i in 0..=100 {
        println!("{:#?}",json.messages[i].get_link_images(&txt))

    };
    Ok(())
}
