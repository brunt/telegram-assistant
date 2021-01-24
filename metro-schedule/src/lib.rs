use std::fmt;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct NextArrivalRequest {
    pub station: String,
    pub direction: String,
}

#[derive(Serialize, Deserialize)]
pub struct NextArrivalResponse {
    pub station: String,
    pub direction: String,
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

impl fmt::Display for NextArrivalResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "station: {}\ndirection: {}\nline: {}\ntime: {}",
            self.station, self.direction, self.line, self.time
        )
    }
}