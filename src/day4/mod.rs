use crate::parser::{read_input, InputType};

#[derive(Debug, Clone)]
struct Scratchcard {
    id: usize,
    winning_numbers: Vec<usize>,
    personal_numbers: Vec<usize>
}

impl Scratchcard {
    fn new(id: &str, winning_numbers: Vec<usize>, personal_numbers: Vec<usize>) -> Self {
        Self {
            id: id.parse().expect("id not valid"),
            winning_numbers,
            personal_numbers
        }
    }

    fn from_row(row: String) -> Self {
        let binding = row
            .split(':')
            .collect::<Vec<&str>>();

        let (raw_id, raw_numbers) = binding
            .split_first()
            .unwrap();

        let id = raw_id
            .split(' ')
            .last()
            .unwrap();

        let numbers: Vec<Vec<usize>> = raw_numbers
            .first()
            .and_then(|numbers| numbers
                .split('|')
                .map(|n_list| n_list
                    .split(' ')
                    .filter(|n| !n.is_empty())
                    .map(|n| n.parse().ok())
                    .collect()
                ).collect()
            ).expect("numbers not valid");
        
        Self::new(id,numbers[0].clone(), numbers[1].clone())
    }

    fn get_personal_winning_numbers(&self) -> Vec<usize> {
        self.personal_numbers
            .clone()
            .into_iter()
            .filter_map(|n| {
                self.winning_numbers.clone().into_iter().find(|w| n == *w)
            }).collect()
    }
}

pub fn run () {
    println!("DAY 4");

    let scratchcards: Vec<Scratchcard> = read_input(4, &InputType::Full)
        .into_iter()
        .map(Scratchcard::from_row)
        .collect();

    let a = part_one(&scratchcards);
    let b = part_two(&scratchcards);

    println!("part one: {a}");
    println!("part two: {b}");
}

fn part_one(cards: &Vec<Scratchcard>) -> usize {
    cards
        .iter()
        .map(Scratchcard::get_personal_winning_numbers)
        .filter(|arr| !arr.is_empty())
        .map(|scratchcard| scratchcard
            .into_iter()
            .enumerate()
            .fold(0, |total, (i, _)| (
                if i == 0 { 1 } else { total * 2 }
            ))
        )
        .sum()
}

fn part_two(cards: &Vec<Scratchcard>) -> u32 {
    let mut queue = cards.clone();
    let mut total_cards = 0;

    while let Some(card) = queue.pop() {
        let copy_number: usize = card.get_personal_winning_numbers().len();
        total_cards += 1;

        if copy_number > 0 {
            let start = card.id + 1;
            let end = start + copy_number;

            (start..end).for_each(|i| {
                queue.push(cards[i - 1].clone());
            });
        }
    }

    total_cards
}