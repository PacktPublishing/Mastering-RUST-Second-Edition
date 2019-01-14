// hews/src/hackernews.rs

use crate::app::Msg;
use reqwest::Client;
use serde_derive::Deserialize;
use serde_json::Value;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread;

const HN_BASE_URL: &str = "https://hacker-news.firebaseio.com/v0/";

#[derive(Deserialize, Debug)]
pub struct Story {
    pub by: String,
    pub id: u32,
    pub score: u64,
    pub time: u64,
    pub title: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub url: Option<String>,
    pub kids: Option<Value>,
    pub descendents: Option<u64>,
}

fn fetch_stories_parsed(client: &Client) -> Result<Value, reqwest::Error> {
    let stories_url = format!("{}topstories.json", HN_BASE_URL);
    let body = client.get(&stories_url).send()?.text()?;
    let story_ids: Value = serde_json::from_str(&body).unwrap();
    Ok(story_ids)
}

pub fn top_stories(client: Arc<Client>, count: usize, tx: &Sender<Msg>) {
    let tx_clone = tx.clone();
    thread::spawn(move || {
        let story_ids = fetch_stories_parsed(&client).unwrap();
        let filtered: Vec<&Value> = story_ids.as_array()
                                             .unwrap()
                                             .iter()
                                             .take(count)
                                             .collect();

        let loaded = !filtered.is_empty();

        for id in filtered {
            let id = id.as_u64().unwrap();
            let story_url = format!("{}item/{}.json", HN_BASE_URL, id);
            let story = client.get(&story_url)
                              .send()
                              .unwrap()
                              .text()
                              .unwrap();
            let story: Story = serde_json::from_str(&story).unwrap();
            tx_clone.send(Msg::NewStory(story)).unwrap();
        }

        if loaded {
            tx_clone.send(Msg::Loaded).unwrap();
        }
    });
}
