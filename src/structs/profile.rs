use chrono::Datelike;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct ProfileCustomisable {
    pub description: String,
    pub details: Vec<ProfileDetail>,
}

#[cfg(feature = "profile-validate")]
impl ProfileCustomisable {
    pub fn validate(&self) -> Result<(), Option<usize>> {
        if self.description.len() > 2000
            || self.details.len() > 20
            || self
                .details
                .iter()
                .filter(|detail| {
                    matches!(
                        detail,
                        ProfileDetail::CakeDay { .. } | ProfileDetail::BirthDay { .. }
                    )
                })
                .count()
                > 1
        {
            return Err(None);
        }

        for (i, detail) in self.details.iter().enumerate() {
            if !detail.validate() {
                return Err(Some(i));
            }
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
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

#[cfg(feature = "profile-validate")]
impl ProfileDetail {
    pub fn validate(&self) -> bool {
        match self {
            Self::CakeDay { value } => value.validate(),
            Self::BirthDay { value } => value.validate(),
            _ => true,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct CakeDayDetail {
    pub day: u8,
    pub month: u8,
}

#[cfg(feature = "profile-validate")]
const DAY_IN_MONTH: [u8; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

#[cfg(feature = "profile-validate")]
impl CakeDayDetail {
    pub fn validate(&self) -> bool {
        self.day <= DAY_IN_MONTH[self.month as usize - 1]
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct BirthDayDetail {
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

#[cfg(feature = "profile-validate")]
impl BirthDayDetail {
    pub fn validate(&self) -> bool {
        if self.month == 2 && self.day == 29 && self.year % 4 != 0 {
            return false;
        }

        let now = chrono::Utc::now();

        self.day <= DAY_IN_MONTH[self.month as usize - 1]
            && self.year <= now.year() as u16
            && self.month <= now.month() as u8
            && self.day <= now.day() as u8
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum ContactDetail {
    #[cfg_attr(feature = "serde-any", serde(rename = "email"))]
    Email { name: String, instance: String },
    #[cfg_attr(feature = "serde-any", serde(rename = "matrix"))]
    Matrix { name: String, instance: String },
    #[cfg_attr(feature = "serde-any", serde(rename = "mastodon"))]
    Mastodon { name: String, instance: String },
    #[cfg_attr(feature = "serde-any", serde(rename = "lemmy"))]
    Lemmy { name: String, instance: String },
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
    Odysee { name: String, discriminator: u16 },
    #[cfg_attr(feature = "serde-any", serde(rename = "website"))]
    Website { value: String },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct ProfileAccount {
    pub id: i64,
    pub username: String,
    pub verified: bool,
    pub created: u64,
    pub status: String,
}
