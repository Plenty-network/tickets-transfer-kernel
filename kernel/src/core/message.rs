use crate::core::hash::Blake2b;
use crate::core::nonce::Nonce;
use crate::core::public_key::PublicKey;
use crate::core::public_key_hash::PublicKeyHash;
use crate::core::signature::Signature;
use serde::{Deserialize, Serialize};
use tezos_smart_rollup::michelson::{ticket::BytesTicket, MichelsonContract, MichelsonPair};

use super::token::Token;

#[derive(Deserialize, Serialize)]
pub enum Message {
    Bridge(BridgeMessage),
    Transfer(TransferMessage),
}

#[derive(Deserialize, Serialize)]
pub struct BridgeMessage {
    pub account: PublicKeyHash,
    pub token: Token,
    pub amount: u128,
}

impl From<MichelsonPair<BytesTicket, MichelsonContract>> for BridgeMessage {
    fn from(michelson_payload: MichelsonPair<BytesTicket, MichelsonContract>) -> Self {
        BridgeMessage {
            account: PublicKeyHash::from_b58(michelson_payload.1 .0.to_b58check().as_str())
                .unwrap(),
            token: Token::from(&michelson_payload.0.contents().0),
            amount: michelson_payload.0.amount_as().unwrap(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct TransferContent {
    pub token: Token,
    pub destination: PublicKeyHash,
    pub amount: u128,
}

#[derive(Deserialize, Serialize)]
pub struct Inner {
    pub nonce: Nonce,
    pub content: TransferContent,
}

impl Inner {
    /// Returns the nonce of the inner
    pub fn nonce(&self) -> &Nonce {
        &self.nonce
    }
}

#[derive(Deserialize, Serialize)]
pub struct TransferMessage {
    pub pkey: PublicKey,
    pub signature: Signature,
    pub inner: Inner,
}

impl TransferMessage {
    /// Returns the public key of the message
    pub fn public_key(&self) -> &PublicKey {
        &self.pkey
    }

    /// Returns the signature of the message
    pub fn signature(&self) -> &Signature {
        &self.signature
    }

    /// Returns the inner of the message
    pub fn inner(&self) -> &Inner {
        &self.inner
    }
}

impl Inner {
    /// Hash of the message
    /// This hash is what the client should signed
    pub fn hash(&self) -> Blake2b {
        // The nonce, and content should be hashed
        let Inner { nonce, content } = &self;
        let string = format!(
            "{}{}{}{}",
            nonce.to_string(),
            content.token.to_hex_string(),
            content.destination.to_string(),
            content.amount
        );
        Blake2b::from(string.as_bytes())
    }
}
