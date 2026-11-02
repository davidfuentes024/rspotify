use chrono::Duration;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    custom_serde::duration_ms, AudiobookId, ChapterId, Copyright, DatePrecision, Image, Page,
    Restriction, ResumePoint,
};

/// Author object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Author {
    pub name: String,
}

/// Narrator object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Narrator {
    pub name: String,
}

/// Simplified audiobook object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimplifiedAudiobook {
    pub authors: Vec<Author>,
    pub copyrights: Vec<Copyright>,
    pub description: String,
    pub edition: Option<String>,
    pub explicit: bool,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub html_description: String,
    pub id: AudiobookId<'static>,
    pub images: Vec<Image>,
    // Only present in search results.
    pub is_externally_hosted: Option<bool>,
    pub languages: Vec<String>,
    pub media_type: String,
    pub name: String,
    pub narrators: Vec<Narrator>,
    // Spotify returns `null` here outside of the full audiobook endpoint.
    pub total_chapters: Option<u32>,
}

/// Full audiobook object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FullAudiobook {
    pub authors: Vec<Author>,
    pub chapters: Page<SimplifiedChapter>,
    pub copyrights: Vec<Copyright>,
    pub description: String,
    pub edition: Option<String>,
    pub explicit: bool,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub html_description: String,
    pub id: AudiobookId<'static>,
    pub images: Vec<Image>,
    // Not present in search results.
    pub is_playable: Option<bool>,
    pub languages: Vec<String>,
    pub media_type: String,
    pub name: String,
    pub narrators: Vec<Narrator>,
    pub total_chapters: Option<u32>,
}

/// Simplified chapter object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimplifiedChapter {
    pub chapter_number: u32,
    pub description: String,
    #[serde(with = "duration_ms", rename = "duration_ms")]
    pub duration: Duration,
    pub explicit: bool,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub html_description: String,
    pub id: ChapterId<'static>,
    pub images: Vec<Image>,
    pub is_playable: bool,
    pub languages: Vec<String>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: DatePrecision,
    pub restrictions: Option<Restriction>,
    // Only present when the request is made on behalf of a user.
    pub resume_point: Option<ResumePoint>,
}

/// Full chapter object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FullChapter {
    pub audiobook: SimplifiedAudiobook,
    pub chapter_number: u32,
    pub description: String,
    #[serde(with = "duration_ms", rename = "duration_ms")]
    pub duration: Duration,
    pub explicit: bool,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub html_description: String,
    pub id: ChapterId<'static>,
    pub images: Vec<Image>,
    pub is_playable: bool,
    pub languages: Vec<String>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: DatePrecision,
    pub restrictions: Option<Restriction>,
    pub resume_point: Option<ResumePoint>,
}
