use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::u32,
    combinator::map,
    multi::separated_list1,
    sequence::{terminated, tuple},
    IResult,
};

day!(Game, calc2);

use crate::common::Day;

pub struct Game;

#[derive(Debug, PartialEq, Eq)]
pub struct Games(Vec<G>);

#[derive(Debug, PartialEq, Eq)]
pub struct G {
    id: u32,
    rounds: Vec<Round>,
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl Day for Game {
    type Input = Games;
    type Output = usize;

    fn parse(buf: &[u8]) -> Self::Input {
        parse_games(buf).unwrap().1
    }

    fn calc(input: Self::Input) -> Self::Output {
        let b = Round {
            red: 12,
            green: 13,
            blue: 14,
        };

        let mut sum = 0;
        for g in input.0 {
            if g.rounds.iter().all(|r| !blown(&b, r)) {
                sum += g.id;
            }
        }

        sum as usize
    }

    fn calc2(input: Self::Input) -> Self::Output {
        let mut sum = 0;

        for g in input.0 {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for r in g.rounds {
                red = red.max(r.red);
                green = green.max(r.green);
                blue = blue.max(r.blue);
            }

            sum += red * green * blue;
        }

        sum as usize
    }
}

fn blown(bound: &Round, space: &Round) -> bool {
    space.red > bound.red || space.green > bound.green || space.blue > bound.blue
}

fn parse_games(buf: &[u8]) -> IResult<&[u8], Games> {
    let (buf, games) = nom::multi::many1(parse_game)(buf)?;

    Ok((buf, Games(games)))
}

fn parse_game(buf: &[u8]) -> IResult<&[u8], G> {
    let (buf, _) = tag(b"Game ")(buf)?;
    let (buf, id) = u32(buf)?;
    let (buf, _) = tag(b": ")(buf)?;
    let (buf, rounds) = terminated(separated_list1(tag(b"; "), parse_round), tag(b"\n"))(buf)?;

    Ok((buf, G { id, rounds }))
}

impl std::ops::AddAssign for Round {
    fn add_assign(&mut self, rhs: Self) {
        self.red += rhs.red;
        self.green += rhs.green;
        self.blue += rhs.blue;
    }
}

fn parse_round(buf: &[u8]) -> IResult<&[u8], Round> {
    enum Dim {
        Red,
        Green,
        Blue,
    }

    fn parse_dim(buf: &[u8]) -> IResult<&[u8], Dim> {
        let (buf, dim) = alt((tag(b"blue"), tag(b"red"), tag(b"green")))(buf)?;
        match dim {
            b"red" => Ok((buf, Dim::Red)),
            b"green" => Ok((buf, Dim::Green)),
            b"blue" => Ok((buf, Dim::Blue)),
            _ => unreachable!(),
        }
    }

    let parse_round_dim = map(
        tuple((u32, tag(b" "), parse_dim)),
        |(mag, _, dim)| match dim {
            Dim::Red => Round {
                red: mag,
                green: 0,
                blue: 0,
            },

            Dim::Green => Round {
                green: mag,
                red: 0,
                blue: 0,
            },

            Dim::Blue => Round {
                blue: mag,
                red: 0,
                green: 0,
            },
        },
    );

    let (buf, sub_rounds) = separated_list1(tag(b", "), parse_round_dim)(buf)?;

    let mut acc = Round::default();

    for sub in sub_rounds {
        acc += sub;
    }

    Ok((buf, acc))
}

#[test]
fn test_parse() {
    let input = Game::parse(include_bytes!("example.txt"));

    assert_eq!(
        input,
        Games(vec![
            G {
                id: 1,
                rounds: vec![
                    Round {
                        red: 4,
                        green: 0,
                        blue: 3,
                    },
                    Round {
                        red: 1,
                        green: 2,
                        blue: 6,
                    },
                    Round {
                        red: 0,
                        green: 2,
                        blue: 0,
                    }
                ]
            },
            G {
                id: 2,
                rounds: vec![
                    Round {
                        blue: 1,
                        green: 2,
                        red: 0
                    },
                    Round {
                        green: 3,
                        blue: 4,
                        red: 1
                    },
                    Round {
                        green: 1,
                        blue: 1,
                        red: 0
                    },
                ]
            },
            G {
                id: 3,
                rounds: vec![
                    Round {
                        green: 8,
                        blue: 6,
                        red: 20
                    },
                    Round {
                        blue: 5,
                        red: 4,
                        green: 13
                    },
                    Round {
                        green: 5,
                        red: 1,
                        blue: 0
                    },
                ]
            },
            G {
                id: 4,
                rounds: vec![
                    Round {
                        green: 1,
                        red: 3,
                        blue: 6
                    },
                    Round {
                        green: 3,
                        red: 6,
                        blue: 0
                    },
                    Round {
                        green: 3,
                        blue: 15,
                        red: 14
                    },
                ]
            },
            G {
                id: 5,
                rounds: vec![
                    Round {
                        red: 6,
                        blue: 1,
                        green: 3
                    },
                    Round {
                        blue: 2,
                        red: 1,
                        green: 2
                    },
                ]
            },
        ])
    );
}

#[test]
fn example() {
    let input = Game::parse(include_bytes!("example.txt"));
    let output = Game::calc(input);
    assert_eq!(8, output);
}

#[test]
fn example2() {
    let input = Game::parse(include_bytes!("example.txt"));
    let output = Game::calc2(input);
    assert_eq!(2286, output);
}
