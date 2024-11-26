//! Anchor-compatible SDK for the Raydium staking program.

#![deny(rustdoc::all)]
#![allow(rustdoc::missing_doc_code_examples)]
#![allow(clippy::nonstandard_macro_braces)]

mod accounts;
mod instructions;

pub use accounts::*;
pub use instructions::*;

use anchor_lang::prelude::*;

declare_id!("CqM9ZcDg7xzusr5srrutHBD6DNrGiMq2BpdQQRSas6tH");

/// Farm Program
#[derive(Clone)]
pub struct Staking;

impl anchor_lang::Id for Staking {
    fn id() -> Pubkey {
        ID
    }
}
