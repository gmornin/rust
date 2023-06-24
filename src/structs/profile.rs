use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Clone)]
// #[cfg_attr(feature = "debug", derive(Debug))]
// pub struct Profile {
//     customisable: ProfileCustomisable,
//     account: ProfileAccount,
// }

#[derive(Serialize, Deserialize, Clone, Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct ProfileCustomisable {
    pub description: String,
    pub details: Vec<ProfileDetail>,
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum ProfileDetail {
    #[cfg_attr(feature = "serde-any", serde(rename = "cake day"))]
    CakeDay { value: CakeDayDetail },
    #[cfg_attr(feature = "serde-any", serde(rename = "birthday"))]
    BirthDay { value: BirthDayDetail },
    #[cfg_attr(feature = "serde-any", serde(rename = "location"))]
    Location { value: String },
    #[cfg_attr(feature = "serde-any", serde(rename = "occupation"))]
    Occupation { value: String },
    #[cfg_attr(feature = "serde-any", serde(rename = "contact"))]
    Contact { value: ContactDetail },
    #[cfg_attr(feature = "serde-any", serde(rename = "company"))]
    Company { value: String },
    #[cfg_attr(feature = "serde-any", serde(rename = "school"))]
    School { value: String },
    #[cfg_attr(feature = "serde-any", serde(rename = "education"))]
    EducationLevel { value: String },
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct CakeDayDetail {
    pub day: u8,
    pub month: u8,
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct BirthDayDetail {
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum ContactDetail {
    #[cfg_attr(feature = "serde-any", serde(rename = "email"))]
    Email { value: String },
    #[cfg_attr(feature = "serde-any", serde(rename = "matrix"))]
    Matrix { value: String },
    #[cfg_attr(feature = "serde-any", serde(rename = "mastodon"))]
    Mastodon { value: String },
    #[cfg_attr(feature = "serde-any", serde(rename = "lemmy"))]
    Lemmy { value: String },
    #[cfg_attr(feature = "serde-any", serde(rename = "github"))]
    Github { value: String },
    #[cfg_attr(feature = "serde-any", serde(rename = "gitlab"))]
    Gitlab { value: String },
    #[cfg_attr(feature = "serde-any", serde(rename = "bitbucket"))]
    Bitbucket { value: String },
    #[cfg_attr(feature = "serde-any", serde(rename = "reddit"))]
    Reddit { value: String },
    #[cfg_attr(feature = "serde-any", serde(rename = "discord"))]
    Discord { value: String },
    #[cfg_attr(feature = "serde-any", serde(rename = "twitter"))]
    Twitter { value: String },
    #[cfg_attr(feature = "serde-any", serde(rename = "youtube"))]
    Youtube { value: String },
    #[cfg_attr(feature = "serde-any", serde(rename = "odysee"))]
    Odysee { value: String },
    #[cfg_attr(feature = "serde-any", serde(rename = "website"))]
    Website { value: String },
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct ProfileAccount {
    pub id: i64,
    pub username: String,
    pub verified: bool,
    pub created: u64,
}
