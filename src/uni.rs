#![allow(dead_code)]
use ethers::prelude::*;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//  Router
//  - Reference: https://docs.uniswap.org/protocol/V2/reference/smart-contracts/library
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// Given an input asset amount, returns the maximum output amount of the other asset (accounting for fees) given reserves.
// Uniswap v2; x * y = k formula
// How much out do we get if we supply in?
pub fn get_amount_out(a_in: U256, reserve_in: U256, reserve_out: U256) -> (U256, U256, U256) {
    let a_in_with_fee = a_in * 997;
    let numerator = a_in_with_fee * reserve_out;
    let denominator = a_in_with_fee + reserve_in * 1000;
    let a_out = numerator / denominator;

    // Underflow
    let mut new_reserve_out = reserve_out - a_out;
    if new_reserve_out < U256::zero() || new_reserve_out > reserve_out {
        new_reserve_out = U256::one();
    }

    // Overflow
    let mut new_reserve_in = reserve_in + a_in;
    if new_reserve_in < reserve_in {
        new_reserve_in = U256::MAX;
    }

    (a_out, new_reserve_in, new_reserve_out)
}

// Returns the minimum input asset amount required to buy the given output asset amount (accounting for fees) given reserves.
// Uniswap v2; x * y = k formula
// How much out do we get if we supply out?
pub fn get_amount_in(a_out: U256, reserve_in: U256, reserve_out: U256) -> (U256, U256, U256) {
    // Underflow
    let mut new_reserve_out = reserve_out - a_out;
    if new_reserve_out < U256::zero() || reserve_out > reserve_out {
        new_reserve_out = U256::one();
    }

    let numerator = reserve_in * a_out * 1000;
    let denominator = new_reserve_out * 997;
    let a_amount_in = numerator / denominator + U256::one();

    // Overflow
    let mut new_reserve_in = reserve_in + a_amount_in;
    if new_reserve_in < reserve_in {
        new_reserve_in = U256::MAX;
    }

    (a_amount_in, new_reserve_in, new_reserve_out)
}