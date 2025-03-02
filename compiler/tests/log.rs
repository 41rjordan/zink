//! Tests for instruction `select`.

use anyhow::Result;
use zint::{Bytes32, EVM};

mod common;

#[test]
fn log0() -> Result<()> {
    let bytecode = common::load("log", "log0")?;

    // returns the bigger number.
    let info = EVM::run(&bytecode, &[]);
    assert_eq!(info.logs[0].data.to_vec(), b"Ping".to_vec().to_bytes32());
    Ok(())
}

#[test]
fn log1() -> Result<()> {
    let bytecode = common::load("log", "log1")?;

    // returns the bigger number.
    let info = EVM::run(&bytecode, &[]);
    assert_eq!(info.logs[0].data.to_vec(), b"Ping".to_vec().to_bytes32());
    assert_eq!(
        info.logs[0].topics[0].to_vec(),
        b"pong".to_vec().to_bytes32()
    );
    Ok(())
}

#[test]
fn log2() -> Result<()> {
    let bytecode = common::load("log", "log2")?;

    // returns the bigger number.
    let info = EVM::run(&bytecode, &[]);
    assert_eq!(info.logs[0].data.to_vec(), b"Ping".to_vec().to_bytes32());
    assert_eq!(
        info.logs[0].topics[0].to_vec(),
        b"pong".to_vec().to_bytes32()
    );
    assert_eq!(
        info.logs[0].topics[1].to_vec(),
        b"ping".to_vec().to_bytes32()
    );
    Ok(())
}

#[test]
fn log3() -> Result<()> {
    let bytecode = common::load("log", "log3")?;

    // returns the bigger number.
    let info = EVM::run(&bytecode, &[]);
    assert_eq!(info.logs[0].data.to_vec(), b"Ping".to_vec().to_bytes32());
    assert_eq!(
        info.logs[0].topics[0].to_vec(),
        b"pong".to_vec().to_bytes32()
    );
    assert_eq!(
        info.logs[0].topics[1].to_vec(),
        b"ping".to_vec().to_bytes32()
    );
    assert_eq!(
        info.logs[0].topics[2].to_vec(),
        b"pong".to_vec().to_bytes32()
    );
    Ok(())
}

#[test]
fn log4() -> Result<()> {
    let bytecode = common::load("log", "log4")?;

    // returns the bigger number.
    let info = EVM::run(&bytecode, &[]);
    assert_eq!(info.logs[0].data.to_vec(), b"Ping".to_vec().to_bytes32());
    assert_eq!(
        info.logs[0].topics[0].to_vec(),
        b"pong".to_vec().to_bytes32()
    );
    assert_eq!(
        info.logs[0].topics[1].to_vec(),
        b"ping".to_vec().to_bytes32()
    );
    assert_eq!(
        info.logs[0].topics[2].to_vec(),
        b"pong".to_vec().to_bytes32()
    );
    assert_eq!(
        info.logs[0].topics[3].to_vec(),
        b"pong".to_vec().to_bytes32()
    );
    Ok(())
}
