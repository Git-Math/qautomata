use clap::Args;

#[derive(Args, Debug)]
pub struct GenCmd {
    /// generate a state, which consist of a list of configurations
    #[clap(short, long, value_parser)]
    state: bool,

    /// store generated data in a Json file
    #[clap(short, long, value_parser)]
    out: Option<String>,
}

pub fn generate(cmd: &GenCmd) {
    println!("{:#?}", cmd) // Debug
}
