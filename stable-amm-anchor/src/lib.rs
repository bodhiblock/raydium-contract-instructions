//! Anchor-compatible SDK for the Raydium AMM program.

#![deny(rustdoc::all)]
#![allow(rustdoc::missing_doc_code_examples)]
#![allow(clippy::nonstandard_macro_braces)]

mod accounts;
mod instructions;

pub use accounts::*;
pub use instructions::*;

use anchor_lang::prelude::*;

declare_id!("4WGQCaPS5nfBQWLKfoZrpnH2ehPe7YAp5VqfU31BtpXa");

/// Stable Amm Program
#[derive(Clone)]
pub struct StableAmm;

impl anchor_lang::Id for StableAmm {
    fn id() -> Pubkey {
        ID
    }
}
