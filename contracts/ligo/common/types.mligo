#if !TYPES
#define TYPES

type token = 
    | Fa12 of address
    | Fa2 of address * nat

type fa12_transfer_params = [@layout:comb] {
    [@annot:from] from_: address;
    [@annot:to] to_: address;
    value: nat;
}

type fa2_transfer_txs_item = [@layout:comb] {
    to_: address;
    token_id: nat;
    amount: nat;
}

type fa2_transfer_txs = fa2_transfer_txs_item list

type fa2_transfer_params = [@layout:comb] {
    from_: address;
    txs: fa2_transfer_txs;
} list

type rollup_entry_params = (bytes ticket * address)

#endif
