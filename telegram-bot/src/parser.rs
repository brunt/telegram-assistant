use metro_schedule::{Direction, NextArrivalRequest, Station};
use winnow::ascii::{digit0, digit1, space0, space1, Caseless};
use winnow::combinator::{alt, opt, preceded, separated_pair};
use winnow::token::literal;
use winnow::{PResult, Parser};

use spending_tracker::{Category, SpentRequest};

pub fn parse_metro_request(s: String) -> Option<NextArrivalRequest> {
    parse_station_and_direction(&mut s.as_str())
        .ok()
        .map(|(direction, station)| NextArrivalRequest { station, direction })
}

pub fn parse_spending_request(s: String) -> Option<SpentRequest> {
    parse_amount_and_category(&mut s.as_str())
        .ok()
        .map(|(amount, category)| SpentRequest { category, amount })
}

pub fn parse_budget_request(s: String) -> Option<SpentRequest> {
    parse_budget_and_amount(&mut s.as_str())
        .ok()
        .map(|amount| SpentRequest {
            amount,
            category: None,
        })
}

pub fn is_spending_total_request(s: String) -> bool {
    parse_spending_total_request(&mut s.as_str()).is_ok()
}

pub fn is_spending_reset_request(s: String) -> bool {
    parse_spending_reset_request(&mut s.as_str()).is_ok()
}

fn parse_spending_total_request<'s>(s: &mut &'s str) -> PResult<(&'s str, &'s str)> {
    separated_pair(
        literal(Caseless("spent")),
        space1,
        literal(Caseless("total")),
    )
    .parse_next(s)
}

fn parse_spending_reset_request<'s>(s: &mut &'s str) -> PResult<(&'s str, &'s str)> {
    (separated_pair(
        literal(Caseless("spent")),
        space1,
        literal(Caseless("reset")),
    ))
    .parse_next(s)
}

fn parse_amount_and_category(s: &mut &str) -> PResult<(f32, Option<Category>)> {
    preceded(
        literal(Caseless("spent ")),
        separated_pair(parse_price, space0, opt(parse_category)),
    )
    .parse_next(s)
}

fn parse_budget_and_amount(s: &mut &str) -> PResult<f32> {
    preceded(literal(Caseless("budget ")), parse_price).parse_next(s)
}

// d+.?d*
fn parse_price(s: &mut &str) -> PResult<f32> {
    (digit1, opt('.'), digit0)
        .take()
        .try_map(|n: &str| n.parse())
        .parse_next(s)
}

fn parse_station_and_direction(s: &mut &str) -> PResult<(Direction, Station)> {
    separated_pair(parse_direction, space1, parse_station).parse_next(s)
}

fn parse_direction(s: &mut &str) -> PResult<Direction> {
    alt((
        literal(Caseless("west")).value(Direction::West),
        literal(Caseless("east")).value(Direction::East),
    ))
    .parse_next(s)
}

fn parse_station(s: &mut &str) -> PResult<Station> {
    alt((
        literal(Caseless("lambert2")).value(Station::LambertT2),
        literal(Caseless("lambert")).value(Station::LambertT1),
        literal(Caseless("hanley")).value(Station::NorthHanley),
        literal(Caseless("umsl north")).value(Station::UMSLNorth),
        literal(Caseless("umsl south")).value(Station::UMSLSouth),
        literal(Caseless("umsl")).value(Station::UMSLNorth),
        literal(Caseless("rock road")).value(Station::RockRoad),
        literal(Caseless("wellston")).value(Station::Wellston),
        literal(Caseless("delmar")).value(Station::DelmarLoop),
        literal(Caseless("shrewsbury")).value(Station::Shrewsbury),
        alt((
            literal(Caseless("sunnen")).value(Station::Sunnen),
            literal(Caseless("maplewood")).value(Station::MaplewoodManchester),
            literal(Caseless("brentwood")).value(Station::Brentwood),
            literal(Caseless("richmond")).value(Station::RichmondHeights),
            literal(Caseless("clayton")).value(Station::Clayton),
            literal(Caseless("forsyth")).value(Station::Forsyth),
            literal(Caseless("ucity")).value(Station::UCity),
            literal(Caseless("skinker")).value(Station::Skinker),
            literal(Caseless("forest park")).value(Station::ForestPark),
            literal(Caseless("cwe")).value(Station::CWE),
            literal(Caseless("central west end")).value(Station::CWE),
            literal(Caseless("cortex")).value(Station::Cortex),
            alt((
                literal(Caseless("grand")).value(Station::Grand),
                literal(Caseless("union")).value(Station::Union),
                literal(Caseless("civic")).value(Station::CivicCenter),
                literal(Caseless("stadium")).value(Station::Stadium),
                literal(Caseless("8th pine")).value(Station::EighthPine),
                literal(Caseless("8th and pine")).value(Station::EighthPine),
                literal(Caseless("convention")).value(Station::ConventionCenter),
                literal(Caseless("lacledes")).value(Station::LacledesLanding),
                literal(Caseless("lacledes landing")).value(Station::LacledesLanding),
                literal(Caseless("riverfront")).value(Station::EastRiverfront),
                literal(Caseless("5th missouri")).value(Station::FifthMissouri),
                literal(Caseless("fifth missouri")).value(Station::FifthMissouri),
                literal(Caseless("emerson")).value(Station::EmersonPark),
                literal(Caseless("jjk")).value(Station::JJK),
                literal(Caseless("jackie joiner")).value(Station::JJK),
                literal(Caseless("washington")).value(Station::Washington),
                literal(Caseless("fvh")).value(Station::FairviewHeights),
                literal(Caseless("memorial")).value(Station::MemorialHospital),
                alt((
                    literal(Caseless("memorial hospital")).value(Station::MemorialHospital),
                    literal(Caseless("swansea")).value(Station::Swansea),
                    literal(Caseless("belleville")).value(Station::Belleville),
                    literal(Caseless("college")).value(Station::College),
                    literal(Caseless("shiloh")).value(Station::ShilohScott),
                    literal(Caseless("shiloh scott")).value(Station::ShilohScott),
                )),
            )),
        )),
    ))
    .parse_next(s)
}

