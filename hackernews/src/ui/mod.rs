#![allow(non_snake_case)]
use dioxus::prelude::*;
mod story_comment;
mod story_item;
use crate::{
    api::{get_story_comments, get_top_stories},
    StoryData, StoryItem,
};
use story_item::StoryItems;

pub fn App() -> Element {
    rsx! {
        document::Stylesheet {
            // Urls are relative to your Cargo.toml file
            href: asset!("/assets/tailwind.css"),
        }
        main { class: "flex w-full h-full shadow-lg rounded-3xl",
            section { class: "flex flex-col w-4/12 h-full pt-3 overflow-y-scroll bg-gray-50",
                Stories {}
            }
            section { class: "flex flex-col w-8/12 px-4 bg-white rounded-r-3xl",
                section {
                    ul {}
                }
            }
        }
    }
}

#[component]
pub fn Stories() -> Element {
    let stories = use_resource(move || get_top_stories(20));
    match &*stories.read_unchecked() {
        Some(Ok(stories)) => rsx! {
          ul {
            for story in stories {
              StoryItems { story: story.clone() }
            }
          }
        },
        Some(Err(err)) => rsx! {
          div { class: "mt-6 text-red-500",
            p { "Failed to fetch stories" }
            p { "{err}" }
          }
        },
        None => rsx! {
          div { class: "mt-6",
            p { "Loading stories..." }
          }
        },
    }
}
