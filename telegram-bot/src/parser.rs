use metro_schedule::{Direction, NextArrivalRequest, Station};
use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::{char, digit0, digit1, space0, space1};
use nom::combinator::{map, map_res, opt, recognize};
use nom::sequence::{pair, preceded, separated_pair};
use nom::{Finish, IResult};
use spending_tracker::{Category, SpentRequest};

pub(crate) fn parse_metro_request(s: String) -> Option<NextArrivalRequest> {
    parse_station_and_direction(s.as_str())
        .finish()
        .ok()
        .map(|(_, (direction, station))| NextArrivalRequest { station, direction })
}

pub(crate) fn parse_spending_request(s: String) -> Option<SpentRequest> {
    parse_amount_and_category(s.as_str())
        .finish()
        .ok()
        .map(|(_, (amount, category))| SpentRequest { category, amount })
}

pub(crate) fn parse_budget_request(s: String) -> Option<SpentRequest> {
    parse_budget_and_amount(s.as_str())
        .finish()
        .ok()
        .map(|(_, amount)| SpentRequest {
            amount,
            category: None,
        })
}

pub(crate) fn is_spending_total_request(s: String) -> bool {
    parse_spending_total_request(s.as_str()).finish().is_ok()
}

pub(crate) fn is_spending_reset_request(s: String) -> bool {
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
    map_res(
        recognize(pair(digit1, pair(opt(char('.')), digit0))),
        |n: &str| n.parse(),
    )(s)
}

fn parse_station_and_direction(s: &str) -> IResult<&str, (Direction, Station)> {
    separated_pair(parse_direction, space1, parse_station)(s)
}

fn parse_direction(s: &str) -> IResult<&str, Direction> {
    map_res(
        alt((tag_no_case("west"), tag_no_case("east"))),
        Direction::try_from,
    )(s)
}

fn parse_station(s: &str) -> IResult<&str, Station> {
    map_res(
        alt((
            tag_no_case("lambert"),
            tag_no_case("lambert2"),
            tag_no_case("hanley"),
            tag_no_case("umsl north"),
            tag_no_case("umsl"),
            tag_no_case("umsl south"),
            tag_no_case("rock road"),
            tag_no_case("wellston"),
            tag_no_case("delmar"),
            tag_no_case("shrewsbury"),
            tag_no_case("sunnen"),
            tag_no_case("maplewood"),
            tag_no_case("brentwood"),
            tag_no_case("richmond"),
            tag_no_case("clayton"),
            tag_no_case("forsyth"),
            tag_no_case("ucity"),
            tag_no_case("skinker"),
            tag_no_case("forest park"),
            tag_no_case("cwe"),
            alt((
                tag_no_case("central west end"),
                tag_no_case("cortex"),
                tag_no_case("grand"),
                tag_no_case("union"),
                tag_no_case("civic"),
                tag_no_case("stadium"),
                tag_no_case("8th pine"),
                tag_no_case("8th and pine"),
                tag_no_case("convention"),
                tag_no_case("lacledes"),
                tag_no_case("lacledes landing"),
                tag_no_case("riverfront"),
                tag_no_case("5th missouri"),
                tag_no_case("fifth missouri"),
                tag_no_case("emerson"),
                tag_no_case("jjk"),
                tag_no_case("jackie joiner"),
                tag_no_case("washington"),
                tag_no_case("fvh"),
                alt((
                    tag_no_case("memorial"),
                    tag_no_case("memorial hospital"),
                    tag_no_case("swansea"),
                    tag_no_case("belleville"),
                    tag_no_case("college"),
                    tag_no_case("shiloh"),
                    tag_no_case("shiloh scott"),
                )),
            )),
        )),
        Station::try_from,
    )(s)
}

fn parse_category(s: &str) -> IResult<&str, Category> {
    map(
        alt((
            tag_no_case("dining"),
            tag_no_case("grocery"),
            tag_no_case("merchandise"),
            tag_no_case("travel"),
            tag_no_case("entertainment"),
            tag_no_case("other"),
        )),
        Category::from,
    )(s) //anychar?
}
