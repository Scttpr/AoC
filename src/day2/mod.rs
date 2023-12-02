use crate::parser::{read_input, InputType};

pub fn run () {
    println!("DAY 2");

    let games = read_input(2, &InputType::Full)
        .into_iter()
        .filter_map(|line| Game::from_str(&line));

    let first_total = part_one(games.clone());
    let second_total = part_two(games);

    println!("part A total valid games is {first_total}");
    println!("part B total combination is {second_total}");
    println!("\n");
}
#[derive(Debug, Clone)]
struct Game {
    id: u32,
    sets: Vec<Vec<Draw>>,
}

impl Game {
    fn from_str(str: &str) -> Option<Self> {
        let binding = str
            .split(&[':', ';'])
            .collect::<Vec<&str>>();
        let (id_split, sets_split) = binding
            .split_first()?;

        let id: u32 = id_split
            .split(' ')
            .collect::<Vec<&str>>()
            .get(1)?
            .parse()
            .expect("int is not valid");

        let sets: Vec<Vec<Draw>> = sets_split
            .iter()
            .map(|set| {
                set
                    .split(',')
                    .filter_map(|draw| {
                        Draw::from_prepared_str(draw.trim())
                    })
                    .collect::<Vec<Draw>>()
            })
            .collect();

        Some(
            Self {
                id,
                sets,
            }
        )

    }
}

#[derive(Debug, Clone)]
struct Draw {
    color: Color,
    value: u32,
}

impl Draw {
    fn from_prepared_str(str: &str) -> Option<Self> {
        let splits: Vec<&str> = str.split(' ').collect();
        let value = splits.first()?;
        let color = splits.get(1)?;
       
        
        Some(
            Self {
                color: Color::from_str(color)?,
                value: value.parse().expect("int not valid"),
            }
        )
    }

    const fn match_requirement(&self, requirement: u32) -> bool {
        self.value <= requirement
    }
}

#[derive(Debug, Clone)]
enum Color {
    Blue, Green, Red
}

impl Color {
    fn from_str(str: &str) -> Option<Self> {
        match str {
            "blue" => Some(Self::Blue),
            "red" => Some(Self::Red),
            "green" => Some(Self::Green),
            _ => None
        }
    }
}

fn part_one(games: impl Iterator<Item= Game>) -> u32 {
    games
        .filter(is_possible)
        .map(|game| game.id)
        .sum()
}

fn is_possible(game: &Game) -> bool {
    let max_blue = 14;
    let max_green = 13;
    let max_red = 12;

    game.sets.clone()
        .into_iter()
        .map(|set| {
            set
                .into_iter()
                .map(|draw| {
                    match draw.color {
                        Color::Red => draw.match_requirement(max_red),
                        Color::Green => draw.match_requirement(max_green),
                        Color::Blue => draw.match_requirement(max_blue),
                    }
                })
                .all(|is_valid_draw| is_valid_draw)
        })
        .all(|is_valid_set| is_valid_set)
}

fn part_two(games: impl Iterator<Item= Game>) -> u32 {
    games
        .map(|game| {
            let (red, green, blue) = game.sets
                .into_iter()
                .fold((0, 0, 0), |(mut red, mut green, mut blue): (u32, u32, u32), set| {
                    for draw in set {
                        match draw.color {
                            Color::Red => {
                                if draw.value > red {
                                    red = draw.value;
                                }
                            },
                            Color::Green =>{
                                if draw.value > green {
                                    green = draw.value;
                                }
                            },
                            Color::Blue => {
                                if draw.value > blue {
                                    blue = draw.value;
                                }
                            },

                        };
                    }

                    (red, green, blue)
                });

                red * green * blue
        })
        .sum()
}