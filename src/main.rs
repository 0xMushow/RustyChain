mod block;
mod blockchain;

use crate::blockchain::Blockchain;

fn main() {
    let mut chain = Blockchain::new(2);

    chain.add_block("Hello, RustyChain!".into());
    chain.add_block("Another block".into());
    println!("Blockchain: {:#?}", chain);
}