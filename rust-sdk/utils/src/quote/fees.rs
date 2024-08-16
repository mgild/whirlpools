use core::ops::Shl;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use crate::{CollectFeesQuote, U128};

/// Calculate fees owed for a position
///
/// # Paramters
/// - `fee_growth_outside_a_lower`: The fee growth outside the lower tick
/// - `fee_growth_outside_b_lower`: The fee growth outside the lower tick
/// - `fee_growth_outside_a_upper`: The fee growth outside the upper tick
/// - `fee_growth_outside_b_upper`: The fee growth outside the upper tick
/// - `fee_growth_global_a`: The global fee growth for token A
/// - `fee_growth_global_b`: The global fee growth for token B
/// - `fee_growth_checkpoint_a`: The fee growth checkpoint for token A
/// - `fee_growth_checkpoint_b`: The fee growth checkpoint for token B
/// - `position_liquidity`: The liquidity of the position
/// - `position_fee_owed_a`: The fee owed for token A
/// - `position_fee_owed_b`: The fee owed for token B
/// - `tick_current_index`: The current tick index
/// - `tick_lower_index`: The lower tick index of the position
/// - `tick_upper_index`: The upper tick index of the position
///
/// # Returns
/// - `CollectFeesQuote`: The fees owed for token A and token B
#[allow(clippy::too_many_arguments)]
#[cfg_attr(feature = "wasm", wasm_bindgen(js_name = collectFeesQuote, skip_jsdoc))]
pub fn collect_fees_quote(
    fee_growth_outside_a_lower: U128,
    fee_growth_outside_b_lower: U128,
    fee_growth_outside_a_upper: U128,
    fee_growth_outside_b_upper: U128,
    fee_growth_global_a: U128,
    fee_growth_global_b: U128,
    fee_growth_checkpoint_a: U128,
    fee_growth_checkpoint_b: U128,
    position_liquidity: U128,
    position_fee_owed_a: U128,
    position_fee_owed_b: U128,
    tick_current_index: i32,
    tick_lower_index: i32,
    tick_upper_index: i32,
) -> CollectFeesQuote {
    let mut fee_growth_below_a_x_64: u128 = fee_growth_outside_a_lower.into();
    let mut fee_growth_above_a_x_64: u128 = fee_growth_outside_a_upper.into();
    let mut fee_growth_below_b_x_64: u128 = fee_growth_outside_b_lower.into();
    let mut fee_growth_above_b_x_64: u128 = fee_growth_outside_b_upper.into();

    let fee_growth_global_a_x_64: u128 = fee_growth_global_a.into();
    let fee_growth_global_b_x_64: u128 = fee_growth_global_b.into();

    let fee_growth_checkpoint_a_x_64: u128 = fee_growth_checkpoint_a.into();
    let fee_growth_checkpoint_b_x_64: u128 = fee_growth_checkpoint_b.into();

    let position_liquidity_u128: u128 = position_liquidity.into();
    let position_fee_owed_a_u128: u128 = position_fee_owed_a.into();
    let position_fee_owed_b_u128: u128 = position_fee_owed_b.into();

    if tick_current_index < tick_lower_index {
        fee_growth_below_a_x_64 = fee_growth_global_a_x_64.wrapping_sub(fee_growth_below_a_x_64);
        fee_growth_below_b_x_64 = fee_growth_global_b_x_64.wrapping_sub(fee_growth_below_b_x_64);
    }

    if tick_current_index > tick_upper_index {
        fee_growth_above_a_x_64 = fee_growth_global_a_x_64.wrapping_sub(fee_growth_above_a_x_64);
        fee_growth_above_b_x_64 = fee_growth_global_b_x_64.wrapping_sub(fee_growth_above_b_x_64);
    }

    let fee_growth_inside_a_x_64 = fee_growth_global_a_x_64
        .wrapping_sub(fee_growth_below_a_x_64)
        .wrapping_sub(fee_growth_above_a_x_64);

    let fee_growth_inside_b_x_64 = fee_growth_global_b_x_64
        .wrapping_sub(fee_growth_below_b_x_64)
        .wrapping_sub(fee_growth_above_b_x_64);

    let fee_owed_delta_a_x_64 = fee_growth_inside_a_x_64
        .wrapping_sub(fee_growth_checkpoint_a_x_64)
        .wrapping_mul(position_liquidity_u128);

    let fee_owed_delta_b_x_64 = fee_growth_inside_b_x_64
        .wrapping_sub(fee_growth_checkpoint_b_x_64)
        .wrapping_mul(position_liquidity_u128);

    let fee_owed_a: u128 = position_fee_owed_a_u128 + fee_owed_delta_a_x_64.shl(64);
    let fee_owed_b: u128 = position_fee_owed_b_u128 + fee_owed_delta_b_x_64.shl(64);

    // TOOD: remove transfer fee deduction

    CollectFeesQuote {
        fee_owed_a: fee_owed_a.into(),
        fee_owed_b: fee_owed_b.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_out_of_range_lower() {
        let result1 = collect_fees_quote(
            100u128.into(),
            200u128.into(),
            300u128.into(),
            400u128.into(),
            500u128.into(),
            600u128.into(),
            700u128.into(),
            800u128.into(),
            900u128.into(),
            1000u128.into(),
            1100u128.into(),
            5,
            3,
            7,
        );
        assert_eq!(result1.fee_owed_a, 1000);
        assert_eq!(result1.fee_owed_b, 1100);
    }

    #[test]
    fn test_in_range() {
        let result1 = collect_fees_quote(
            100u128.into(),
            200u128.into(),
            300u128.into(),
            400u128.into(),
            500u128.into(),
            600u128.into(),
            700u128.into(),
            800u128.into(),
            900u128.into(),
            1000u128.into(),
            1100u128.into(),
            5,
            5,
            7,
        );
        assert_eq!(result1.fee_owed_a, 1000);
        assert_eq!(result1.fee_owed_b, 1100);
    }

    #[test]
    fn test_collect_out_of_range_upper() {
        let result2 = collect_fees_quote(
            200u128.into(),
            400u128.into(),
            600u128.into(),
            800u128.into(),
            1000u128.into(),
            1200u128.into(),
            1400u128.into(),
            1600u128.into(),
            1800u128.into(),
            2000u128.into(),
            2200u128.into(),
            9,
            7,
            5,
        );
        assert_eq!(result2.fee_owed_a, 2000);
        assert_eq!(result2.fee_owed_b, 2200);

        // Test case 3: tick_current_index == tick_lower_index
        let result3 = collect_fees_quote(
            300u128.into(),
            600u128.into(),
            900u128.into(),
            1200u128.into(),
            1500u128.into(),
            1800u128.into(),
            2100u128.into(),
            2400u128.into(),
            2700u128.into(),
            3000u128.into(),
            3300u128.into(),
            5,
            5,
            7,
        );
        assert_eq!(result3.fee_owed_a, 2700);
        assert_eq!(result3.fee_owed_b, 3000);
    }
}
