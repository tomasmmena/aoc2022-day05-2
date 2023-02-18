use std::env;
use std::fs;
use std::io::{BufRead, self};
use regex::Regex;


struct Crate {
    label: char
}

struct CrateMove {
    from: usize,
    to: usize,
    amount: usize
}

enum CraneType {
    CrateMover9000,
    CrateMover9001
}

// introduce lifetime here
struct CrateStacks {
    index: Vec<Vec<Crate>>,
    crane_type: CraneType
}

#[derive(Debug)]
enum InvalidMove {
    SourceOutOfRange,
    DestionationOutOfRange,
    NotEnoughCrates
}

impl CrateStacks {
    fn apply_move(&mut self, crate_move: CrateMove) -> Result<(), InvalidMove> {
        
        match self.crane_type {
            CraneType::CrateMover9000 => {
                for _ in (0..crate_move.amount).into_iter() {
                    let crate_aux = {
                        let source = self.index.get_mut(crate_move.from).ok_or(InvalidMove::SourceOutOfRange)?;
                        source.pop().ok_or(InvalidMove::NotEnoughCrates)?
                    };
                    let destination = self.index.get_mut(crate_move.to).ok_or(InvalidMove::DestionationOutOfRange)?;
                    destination.push(crate_aux);
                }
            },
            CraneType::CrateMover9001 => {
                let mut aux_vec: Vec<Crate> = vec![];
                {
                    let source = self.index.get_mut(crate_move.from).ok_or(InvalidMove::SourceOutOfRange)?;
                    for _ in (0..crate_move.amount).into_iter() {
                        aux_vec.push(source.pop().ok_or(InvalidMove::NotEnoughCrates)?)
                    }
                }
                let destination = self.index.get_mut(crate_move.to).ok_or(InvalidMove::DestionationOutOfRange)?;
                for _ in (0..crate_move.amount).into_iter() {
                    destination.push(aux_vec.pop().unwrap());
                }
            }
        }
        
        Ok(())
    }

    fn show(&self) {
        let mut level: usize = 0;
        let mut done = false;
        let mut lines: Vec<String> = vec![];

        while !done {
            done = true;
            let mut new_line = String::new();

            for stack in self.index.iter() {
                if let Some(c) = stack.get(level) {
                    done = false;
                    new_line = new_line + format!(" [{}] ", c.label).as_str();
                } else {
                    new_line = new_line + "     "
                }
            }

            level += 1;
            lines.push(new_line);
        }

        lines.reverse();
        for line in lines {
            println!("{}", line);
        }
        
    }

    fn top_crates(&self) -> String {
        let mut value = String::new();
        for stack in self.index.iter() {
            value.push(
                match stack.last() {
                    Some(crat) => crat.label,
                    _ => ' '
                }
            );
        }
        value
    }
}


fn main() {
    let path = env::args().nth(1).expect("No file path provided.");

    let data: Vec<String> = io::BufReader::new(
        fs::File::open(path).expect("Could not open file."))
        .lines()
        .map(| line | line.unwrap_or("".to_string()))
        .collect();

    let re_crates = Regex::new(r"(\s{3}|\[[A-Z]\])\s?").unwrap();

    let mut crate_lines = Vec::from_iter(    
        data.iter().filter(| line | re_crates.is_match(&line))
    );

    crate_lines.reverse();
    let mut stacks = CrateStacks {
        index: vec![],
        crane_type: CraneType::CrateMover9001
    };

    for line in crate_lines {
        let captures = re_crates.captures_iter(&line);
        for (i, capture) in captures.enumerate() {
            if stacks.index.len() <= i {
                stacks.index.push(vec![]);
            }
            if let Some(Some(value)) = capture.iter().nth(1) {
                if value.as_str().ends_with("]") && value.as_str().starts_with("[") {
                    stacks.index.get_mut(i).unwrap().push(
                        Crate {
                            label: value.as_str().chars().nth(1).unwrap()
                        }
                    );   
                }
            }
        }
    }

    let re_moves = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let moves = data
        .iter()
        .filter(| line | re_moves.is_match(&line))
        .map(|line| {
            let captures = re_moves.captures(line.as_str()).unwrap();
            CrateMove {
                from: captures.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1,
                to: captures.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1,
                amount: captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            }
        });

    stacks.show();
    for m in moves {
        stacks.apply_move(m).expect("Invalid move!");
    }
    stacks.show();

    println!("Final state: {}", stacks.top_crates());
}
