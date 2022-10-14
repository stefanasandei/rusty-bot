use std::io::Error;
use rand::Rng;

use crate::models::reddit::RedditSubreddit;

async fn fetch_subreddit(subreddit: &str) -> Option<RedditSubreddit> {
    let request_url = format!("https://www.reddit.com/r/{}.json", subreddit);
    let response = reqwest::get(request_url).await.unwrap();

    if !response.status().is_success() || response.content_length().unwrap() < 200 {
        return None;
    }

    response.json().await.unwrap()
}

pub async fn random_meme(subreddit: &str) -> Result<String, Error> {
    let subreddit_data = fetch_subreddit(subreddit).await;
    if subreddit_data.is_none() {
        return Ok("Subreddit not found!".to_string());
    }

    let posts = subreddit_data.unwrap().data.children;
    let mut rng = rand::thread_rng();

    Ok(posts[rng.gen_range(2..posts.len())].data.url.to_owned())
}
