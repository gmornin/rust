use chrono::Datelike;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Eq, Debug)]
pub struct ProfileCustomisable {
    pub description: String,
    pub details: Vec<ProfileDetail>,
}

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

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
#[serde(tag = "type")]
pub enum ProfileDetail {
    #[serde(rename = "cake day")]
    CakeDay { value: CakeDayDetail },
    #[serde(rename = "birthday")]
    BirthDay { value: BirthDayDetail },
    #[serde(rename = "location")]
    Location { value: String },
    #[serde(rename = "occupation")]
    Occupation { value: String },
    #[serde(rename = "contact")]
    Contact { value: ContactDetail },
    #[serde(rename = "company")]
    Company { value: String },
    #[serde(rename = "school")]
    School { value: String },
    #[serde(rename = "education")]
    EducationLevel { value: String },
}

impl ProfileDetail {
    pub fn validate(&self) -> bool {
        match self {
            Self::CakeDay { value } => value.validate(),
            Self::BirthDay { value } => value.validate(),
            Self::Location { value }
            | Self::Occupation { value }
            | Self::Company { value }
            | Self::School { value }
            | Self::EducationLevel { value }
            | Self::Contact {
                value:
                    ContactDetail::Github { value }
                    | ContactDetail::Gitlab { value }
                    | ContactDetail::Bitbucket { value }
                    | ContactDetail::Reddit { value }
                    | ContactDetail::Discord { value }
                    | ContactDetail::Twitter { value }
                    | ContactDetail::Youtube { value }
                    | ContactDetail::Website { value },
            } => value.len() < 50,
            Self::Contact {
                value:
                    ContactDetail::Email { name, instance }
                    | ContactDetail::Matrix { name, instance }
                    | ContactDetail::Mastodon { name, instance }
                    | ContactDetail::Lemmy { name, instance },
            } => name.len() < 30 && instance.len() < 30 && instance.contains('.'),
            Self::Contact {
                value: ContactDetail::Odysee { name, .. },
            } => name.len() < 30,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct CakeDayDetail {
    pub day: u8,
    pub month: u8,
}

const DAY_IN_MONTH: [u8; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

impl CakeDayDetail {
    pub fn validate(&self) -> bool {
        self.day <= DAY_IN_MONTH[self.month as usize - 1]
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct BirthDayDetail {
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

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

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
#[serde(tag = "type")]
pub enum ContactDetail {
    #[serde(rename = "email")]
    Email { name: String, instance: String },
    #[serde(rename = "matrix")]
    Matrix { name: String, instance: String },
    #[serde(rename = "mastodon")]
    Mastodon { name: String, instance: String },
    #[serde(rename = "lemmy")]
    Lemmy { name: String, instance: String },
    #[serde(rename = "github")]
    Github { value: String },
    #[serde(rename = "gitlab")]
    Gitlab { value: String },
    #[serde(rename = "bitbucket")]
    Bitbucket { value: String },
    #[serde(rename = "reddit")]
    Reddit { value: String },
    #[serde(rename = "discord")]
    Discord { value: String },
    #[serde(rename = "twitter")]
    Twitter { value: String },
    #[serde(rename = "youtube")]
    Youtube { value: String },
    #[serde(rename = "odysee")]
    Odysee { name: String, discriminator: u16 },
    #[serde(rename = "website")]
    Website { value: String },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct ProfileAccount {
    pub id: i64,
    pub username: String,
    pub verified: bool,
    pub created: u64,
    pub status: String,
}
