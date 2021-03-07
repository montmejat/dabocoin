mod blockchain;
use blockchain::{wallet::Wallet, chain::Chain};

fn main() {
    println!("Starting program!");

    let mut chain = Chain::new();
    let aurelien_wallet = Wallet::new();
    let plouf_wallet = Wallet::new();

    aurelien_wallet.send_money(&mut chain, 42.0, plouf_wallet.public_key.clone());
    plouf_wallet.send_money(&mut chain, 66.6, aurelien_wallet.public_key.clone());
}