fn parse_category(s: &mut &str) -> PResult<Category> {
    alt((
        literal(Caseless("dining")).value(Category::Dining),
        literal(Caseless("grocery")).value(Category::Grocery),
        literal(Caseless("merchandise")).value(Category::Merchandise),
        literal(Caseless("travel")).value(Category::Travel),
        literal(Caseless("entertainment")).value(Category::Entertainment),
        literal(Caseless("other")).value(Category::Other),
    ))
    .parse_next(s)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_category() {
        assert_eq!(parse_category(&mut "DINING").unwrap(), Category::Dining)
    }

    #[test]
    fn test_parse_station() {
        assert_eq!(parse_station(&mut "cwe").unwrap(), Station::CWE)
    }
    #[test]
    fn test_parse_price() {
        assert_eq!(parse_price(&mut "1.57").unwrap(), (1.57f32));
    }

    #[test]
    fn test_parse_station_and_direction() {
        assert_eq!(
            parse_station_and_direction(&mut "west cortex").unwrap(),
            (Direction::West, Station::Cortex)
        );

        assert_eq!(parse_station_and_direction(&mut "east nowhere").ok(), None);

        assert_eq!(
            parse_station_and_direction(&mut "east lambert2").unwrap(),
            (Direction::East, Station::LambertT2)
        );
    }

    #[test]
    fn test_parse_budget_and_amount() {
        assert_eq!(parse_budget_and_amount(&mut "budget 500").unwrap(), 500f32)
    }
    #[test]
    fn test_parse_amount_and_category() {
        assert_eq!(
            parse_amount_and_category(&mut "spent 24.78 dining").unwrap(),
            (24.78f32, Some(Category::Dining))
        );
        assert_eq!(
            parse_amount_and_category(&mut "spent 24.78 grocery").unwrap(),
            (24.78f32, Some(Category::Grocery))
        );
        assert_eq!(
            parse_amount_and_category(&mut "spent 24.78 merchandise").unwrap(),
            (24.78f32, Some(Category::Merchandise))
        );
        assert_eq!(
            parse_amount_and_category(&mut "spent 24.78 travel").unwrap(),
            (24.78f32, Some(Category::Travel))
        );
        assert_eq!(
            parse_amount_and_category(&mut "spent 24.78 entertainment").unwrap(),
            (24.78f32, Some(Category::Entertainment))
        );
        assert_eq!(
            parse_amount_and_category(&mut "spent 24.78 other").unwrap(),
            (24.78f32, Some(Category::Other))
        );
        assert_eq!(
            parse_amount_and_category(&mut "spent 24.78").unwrap(),
            (24.78f32, None)
        );
    }

    #[test]
    fn test_parse_spending_reset_request() {
        assert!(parse_spending_reset_request(&mut "spent reset").is_ok());
        assert!(parse_spending_reset_request(&mut "other string").is_err());
        assert_eq!(is_spending_reset_request("spent reset".to_string()), true);
        assert_eq!(is_spending_reset_request("other string".to_string()), false);
    }

    #[test]
    fn test_parse_spending_total_request() {
        assert!(parse_spending_total_request(&mut "spent total").is_ok());
        assert!(parse_spending_total_request(&mut "other string").is_err());
        assert_eq!(is_spending_total_request("spent total".to_string()), true);
        assert_eq!(is_spending_total_request("other string".to_string()), false);
    }
}
