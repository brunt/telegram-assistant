use metro_schedule::{Direction, NextArrivalRequest, Station};
use winnow::branch::alt;
use winnow::bytes::tag_no_case;
use winnow::character::{digit0, digit1, space0, space1};
use winnow::combinator::opt;
use winnow::sequence::{preceded, separated_pair};
use winnow::{FinishIResult, IResult, Parser};

use spending_tracker::{Category, SpentRequest};

pub fn parse_metro_request(s: String) -> Option<NextArrivalRequest> {
    parse_station_and_direction(s.as_str())
        .finish()
        .ok()
        .map(|(direction, station)| NextArrivalRequest { station, direction })
}

pub fn parse_spending_request(s: String) -> Option<SpentRequest> {
    parse_amount_and_category(s.as_str())
        .finish()
        .ok()
        .map(|(amount, category)| SpentRequest { category, amount })
}

pub fn parse_budget_request(s: String) -> Option<SpentRequest> {
    parse_budget_and_amount(s.as_str())
        .finish()
        .ok()
        .map(|amount| SpentRequest {
            amount,
            category: None,
        })
}

pub fn is_spending_total_request(s: String) -> bool {
    parse_spending_total_request(s.as_str()).finish().is_ok()
}

pub fn is_spending_reset_request(s: String) -> bool {
    parse_spending_reset_request(s.as_str()).finish().is_ok()
}

fn parse_spending_total_request(s: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(tag_no_case("spent"), space1, tag_no_case("total"))(s)
}

fn parse_spending_reset_request(s: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(tag_no_case("spent"), space1, tag_no_case("reset"))(s)
}

fn parse_amount_and_category(s: &str) -> IResult<&str, (f32, Option<Category>)> {
    preceded(
        tag_no_case("spent "),
        separated_pair(parse_price, space0, opt(parse_category)),
    )(s)
}

fn parse_budget_and_amount(s: &str) -> IResult<&str, f32> {
    preceded(tag_no_case("budget "), parse_price)(s)
}

// d+.?d*
fn parse_price(s: &str) -> IResult<&str, f32> {
    (digit1, opt('.'), digit0)
        .recognize()
        .map_res(|n: &str| n.parse())
        .parse_next(s)
}

fn parse_station_and_direction(s: &str) -> IResult<&str, (Direction, Station)> {
    separated_pair(parse_direction, space1, parse_station)(s)
}

fn parse_direction(s: &str) -> IResult<&str, Direction> {
    alt((
        tag_no_case("west").value(Direction::West),
        tag_no_case("east").value(Direction::East),
    ))(s)
}

