use crate::core::error::ReadInputError;
use crate::core::message::Message;
use tezos_smart_rollup::{host::Runtime, kernel_entry};
use utils::read_input;

mod constants;
mod core;
mod utils;

pub fn entry<Host: Runtime>(host: &mut Host) {
    execute(host);
}

fn execute<Host: Runtime>(host: &mut Host) {
    match read_input(host) {
        Ok(message) => {
            match message {
                Message::Bridge(b) => host.write_debug(
                    format!("Bridge message: {} {} {}\n", b.account, b.token, b.amount).as_str(),
                ),
                Message::Transfer(_) => host.write_debug("Transfer message"),
            }

            execute(host)
        }
        Err(ReadInputError::EndOfInbox) => (),
        Err(_) => execute(host),
    }
}

kernel_entry!(entry);
