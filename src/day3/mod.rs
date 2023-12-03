use crate::parser::{read_input, InputType};

pub fn run () {
    println!("DAY 3");

    let engine_schematic: Vec<Vec<char>> = read_input(3, &InputType::Full)
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();

    part_one(engine_schematic);
    // let second_total = part_two(games);

    // println!("part A total valid games is {first_total}");
    // println!("part B total combination is {second_total}");
    println!("\n");
}

#[derive(Debug, Clone, PartialEq)]

struct Part {
    value: usize,
    symbol: Symbol,
}

impl Part {
    const fn new(value: usize, symbol: Symbol) -> Self {
        Self { value, symbol }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Symbol {
    id: String,
    position: (usize, usize),
    value: char
}

impl Symbol {
    fn new((x, y): (usize, usize), value: char) -> Self {
        let id = format!("{x}--{y}");
        Self { id, position: (x, y), value }
    }

    const fn is_gear_symbol(&self) -> bool {
        self.value == '*'
    }
}

#[derive(Debug, Clone)]
struct Gear {
    id: String,
    numbers: Vec<usize>
}

impl Gear {
    fn new(id: String, numbers: Vec<usize>) -> Self {
        Self { id, numbers }
    }
}

fn part_one(engine_schematic: Vec<Vec<char>>) {
    let (parts, _, _) = engine_schematic.clone()
        .into_iter()
        .enumerate()
        .fold((vec![], (None, String::new()), false), |(mut parts, (mut current_symbol, mut current_number), mut already_a_part): (Vec<Part>, (Option<Symbol>, String), bool), (y, line)| {
            line
                .into_iter()
                .enumerate()
                .for_each(|(x, chr)| {
                    if chr.is_numeric() {
                        let surrounding_symbols = get_surrounding_symbols(engine_schematic.clone(), (x, y));    
                        let is_part = !surrounding_symbols.is_empty();

                        if is_part {
                            already_a_part = true;
                            current_symbol = Some(surrounding_symbols[0].clone());
                        }
                        
                        current_number.push(chr);
                    } else {
                        if already_a_part {
                            parts.push(Part::new(current_number.parse().expect("int not valid"), current_symbol.clone().unwrap()));
                            already_a_part = false;
                        }
                        current_number = String::new();
                        current_symbol = None;
                    }
                });


            (parts, (current_symbol, current_number), already_a_part)
        });

        // println!("{parts:?}");

        // let sum: usize = parts.into_iter().map(|part| part.value).sum();
        // println!("part one is: {sum}");

        let gears: Vec<Gear> = parts
            .clone()
            .into_iter()
            .filter(|part| part
                .symbol.is_gear_symbol() && parts
                .clone()
                .into_iter()
                .filter(|p| p.symbol.id == part.symbol.id)
                .count() > 1
            )
            .fold(vec![], |mut gears: Vec<Gear>, part: Part| {
                println!("in fold");
                let is_already_in = gears.clone().into_iter().position(|gear| gear.id == part.symbol.id);

                if let Some(index) = is_already_in {
                    println!("is in");
                    gears[index].numbers.push(part.value);
                } else {
                    println!("not in");
                    gears.push(Gear::new(part.symbol.id, vec![part.value]));
                }

                gears
            });

        println!("{gears:?}");

        let sum: usize = gears.into_iter().map(|gear| gear.numbers[0] * gear.numbers[1]).sum();

        println!("{sum:?}");
}

fn get_surrounding_symbols(engine_schematic: Vec<Vec<char>>, (x, y): (usize, usize)) -> Vec<Symbol> {
    let mut symbols: Vec<Symbol> = vec![];

    if y > 0 {
        if x > 0 {
            let top_left = engine_schematic.get(y - 1).and_then(|row| row.get(x - 1)).unwrap();
            if is_special_char(*top_left) {
                symbols.push(Symbol::new((y - 1, x - 1), *top_left));
            }
        }

        let top = engine_schematic.get(y - 1).and_then(|row| row.get(x)).unwrap();

        if is_special_char(*top) {
            symbols.push(Symbol::new((y - 1, x), *top));
        }

        if x + 1 < engine_schematic[0].len() {
            let top_right = engine_schematic.get(y - 1).and_then(|row| row.get(x + 1)).unwrap();

            if is_special_char(*top_right) {
                symbols.push(Symbol::new((y - 1, x + 1), *top_right));
            }
        }
    }

    if x > 0 {
        let left = engine_schematic.get(y).and_then(|row| row.get(x - 1)).unwrap();
        if is_special_char(*left) {
            symbols.push(Symbol::new((y, x - 1), *left));
        }

        if y + 1 < engine_schematic.len() {
            let bottom_left = engine_schematic.get(y + 1).and_then(|row| row.get(x - 1)).unwrap();
            if is_special_char(*bottom_left) {
                symbols.push(Symbol::new((y + 1, x - 1), *bottom_left));
            }
        }

    }

    if x + 1 < engine_schematic[0].len() {
        let right = engine_schematic.get(y).and_then(|row| row.get(x + 1)).unwrap();
        if is_special_char(*right) {
            symbols.push(Symbol::new((y, x + 1), *right));
        }
    }

    if y + 1 < engine_schematic.len() {
        let bottom = engine_schematic.get(y + 1).and_then(|row| row.get(x)).unwrap();
        if is_special_char(*bottom) {
            symbols.push(Symbol::new((y + 1, x), *bottom));
        }


        if x + 1 < engine_schematic[0].len() {
            let bottom_right = engine_schematic.get(y + 1).and_then(|row| row.get(x + 1)).unwrap();
            if is_special_char(*bottom_right) {
                symbols.push(Symbol::new((y + 1, x + 1), *bottom_right));
            }
        }
    }


        
    symbols
}

fn is_special_char(chr: char) -> bool {
    !chr.is_numeric() && chr != '.'
}

// const fn is_gear_part(chr: char) -> bool {
//     chr == '*'
// }