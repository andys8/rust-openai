use clap::Parser;

#[derive(Parser)]
#[command(name = "openai")]
#[command(bin_name = "openai")]
enum OpenAiCli {
    Run(Args),
}

#[derive(clap::Args)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    input: Option<String>,
}

fn main() {
    let OpenAiCli::Run(args) = OpenAiCli::parse();
    println!("{:?}", args.input);
}
