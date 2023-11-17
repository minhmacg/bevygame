use serde::{Serialize,Deserialize};
use serde_json::{Value, Number};
use chrono::{TimeZone, Local};


#[derive(Serialize,Deserialize,Debug)]
#[derive(PartialEq, Clone, Copy)]
pub enum Kdv {
    Đạt,
    Bách,
    Empty
} 

#[derive(Serialize,Deserialize,Debug)]
#[derive(Clone)]
pub struct Messages{pub messages: Vec<MessItems>}

impl Messages {
    pub fn remove_newline_total(&mut self) {
        for i in self.messages.iter_mut() {
            i.content = Some(i.remove_newline())
        }
    }

    pub fn converttime(&mut self) {
        for i in self.messages.iter_mut() {
            i.time = Some(i.time())
        }
    }

    pub fn remove_none_reaction(&mut self) {
        self.messages.retain(|x| x.reactions.is_some() )
    }

    pub fn find_duplicate(&self) -> Vec<(usize,usize,Number,String)>{
        let mut rs: Vec<(usize,usize,Number,String)> = Vec::new();
        for i in 0..self.messages.len()-1 {
            let messvec = &self.messages;
            let messitem_1: &MessItems = & messvec[i];
            let messitem_2: &MessItems = & messvec[i+1];
            let duration: u32 =( 
                                messitem_1.timestamp_ms.as_i64().unwrap()
                              - messitem_2.timestamp_ms.as_i64().unwrap()
                            ).abs().try_into().unwrap();
            if (duration <= 30000) 
            & (messitem_1.sender_name == messitem_2.sender_name)
            & (messitem_1.content.is_some() ^ messitem_2.content.is_some()) {
                if  messitem_1.photos.is_some() 
                & messitem_1.content.is_none() {
                        rs.push((i,i+1,
                                // &Number to Number
                                messvec[i+1].timestamp_ms
                                .to_owned(),
                                 // Option String khong copy duoc nen [hai lay
                                // ref (&String) sau do thanh string
                                messvec[i+1].content
                                .as_ref().unwrap().to_string()))  
                    }
                if  messitem_2.photos.is_some() 
                & messitem_2.content.is_none() {
                        rs.push((i+1,i,
                                // &Number to Number
                                messvec[i].timestamp_ms
                                .to_owned(),
                                 // Option String khong copy duoc nen [hai lay
                                // ref (&String) sau do thanh string
                                messvec[i].content
                                .as_ref().unwrap().to_string()))  
                }

            }
        }
        rs
    }
}

#[derive(Serialize,Deserialize,Debug)]
#[derive(Clone)]
pub struct MessItems {
    time: Option<String>,
    timestamp_ms: Number,
    sender_name: String,
    pub content: Option<String>,
    photos: Option<Vec<Photos>>,
    videos: Option<Vec<Videos>>,
    reactions: Option<Vec<Reactions>>,
    share: Option<Link>,
}
impl MessItems {
    pub fn remove_newline(&self) -> String {
       if self.content.is_some() {
           self.content.as_ref().unwrap().replace("\n"," ")
       }
        else {" ".to_string()}
    }
    pub fn time(&self) -> String{
        format!("{}",
            Local.
            timestamp_opt(
                    (self.timestamp_ms
                    .as_i64().expect("timestamp not valid")
                    )/1000
                    ,0
                )
                    .unwrap()
                    .format("%Y/%m/%d"))
    }
    pub fn checkkdv(&self) -> Kdv {
        let listreact: Vec<Kdv> = self.reactions.as_ref()
                                    .unwrap().iter()
                                    .map(|x| x.filter_kdv())
                                    .collect();
        let test_a: bool = listreact.iter().any(|&x| x == Kdv::Đạt);     
        let test_b: bool = listreact.iter().any(|&x| x == Kdv::Bách);     
        let test_c: bool = test_a ^ test_b;
        if test_c == true {
            if test_a == true {
                return Kdv::Đạt
            } else {
                return Kdv::Bách
            } 
        } else {
            return Kdv::Empty
        };
        Kdv::Empty
    }
}
#[derive(Serialize,Deserialize,Debug)]
enum Oke {
    D,
    S,
    K,
}
#[derive(Serialize,Deserialize,Debug)]
#[derive(Clone)]
struct Videos{
    uri: String,
    creation_timestamp: Number,
}


#[derive(Serialize,Deserialize,Debug)]
#[derive(Clone)]
struct Photos{
    uri: String,
    creation_timestamp: Number,
}

#[derive(Serialize,Deserialize,Debug)]
#[derive(Clone)]
struct Link{
    link: String,
}

#[derive(Serialize,Deserialize,Debug)]
#[derive(Clone)]
struct Reactions{
    actor: String,
    reaction: String,
}
impl Reactions {
    fn filter_kdv(&self) -> Kdv {
        match &self.actor {
            x if x == "Thành Đạt" => Kdv::Đạt,
            x if x == "Đức Bách" => Kdv::Bách,
            _ => Kdv::Empty,
        }                          
    }
}
