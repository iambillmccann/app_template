use crate::types::comment::Comment as CommentType;
use dioxus::prelude::*;

#[component]
pub fn Comment(comment: CommentType) -> Element {
    rsx! {
        div { padding: "0.5rem",
            div { color: "gray", "by {comment.by}" }
            div { dangerous_inner_html: "{comment.text}" }
            for kid in &comment.sub_comments {
                Comment { comment: kid.clone() }
            }
        }
    }
}
