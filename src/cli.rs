use clap::{Parser, ValueEnum};

#[derive(Parser)]
pub struct Cli {
    #[clap(short, long)]
    pub input: Input,

    #[clap(default_value_t = 0)]
    pub day: usize,
}

#[derive(ValueEnum, Clone, Default)]
pub enum Input {
    #[default]
    Primary,
    Secondary,
}
