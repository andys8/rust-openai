use async_openai::{types::CreateCompletionRequestArgs, Client};
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
    input: String,
}

#[tokio::main]
async fn main() {
    let OpenAiCli::Run(args) = OpenAiCli::parse();
    println!(">> {:?}", args.input);
    run_completion(args.input).await;
}

async fn run_completion(input: String) {
    let request = CreateCompletionRequestArgs::default()
        .model("text-davinci-003")
        .prompt(input)
        .max_tokens(40_u16)
        .build()
        .unwrap();

    let response = Client::new().completions().create(request).await;

    match response {
        Ok(r) => println!("{}", r.choices.first().unwrap().text),
        Err(error) => panic!("Problem executing request: {:?}", error),
    };
}
