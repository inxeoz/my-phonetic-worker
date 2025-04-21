use std::collections::HashMap;
use std::path::Path;
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use once_cell::sync::Lazy;
use std::sync::Mutex;

// Static dictionary loaded only once at runtime
static IPA_DICT: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| {
    let mut dict = HashMap::new();
    let data = include_str!("en_UK.txt");
    for line in data.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let word = parts[0].to_lowercase();
            let phonetic = parts[1..].join(" ");
            dict.insert(word, phonetic);
        }
    }
    Mutex::new(dict)
});

//TODO need to unccoment
// static CMU_DICT: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| {
//     let mut dict = HashMap::new();
//     let data = include_str!("cmudict.dict");
//     for line in data.lines() {
//         let parts: Vec<&str> = line.split_whitespace().collect();
//         if parts.len() >= 2 {
//             let word = parts[0].to_lowercase();
//             let phonetic = parts[1..].join(" ");
//             dict.insert(word, phonetic);
//         }
//     }
//     Mutex::new(dict)
// });
// 
// pub fn mapper(file_path : &str) -> Mutex<HashMap<String, String>> {
//     let mut dict = HashMap::new();
//     let data = include_str!("cmudict.dict");
//     for line in data.lines() {
//         let parts: Vec<&str> = line.split_whitespace().collect();
//         if parts.len() >= 2 {
//             let word = parts[0].to_lowercase();
//             let phonetic = parts[1..].join(" ");
//             dict.insert(word, phonetic);
//         }
//     }
//     Mutex::new(dict)
// }
#[derive(Deserialize)]
struct RequestData {
    text: String,
}

#[derive(Serialize)]
struct Pair {
    text: String,
    phonetic: String,
}

#[derive(Serialize)]
struct ResponseData {
    phonetic: Vec<Pair>,
}

#[wasm_bindgen]
pub struct PhoneticConverter;

#[wasm_bindgen]
impl PhoneticConverter {
    #[wasm_bindgen(constructor)]
    pub fn new() -> PhoneticConverter {
        PhoneticConverter
    }

    pub fn convert(&self, input: &str) -> JsValue {
        let dict = IPA_DICT.lock().unwrap();
        let mut response = ResponseData { phonetic: Vec::new() };

        for word in input.split_whitespace() {
            let key = word.to_lowercase().replace(".", "");
            let phonetic = dict.get(&key).cloned().unwrap_or_else(|| word.to_string());
            response.phonetic.push(Pair { text: word.to_string(), phonetic });
        }

        to_value(&response).unwrap()
    }
}

fn main() {}
