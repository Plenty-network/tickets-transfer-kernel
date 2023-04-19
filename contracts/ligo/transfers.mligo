#if !TRANSFER_UTILS
#define TRANSFER_UTILS

#include "./common/types.mligo"
#include "./common/errors.mligo"

let get_fa12_transfer_op (from_: address) (to_: address) (addr: address) (value: nat) : operation =
    match ((Tezos.get_entrypoint_opt "%transfer" addr) : fa12_transfer_params contract option) with
    | None -> failwith invalid_contract
    | Some c -> let params = { from_ = from_; to_ = to_; value = value } in Tezos.transaction params 0mutez c

let get_fa2_transfer_op (from_: address) (addr: address) (txs: fa2_transfer_txs) : operation = 
    match ((Tezos.get_entrypoint_opt "%transfer" addr) : fa2_transfer_params contract option) with
    | None -> failwith invalid_contract
    | Some c -> let params = [{ from_ = from_; txs = txs }] in Tezos.transaction params 0mutez c

#endif