use chrono::{Local, TimeZone};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use std::{collections::HashMap, error::Error};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Messages {
    pub messages: Vec<MessItems>,
}

impl Messages {
    pub fn tiktokpage(&mut self) {
        for i in 0..self.messages.len() {
            let sender_name = self.messages[i].sender_name.clone();
            if sender_name.eq("Ánh") | sender_name.eq("Khánh Vũ") {
                if let Some(capture) = self.messages[i].content.clone() {
                    if capture.contains("@Dương Lê") {
                        self.messages[i + 1].tiktok = Some(
                            "BeatVn".to_string())
                    }
                    if capture.contains("@Linh Phương") {
                        self.messages[i + 1].tiktok = Some(
                            "Hello Vn".to_string())
                    }
                }
            }
            if sender_name.eq("Quang Hiếu") {
                self.messages[i].tiktok = Some(
                    "Beat Viral World".to_string());
            }
        }
    }
    //pub fn get_page_tiktok(&self) -> String {
    //    let check_post = |str: String| -> bool {
    //        if str.contains("drive.google.com") {true}
    //        else {false}
    //    };
    //    for (i,v) in self.messages.iter().enumerate() {

    //        if check_post(v.content.unwrap()) {
    //            if self.messages[i-1].content.unwrap.contains("@Dương Lê") {
    //            "Beatvn".to_string()
    //            }
    //            if self.messages[i-1].content.unwrap.contains("@Linh Phương") {
    //
    //        }
    //    }
    //}

    pub fn remove_contentnewline_total(&mut self) {
        for i in self.messages.iter_mut() {
            i.content = Some(i.remove_newline())
        }
    }

    pub fn converttime(&mut self) {
        for i in self.messages.iter_mut() {
            i.time = Some(i.time())
        }
    }
    //remove_none_reaction
    pub fn retain_only_likes(&mut self) -> (Messages, Messages) {
        let checklike = |x: &Option<Vec<Reactions>>| -> bool {
            if let Some(a) = x {
                a.iter().any(|reacts| reacts.reaction == "👍".to_string())
            } else {
                false
            }
        };

        let iter = self.messages.iter();

        let done: Messages = Messages {
            messages: iter
                .clone()
                .filter(|&messitem| checklike(&messitem.reactions))
                .map(|x| x.to_owned())
                .collect(),
        };
        let notyet: Messages = Messages {
            messages: iter
                .filter(|&messitem| !checklike(&messitem.reactions))
                .map(|x| x.to_owned())
                .collect(),
        };
        (done, notyet)
    }

