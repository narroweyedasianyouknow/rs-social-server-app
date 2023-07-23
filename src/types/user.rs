use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

lazy_static! {
      static ref RE_USER_NAME: Regex = Regex::new(r"^[a-zA-Z0-9]{6,}$").unwrap();
}

fn validate_password(password: &str) -> Result<(), ValidationError> {
      let mut has_whitespace = false;
      let mut has_upper = false;
      let mut has_lower = false;
      let mut has_digit = false;

      for c in password.chars() {
            has_whitespace |= c.is_whitespace();
            has_lower |= c.is_lowercase();
            has_upper |= c.is_uppercase();
            has_digit |= c.is_digit(10);
      }
      if !has_whitespace && has_upper && has_lower && has_digit && password.len() >= 8 {
            Ok(())
      } else {
            return Err(ValidationError::new("Password Validation Failed"));
      }
}
// Структура для типа UserType
#[derive(Debug, Serialize, Deserialize)]
pub struct UserType {
      pub(crate) username: String,
      pub(crate) full_name: Option<String>,
      pub(crate) bio: Option<String>,
      pub(crate) website: Option<String>,
      pub(crate) followers: u32,
      pub(crate) following: u32,
      pub(crate) posts: u32,
      pub(crate) profile_picture: Option<String>,
      pub(crate) is_private: bool,
      pub(crate) is_verified: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoredUserType {
      pub(crate) username: String,
      pub(crate) full_name: Option<String>,
      pub(crate) bio: Option<String>,
      pub(crate) website: Option<String>,
      pub(crate) followers: u32,
      pub(crate) following: u32,
      pub(crate) posts: u32,
      pub(crate) profile_picture: Option<String>,
      pub(crate) is_private: bool,

      pub(crate) is_verified: bool,
      pub(crate) password: String,

      pub(crate) email: Option<String>,
      pub(crate) phone: Option<String>,
}

// Тип MinimalUser - это структура с полями username и profile_picture
#[derive(Debug, Serialize, Deserialize)]
pub struct MinimalUser {
      pub(crate) username: String,
      pub(crate) profile_picture: Option<String>,
}

// Тип MediumUser - это структура с полями, кроме bio, website, followers, following, posts и is_verified
#[derive(Debug, Serialize, Deserialize)]
pub struct MediumUser {
      pub(crate) username: String,
      pub(crate) full_name: Option<String>,
      pub(crate) profile_picture: Option<String>,
      pub(crate) is_private: bool,
}

// Тип DefaultReturnUser - это структура с полями, кроме password
#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultReturnUser {
      pub(crate) username: String,
      pub(crate) full_name: Option<String>,
      pub(crate) bio: Option<String>,
      pub(crate) website: Option<String>,
      pub(crate) followers: u32,
      pub(crate) following: u32,
      pub(crate) posts: u32,
      pub(crate) profile_picture: Option<String>,
      pub(crate) is_private: bool,
      pub(crate) is_verified: bool,
      pub(crate) email: Option<String>,
      pub(crate) phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegisterDto {
      #[validate(
            email,
            contains(pattern = "gmail", message = "Email must be valid gmail address")
      )]
      pub(crate) email: String,

      pub(crate) full_name: String,

      #[validate(regex(
            path = "RE_USER_NAME",
            message = "Username must number and alphabets only and must be 6 characters long"
      ))]
      pub(crate) username: String,
      #[validate(custom(
            function = "validate_password",
            message = "Must Contain At Least One Upper Case, Lower Case and Number. Don't use spaces."
      ))]
      pub(crate) password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginDto {
      #[validate(
            email,
            contains(pattern = "gmail", message = "Email must be valid gmail address")
      )]
      pub(crate) email: Option<String>,

      #[validate(regex(
            path = "RE_USER_NAME",
            message = "Username must number and alphabets only and must be 6 characters long"
      ))]
      pub(crate) username: Option<String>,
      #[validate(custom(
            function = "validate_password",
            message = "Must Contain At Least One Upper Case, Lower Case and Number. Don't use spaces."
      ))]
      pub(crate) password: String,
}
