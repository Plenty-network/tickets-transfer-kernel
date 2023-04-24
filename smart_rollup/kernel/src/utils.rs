use crate::constants;
use crate::core::message::{BridgeMessage, TransferContent, TransferMessage};
use crate::core::nonce::Nonce;
use crate::core::public_key_hash::PublicKeyHash;
use crate::core::{error::*, message::Message};
use crate::storage::{read_balance, read_nonce, store_balance, store_nonce};
use tezos_smart_rollup::{
    host::Runtime,
    inbox::{InboxMessage, InternalInboxMessage},
    michelson::{ticket::BytesTicket, MichelsonContract, MichelsonPair},
};

pub fn read_input<Host: Runtime>(host: &mut Host) -> std::result::Result<Message, ReadInputError> {
    let input = host.read_input().map_err(ReadInputError::Runtime)?;
    match input {
        None => Err(ReadInputError::EndOfInbox),
        Some(msg) => {
            match <InboxMessage<MichelsonPair<BytesTicket, MichelsonContract>>>::parse(msg.as_ref())
            {
                Ok((_, parsed_message)) => {
                    match parsed_message {
                        // Internal bridge transfer
                        InboxMessage::Internal(t) => {
                            match t {
                                InternalInboxMessage::Transfer(transfer) => {
                                    if transfer.sender.to_string()
                                        != constants::L1_BRIDGE_CONTRACT_ADDRESS
                                    {
                                        Err(ReadInputError::NotFromBridge)
                                    } else {
                                        Ok(Message::Bridge(BridgeMessage::from(transfer.payload)))
                                    }
                                }
                                // Other internal messages can be ignored
                                _ => Err(ReadInputError::NotForKernel),
                            }
                        }
                        // External transfer transaction
                        InboxMessage::External(data) => {
                            match data {
                                [constants::EXTERNAL_MESSAGE_REP, ..] => {
                                    let transfer_bytes = data.iter().skip(1).copied().collect();
                                    let str = String::from_utf8(transfer_bytes)
                                        .map_err(ReadInputError::FromUtf8Error)?;
                                    let msg = serde_json_wasm::from_str(&str)
                                        .map_err(ReadInputError::SerdeJson)?;
                                    Ok(msg)
                                }
                                _ => Err(ReadInputError::NotForKernel), // TODO: this can be more specific
                            }
                        }
                    }
                }
                Err(_) => Err(ReadInputError::NotForKernel),
            }
        }
    }
}

pub fn process_bridge_message<Host: Runtime>(
    host: &mut Host,
    message: BridgeMessage,
) -> Result<()> {
    // Simply update the existing balance of the account
    let current_balance = read_balance(host, &message.account, &message.token)?;
    store_balance(
        host,
        &message.account,
        &message.token,
        &(current_balance + message.amount),
    )
}

pub fn process_transfer_message<Host: Runtime>(
    host: &mut Host,
    message: TransferMessage,
) -> Result<()> {
    let sig = message.signature();
    let pk = message.public_key();
    let inner = message.inner();
    let hash = inner.hash();

    sig.verify(pk, hash.as_ref())?;

    let pkh = PublicKeyHash::from(pk);
    let nonce = Nonce(read_nonce(host, &pkh)?);

    if &nonce.next() != inner.nonce() {
        return Err(Error::InvalidNonce);
    }

    // Update the nonce
    store_nonce(host, &pkh, &inner.nonce().0)?;

    let TransferContent {
        token,
        destination,
        amount,
    } = &inner.content;

    // Initial balances
    let source_balance = read_balance(host, &pkh, token)?;
    let destination_balance = read_balance(host, destination, token)?;

    if source_balance < *amount {
        Err(Error::InvalidTransferAmount)
    } else {
        // Update balances by making a transfer
        store_balance(host, &pkh, token, &(source_balance - amount))?;
        store_balance(host, destination, token, &(destination_balance + amount))?;
        Ok(())
    }
}
