use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct RedditPostMetadata {
    pub title: String,
    pub score: u64,
    pub pinned: bool, 
    pub over_18: bool,
    pub author: String,
    pub url: String,
    pub upvote_ratio: f32
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct RedditPost {
    pub kind: String,
    pub data: RedditPostMetadata
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct RedditSubredditData {
    pub after: String,
    pub modhash: String,
    pub children: Vec<RedditPost>
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct RedditSubreddit {
    pub kind: String,
    pub data: RedditSubredditData
}