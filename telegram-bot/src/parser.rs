use anyhow::{anyhow, bail, Result};
use chrono::format::parse;
use metro_schedule::{Direction, NextArrivalRequest, Station};
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::space1;
use nom::combinator::map_res;
use nom::sequence::separated_pair;
use nom::{Finish, IResult};

pub(crate) fn parse_metro_request(s: &str) -> Option<NextArrivalRequest> {
    if let Ok((_, (direction, station))) = parse_station_and_direction(s).finish() {
        return Some(NextArrivalRequest { station, direction });
    }
    None
}

fn parse_station_and_direction(s: &str) -> IResult<&str, (Direction, Station)> {
    separated_pair(parse_direction, space1, parse_station)(s)
}

fn parse_direction(s: &str) -> IResult<&str, Direction> {
    map_res(alt((tag_no_case("west"), tag_no_case("east"))), Direction::try_from)(s)
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
