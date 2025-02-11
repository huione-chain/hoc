// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// Coin<HC> is the token used to pay for gas in HC.
/// It has 9 decimals, and the smallest unit (10^-9) is called "mist".
module sui::hc;

use sui::balance::Balance;
use sui::coin;

const EAlreadyMinted: u64 = 0;
/// Sender is not @0x0 the system address.
const ENotSystemAddress: u64 = 1;

#[allow(unused_const)]
/// The amount of Mist per Sui token based on the fact that mist is
/// 10^-9 of a Sui token
const MIST_PER_HC: u64 = 1_000_000_000;

#[allow(unused_const)]
/// The total supply of Sui denominated in whole Sui tokens (10 Billion)
const TOTAL_SUPPLY_HC: u64 = 10_000_000_000;

/// The total supply of Sui denominated in Mist (10 Billion * 10^9)
const TOTAL_SUPPLY_MIST: u64 = 10_000_000_000_000_000_000;

/// Name of the coin
public struct HC has drop {}

#[allow(unused_function)]
/// Register the `HC` Coin to acquire its `Supply`.
/// This should be called only once during genesis creation.
fun new(ctx: &mut TxContext): Balance<HC> {
    assert!(ctx.sender() == @0x0, ENotSystemAddress);
    assert!(ctx.epoch() == 0, EAlreadyMinted);

    let (treasury, metadata) = coin::create_currency(
        HC {},
        9,
        b"HC",
        b"HC",
        // TODO: add appropriate description and logo url
        b"",
        option::none(),
        ctx,
    );
    transfer::public_freeze_object(metadata);
    let mut supply = treasury.treasury_into_supply();
    let total_sui = supply.increase_supply(TOTAL_SUPPLY_MIST);
    supply.destroy_supply();
    total_sui
}

public entry fun transfer(c: coin::Coin<HC>, recipient: address) {
    transfer::public_transfer(c, recipient)
}
