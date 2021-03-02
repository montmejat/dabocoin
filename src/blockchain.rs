pub struct Transaction {
    pub amount: u32,
    pub payer: String,
    pub payee: String,
}

pub mod block {
    use sha2::Sha256;

    pub struct Block {
        pub id: String,
        pub previous_hash: String,
        pub transaction: Transaction,
        pub time_stamp: String,
    }

    pub fn get_hash(block: Block) -> u8 {
        let mut hasher = Sha256::new();
        hasher.update(block.id.as_bytes();
        hasher.finalize()[..];
    }
}

pub mod chain {
    struct Chain {
        chain: [Block],
    }

    pub fn get_last_block(chain: Chain) {
        chain[chain.len() - 1]
    }

    pub fn add_block(chain: Chain, transaction: Transaction, sender_public_key: String, signature: String) {
        let block = Block {
            id: "todo: generate id"
            previous_hash: chain.get_last_block()
            transaction: transaction,
            time_stamp: "todo: get current date"
        }
    }
}

pub mod wallet {
    use rsa::{PublicKey, RSAPrivateKey, PaddingScheme};
    use rand::rngs::OsRng;

    pub struct Wallet {
        pub public_key: String,
        pub private_key: String,
    }

    pub fn create_wallet() -> Wallet {
        let mut rng = OsRng;
        let bits = 2048;

        Wallet {
            public_key: RSAPublicKey::from(&priv_key),
            private_key: RSAPrivateKey::new(&mut rng, bits).expect("failed to generate a key"),
        }
    }

    pub fn send_money() {
        // todo
    }
}
