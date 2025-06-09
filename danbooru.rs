use reqwest::header::USER_AGENT;
use reqwest::blocking::Client;
use serde_json;

//needs username and api key

pub struct Danbooru;

impl Danbooru {

    pub fn danbooru(tags: &str) -> String {

            let url = format!("https://danbooru.donmai.us/posts.json?tags={}+order:random+rating:general&login=&api_key=&limit=1", tags);
            let client = reqwest::blocking::Client::new();
            let res = client.get(url)
            .header(USER_AGENT, "MyRustBot/0.1").send().expect("REASON").text().unwrap();
            let data: serde_json::Value = serde_json::from_str(&res).unwrap();
            if data[0]["media_asset"]["variants"].as_array().is_some(){
                let result = &data[0]["media_asset"]["variants"];
                let result = result.as_array().unwrap();
                for i in result{
                    if i["type"] == "original"{
                        return i["url"].as_str().unwrap().to_string();
                    }
                }

                "No result found.".to_string()

            }
            else{
                "No results found. You can only search one tag.".to_string()
            }
    }
}
