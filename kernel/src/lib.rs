use tezos_smart_rollup::{
    host::Runtime, inbox::InboxMessage, kernel_entry, michelson::ticket::BytesTicket,
};

mod core;

pub fn entry<Host: Runtime>(host: &mut Host) {
    execute(host);
}

fn execute<Host: Runtime>(host: &mut Host) {
    let input = host.read_input();

    match input {
        Err(_) | Ok(None) => (),
        Ok(Some(msg)) => match <InboxMessage<BytesTicket>>::parse(msg.as_ref()) {
            Ok(parsed_message) => match parsed_message {
                (_, _) => {}
            },
            Err(_) => {}
        },
    }

    execute(host)
}

kernel_entry!(entry);
