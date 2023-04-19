#include "./transfers.mligo"
#include "./common/types.mligo"
#include "./common/errors.mligo"

type storage = address (* Smart rollup address *)

type parameter = 
    | Deposit of token * nat (* (token, token-amount) *)
    | Release of bytes ticket * address

type return = operation list * storage


(* Deposits a ticket of content type `bytes` to the rollup inbox in exchange of a token *)
let deposit (store, (token, amount): storage * (token * nat)) : operation list =
    (* Retrieve token from the user and lock up in the contract *)
    let transfer_op = 
        match token with 
        | Fa12 addr -> get_fa12_transfer_op (Tezos.get_sender ()) (Tezos.get_self_address ()) addr amount 
        | Fa2 (addr, token_id) -> begin
            let txs = [ { to_ = Tezos.get_self_address (); token_id = token_id; amount = amount; } ] in
            get_fa2_transfer_op (Tezos.get_sender ()) addr txs
        end in 

    (* Create and deposit the ticket in the smart rollup *)
    let sr_ticket: bytes ticket = 
        match Tezos.create_ticket (Bytes.pack token) amount with 
        | None -> failwith ticket_creation_error
        | Some t -> t in
    let rollup_contract: rollup_entry_params contract = 
        match Tezos.get_contract_opt store with
        | None -> failwith invalid_contract
        | Some c -> c in
    let rollup_op = Tezos.transaction (sr_ticket, Tezos.get_sender ()) 0mutez rollup_contract in
    
    [transfer_op; rollup_op]


(* Unlocks tokens in exchange of a valid ticket *)
let release (_, (sr_ticket, destination): storage * ( bytes ticket * address)): operation list =
    let (ticketer, (token_packed, amount)), _ = Tezos.read_ticket sr_ticket in

    (* Note: It is not required to verify that the sender is the smart rollup since the ticketer is already
    being checked *)

    (* Verify that the ticketer is the bridge itself *)
    let _ = if ticketer <> Tezos.get_self_address () then failwith unauthorised_ticketer else unit in

    (* Find the token the ticket points at *)
    let token: token = match Bytes.unpack token_packed with None -> failwith invalid_token | Some v -> v in
    
    (* Forward the locked tokens to the provided destination *)
    let transfer_op = 
        match token with 
        | Fa12 addr -> get_fa12_transfer_op (Tezos.get_self_address ()) destination addr amount 
        | Fa2 (addr, token_id) -> begin
            let txs = [ { to_ = destination; token_id = token_id; amount = amount; } ] in
            get_fa2_transfer_op (Tezos.get_self_address ()) addr txs
        end in 

    [transfer_op]


let main (action, store: parameter * storage): return = 
    match action with
    | Deposit p -> deposit (store, p), store
    | Release p -> release (store, p), store