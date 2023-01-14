use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug)]
pub struct NextArrivalRequest {
    pub station: Station,
    pub direction: Direction,
}

#[derive(Serialize, Deserialize)]
pub struct NextArrivalResponse {
    pub station: Station,
    pub direction: Direction,
    pub line: String,
    pub time: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StationTimeSlice {
    #[serde(rename = "Lambert Airport Terminal # 1")]
    pub lambert_t1: Option<String>,
    #[serde(rename = "Lambert Airport Terminal # 2")]
    pub lambert_t2: Option<String>,
    #[serde(rename = "North Hanley Station")]
    pub north_hanley: Option<String>,
    #[serde(rename = "UMSL North Station")]
    pub umsl_north: Option<String>,
    #[serde(rename = "UMSL South Station")]
    pub umsl_south: Option<String>,
    #[serde(rename = "Rock Road Station")]
    pub rock_road: Option<String>,
    #[serde(rename = "Wellston Station")]
    pub wellston: Option<String>,
    #[serde(rename = "Delmar Loop Station")]
    pub delmar_loop: Option<String>,
    #[serde(rename = "ShrewsburyLansdowne I44 Station")]
    pub shrewsbury: Option<String>,
    #[serde(rename = "Sunnen Station")]
    pub sunnen: Option<String>,
    #[serde(rename = "MaplewoodManchester Station")]
    pub maplewood_manchester: Option<String>,
    #[serde(rename = "Brentwood I64 Station")]
    pub brentwood: Option<String>,
    #[serde(rename = "Richmond Heights Station")]
    pub richmond_heights: Option<String>,
    #[serde(rename = "Clayton Station")]
    pub clayton: Option<String>,
    #[serde(rename = "Forsyth Station")]
    pub forsyth: Option<String>,
    #[serde(rename = "University CityBig Bend Station")]
    pub u_city: Option<String>,
    #[serde(rename = "Skinker Station")]
    pub skinker: Option<String>,
    #[serde(rename = "Forest ParkDeBaliviere Station")]
    pub forest_park: Option<String>,
    #[serde(rename = "Central West End Station")]
    pub cwe: Option<String>,
    #[serde(rename = "Cortex Station")]
    pub cortex: Option<String>,
    #[serde(rename = "Grand Station")]
    pub grand: Option<String>,
    #[serde(rename = "Union Station")]
    pub union: Option<String>,
    #[serde(rename = "Civic Center Station")]
    pub civic_center: Option<String>,
    #[serde(rename = "Stadium Station")]
    pub stadium: Option<String>,
    #[serde(rename = "8th & Pine Station")]
    pub eight_pine: Option<String>,
    #[serde(rename = "Convention Center Station")]
    pub convention_center: Option<String>,
    #[serde(rename = "Laclede's Landing Station")]
    pub lacledes_landing: Option<String>,
    #[serde(rename = "East Riverfront Station")]
    pub east_riverfront: Option<String>,
    #[serde(rename = "5th & Missouri Station")]
    pub fifth_missouri: Option<String>,
    #[serde(rename = "Emerson Park Station")]
    pub emerson_park: Option<String>,
    #[serde(rename = "JJK Center Station")]
    pub jjk: Option<String>,
    #[serde(rename = "Washington Park Station")]
    pub washington: Option<String>,
    #[serde(rename = "Fairview Heights Station")]
    pub fairview_heights: Option<String>,
    #[serde(rename = "Memorial Hospital Station")]
    pub memorial_hospital: Option<String>,
    #[serde(rename = "Swansea Station")]
    pub swansea: Option<String>,
    #[serde(rename = "Belleville Station")]
    pub belleville: Option<String>,
    #[serde(rename = "College Station")]
    pub college: Option<String>,
    #[serde(rename = "ShilohScott Station")]
    pub shiloh_scott: Option<String>,
}

impl Display for NextArrivalResponse {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "station: {}\ndirection: {}\nline: {}\ntime: {}",
            self.station, self.direction, self.line, self.time
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Station {
    LambertT1,
    LambertT2,
    NorthHanley,
    UMSLNorth,
    UMSLSouth,
    RockRoad,
    Wellston,
    DelmarLoop,
    Shrewsbury,
    Sunnen,
    MaplewoodManchester,
    Brentwood,
    RichmondHeights,
    Clayton,
    Forsyth,
    UCity,
    Skinker,
    ForestPark,
    CWE,
    Cortex,
    Grand,
    Union,
    CivicCenter,
    Stadium,
    EighthPine,
    ConventionCenter,
    LacledesLanding,
    EastRiverfront,
    FifthMissouri,
    EmersonPark,
    JJK,
    Washington,
    FairviewHeights,
    MemorialHospital,
    Swansea,
    Belleville,
    College,
    ShilohScott,
}

impl Display for Station {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::LambertT1 => write!(f, "Lambert Terminal 1"),
            Self::LambertT2 => write!(f, "Lambert Terminal 2"),
            Self::NorthHanley => write!(f, "North Hanley"),
            Self::UMSLNorth => write!(f, "UMSL North"),
            Self::UMSLSouth => write!(f, "UMSL South"),
            Self::RockRoad => write!(f, "Rock Road"),
            Self::Wellston => write!(f, "Wellston"),
            Self::DelmarLoop => write!(f, "Delmar Loop"),
            Self::Shrewsbury => write!(f, "Shrewsberry"),
            Self::Sunnen => write!(f, "Sunnen"),
            Self::MaplewoodManchester => write!(f, "Maplewood Manchester"),
            Self::Brentwood => write!(f, "Brentwood"),
            Self::RichmondHeights => write!(f, "Richmond Heights"),
            Self::Clayton => write!(f, "Clayton"),
            Self::Forsyth => write!(f, "Forsyth"),
            Self::UCity => write!(f, "University City"),
            Self::Skinker => write!(f, "Skinker"),
            Self::ForestPark => write!(f, "Forest Park"),
            Self::CWE => write!(f, "Central West End"),
            Self::Cortex => write!(f, "Cortex"),
            Self::Grand => write!(f, "Grand"),
            Self::Union => write!(f, "Union"),
            Self::CivicCenter => write!(f, "Civic Center"),
            Self::Stadium => write!(f, "Stadium"),
            Self::EighthPine => write!(f, "Eighth and Pine"),
            Self::ConventionCenter => write!(f, "Convention Center"),
            Self::LacledesLanding => write!(f, "Lacledes Landing"),
            Self::EastRiverfront => write!(f, "East Riverfront"),
            Self::FifthMissouri => write!(f, "Fifth and Missouri"),
            Self::EmersonPark => write!(f, "Emerson Park"),
            Self::JJK => write!(f, "JJK"),
            Self::Washington => write!(f, "Washington"),
            Self::FairviewHeights => write!(f, "Fairview Heights"),
            Self::MemorialHospital => write!(f, "Memorial Hospital"),
            Self::Swansea => write!(f, "Swansea"),
            Self::Belleville => write!(f, "Belleville"),
            Self::College => write!(f, "College"),
            Self::ShilohScott => write!(f, "Shiloh Scott"),
        }
    }
}

