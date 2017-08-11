use exonum::crypto::{PublicKey, Hash};

use TIMESTAMPING_SERVICE;

pub const TX_UPDATE_USER_ID: u16 = 0;
pub const TX_PAYMENT_ID: u16 = 1;
pub const TX_TIMESTAMP_ID: u16 = 2;

encoding_struct! {
    /// Information about timestapming user.
    struct UserInfo {
        const SIZE = 56;

        /// Unique user identifier.
        field id:                       &str        [00 => 08]
        /// Public key of user.
        field pub_key:                  &PublicKey  [08 => 40]
        // FIXME fix serialize issue in core (Vec<u8> must serialize as hex!)
        /// Encrypted secret key.
        field encrypted_secret_key:     &[u8]       [40 => 48] 
        /// Additional metadata.
        field metadata:                 &str        [48 => 56]
    }
}

encoding_struct! {
    /// Information about payment.
    struct PaymentInfo {
        const SIZE = 24;

        /// User identifier.
        field user_id:                  &str        [00 => 08]
        /// Total amount of available transactions.
        field total_amount:             u64         [08 => 16]
        /// Additional metadata.
        field metadata:                 &str        [16 => 24]
    }
}

encoding_struct! {
    /// Information about payment.
    struct Timestamp {
        const SIZE = 48;

        /// User identifier.
        field user_id:                  &str        [00 => 08]
        /// Hash of content.
        field content_hash:             &Hash       [08 => 40]
        /// Additional metadata.
        field metadata:                 &str        [40 => 48]
    }
}

encoding_struct! {
    /// User information entry.
    struct UserInfoEntry {
        const SIZE = 80;

        /// User information entry.
        field info:                     UserInfo    [00 => 08]
        /// Total amount of available transactions
        field available_timestamps:     i64         [08 => 16]
        /// Root hash of user timestamps.
        field timestamps_hash:          &Hash       [16 => 48]
        /// Root hash of user payments.
        field payments_hash:            &Hash       [48 => 80] 
    }
}

encoding_struct! {
    /// Timestamp entry
    struct TimestampEntry {
        const SIZE = 40;

        /// User identifier.
        field timestamp:                Timestamp   [00 => 08]
        /// Hash of tx.
        field tx_hash:                  &Hash       [08 => 40]
    }
}


message! {
    /// Create or update user.
    struct TxUpdateUser {
        const TYPE = TIMESTAMPING_SERVICE;
        const ID = TX_UPDATE_USER_ID;
        const SIZE = 40;

        /// Public key of transaction.
        field pub_key:                  &PublicKey  [00 => 32]
        /// User information content.
        field content:                  UserInfo    [32 => 40]
    }
}

message! {
    /// A payment transaction.
    struct TxPayment {
        const TYPE = TIMESTAMPING_SERVICE;
        const ID = TX_PAYMENT_ID;
        const SIZE = 40;

        /// Public key of transaction.
        field pub_key:                  &PublicKey  [00 => 32]
        /// Information about payment.
        field content:                  PaymentInfo [32 => 40]
    }
}

message! {
    /// A timestamp transaction.
    struct TxTimestamp {
        const TYPE = TIMESTAMPING_SERVICE;
        const ID = TX_TIMESTAMP_ID;
        const SIZE = 40;

        /// Public key of transaction.
        field pub_key:                  &PublicKey  [00 => 32]
        /// Timestamp content.
        field content:                  Timestamp   [32 => 40]
    }
}

// TODO content tests