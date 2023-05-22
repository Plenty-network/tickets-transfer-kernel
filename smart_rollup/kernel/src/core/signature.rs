use crate::core::error::*;
use crate::core::hash::Blake2b;
use crate::core::public_key::PublicKey;
use serde::{Deserialize, Serialize};
use tezos_crypto_rs::hash::Ed25519Signature;

#[derive(Deserialize, Serialize)]
pub enum Signature {
    Ed25519(Ed25519Signature),
}

impl Signature {
    pub fn verify(&self, public_key: &PublicKey, message: &[u8]) -> Result<()> {
        match (self, public_key) {
            (Signature::Ed25519(sig), PublicKey::Ed25519(pkey)) => {
                // TODO: There should be another way to do it
                // TODO: remove the unwrap
                let data = Blake2b::from(message);
                let data = data.as_ref();
                let signature =
                    ed25519_compact::Signature::from_slice(sig.as_ref()).map_err(Error::from)?;
                let pkey =
                    ed25519_compact::PublicKey::from_slice(pkey.as_ref()).map_err(Error::from)?;

                pkey.verify(data, &signature)
                    .map_err(|_| Error::InvalidSignature)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use tezos_core::types::encoded::{self, Encoded};
    use tezos_crypto_rs::hash::{Ed25519Signature, SeedEd25519};

    use super::Signature;
    use crate::constants::{
        DAPP_URL, MICHELINE_EXPRESSION_BYTE, MICHELINE_STRING_BYTE, TEZOS_SIGNED_MESSAGE,
    };
    use crate::core::message::{Inner, Message, TransferContent, TransferMessage};
    use crate::core::nonce::Nonce;
    use crate::core::public_key::PublicKey;
    use crate::core::public_key_hash::PublicKeyHash;
    use crate::core::token::Token;

    impl Signature {
        pub fn to_b58(&self) -> String {
            match self {
                Signature::Ed25519(sig) => sig.to_base58_check(),
            }
        }

        pub fn from_b58(data: &str) -> std::result::Result<Self, &'static str> {
            let ed25519 = Ed25519Signature::from_base58_check(data).ok();
            match ed25519 {
                Some(pkey) => Ok(Signature::Ed25519(pkey)),
                None => Err("Cannot decode b58"),
            }
        }
    }

    #[test]
    fn test_generate_and_verify() {
        let (pk, sk) = SeedEd25519::from_base58_check(
            "edsk31vznjHSSpGExDMHYASz45VZqXN4DPxvsa4hAyY8dHM28cZzp6",
        )
        .unwrap()
        .keypair()
        .unwrap();
        let pk = PublicKey::from_b58(pk.to_string().as_str()).unwrap();
        let pkh = PublicKeyHash::from(&pk);

        println!("{}", sk.to_string());

        println!("{}", pkh.to_string());

        let inner = Inner {
            nonce: Nonce(1),
            content: TransferContent {
                token: Token(vec![0x12, 0x34]),
                destination: PublicKeyHash::from_b58("tz1Pe4aBjsW9ZGWaFXa47megxFD1LGGFAW3C")
                    .unwrap(),
                amount: 10000000,
            },
        };

        let timestamp = String::from("2023-05-19T05:45:50.473Z");

        let bytes = vec![
            TEZOS_SIGNED_MESSAGE.to_string(),
            DAPP_URL.to_string(),
            timestamp.clone(),
            inner.hash().to_string(),
        ]
        .join(" ")
        .as_bytes()
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>();
        let bytes_length = format!("{:08x}", bytes.len() / 2);
        let data = vec![
            MICHELINE_EXPRESSION_BYTE.to_string(),
            MICHELINE_STRING_BYTE.to_string(),
            bytes_length,
            bytes,
        ]
        .join("");

        println!("{:?}", inner.hash().to_string());
        println!("{:?}", inner.hash().as_ref());

        println!("{}", data);

        fn hex_to_string(hex: &str) -> Vec<u8> {
            let bytes = (0..hex.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
                .collect::<Vec<u8>>();
            bytes
        }

        let x: &[u8] = &hex_to_string(data.as_str());

        println!("{:?}", x);

        let gen_sig = sk.sign([x]).unwrap();
        let gen_sig = encoded::Signature::from_bytes(gen_sig.as_ref())
            .unwrap()
            .to_generic_signature()
            .unwrap();

        println!("{}", gen_sig.value());

        let ed25519_sig = encoded::Ed25519Signature::try_from(&gen_sig).unwrap();

        println!("{}", ed25519_sig.value().to_string());

        let transfer_message = Message::Transfer(TransferMessage {
            pkey: pk.clone(),
            timestamp: timestamp.clone(),
            signature: Signature::from_b58(ed25519_sig.value()).unwrap(),
            inner,
        });

        // Bytes for the external message (without the magic bytes and user byte)
        println!(
            "{}",
            serde_json_wasm::to_vec(&transfer_message)
                .unwrap()
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>()
        );

        assert!(Signature::from_b58(ed25519_sig.value())
            .unwrap()
            .verify(&pk, x)
            .is_ok());

        // assert!(Signature::from_b58("edsigtpKbZVYcJmRtzpyAhuvdJVNz3BsDEM8j7SRpe8x7EkLjtVBJ2oYnJMAxEHE2rhfdDbWAr7pQxXN8Swm7zfVkVBgepD8cUd")
        //     .unwrap()
        //     .verify(&pk, data.as_ref())
        //     .is_err());
    }

    #[test]
    fn test_ed25519_signature_deserialization() {
        let signature = "edsigu1mRCtZquLvspcxaYXVZdsKKSqHnXevnrmh1T63Dq1Rr8M1giVLvapiDFK6TQCEyY6xytdGnKgZyVSHDVnub7puy54bD1y";
        let res = Signature::from_b58(signature);
        assert!(res.is_ok());
    }

    #[test]
    fn test_ed25519_signature_serialization() {
        let sig = "edsigu1mRCtZquLvspcxaYXVZdsKKSqHnXevnrmh1T63Dq1Rr8M1giVLvapiDFK6TQCEyY6xytdGnKgZyVSHDVnub7puy54bD1y";
        let serialized = Signature::from_b58(sig).unwrap().to_b58();
        assert_eq!(sig, &serialized)
    }

    #[test]
    fn test_ed25519_signature_verification() {
        let signature = Signature::from_b58("edsigu1mRCtZquLvspcxaYXVZdsKKSqHnXevnrmh1T63Dq1Rr8M1giVLvapiDFK6TQCEyY6xytdGnKgZyVSHDVnub7puy54bD1y").unwrap();
        let pkey =
            PublicKey::from_b58("edpkuDMUm7Y53wp4gxeLBXuiAhXZrLn8XB1R83ksvvesH8Lp8bmCfK").unwrap();
        let data = [
            0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64,
        ]
        .as_slice();

        let verification = signature.verify(&pkey, data);
        assert!(verification.is_ok());
    }
}
