use clap::Parser;

#[derive(Parser)]
#[command(name = "openai")]
#[command(bin_name = "openai")]
enum OpenAiCli {
    OpenAiCliArgs(Args),
}

#[derive(clap::Args)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    manifest_path: Option<std::path::PathBuf>,
}

fn main() {
    let OpenAiCli::OpenAiCliArgs(args) = OpenAiCli::parse();
    println!("{:?}", args.manifest_path);
}
