pub mod data_input;
pub mod data_output;
use data_input::{Messages, MessItems};
use std::error::Error;
use std::io::{prelude,BufReader,Result, Read};
use std::fs::{File,read_to_string};
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
fn read_line_to_vec() -> Vec<String>{
    read_to_string("src/kkn.txt")
        .unwrap().lines().map(String::from).collect() 
}
fn writecsv(input: &Messages, path: &Path) -> Result<()> {
    let mut wrt = Writer::from_path(path)?;
    for i in input.messages.iter() {
        let a = wrt.serialize(i);
        match a {
            Ok(ok) => ok,
            Err(err) => {
                panic!("{:#?}, {:#?}",i,err)
            }
        }
    } 
    Ok(())

} 
fn main() -> Result<()>{
    let file = File::open("output_beatnow.json")?;
    let mut test: Messages = serde_json::from_reader(file)?;
    println!("{:#?}", test.messages.len());
    test.remove_none_reaction();
    println!("{:?}", test.messages[82].checkkdv());
    println!("{:#}", test.messages[2].time());
    println!("{:#?}", test.messages.len());
    let vec_dup = test.find_duplicate();
    remove_duplicate(&mut test,&vec_dup);
    println!("{}", vec_dup.len());
    println!("{:#?}", test.messages.len());
    test.converttime();
    test.remove_newline_total();
    println!("{:#?}", test);
    println!("{:#?}", test.messages.len());
    writecsv(&test, "test.csv".as_ref()); 
    Ok(())
}
