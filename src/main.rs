mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use anyhow::anyhow;
use clap::Clap;

#[derive(Clap)]
#[clap(
    version = "1.0",
    author = "Alexander Kjäll <alexander.kjall@gmail.com>"
)]
struct Opts {
    #[clap(short, long, default_value = "1")]
    day: u8,
}

fn main() -> Result<(), anyhow::Error> {
    let opts: Opts = Opts::parse();

    let (part1, part2): (String, String) = match opts.day {
        1 => day1::run()?,
        2 => day2::run()?,
        3 => day3::run()?,
        4 => day4::run()?,
        5 => day5::run()?,
        6 => day6::run()?,
        7 => day7::run()?,
        8 => day8::run()?,
        9 => day9::run()?,
        10 => day10::run()?,
        11 => day11::run()?,
        12 => day12::run()?,
        13 => day13::run()?,
        _ => return Err(anyhow!("illegal day number")),
    };

    println!("result day {} part 1 {}", opts.day, part1);
    println!("result day {} part 2 {}", opts.day, part2);

    Ok(())
}