fn parse_station(s: &str) -> IResult<&str, Station> {
    alt((
        tag_no_case("lambert2").value(Station::LambertT2),
        tag_no_case("lambert").value(Station::LambertT1),
        tag_no_case("hanley").value(Station::NorthHanley),
        tag_no_case("umsl north").value(Station::UMSLNorth),
        tag_no_case("umsl south").value(Station::UMSLSouth),
        tag_no_case("umsl").value(Station::UMSLNorth),
        tag_no_case("rock road").value(Station::RockRoad),
        tag_no_case("wellston").value(Station::Wellston),
        tag_no_case("delmar").value(Station::DelmarLoop),
        tag_no_case("shrewsbury").value(Station::Shrewsbury),
        tag_no_case("sunnen").value(Station::Sunnen),
        tag_no_case("maplewood").value(Station::MaplewoodManchester),
        tag_no_case("brentwood").value(Station::Brentwood),
        tag_no_case("richmond").value(Station::RichmondHeights),
        tag_no_case("clayton").value(Station::Clayton),
        tag_no_case("forsyth").value(Station::Forsyth),
        tag_no_case("ucity").value(Station::UCity),
        tag_no_case("skinker").value(Station::Skinker),
        tag_no_case("forest park").value(Station::ForestPark),
        tag_no_case("cwe").value(Station::CWE),
        alt((
            tag_no_case("central west end").value(Station::CWE),
            tag_no_case("cortex").value(Station::Cortex),
            tag_no_case("grand").value(Station::Grand),
            tag_no_case("union").value(Station::Union),
            tag_no_case("civic").value(Station::CivicCenter),
            tag_no_case("stadium").value(Station::Stadium),
            tag_no_case("8th pine").value(Station::EighthPine),
            tag_no_case("8th and pine").value(Station::EighthPine),
            tag_no_case("convention").value(Station::ConventionCenter),
            tag_no_case("lacledes").value(Station::LacledesLanding),
            tag_no_case("lacledes landing").value(Station::LacledesLanding),
            tag_no_case("riverfront").value(Station::EastRiverfront),
            tag_no_case("5th missouri").value(Station::FifthMissouri),
            tag_no_case("fifth missouri").value(Station::FifthMissouri),
            tag_no_case("emerson").value(Station::EmersonPark),
            tag_no_case("jjk").value(Station::JJK),
            tag_no_case("jackie joiner").value(Station::JJK),
            tag_no_case("washington").value(Station::Washington),
            tag_no_case("fvh").value(Station::FairviewHeights),
            alt((
                tag_no_case("memorial").value(Station::MemorialHospital),
                tag_no_case("memorial hospital").value(Station::MemorialHospital),
                tag_no_case("swansea").value(Station::Swansea),
                tag_no_case("belleville").value(Station::Belleville),
                tag_no_case("college").value(Station::College),
                tag_no_case("shiloh").value(Station::ShilohScott),
                tag_no_case("shiloh scott").value(Station::ShilohScott),
            )),
        )),
    ))
    .parse_next(s)
}

fn parse_category(s: &str) -> IResult<&str, Category> {
    alt((
        tag_no_case("dining").value(Category::Dining),
        tag_no_case("grocery").value(Category::Grocery),
        tag_no_case("merchandise").value(Category::Merchandise),
        tag_no_case("travel").value(Category::Travel),
        tag_no_case("entertainment").value(Category::Entertainment),
        tag_no_case("other").value(Category::Other),
    ))(s)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_price() {
        assert_eq!(parse_price("1.57").unwrap(), ("", 1.57f32));
    }

    #[test]
    fn test_parse_station_and_direction() {
        assert_eq!(
            parse_station_and_direction("west cortex").unwrap(),
            ("", (Direction::West, Station::Cortex))
        );

        assert_eq!(parse_station_and_direction("east nowhere").ok(), None);

        assert_eq!(
            parse_station_and_direction("east lambert2").unwrap(),
            ("", (Direction::East, Station::LambertT2))
        );
    }

    #[test]
    fn test_parse_amount_and_category() {
        assert_eq!(
            parse_amount_and_category("spent 24.78 dining").unwrap(),
            ("", (24.78f32, Some(Category::Dining)))
        );
        assert_eq!(
            parse_amount_and_category("spent 24.78 grocery").unwrap(),
            ("", (24.78f32, Some(Category::Grocery)))
        );
        assert_eq!(
            parse_amount_and_category("spent 24.78 merchandise").unwrap(),
            ("", (24.78f32, Some(Category::Merchandise)))
        );
        assert_eq!(
            parse_amount_and_category("spent 24.78 travel").unwrap(),
            ("", (24.78f32, Some(Category::Travel)))
        );
        assert_eq!(
            parse_amount_and_category("spent 24.78 entertainment").unwrap(),
            ("", (24.78f32, Some(Category::Entertainment)))
        );
        assert_eq!(
            parse_amount_and_category("spent 24.78 other").unwrap(),
            ("", (24.78f32, Some(Category::Other)))
        );
        assert_eq!(
            parse_amount_and_category("spent 24.78").unwrap(),
            ("", (24.78f32, None))
        );
    }
}
