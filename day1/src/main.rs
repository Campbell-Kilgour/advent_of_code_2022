use std::fs;

fn main() {
    let file_path = String::from("day1_input");

    let file_contents = fs::read_to_string(file_path).expect("Unable to read file contents!");
    let line_vector = file_contents.lines(); //split("\n");
    let mut elf_pos = 0;
    let first_elf = Elf {
        items: Vec::new(),
        position: elf_pos,
    };

    let mut elfs = vec![first_elf];

    for line in line_vector {
        match line.is_empty() {
            true => elfs.push(Elf {
                items: Vec::new(),
                position: elf_pos,
            }),
            false => {
                // Find newest Elf
                let index = elfs.len();
                let latest_elf = &mut elfs[index - 1];
                latest_elf
                    .items
                    .push(line.parse().expect("Failed to parse line to int!"));
            }
        }

        elf_pos = elf_pos + 1;
    }

    let mut best_elf = Elf {
        items: Vec::new(),
        position: 0,
    };

    for elf in elfs {
        if best_elf.sum_cals() < elf.sum_cals() {
            best_elf = elf;
        }
    }

    let best_elf_index = best_elf.position;
    let best_elf_cals = best_elf.sum_cals();

    println!("The Elf with the most calories is elf number {best_elf_index} with {best_elf_cals} calories!");
}

struct Elf {
    items: Vec<i32>,
    position: i32,
}

impl Elf {
    fn sum_cals(&self) -> i32 {
        let mut total = 0;

        for line in &self.items {
            total += line;
        }

        return total;
    }
}
