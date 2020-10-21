fn main() {
    for num in (0..=99).rev() {
        verse(num)
    }
}

fn verse(num: i16) {
    println!("{} on the wall", beers_on_wall(num));
    println!("{}", beers_on_wall(num));
    match num {
        0 => {
            println!("Go to the store, buy some more");
            println!("{} on the wall", beers_on_wall(99));
        }
        _ => {
            println!("Take one down, pass it around");
            println!("{} on the wall", beers_on_wall(num - 1))
        }
    };
    println!();
}

fn beers_on_wall(num: i16) -> String {
    match num {
        2..=99 => format!("{} bottles of beer", num),
        1 => format!("1 bottle of beer"),
        0 => format!("No more bottles of beer"),
        _ => panic!("Enexpected number of beers"),
    }
}
