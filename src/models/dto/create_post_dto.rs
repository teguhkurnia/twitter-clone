use std::str::FromStr;

use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::models::post::PostType;

#[derive(Validate, Deserialize, Serialize, Debug)]
pub struct CreatePostDto {
    #[garde(skip)]
    pub parent_id: Option<i32>,
    #[garde(skip)]
    pub attached_post_id: Option<i32>,
    #[garde(skip)]
    pub is_thread: Option<bool>,
    #[garde( length(min = 1))]
    pub content: String,
    #[garde(length(min = 1), custom(validate_post_type(&self.post_type)))]
    pub post_type: String,
}

fn validate_post_type(value: &str) -> impl FnOnce(&str, &()) -> garde::Result + '_ {
    move |_, _| {
        match PostType::from_str(value) {
            Ok(_) => Ok(()),
            Err(_) => Err(garde::Error::new(format!("Invalid post type: {}", value))),
        }
    }
}