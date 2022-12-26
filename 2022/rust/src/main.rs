use std::env::{self, Args};

mod aoc2022;
pub mod util;

fn main() {
    let mut args = env::args();
    args.next();

    // args.into_iter()
    //     .enumerate()
    //     .for_each(|(i, d)| println!("{}: {}", i, d));

    let day = parse_or_request(&mut args, "Please enter a day: ");

    println!("Executing task for Year: 2022; Day: {}", day);

    aoc2022::day(day);
}

fn parse_or_request(args: &mut Args, message: &str) -> u16 {
    if let Some(data) = args.next() {
        data
    } else {
        println!("{}", message);
        util::read_line()
    }
    .parse::<u16>()
    .unwrap()
}