    pub fn find_duplicate(&self) -> Vec<(usize, usize, Number, String)> {
        let mut rs: Vec<(usize, usize, Number, String)> = Vec::new();
        for i in 0..self.messages.len() - 1 {
            let messvec = &self.messages;
            let messitem_1: &MessItems = &messvec[i];
            let messitem_2: &MessItems = &messvec[i + 1];
            let duration: u32 = (messitem_1.timestamp_ms.as_i64().unwrap()
                - messitem_2.timestamp_ms.as_i64().unwrap())
            .abs()
            .try_into()
            .unwrap();
            if (duration <= 300000)
                & (messitem_1.sender_name == messitem_2.sender_name)
                & (messitem_1.content.is_some() ^ messitem_2.content.is_some())
            {
                if messitem_1.photos.is_some() & messitem_1.content.is_none() {
                    rs.push((
                        i,
                        i + 1,
                        // &Number to Number
                        messvec[i + 1].timestamp_ms.to_owned(),
                        // Option String khong copy duoc nen phai lay
                        // ref (&String) sau do thanh string
                        messvec[i + 1].content.as_ref().unwrap().to_string(),
                    ))
                }
                if messitem_2.photos.is_some() & messitem_2.content.is_none() {
                    rs.push((
                        i + 1,
                        i,
                        // &Number to Number
                        messvec[i].timestamp_ms.to_owned(),
                        // Option String khong copy duoc nen phai lay
                        // ref (&String) sau do thanh string
                        messvec[i].content.as_ref().unwrap().to_string(),
                    ))
                }
            }
        }
        rs
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessItems {
    time: Option<String>,
    timestamp_ms: Number,
    pub sender_name: String,
    pub content: Option<String>,
    photos: Option<Vec<Photos>>,
    videos: Option<Vec<Videos>>,
    reactions: Option<Vec<Reactions>>,
    share: Option<Link>,
    tiktok: Option<String>,
}
impl MessItems {
    pub fn tiktok_page(&self) -> String {
        self.tiktok.clone().unwrap_or("".to_string())
    }

    pub fn get_link_share(&self) -> String {
        if let Some(share) = &self.share {
            if let Some(links) = &share.link {
                links.to_string()
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        }
    }

    pub fn get_thumbnail_vid(str: String) -> String {
        let re = Regex::new(r"d\/(.+)\/").unwrap();
        if let Some(captured) = re.captures(&str) {
            let capture = captured[1].to_string();
            String::from(format!("https://drive.google.com/thumbnail?id={}"
                                 , capture))
        } else {
            String::from("")
        }
    }

    pub fn get_link_images(&self, link: &Vec<String>) -> Result<Vec<String>, Box<dyn Error>> {
        let mut test = Vec::new();
        let re = Regex::new(r"\/(\w+)_n(\w+).(png|jpg)$")?;
        if let Some(vecphoto) = &self.photos {
            for photos in vecphoto.iter() {
                if let Some(captured) = re.captures(&photos.uri) {
                    for links in link.iter() {
                        if links.contains(&captured[1]) {
                            test.push(links.clone())
                        }
                    }
                }
            }
        }
        Ok(test)
    }

    pub fn get_id_images(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut test = Vec::new();
        let re = Regex::new(r"\/(\w+.(png|jpg))$")?;
        //let re = Regex::new(r"\/(\w+)_n(\w+).(png|jpg)$")?;
        //if let Some(vecphoto) = &self.photos {
        //    for photos in vecphoto.iter() {
        //        if let Some(captured) = re.captures(&photos.uri) {
        //            for links in link.iter(){
        //                if links.contains(&captured[1]) {
        //                    test.push(links.clone())
        //                }
        //            }
        //        }
        //    }
        //}
        //}
        if let Some(photos) = &self.photos {
            for photo in photos {
                if let Some(captured) = re.captures(&photo.uri) {
                    test.push(captured[1].to_string());
                }
            }
        }

        Ok(test)
    }

    pub fn get_link_videos(&self, link: &Vec<String>) -> Result<Vec<String>, Box<dyn Error>> {
        let mut test = Vec::new();
        let re = Regex::new(r"/(\w+)_n(\w+).mp4$")?;

        if let Some(rs) = &self.videos {
            for i in rs.iter() {
                if let Some(captured) = re.captures(&i.uri) {
                    for links in link.iter() {
                        if links.contains(&captured[1]) {
                            test.push(links.clone())
                        }
                    }
                }
            }
        }
        Ok(test)
    }

    pub fn remove_newline(&self) -> String {
        if self.content.is_some() {
            self.content.as_ref().unwrap().replace("\n", " ")
        } else {
            " ".to_string()
        }
    }
    pub fn time(&self) -> String {
        format!(
            "{}",
            Local
                .timestamp_opt(
                    (self.timestamp_ms.as_i64().expect("timestamp not valid")) / 1000,
                    0
                )
                .unwrap()
                .format("%Y/%m/%d")
        )
    }
    pub fn page_from_name(&self) -> String {
        match self.sender_name.as_ref() {
            "Nguyễn Đức Trọng"
            | "Hoang Tuấn Anh"
            | "Thu Trangg"
            | "Khanh Huyen"
            | "Tuan Dinh Zin"
            | "Bánh Bòa" => "Chuyện của Hà Nội".to_string(),
            "Giang Uyên" | "Thao Thu Giap" => "Inside The Box".to_string(),
            "Nguyễn Thành Vĩnh" | "Đồng Lan Phương" => "Sài Gòn Nghenn".to_string(),
            "Hồng Thư Thư" => "What The Duck".to_string(),
            _ => "".to_string(),
        }
    }
    pub fn checkkdv(&self, mut pre_kdv: &str) -> String {
        let filter_kdv = |reac: &Reactions| -> Kdv {
            let actor: &str = &reac.actor;
            let reaction: &str = &reac.reaction;
            if actor.contains("Thành Đạt") & reaction.eq("👍") {
                Kdv::Đạt
            } else if actor.contains("Đức Bách") & reaction.eq("👍") {
                Kdv::Bách
            } else {
                Kdv::Empty
            }
        };
        if let Some(reaction) = &self.reactions {
            let listreact: Vec<Kdv> = reaction.iter().map(|x| -> Kdv { filter_kdv(x) }).collect();
            let test_a: bool = listreact.iter().any(|&x| x == Kdv::Đạt);
            let test_b: bool = listreact.iter().any(|&x| x == Kdv::Bách);
            let test_c: bool = test_a ^ test_b;
            if test_c {
                if test_a {
                    pre_kdv = "Đạt";
                    "Đạt".to_string()
                } else {
                    pre_kdv = "Bách";
                    "Bách".to_string()
                }
            } else {
                pre_kdv.to_string()
            }
        } else {
            pre_kdv.to_string()
        }
    }

    pub fn getvideos(&self) -> String {
        let mut vec_str = Vec::new();
        if let Some(vec_t) = &self.videos {
            for i in vec_t.iter() {
                vec_str.push(i.uri.clone());
            }
            vec_str.join(" \n ")
        } else {
            " ".to_string()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum Oke {
    D,
    S,
    K,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Videos {
    uri: String,
    creation_timestamp: Number,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Photos {
    uri: String,
    creation_timestamp: Number,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Link {
    link: Option<String>,
    share: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct Reactions {
    actor: String,
    reaction: String,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum Kdv {
    Đạt,
    Bách,
    Empty,
}
