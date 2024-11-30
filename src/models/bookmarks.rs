use crate::models::account::Account;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Bookmark {
    pub id: Option<i64>,
    pub user_account_id: Option<i64>,
    pub linkding_internal_id: Option<i64>,
    pub url: String,
    pub title: String,
    pub description: String,
    pub website_title: Option<String>,
    pub website_description: Option<String>,
    pub notes: String,
    pub web_archive_snapshot_url: String,
    pub favicon_url: Option<String>,
    pub preview_image_url: Option<String>,
    pub is_archived: bool,
    pub unread: bool,
    pub shared: bool,
    pub tag_names: Vec<String>,
    pub date_added: Option<String>,
    pub date_modified: Option<String>,
}

impl Bookmark {
    pub fn new(
        account_id: Option<i64>,
        linkding_id: Option<i64>,
        linkding_url: String,
        linkding_title: String,
        linkding_description: String,
        linkding_website_title: String,
        linkding_website_description: String,
        linkding_notes: String,
        linkding_web_archive_snapshot_url: String,
        linkding_favicon_url: String,
        linkding_preview_image_url: String,
        linkding_is_archived: bool,
        linkding_unread: bool,
        linkding_shared: bool,
        linkding_tag_names: Vec<String>,
        linkding_date_added: Option<String>,
        linkding_date_modified: Option<String>,
    ) -> Self {
        Self {
            id: None,
            user_account_id: account_id,
            linkding_internal_id: linkding_id,
            url: linkding_url,
            title: linkding_title,
            description: linkding_description,
            website_title: Some(linkding_website_title),
            website_description: Some(linkding_website_description),
            notes: linkding_notes,
            web_archive_snapshot_url: linkding_web_archive_snapshot_url,
            favicon_url: Some(linkding_favicon_url),
            preview_image_url: Some(linkding_preview_image_url),
            is_archived: linkding_is_archived,
            unread: linkding_unread,
            shared: linkding_shared,
            tag_names: linkding_tag_names,
            date_added: linkding_date_added,
            date_modified: linkding_date_modified,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarksApiResponse {
    pub count: u64,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<Bookmark>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedResponse {
    pub account: Account,
    pub timestamp: i64,
    pub successful: bool,
    pub bookmarks: Option<Vec<Bookmark>>,
}

impl DetailedResponse {
    pub fn new(
        response_account: Account,
        response_timestamp: i64,
        response_successful: bool,
        response_bookmarks: Option<Vec<Bookmark>>,
    ) -> Self {
        Self {
            account: response_account,
            timestamp: response_timestamp,
            successful: response_successful,
            bookmarks: response_bookmarks,
        }
    }
}
