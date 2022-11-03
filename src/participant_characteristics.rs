use std::{fs::File, io, path::Path, time};

use chrono::{Date, DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::{de::Error, Deserialize, Deserializer};
use serde_repr::Deserialize_repr;

#[derive(Debug, Deserialize)]
pub enum Setting {
    Laptop,
    PC,
}

#[derive(Debug, Deserialize)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Deserialize)]
pub enum Race {
    Asian,
    Black,
    White,
    Other,
}

#[derive(Debug, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum Skin {
    C1 = 1,
    C2 = 2,
    C3 = 3,
    C4 = 4,
    C5 = 5,
}

#[derive(Debug, Deserialize)]
pub enum EyeColor {
    #[serde(rename = "Dark Brown to Brown")]
    DarkBrownToBrown,
    #[serde(rename = "Gray to Blue or Pink")]
    GrayToBlueOrPink,
    #[serde(rename = "Green-Hazel to Blue-Hazel")]
    GreenHazelToBlueHazel,
    #[serde(rename = "Green-Hazel to Green")]
    GreenHazelToGreen,
    Amber,
}

#[derive(Debug, Deserialize)]
pub enum FacialHair {
    Beard,
    Little,
    None,
}

#[derive(Debug, Deserialize)]
pub enum Vision {
    Normal,
    Glasses,
    Contacts,
}

#[derive(Debug, Deserialize)]
pub enum Handedness {
    Left,
    Right,
}

#[derive(Debug, Deserialize)]
pub enum Weather {
    Cloudy,
    Indoors,
    Sunny,
}

#[derive(Debug, Deserialize)]
pub enum PointingDevice {
    Trackpad,
    Mouse,
}

pub type ParticipantId = String;
pub type ParticipantLogId = String;

#[derive(Debug, Deserialize)]
pub struct ParticipantCharacteristic {
    #[serde(rename = "Participant ID")]
    pub participant_id: ParticipantId,

    #[serde(rename = "Participant Log ID")]
    pub participant_log_id: ParticipantLogId,

    #[serde(rename = "Date", deserialize_with = "deserialize_date")]
    pub date: Date<Utc>,

    #[serde(rename = "Setting")]
    pub setting: Setting,

    #[serde(rename = "Display Width (pixels)")]
    pub display_width: u32,

    #[serde(rename = "Display Height (pixels)")]
    pub display_height: u32,

    #[serde(rename = "Screen Width (cm)")]
    pub screen_width: f64,

    #[serde(rename = "Screen Height (cm)")]
    pub screen_height: f64,

    #[serde(rename = "Distance From Screen (cm)")]
    pub destance_from_screen: Option<f64>,

    #[serde(
        rename = "Screen Recording Start Time (Unix milliseconds)",
        deserialize_with = "deserialize_start_time_unix"
    )]
    pub screen_recording_start_time_unix: Option<time::Duration>,

    #[serde(
        rename = "Screen Recording Start Time (Wall Clock UTC)",
        deserialize_with = "deserialize_start_time_utc"
    )]
    pub screen_recording_start_time_utc: Option<DateTime<Utc>>,

    #[serde(rename = "Gender")]
    pub gender: Gender,

    #[serde(rename = "Age", deserialize_with = "csv::invalid_option")]
    pub age: Option<u8>,

    #[serde(rename = "Self-Reported Race")]
    pub self_reported_race: Race,

    #[serde(rename = "Self-Reported Skin Color")]
    pub self_reported_skin_color: Skin,

    #[serde(rename = "Self-Reported Eye Color")]
    pub self_reported_eye_color: EyeColor,

    #[serde(rename = "Facial Hair")]
    pub facial_hair: FacialHair,

    #[serde(rename = "Self-Reported Vision")]
    pub self_reported_vision: Vision,

    #[serde(rename = "Touch Typer", deserialize_with = "deserialize_bool")]
    pub touch_typer: bool,

    #[serde(rename = "Self-Reported Handedness")]
    pub self_reported_handedness: Handedness,

    #[serde(rename = "Weather")]
    pub weather: Weather,

    #[serde(rename = "Pointing Device")]
    pub pointing_device: PointingDevice,

    #[serde(rename = "Notes")]
    pub notes: String,

    #[serde(rename = "Time of day", deserialize_with = "deserialize_time_of_day")]
    pub time_of_day: chrono::Duration,

    #[serde(rename = "Duration", deserialize_with = "deserialize_duration")]
    pub duration: chrono::Duration,
}

fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;

    match s {
        "Yes" => Ok(true),
        "No" => Ok(false),
        _ => Err(Error::unknown_variant(s, &["Yes", "No"])),
    }
}

fn deserialize_date<'de, D>(deserializer: D) -> Result<Date<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let items: Vec<&str> = s.split("/").collect();
    let (year, month, day): (i32, u32, u32) = match &items[..] {
        &[month, day, year, ..] => (
            year.parse().unwrap(),
            month.parse().unwrap(),
            day.parse().unwrap(),
        ),
        _ => return Err(Error::custom("wrong date data")),
    };

    Ok(Date::from_utc(NaiveDate::from_ymd(year, month, day), Utc))
}

fn deserialize_start_time_unix<'de, D>(deserializer: D) -> Result<Option<time::Duration>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        match s.parse::<u64>() {
            Ok(millis) => Ok(Some(time::Duration::from_millis(millis))),
            Err(err) => Err(Error::custom(err.to_string())),
        }
    }
}

fn deserialize_start_time_utc<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        match NaiveDateTime::parse_from_str(s, "%m/%d/%Y %H:%M") {
            Ok(datetime) => Ok(Some(DateTime::from_utc(datetime, Utc))),
            Err(err) => Err(Error::custom(err.to_string())),
        }
    }
}

fn deserialize_time_of_day<'de, D>(deserializer: D) -> Result<chrono::Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let items: Vec<&str> = s.split(":").collect();
    match &items[..] {
        &[hours, minutes, ..] => Ok(chrono::Duration::hours(hours.parse().unwrap())
            + chrono::Duration::minutes(minutes.parse().unwrap())),
        _ => Err(Error::custom("")),
    }
}

fn deserialize_duration<'de, D>(deserializer: D) -> Result<chrono::Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let items: Vec<&str> = s.split(":").collect();
    match &items[..] {
        &[minutes, seconds, ..] => Ok(chrono::Duration::minutes(minutes.parse().unwrap())
            + chrono::Duration::seconds(seconds.parse().unwrap())),
        _ => Err(Error::custom("")),
    }
}

pub fn load_participant_characteristics<P: AsRef<Path>>(
    path: P,
) -> io::Result<Vec<ParticipantCharacteristic>> {
    let file = File::open(path)?;

    let mut reader = csv::Reader::from_reader(file);

    reader
        .deserialize()
        .map(|result| {
            let record: ParticipantCharacteristic = result?;
            Ok(record)
        })
        .collect()
}
