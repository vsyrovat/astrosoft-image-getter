use ::clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, default_value = "turtles.txt")]
    pub file: String,

    #[arg(short, long, default_value = "out")]
    pub outdir: String,

    #[arg(short, long, default_value_t = 5)]
    pub threads: usize,
}
