mod cli;

use clap::Parser;
use dotenv::dotenv;
use std::env;
use tokio;
use cli::Cli;



#[tokio::main()]
async fn main(){
    let mcli = Cli::parse();
    dotenv().ok();
    let url = env::var("URL").expect("URL not set");
    let token = env::var("TOKEN").expect("TOKEN not set");

}