impl TryFrom<&str> for Station {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "lambert" => Ok(Self::LambertT1),
            "lambert2" => Ok(Self::LambertT2),
            "hanley" => Ok(Self::NorthHanley),
            "umsl north" | "umsl" => Ok(Self::UMSLNorth),
            "umsl south" => Ok(Self::UMSLSouth),
            "rock road" => Ok(Self::RockRoad),
            "wellston" => Ok(Self::Wellston),
            "delmar" => Ok(Self::DelmarLoop),
            "shrewsbury" => Ok(Self::Shrewsbury),
            "sunnen" => Ok(Self::Sunnen),
            "maplewood" => Ok(Self::MaplewoodManchester),
            "brentwood" => Ok(Self::Brentwood),
            "richmond" => Ok(Self::RichmondHeights),
            "clayton" => Ok(Self::Clayton),
            "forsyth" => Ok(Self::Forsyth),
            "ucity" => Ok(Self::UCity),
            "skinker" => Ok(Self::Skinker),
            "forest park" => Ok(Self::ForestPark),
            "cwe" | "central west end" => Ok(Self::CWE),
            "cortex" => Ok(Self::Cortex),
            "grand" => Ok(Self::Grand),
            "union" => Ok(Self::Union),
            "civic" => Ok(Self::CivicCenter),
            "stadium" => Ok(Self::Stadium),
            "8th pine" | "8th and pine" => Ok(Self::EighthPine),
            "convention" => Ok(Self::ConventionCenter),
            "lacledes" | "lacledes landing" => Ok(Self::LacledesLanding),
            "riverfront" => Ok(Self::EastRiverfront),
            "5th missouri" | "fifth missouri" => Ok(Self::FifthMissouri),
            "emerson" => Ok(Self::EmersonPark),
            "jjk" | "jackie joiner" => Ok(Self::JJK),
            "washington" => Ok(Self::Washington),
            "fvh" => Ok(Self::FairviewHeights),
            "memorial" | "memorial hospital" => Ok(Self::MemorialHospital),
            "swansea" => Ok(Self::Swansea),
            "belleville" => Ok(Self::Belleville),
            "college" => Ok(Self::College),
            "shiloh" | "shiloh scott" => Ok(Self::ShilohScott),
            _ => Err(String::from("no station by that name")),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Direction {
    East,
    West,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::East => write!(f, "East"),
            Self::West => write!(f, "West"),
        }
    }
}

impl TryFrom<&str> for Direction {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "west" => Ok(Self::West),
            "east" => Ok(Self::East),
            _ => Err(String::from("neither")),
        }
    }
}
