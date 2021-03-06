pub mod transaction {
    use rsa::RSAPublicKey;
    use rsa::PublicKeyParts;

    pub struct Transaction {
        pub amount: f32,
        pub payer: RSAPublicKey,
        pub payee: RSAPublicKey,
    }

    impl Transaction {
        pub fn to_string(&self) -> String {
            String::from(self.amount.to_string() + &self.payer.n().to_str_radix(16) + &self.payee.n().to_str_radix(16))
        }
    }
}

pub mod block {
    use super::transaction::Transaction;
    use sha2::Digest;
    use sha2::Sha256;

    pub struct Block {
        pub previous_hash: String,
        pub transaction: Transaction,
        pub time_stamp: String,
    }

    impl Block {
        pub fn new(previous_hash: String, transaction: Transaction, time_stamp: String) -> Block {
            Block {
                previous_hash: previous_hash,
                transaction: transaction,
                time_stamp: time_stamp,
            }
        }

        // fn get_hash(&self) -> &[u8] {
        //     let mut hasher = Sha256::new();
        //     let value = &self.transaction.amount.to_string();
        //     hasher.update(value.as_bytes());
        //     &hasher.finalize()[..]
        // }
    }
}

pub mod chain {
    use super::transaction::Transaction;
    use super::block::Block;
    use rsa::{RSAPublicKey, PublicKey, PaddingScheme};
    use std::collections::LinkedList;

    pub struct Chain {
        chain: LinkedList<Block>,
    }

    impl Chain {
        pub fn new() -> Chain {
            let chain: LinkedList<Block> = LinkedList::new();
            Chain {
                chain: chain,
            }
        }

        fn get_last_block(&self) -> Option<&Block> {
            self.chain.back()
        }

        pub fn add_block(&mut self, transaction: Transaction, sender_public_key: &RSAPublicKey, signature: Vec<u8>) {
            let padding = PaddingScheme::new_pkcs1v15_encrypt();
            match sender_public_key.verify(padding, transaction.to_string().as_bytes(), &signature) {
                Ok(_) => {
                    let block = Block::new(String::from("todo"), transaction, String::from("todo"));
                    self.chain.push_back(block);
                },
                Err(_) => print!("Could not add the block the chain."),
            }
        }
    }
}

pub mod wallet {
    use super::transaction::Transaction;
    use super::chain::Chain;
    use rsa::{PublicKey, RSAPrivateKey, RSAPublicKey, PaddingScheme};
    use rand::rngs::OsRng;

    pub struct Wallet {
        pub public_key: RSAPublicKey,
        pub private_key: RSAPrivateKey,
    }

    impl Wallet {
        pub fn new() -> Wallet {
            let bits = 2048;
            let private_key = RSAPrivateKey::new(&mut OsRng, bits).expect("failed to generate a key");

            Wallet {
                public_key: RSAPublicKey::from(&private_key),
                private_key: private_key,
            }
        }

        pub fn send_money(& self, chain: &mut Chain, amount: f32, payee_public_key: RSAPublicKey) {
            let transaction = Transaction {
                amount: amount,
                payer: self.public_key.clone(),
                payee: payee_public_key,
            };

            let padding = PaddingScheme::new_pkcs1v15_encrypt();
            let a = transaction.to_string();
            let data = a.as_bytes();
            let signature = self.private_key.encrypt(&mut OsRng, padding, &data[..]).expect("failed to encrypt");
            
            chain.add_block(transaction, &self.public_key, signature);
        }
    }
}
