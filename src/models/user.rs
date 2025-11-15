use serde::{Deserialize, Serialize};
use validator::Validate;
use diesel::prelude::*;
use crate::schema::schema::{ users };

#[derive(Queryable, Serialize)]
pub struct User {
    pub bio: String,
    pub email: String,
    pub first_name: String,
    pub id: i32,
    pub is_active: bool,
    pub last_name: String,
    pub middle_name: String,
    pub privacy_settings: String,
    pub profile_language: String,
    pub profile_picture_url: String,
    pub profile_theme: String,
    pub timezone: String,
    pub username: String
}

#[derive(Insertable, Deserialize, Validate)]
#[diesel(table_name = users)]
pub struct CreateUser {
    pub bio: String,
    pub email: String,
    pub first_name: String,
    pub is_active: bool,
    pub last_name: String,
    pub middle_name: String,
    pub privacy_settings: String,
    pub profile_language: String,
    pub profile_picture_url: String,
    pub profile_theme: String,
    pub timezone: String,
    #[validate(length(min = 1, max = 18, message = "Username must be between 1 and 18 characters"))]
    pub username: String
}