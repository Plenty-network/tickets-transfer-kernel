use crate::core::hash::Blake2b;
use crate::core::nonce::Nonce;
use crate::core::public_key::PublicKey;
use crate::core::public_key_hash::PublicKeyHash;
use crate::core::signature::Signature;
use serde::Deserialize;

#[derive(Deserialize)]
pub enum Message {
    Bridge(BridgeMessage),
    Transfer(TransferMessage),
}

#[derive(Deserialize)]
pub struct BridgeMessage {
    pub account: String,
    pub token: String,
    pub amount: u128,
}

#[derive(Deserialize)]
pub struct TransferContent {
    pub token: String,
    pub desination: PublicKeyHash,
    // TODO: make this big int
    pub amount: u128,
}

#[derive(Deserialize)]
pub struct Inner {
    nonce: Nonce,
    pub content: TransferContent,
}

impl Inner {
    /// Returns the nonce of the inner
    pub fn nonce(&self) -> &Nonce {
        &self.nonce
    }
}

#[derive(Deserialize)]
pub struct TransferMessage {
    pkey: PublicKey,
    signature: Signature,
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

    /// Returns the hash of the message
    pub fn hash(&self) -> Blake2b {
        self.inner.hash()
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
            content.token,
            content.desination.to_string(),
            content.amount
        );
        Blake2b::from(string.as_bytes())
    }
}
