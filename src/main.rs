mod day1;

use clap::Clap;

#[derive(Clap)]
#[clap(version = "1.0", author = "Kevin K. <kbknapp@gmail.com>")]
struct Opts {
    #[clap(short, long, default_value = "1")]
    day: u8,
}

fn main() -> Result<(), anyhow::Error> {
    let opts: Opts = Opts::parse();

    match opts.day {
        1 => day1::run()?,
        _ => eprintln!("illegal day number"),
    };

    Ok(())
}
