use crate::components::StoryListing;
use crate::constants::{BASE_API_URL, ITEM_API};
use crate::types::StoryItem;
use dioxus::prelude::*;
use futures::future::join_all;

async fn get_story_preview(id: i64) -> Result<StoryItem, reqwest::Error> {
    let url = format!("{}{}{}.json", BASE_API_URL, ITEM_API, id);
    reqwest::get(&url).await?.json().await
}

async fn get_stories(count: usize) -> Result<Vec<StoryItem>, reqwest::Error> {
    let url = format!("{}topstories.json", BASE_API_URL);
    let stories_ids = &reqwest::get(&url).await?.json::<Vec<i64>>().await?[..count];

    let story_futures = stories_ids[..usize::min(stories_ids.len(), count)]
        .iter()
        .map(|&story_id| get_story_preview(story_id));
    Ok(join_all(story_futures)
        .await
        .into_iter()
        .filter_map(|story| story.ok())
        .collect())
}

#[component]
pub fn Stories() -> Element {
    let stories = use_resource(move || get_stories(10));

    match &*stories.read_unchecked() {
        Some(Ok(list)) => rsx! {
            div {
                for story in list {
                    StoryListing { story: story.clone() }
                }
            }
        },
        Some(Err(err)) => rsx! {"An error occurred while fetching stories {err}"},
        None => rsx! {"Loading items"},
    }
}
