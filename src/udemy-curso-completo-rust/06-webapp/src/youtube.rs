use gloo_net::{
    http::{Request, Response},
    Error,
};
use serde::Deserialize;
use web_sys::console;

use crate::env::API_KEY;

pub async fn search_youtube_video(text: String) -> Result<VideoItem, Error> {
    let query_url: String = format!(
        "https://www.googleapis.com/youtube/v3/search?part=id%2Csnippet&q={}&key={API_KEY}",
        text
    );

    let response: Response = Request::get(&query_url).send().await?;

    let search_result: SearchResult = response.json::<SearchResult>().await?;

    let empty_video: VideoItem = build_empty_video();
    let video: &VideoItem = match search_result.items.first() {
        Some(video) => video,
        None => &empty_video,
    };
    let message = format!("video: {:#?}", &video);
    console::log_1(&message.into());

    Ok(video.clone())
}

#[derive(Deserialize)]
struct SearchResult {
    items: Vec<VideoItem>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VideoItem {
    pub id: VideoItemId,
    pub snippet: VideoItemSnippet,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VideoItemId {
    pub kind: String,
    pub video_id: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct VideoItemSnippet {
    pub title: String,
    pub description: String,
}

fn build_empty_video() -> VideoItem {
    VideoItem {
        id: VideoItemId {
            kind: "".to_string(),
            video_id: "".to_string(),
        },
        snippet: VideoItemSnippet {
            title: "".to_string(),
            description: "".to_string(),
        },
    }
}
