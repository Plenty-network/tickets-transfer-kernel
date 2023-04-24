use crate::core::{
    error::{Error, Result},
    public_key_hash::PublicKeyHash,
    token::Token,
};
use tezos_smart_rollup::{host::Runtime, storage::path::*};

const LEDGER: RefPath = RefPath::assert_from(b"/ledger");
const NONCE: RefPath = RefPath::assert_from(b"/nonce");

fn get_account_ledger_path(public_key_hash: &PublicKeyHash, token: &Token) -> Result<OwnedPath> {
    let public_key_hash: Vec<u8> = format!("/{}", public_key_hash.to_string()).into();
    let public_key_hash = OwnedPath::try_from(public_key_hash).map_err(Error::from)?;

    let token: Vec<u8> = format!("/{}", token.to_hex_string()).into();
    let token = OwnedPath::try_from(token).map_err(Error::from)?;

    let ledger_key = concat(&public_key_hash, &token).map_err(Error::from)?;

    // Stored as /ledger/${tz1-account}/${token-bytes}/<balance>
    concat(&LEDGER, &ledger_key).map_err(Error::from)
}

fn get_nonce_path(public_key_hash: &PublicKeyHash) -> Result<OwnedPath> {
    let public_key_hash: Vec<u8> = format!("/{}", public_key_hash.to_string()).into();
    let public_key_hash = OwnedPath::try_from(public_key_hash).map_err(Error::from)?;

    // Stored as /nonce/${tz1-account}
    concat(&NONCE, &public_key_hash).map_err(Error::from)
}

pub fn exists<R: Runtime>(host: &mut R, path: &impl Path) -> Result<bool> {
    let exists = Runtime::store_has(host, path)?
        .map(|_| true)
        .unwrap_or_default();
    Ok(exists)
}

fn store_u128<Host: Runtime, P: Path>(host: &mut Host, path: &P, data: &u128) -> Result<()> {
    let data = data.to_be_bytes();
    let data = data.as_slice();

    host.store_write(path, data, 0)
        .map_err(Error::from)
        .map(|_| ())
}

fn read_u128<Host: Runtime, P: Path>(host: &mut Host, path: &P) -> Result<Option<u128>> {
    let is_exists = exists(host, path)?;
    if !is_exists {
        return Ok(None);
    }

    let mut buffer = [0_u8; 16];
    match host.store_read_slice(path, 0, &mut buffer) {
        Ok(16) => Ok(Some(u128::from_be_bytes(buffer))),
        _ => Err(Error::StateDeserializarion),
    }
}

fn store_u64<Host: Runtime, P: Path>(host: &mut Host, path: &P, data: &u64) -> Result<()> {
    let data = data.to_be_bytes();
    let data = data.as_slice();

    host.store_write(path, data, 0)
        .map_err(Error::from)
        .map(|_| ())
}

fn read_u64<Host: Runtime, P: Path>(host: &mut Host, path: &P) -> Result<Option<u64>> {
    let is_exists = exists(host, path)?;
    if !is_exists {
        return Ok(None);
    }

    let mut buffer = [0_u8; 8];
    match host.store_read_slice(path, 0, &mut buffer) {
        Ok(8) => Ok(Some(u64::from_be_bytes(buffer))),
        _ => Err(Error::StateDeserializarion),
    }
}

pub fn read_balance<Host: Runtime>(
    host: &mut Host,
    account: &PublicKeyHash,
    token: &Token,
) -> Result<u128> {
    let path = get_account_ledger_path(account, token)?;
    Ok(read_u128(host, &path)?.unwrap_or_default())
}

pub fn store_balance<Host: Runtime>(
    host: &mut Host,
    account: &PublicKeyHash,
    token: &Token,
    balance: &u128,
) -> Result<()> {
    let path = get_account_ledger_path(account, token)?;
    store_u128(host, &path, balance)?;
    Ok(())
}

pub fn read_nonce<Host: Runtime>(host: &mut Host, account: &PublicKeyHash) -> Result<u64> {
    let path = get_nonce_path(account)?;
    Ok(read_u64(host, &path)?.unwrap_or_default())
}

pub fn store_nonce<Host: Runtime>(
    host: &mut Host,
    account: &PublicKeyHash,
    nonce: &u64,
) -> Result<()> {
    let path = get_nonce_path(account)?;
    store_u64(host, &path, nonce)?;
    Ok(())
}
