use crate::constants::{self, EXTERNAL_MESSAGE_REP};
use crate::core::message::BridgeMessage;
use crate::core::{error::ReadInputError, message::Message};
use tezos_smart_rollup::michelson::{MichelsonContract, MichelsonPair};
use tezos_smart_rollup::{
    host::Runtime,
    inbox::{InboxMessage, InternalInboxMessage},
    michelson::ticket::BytesTicket,
};

pub fn read_input<Host: Runtime>(host: &mut Host) -> Result<Message, ReadInputError> {
    let input = host.read_input().map_err(ReadInputError::Runtime)?;
    match input {
        None => Err(ReadInputError::EndOfInbox),
        Some(msg) => {
            // Try parsing as the type for internal message through the bridge
            match <InboxMessage<MichelsonPair<BytesTicket, MichelsonContract>>>::parse(msg.as_ref())
            {
                Ok((_, parsed_message)) => {
                    match parsed_message {
                        InboxMessage::Internal(t) => {
                            match t {
                                InternalInboxMessage::Transfer(transfer) => {
                                    if transfer.sender.to_string()
                                        != constants::L1_BRIDGE_CONTRACT_ADDRESS
                                    {
                                        Err(ReadInputError::NotFromBridge)
                                    } else {
                                        Ok(Message::Bridge(BridgeMessage {
                                            account: transfer.payload.1 .0.to_b58check(),
                                            token: String::from_utf8(
                                                transfer.payload.0.contents().0.clone(),
                                            )
                                            .unwrap(),
                                            amount: transfer.payload.0.amount_as().unwrap(),
                                        }))
                                    }
                                }
                                // Other internal messages can be ignored
                                _ => Err(ReadInputError::NotForKernel),
                            }
                        }
                        // Ticket must be sent internally through the bridge contract
                        InboxMessage::External(_) => Err(ReadInputError::NotFromBridge),
                    }
                }
                Err(_) => {
                    // Parse as an external message to transfer tokens
                    let data = msg.as_ref();
                    match data {
                        [0x01, EXTERNAL_MESSAGE_REP, ..] => {
                            let transfer_bytes = data.iter().skip(2).copied().collect();
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
    }
}
