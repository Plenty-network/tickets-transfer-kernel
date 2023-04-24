use crate::core::error::ReadInputError;
use crate::core::message::Message;
use tezos_smart_rollup::{host::Runtime, kernel_entry};
use utils::{process_bridge_message, process_transfer_message, read_input};

mod constants;
mod core;
mod storage;
mod utils;

pub fn entry<Host: Runtime>(host: &mut Host) {
    execute(host);
}

fn execute<Host: Runtime>(host: &mut Host) {
    match read_input(host) {
        Ok(message) => {
            match message {
                Message::Bridge(b) => process_bridge_message(host, b).unwrap_or(()),
                Message::Transfer(t) => process_transfer_message(host, t).unwrap_or(()),
            }

            execute(host)
        }
        Err(ReadInputError::EndOfInbox) => (),
        Err(_) => execute(host),
    }
}

kernel_entry!(entry);
