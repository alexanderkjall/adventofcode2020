mod day1;
mod day2;
mod day3;
mod day4;

use clap::Clap;
use anyhow::anyhow;

#[derive(Clap)]
#[clap(version = "1.0", author = "Kevin K. <kbknapp@gmail.com>")]
struct Opts {
    #[clap(short, long, default_value = "1")]
    day: u8,
}

fn main() -> Result<(), anyhow::Error> {
    let opts: Opts = Opts::parse();

    let (part1, part2):(String, String) = match opts.day {
        1 => day1::run()?,
        2 => day2::run()?,
        3 => day3::run()?,
        4 => day4::run()?,
        _ => Err(anyhow!("illegal day number"))?,
    };

    println!("result day {} part 1 {}", opts.day, part1);
    println!("result day {} part 2 {}", opts.day, part2);

    Ok(())
}
