use std::str::FromStr;
use bitcoin::{Address, Network};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[clap(short, long, default_value_t = Network::Bitcoin)]
    from_network: Network,
    #[clap(short, long, default_value_t = Network::Testnet)]
    to_network: Network,
    address: String,
}

fn main() {
    let cli = Cli::parse();
    let from_network = cli.from_network;
    let to_network = cli.to_network;

    let address = Address::from_str(&cli.address).unwrap_or_else(|_| {
        eprintln!("Invalid address");
        std::process::exit(1);
    }).require_network(from_network).unwrap_or_else(|_| {
        eprintln!("Invalid from_network for the provided address");
        std::process::exit(1);
    });

    let address = Address::from_script(&address.script_pubkey(), to_network).unwrap_or_else(|_| {
        eprintln!("Invalid to_network for the provided address");
        std::process::exit(1);
    });

    println!("{}", address);

}
