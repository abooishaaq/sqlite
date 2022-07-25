mod meta;
mod table;

use meta::*;
use table::*;

fn main() {
    let mut table = Table {
        len: 0,
        memory: [0; 4096 * ROW_SIZE]
    };
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input.chars().count() == 0 {
            break;
        }
        if input.chars().next().unwrap() == '.' {
            metacmd(input[1..].trim());
        } else {
            execute(input, &mut table);
        }
    }
}
