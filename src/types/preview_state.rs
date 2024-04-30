use crate::types::StoryPageData;
use std::clone::Clone;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub enum PreviewState {
    Unset,
    Loading,
    Loaded(StoryPageData),
}
