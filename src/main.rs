pub mod data_input;
pub mod data_output;
mod drive;
use csv::{Reader, Writer};
use data_input::{Kdv, MessItems, Messages};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Number;
use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, read_to_string, File};
use std::io;
use std::path::Path;

fn remove_duplicate(messvec: &mut Messages
                    ,vec_dup: Vec<(usize, usize, Number, String)>) {
    let mut index: Vec<usize> = Vec::new();
    for i in vec_dup.iter() {
        index.push(i.1)
    }
    for i in 0..vec_dup.len() {
        messvec.messages[vec_dup[i].0].content = Some(vec_dup[i].3.clone())
    }
    let mut rs: Vec<MessItems> = Vec::new();
    for (i, v) in messvec.messages.iter().enumerate() {
        if !index.contains(&i) {
            rs.push(v.clone())
        }
    }
    messvec.messages = rs
}
fn read_txt_to_vec(path: &Path) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .replace(",", " ")
        .lines()
        .map(String::from)
        .collect()
}
fn remove_dup_txt(vec: &mut Vec<String>) {
    let re = Regex::new(r"\/(\w+)_n").unwrap();
    let mut rs_list: Vec<bool> = Vec::new();
    for i in 0..vec.len() {
        let captured = re.captures(&vec[i]);
        if let Some(capture_rs) = captured {
            print!("\r{:#?}", &capture_rs[1]);
            let pos_option = vec[i + 1..].iter()
                .any(|x| x.contains(&capture_rs[1]));
            if pos_option {
                rs_list.push(true)
            } else {
                rs_list.push(false)
            }
        }
    }
    let mut rs_iter = rs_list.iter();
    vec.retain(|_| !*rs_iter.next().unwrap());
}
fn read_json_to_struct(path: &Path) -> Result<Messages, Box<dyn Error>> {
    let file = File::open(path)?;
    let rs: Messages = serde_json::from_reader(file)?;
    Ok(rs)
}

pub fn vec_link_to_string(input: Vec<String>) -> String {
    let mut vec_str = Vec::new();
    for i in input.iter() {
        vec_str.push(i.clone());
    }
    vec_str.join("\n")
}

fn map_id(vec: &mut Vec<String>, map: &HashMap<String, String>) {
    for i in 0..vec.len() {
        vec[i] = map.get(&vec[i]).unwrap_or(&vec[i]).to_string();
    }
}

fn read_csv_to_map() -> HashMap<String, String> {
    let mut drive_link = HashMap::new();
    let mut reader = Reader::from_path("./drive.csv").unwrap();
    for stringrecord in reader.records() {
        drive_link.insert(
            stringrecord.as_ref().unwrap().get(0).unwrap().to_string(),
            stringrecord.unwrap().get(1).unwrap().to_string(),
        );
    }
    drive_link
}

fn main() -> Result<(), Box<dyn Error>> {
    let drive_link = read_csv_to_map();
    for dir in fs::read_dir("./json/output").unwrap() {
        println!("{:#?}", dir);
        let path = dir.unwrap().path();
        //regex
        let re = Regex::new(r"output_(\w+).+$")?;
        let captured = re.captures(&path.to_str().unwrap()).unwrap();
        //link format
        let outputfmt_done = format!("output_{}.csv", &captured[1]);
        let outputfmt_notyet = format!("output_{}_notyet.csv", &captured[1]);
        let txtfmt = format!("./links/{}.txt", &captured[1]);

        //read json to struct
        let mut json: Messages = read_json_to_struct(&path)?;
        let dup = json.find_duplicate();
        remove_duplicate(&mut json, dup);

        println!("Read json: {:#?} ...", path);

        //read txt to vec
        let mut txt = read_txt_to_vec(Path::new(&txtfmt));
        println!("Read txt: {:#?} ...", &txtfmt);

        //remove dup_ed link from txt
        if !captured[1].contains("tiktok") {
            remove_dup_txt(&mut txt)
        };

        if captured[1].eq("tiktok") {
            json.tiktokpage();
            let (done,_notyet) = json.retain_only_likes();
            let mut wrt = Writer::from_path(&outputfmt_done)?;
            let mut pre_kdv: String = "Đạt".to_string();
            for (i, v) in done.messages.iter().enumerate() {
                //let mut photos:Vec<String> = v.get_id_images(&txt).unwrap();
                //map_id(&mut photos,&drive_link);
                let output = Output {
                    index: i,
                    time: v.time(),
                    page: v.tiktok_page(),
                    sender_name: v.sender_name.clone(),
                    content: v.remove_newline(),
                    photos: vec_link_to_string(
                        v.get_link_images(&txt).unwrap()),
                    videos: vec_link_to_string(
                        v.get_link_videos(&txt).unwrap()),
                    link: v.get_link_share(),
                    thumbnail: MessItems::get_thumbnail_vid(
                        v.get_link_share()),
                    kdv: v.checkkdv(&mut pre_kdv),
                };
                let _ = wrt.serialize(output);
            }
        } else {

            let (done,_notyet) = json.retain_only_likes();
            println!("");
            let mut pre_kdv: String = "Đạt".to_string();
            //write csv
            println!("Writing done csv: {:#?} ...", &outputfmt_done);
            let mut wrt = Writer::from_path(outputfmt_done)?;
            for (i, v) in done.messages.iter().enumerate() {
                //let mut photos:Vec<String> = v.get_id_images(&txt).unwrap();
                //map_id(&mut photos,&drive_link);
                let output = Output {
                    index: i,
                    time: v.time(),
                    page: v.page_from_name(),
                    sender_name: v.sender_name.clone(),
                    content: v.remove_newline(),
                    photos: vec_link_to_string(
                        v.get_link_images(&txt).unwrap()),
                    videos: vec_link_to_string(
                        v.get_link_videos(&txt).unwrap()),
                    link: v.get_link_share(),
                    thumbnail: MessItems::get_thumbnail_vid(
                        v.get_link_share()),
                    kdv: v.checkkdv(&mut pre_kdv),
                };
                let _ = wrt.serialize(output);
            }
        }
    }
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct Output {
    index: usize,
    time: String,
    page: String,
    sender_name: String,
    content: String,
    photos: String,
    videos: String,
    link: String,
    thumbnail: String,
    kdv: String,
}
