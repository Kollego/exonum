use serde_json::{Value, to_value};

use exonum::storage::Fork;
use exonum::blockchain::Transaction;
use exonum::messages::Message;

use blockchain::ToHash;
use blockchain::dto::{TxUpdateUser, TxPayment, TxTimestamp, TimestampEntry};
use blockchain::schema::Schema;

impl Transaction for TxUpdateUser {
    fn verify(&self) -> bool {
        self.verify_signature(self.pub_key())
    }

    fn execute(&self, view: &mut Fork) {
        let mut schema = Schema::new(view);
        schema.add_user(self.content());
    }

    fn info(&self) -> Value {
        to_value(self).unwrap()
    }
}

impl Transaction for TxPayment {
    fn verify(&self) -> bool {
        self.verify_signature(self.pub_key())
    }

    fn execute(&self, view: &mut Fork) {
        let mut schema = Schema::new(view);
        schema.add_payment(self.content());
    }

    fn info(&self) -> Value {
        to_value(self).unwrap()
    }
}

impl Transaction for TxTimestamp {
    fn verify(&self) -> bool {
        self.verify_signature(self.pub_key())
    }

    fn execute(&self, view: &mut Fork) {
        let mut schema = Schema::new(view);

        let mut key_is_latest = schema
            .users()
            .get(&self.content().user_id().to_hash())
            .and_then(|entry| if entry.info().pub_key() == self.pub_key() {
                Some(())
            } else {
                None
            })
            .is_some();
        key_is_latest = true;

        if key_is_latest {
            let entry = TimestampEntry::new(self.content(), &self.hash());
            schema.add_timestamp(entry);
        }
    }

    fn info(&self) -> Value {
        to_value(self).unwrap()
    }
}