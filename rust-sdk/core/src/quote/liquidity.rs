use core::ops::{Shl, Shr};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use ethnum::U256;

use crate::{
    position_status, tick_index_to_sqrt_price, DecreaseLiquidityQuote, IncreaseLiquidityQuote,
    PositionStatus, U128,
};

/// Calculate the quote for decreasing liquidity
///
/// # Parameters
/// - `liquidity_delta` - The amount of liquidity to decrease
/// - `slippage_tolerance` - The slippage tolerance in bps
/// - `tick_current_index` - The current tick index
/// - `tick_lower_index` - The lower tick index of the position
/// - `tick_upper_index` - The upper tick index of the position
/// - `transfer_fee_a` - The transfer fee for token A in bps
/// - `transfer_fee_b` - The transfer fee for token B in bps
///
/// # Returns
/// - A DecreaseLiquidityQuote struct containing the estimated token amounts
#[cfg_attr(feature = "wasm", wasm_bindgen(js_name = decreaseLiquidityQuote, skip_jsdoc))]
pub fn decrease_liquidity_quote(
    liquidity_delta: U128,
    slippage_tolerance: u16,
    tick_current_index: i32,
    tick_lower_index: i32,
    tick_upper_index: i32,
    transfer_fee_a: Option<u16>,
    transfer_fee_b: Option<u16>,
) -> DecreaseLiquidityQuote {
    let liquidity_delta: u128 = liquidity_delta.into();
    if liquidity_delta == 0 {
        return DecreaseLiquidityQuote::default();
    }

    let transfer_fee_a: u128 = transfer_fee_a.unwrap_or(0).into();
    let transfer_fee_b: u128 = transfer_fee_b.unwrap_or(0).into();

    let (token_est_before_fees_a, token_est_before_fees_b) = get_token_estimates_from_liquidity(
        liquidity_delta,
        tick_current_index,
        tick_lower_index,
        tick_upper_index,
    );

    let slippage_tolerance: u128 = slippage_tolerance.into();
    let token_min_before_fees_a =
        token_est_before_fees_a - token_est_before_fees_a * slippage_tolerance / 10000u128;
    let token_min_before_fees_b =
        token_est_before_fees_b - token_est_before_fees_b * slippage_tolerance / 10000u128;

    let token_est_a =
        token_est_before_fees_a - token_est_before_fees_a * transfer_fee_a / 10000u128;
    let token_est_b =
        token_est_before_fees_b - token_est_before_fees_b * transfer_fee_b / 10000u128;
    let token_min_a =
        token_min_before_fees_a - token_min_before_fees_a * transfer_fee_a / 10000u128;
    let token_min_b =
        token_min_before_fees_b - token_min_before_fees_b * transfer_fee_b / 10000u128;

    DecreaseLiquidityQuote {
        liquidity_delta,
        token_est_a,
        token_est_b,
        token_min_a,
        token_min_b,
    }
}

/// Calculate the quote for decreasing liquidity given a token a amount
///
/// # Parameters
/// - `token_delta_a` - The amount of token a to decrease
/// - `slippage_tolerance` - The slippage tolerance in bps
/// - `tick_current_index` - The current tick index
/// - `tick_lower_index` - The lower tick index of the position
/// - `tick_upper_index` - The upper tick index of the position
/// - `transfer_fee_a` - The transfer fee for token A in bps
/// - `transfer_fee_b` - The transfer fee for token B in bps
///
/// # Returns
/// - A DecreaseLiquidityQuote struct containing the estimated token amounts
#[cfg_attr(feature = "wasm", wasm_bindgen(js_name = decreaseLiquidityQuoteA, skip_jsdoc))]
pub fn decrease_liquidity_quote_a(
    token_delta_a: U128,
    slippage_tolerance: u16,
    tick_current_index: i32,
    tick_lower_index: i32,
    tick_upper_index: i32,
    transfer_fee_a: Option<u16>,
    transfer_fee_b: Option<u16>,
) -> DecreaseLiquidityQuote {
    let token_delta_a: u128 = token_delta_a.into();
    if token_delta_a == 0 {
        return DecreaseLiquidityQuote::default();
    }

    let transfer_fee_a_u128: u128 = transfer_fee_a.unwrap_or(0).into();
    let token_delta_a = token_delta_a - token_delta_a * transfer_fee_a_u128 / 10000u128;

    let position_status = position_status(tick_current_index, tick_lower_index, tick_upper_index);

    let liquidity: u128 = match position_status {
        PositionStatus::BelowRange => {
            get_liquidity_from_a(token_delta_a, tick_lower_index, tick_upper_index)
        }
        PositionStatus::AboveRange => 0,
        PositionStatus::InRange => {
            get_liquidity_from_a(token_delta_a, tick_current_index, tick_upper_index)
        }
    };

    decrease_liquidity_quote(
        liquidity.into(),
        slippage_tolerance,
        tick_current_index,
        tick_lower_index,
        tick_upper_index,
        transfer_fee_a,
        transfer_fee_b,
    )
}

/// Calculate the quote for decreasing liquidity given a token b amount
///
/// # Parameters
/// - `token_delta_b` - The amount of token b to decrease
/// - `slippage_tolerance` - The slippage tolerance in bps
/// - `tick_current_index` - The current tick index
/// - `tick_lower_index` - The lower tick index of the position
/// - `tick_upper_index` - The upper tick index of the position
/// - `transfer_fee_a` - The transfer fee for token A in bps
/// - `transfer_fee_b` - The transfer fee for token B in bps
///
/// # Returns
/// - A DecreaseLiquidityQuote struct containing the estimated token amounts
#[cfg_attr(feature = "wasm", wasm_bindgen(js_name = decreaseLiquidityQuoteB, skip_jsdoc))]
pub fn decrease_liquidity_quote_b(
    token_delta_b: U128,
    slippage_tolerance: u16,
    tick_current_index: i32,
    tick_lower_index: i32,
    tick_upper_index: i32,
    transfer_fee_a: Option<u16>,
    transfer_fee_b: Option<u16>,
) -> DecreaseLiquidityQuote {
    let token_delta_b: u128 = token_delta_b.into();
    if token_delta_b == 0 {
        return DecreaseLiquidityQuote::default();
    }

    let transfer_fee_b_u128: u128 = transfer_fee_b.unwrap_or(0).into();
    let token_delta_b = token_delta_b - token_delta_b * transfer_fee_b_u128 / 10000u128;

    let position_status = position_status(tick_current_index, tick_lower_index, tick_upper_index);

    let liquidity: u128 = match position_status {
        PositionStatus::BelowRange => 0,
        PositionStatus::AboveRange => {
            get_liquidity_from_b(token_delta_b, tick_lower_index, tick_upper_index)
        }
        PositionStatus::InRange => {
            get_liquidity_from_b(token_delta_b, tick_lower_index, tick_current_index)
        }
    };

    decrease_liquidity_quote(
        liquidity.into(),
        slippage_tolerance,
        tick_current_index,
        tick_lower_index,
        tick_upper_index,
        transfer_fee_a,
        transfer_fee_b,
    )
}

/// Calculate the quote for increasing liquidity
///
/// # Parameters
/// - `liquidity_delta` - The amount of liquidity to increase
/// - `slippage_tolerance` - The slippage tolerance in bps
/// - `tick_current_index` - The current tick index
/// - `tick_lower_index` - The lower tick index of the position
/// - `tick_upper_index` - The upper tick index of the position
/// - `transfer_fee_a` - The transfer fee for token A in bps
/// - `transfer_fee_b` - The transfer fee for token B in bps
///
/// # Returns
/// - An IncreaseLiquidityQuote struct containing the estimated token amounts
#[cfg_attr(feature = "wasm", wasm_bindgen(js_name = increaseLiquidityQuote, skip_jsdoc))]
pub fn increase_liquidity_quote(
    liquidity_delta: U128,
    slippage_tolerance: u16,
    tick_current_index: i32,
    tick_lower_index: i32,
    tick_upper_index: i32,
    transfer_fee_a: Option<u16>,
    transfer_fee_b: Option<u16>,
) -> IncreaseLiquidityQuote {
    let liquidity_delta: u128 = liquidity_delta.into();
    if liquidity_delta == 0 {
        return IncreaseLiquidityQuote::default();
    }

    let transfer_fee_a: u128 = transfer_fee_a.unwrap_or(0).into();
    let transfer_fee_b: u128 = transfer_fee_b.unwrap_or(0).into();

    let (token_est_before_fees_a, token_est_before_fees_b) = get_token_estimates_from_liquidity(
        liquidity_delta,
        tick_current_index,
        tick_lower_index,
        tick_upper_index,
    );

    let slippage_tolerance: u128 = slippage_tolerance.into();
    let token_max_before_fees_a =
        token_est_before_fees_a + token_est_before_fees_a * slippage_tolerance / 10000u128;
    let token_max_before_fees_b =
        token_est_before_fees_b + token_est_before_fees_b * slippage_tolerance / 10000u128;

    let token_est_a =
        token_est_before_fees_a + token_est_before_fees_a * transfer_fee_a / 10000u128;
    let token_est_b =
        token_est_before_fees_b + token_est_before_fees_b * transfer_fee_b / 10000u128;
    let token_max_a =
        token_max_before_fees_a + token_max_before_fees_a * transfer_fee_a / 10000u128;
    let token_max_b =
        token_max_before_fees_b + token_max_before_fees_b * transfer_fee_b / 10000u128;

    IncreaseLiquidityQuote {
        liquidity_delta,
        token_est_a,
        token_est_b,
        token_max_a,
        token_max_b,
    }
}

/// Calculate the quote for increasing liquidity given a token a amount
///
/// # Parameters
/// - `token_delta_a` - The amount of token a to increase
/// - `slippage_tolerance` - The slippage tolerance in bps
/// - `tick_current_index` - The current tick index
/// - `tick_lower_index` - The lower tick index of the position
/// - `tick_upper_index` - The upper tick index of the position
/// - `transfer_fee_a` - The transfer fee for token A in bps
/// - `transfer_fee_b` - The transfer fee for token B in bps
///
/// # Returns
/// - An IncreaseLiquidityQuote struct containing the estimated token amounts
#[cfg_attr(feature = "wasm", wasm_bindgen(js_name = increaseLiquidityQuoteA, skip_jsdoc))]
pub fn increase_liquidity_quote_a(
    token_delta_a: U128,
    slippage_tolerance: u16,
    tick_current_index: i32,
    tick_lower_index: i32,
    tick_upper_index: i32,
    transfer_fee_a: Option<u16>,
    transfer_fee_b: Option<u16>,
) -> IncreaseLiquidityQuote {
    let token_delta_a: u128 = token_delta_a.into();
    if token_delta_a == 0 {
        return IncreaseLiquidityQuote::default();
    }

    let transfer_fee_a_u128: u128 = transfer_fee_a.unwrap_or(0).into();
    let token_delta_a = token_delta_a - token_delta_a * transfer_fee_a_u128 / 10000u128;

    let position_status = position_status(tick_current_index, tick_lower_index, tick_upper_index);

    let liquidity: u128 = match position_status {
        PositionStatus::BelowRange => {
            get_liquidity_from_a(token_delta_a, tick_lower_index, tick_upper_index)
        }
        PositionStatus::AboveRange => 0,
        PositionStatus::InRange => {
            get_liquidity_from_a(token_delta_a, tick_current_index, tick_upper_index)
        }
    };

    increase_liquidity_quote(
        liquidity.into(),
        slippage_tolerance,
        tick_current_index,
        tick_lower_index,
        tick_upper_index,
        transfer_fee_a,
        transfer_fee_b,
    )
}

/// Calculate the quote for increasing liquidity given a token b amount
///
/// # Parameters
/// - `token_delta_b` - The amount of token b to increase
/// - `slippage_tolerance` - The slippage tolerance in bps
/// - `tick_current_index` - The current tick index
/// - `tick_lower_index` - The lower tick index of the position
/// - `tick_upper_index` - The upper tick index of the position
/// - `transfer_fee_a` - The transfer fee for token A in bps
/// - `transfer_fee_b` - The transfer fee for token B in bps
///
/// # Returns
/// - An IncreaseLiquidityQuote struct containing the estimated token amounts
#[cfg_attr(feature = "wasm", wasm_bindgen(js_name = increaseLiquidityQuoteB, skip_jsdoc))]
pub fn increase_liquidity_quote_b(
    token_delta_b: U128,
    slippage_tolerance: u16,
    tick_current_index: i32,
    tick_lower_index: i32,
    tick_upper_index: i32,
    transfer_fee_a: Option<u16>,
    transfer_fee_b: Option<u16>,
) -> IncreaseLiquidityQuote {
    let token_delta_b: u128 = token_delta_b.into();
    if token_delta_b == 0 {
        return IncreaseLiquidityQuote::default();
    }

    let transfer_fee_b_u128: u128 = transfer_fee_b.unwrap_or(0).into();
    let token_delta_b = token_delta_b - token_delta_b * transfer_fee_b_u128 / 10000u128;

    let position_status = position_status(tick_current_index, tick_lower_index, tick_upper_index);

    let liquidity: u128 = match position_status {
        PositionStatus::BelowRange => 0,
        PositionStatus::AboveRange => {
            get_liquidity_from_b(token_delta_b, tick_lower_index, tick_upper_index)
        }
        PositionStatus::InRange => {
            get_liquidity_from_b(token_delta_b, tick_lower_index, tick_current_index)
        }
    };

    increase_liquidity_quote(
        liquidity.into(),
        slippage_tolerance,
        tick_current_index,
        tick_lower_index,
        tick_upper_index,
        transfer_fee_a,
        transfer_fee_b,
    )
}

// Private functions

fn get_liquidity_from_a(token_delta_a: u128, tick_lower_index: i32, tick_upper_index: i32) -> u128 {
    let sqrt_price_lower: U256 = tick_index_to_sqrt_price(tick_lower_index).into();
    let sqrt_price_upper: U256 = tick_index_to_sqrt_price(tick_upper_index).into();
    let result: U256 = <U256>::from(token_delta_a)
        .saturating_mul(sqrt_price_lower)
        .saturating_mul(sqrt_price_upper)
        .saturating_div(sqrt_price_upper - sqrt_price_lower)
        .shr(64);
    result.try_into().unwrap()
}

fn get_token_a_from_liquidity(
    liquidity_delta: u128,
    tick_lower_index: i32,
    tick_upper_index: i32,
) -> u128 {
    let sqrt_price_lower: U256 = tick_index_to_sqrt_price(tick_lower_index).into();
    let sqrt_price_upper: U256 = tick_index_to_sqrt_price(tick_upper_index).into();
    let numerator: U256 = <U256>::from(liquidity_delta)
        .saturating_mul(sqrt_price_upper - sqrt_price_lower)
        .shl(64);
    let denominator = sqrt_price_upper.saturating_mul(sqrt_price_lower);
    numerator.saturating_div(denominator).try_into().unwrap()
}

fn get_liquidity_from_b(token_delta_b: u128, tick_lower_index: i32, tick_upper_index: i32) -> u128 {
    let sqrt_price_lower: U256 = tick_index_to_sqrt_price(tick_lower_index).into();
    let sqrt_price_upper: U256 = tick_index_to_sqrt_price(tick_upper_index).into();
    let numerator: U256 = <U256>::from(token_delta_b).shl(64);
    let denominator = sqrt_price_upper - sqrt_price_lower;
    numerator.saturating_div(denominator).try_into().unwrap()
}

fn get_token_b_from_liquidity(
    liquidity_delta: u128,
    tick_lower_index: i32,
    tick_upper_index: i32,
) -> u128 {
    let sqrt_price_lower: U256 = tick_index_to_sqrt_price(tick_lower_index).into();
    let sqrt_price_upper: U256 = tick_index_to_sqrt_price(tick_upper_index).into();
    let result: U256 = <U256>::from(liquidity_delta)
        .saturating_mul(sqrt_price_upper - sqrt_price_lower)
        .shr(64);
    result.try_into().unwrap()
}

fn get_token_estimates_from_liquidity(
    liquidity_delta: u128,
    tick_current_index: i32,
    tick_lower_index: i32,
    tick_upper_index: i32,
) -> (u128, u128) {
    if liquidity_delta == 0 {
        return (0, 0);
    }

    let position_status = position_status(tick_current_index, tick_lower_index, tick_upper_index);

    match position_status {
        PositionStatus::BelowRange => {
            let token_a =
                get_token_a_from_liquidity(liquidity_delta, tick_lower_index, tick_upper_index);
            (token_a, 0)
        }
        PositionStatus::InRange => {
            let token_a =
                get_token_a_from_liquidity(liquidity_delta, tick_lower_index, tick_current_index);
            let token_b =
                get_token_b_from_liquidity(liquidity_delta, tick_current_index, tick_upper_index);
            (token_a, token_b)
        }
        PositionStatus::AboveRange => {
            let token_b =
                get_token_b_from_liquidity(liquidity_delta, tick_lower_index, tick_upper_index);
            (0, token_b)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decrease_liquidity_quote() {
        // Below range
        let result = decrease_liquidity_quote(1000000, 100, -20, -10, 10, None, None);
        assert_eq!(result.liquidity_delta, 1000000);
        assert_eq!(result.token_est_a, 999);
        assert_eq!(result.token_est_b, 0);
        assert_eq!(result.token_min_a, 990);
        assert_eq!(result.token_min_b, 0);

        // in range
        let result = decrease_liquidity_quote(1000000, 100, 0, -10, 10, None, None);
        assert_eq!(result.liquidity_delta, 1000000);
        assert_eq!(result.token_est_a, 500);
        assert_eq!(result.token_est_b, 500);
        assert_eq!(result.token_min_a, 495);
        assert_eq!(result.token_min_b, 495);

        // Above range
        let result = decrease_liquidity_quote(1000000, 100, 20, -10, 10, None, None);
        assert_eq!(result.liquidity_delta, 1000000);
        assert_eq!(result.token_est_a, 0);
        assert_eq!(result.token_est_b, 999);
        assert_eq!(result.token_min_a, 0);
        assert_eq!(result.token_min_b, 990);

        // zero liquidity
        let result = decrease_liquidity_quote(0, 100, 20, -10, 10, None, None);
        assert_eq!(result.liquidity_delta, 0);
        assert_eq!(result.token_est_a, 0);
        assert_eq!(result.token_est_b, 0);
        assert_eq!(result.token_min_a, 0);
        assert_eq!(result.token_min_b, 0);
    }

    #[test]
    fn test_decrease_liquidity_quote_a() {
        // Below range
        let result = decrease_liquidity_quote_a(1000, 100, -20, -10, 10, None, None);
        assert_eq!(result.liquidity_delta, 1000049);
        assert_eq!(result.token_est_a, 999);
        assert_eq!(result.token_est_b, 0);
        assert_eq!(result.token_min_a, 990);
        assert_eq!(result.token_min_b, 0);

        // in range
        let result = decrease_liquidity_quote_a(500, 100, 0, -10, 10, None, None);
        assert_eq!(result.liquidity_delta, 1000300);
        assert_eq!(result.token_est_a, 500);
        assert_eq!(result.token_est_b, 500);
        assert_eq!(result.token_min_a, 495);
        assert_eq!(result.token_min_b, 495);

        // Above range
        let result = decrease_liquidity_quote_a(1000, 100, 20, -10, 10, None, None);
        assert_eq!(result.liquidity_delta, 0);
        assert_eq!(result.token_est_a, 0);
        assert_eq!(result.token_est_b, 0);
        assert_eq!(result.token_min_a, 0);
        assert_eq!(result.token_min_b, 0);

        // zero liquidity
        let result = decrease_liquidity_quote_a(0, 100, 20, -10, 10, None, None);
        assert_eq!(result.liquidity_delta, 0);
        assert_eq!(result.token_est_a, 0);
        assert_eq!(result.token_est_b, 0);
        assert_eq!(result.token_min_a, 0);
        assert_eq!(result.token_min_b, 0);
    }

    #[test]
    fn test_decrease_liquidity_quote_b() {
        // Below range
        let result = decrease_liquidity_quote_b(1000, 100, -20, -10, 10, None, None);
        assert_eq!(result.liquidity_delta, 0);
        assert_eq!(result.token_est_a, 0);
        assert_eq!(result.token_est_b, 0);
        assert_eq!(result.token_min_a, 0);
        assert_eq!(result.token_min_b, 0);

        // in range
        let result = decrease_liquidity_quote_b(500, 100, 0, -10, 10, None, None);
        assert_eq!(result.liquidity_delta, 1000300);
        assert_eq!(result.token_est_a, 500);
        assert_eq!(result.token_est_b, 500);
        assert_eq!(result.token_min_a, 495);
        assert_eq!(result.token_min_b, 495);

        // Above range
        let result = decrease_liquidity_quote_b(1000, 100, 20, -10, 10, None, None);
        assert_eq!(result.liquidity_delta, 1000049);
        assert_eq!(result.token_est_a, 0);
        assert_eq!(result.token_est_b, 999);
        assert_eq!(result.token_min_a, 0);
        assert_eq!(result.token_min_b, 990);

        // zero liquidity
        let result = decrease_liquidity_quote_b(0, 100, 20, -10, 10, None, None);
        assert_eq!(result.liquidity_delta, 0);
        assert_eq!(result.token_est_a, 0);
        assert_eq!(result.token_est_b, 0);
        assert_eq!(result.token_min_a, 0);
        assert_eq!(result.token_min_b, 0);
    }

    #[test]
    fn test_increase_liquidity_quote() {
        // Below range
        let result = increase_liquidity_quote(1000000, 100, -20, -10, 10, None, None);
        assert_eq!(result.liquidity_delta, 1000000);
        assert_eq!(result.token_est_a, 999);
        assert_eq!(result.token_est_b, 0);
        assert_eq!(result.token_max_a, 1008);
        assert_eq!(result.token_max_b, 0);

        // in range
        let result = increase_liquidity_quote(1000000, 100, 0, -10, 10, None, None);
        assert_eq!(result.liquidity_delta, 1000000);
        assert_eq!(result.token_est_a, 500);
        assert_eq!(result.token_est_b, 500);
        assert_eq!(result.token_max_a, 505);
        assert_eq!(result.token_max_b, 505);

        // Above range
        let result = increase_liquidity_quote(1000000, 100, 20, -10, 10, None, None);
        assert_eq!(result.liquidity_delta, 1000000);
        assert_eq!(result.token_est_a, 0);
        assert_eq!(result.token_est_b, 999);
        assert_eq!(result.token_max_a, 0);
        assert_eq!(result.token_max_b, 1008);

        // zero liquidity
        let result = increase_liquidity_quote(0, 100, 20, -10, 10, None, None);
        assert_eq!(result.liquidity_delta, 0);
        assert_eq!(result.token_est_a, 0);
        assert_eq!(result.token_est_b, 0);
        assert_eq!(result.token_max_a, 0);
        assert_eq!(result.token_max_b, 0);
    }

    #[test]
    fn test_increase_liquidity_quote_a() {
        // Below range
        let result = increase_liquidity_quote_a(1000, 100, -20, -10, 10, None, None);
        assert_eq!(result.liquidity_delta, 1000049);
        assert_eq!(result.token_est_a, 999);
        assert_eq!(result.token_est_b, 0);
        assert_eq!(result.token_max_a, 1008);
        assert_eq!(result.token_max_b, 0);

        // in range
        let result = increase_liquidity_quote_a(500, 100, 0, -10, 10, None, None);
        assert_eq!(result.liquidity_delta, 1000300);
        assert_eq!(result.token_est_a, 500);
        assert_eq!(result.token_est_b, 500);
        assert_eq!(result.token_max_a, 505);
        assert_eq!(result.token_max_b, 505);

        // Above range
        let result = increase_liquidity_quote_a(1000, 100, 20, -10, 10, None, None);
        assert_eq!(result.liquidity_delta, 0);
        assert_eq!(result.token_est_a, 0);
        assert_eq!(result.token_est_b, 0);
        assert_eq!(result.token_max_a, 0);
        assert_eq!(result.token_max_b, 0);

        // zero liquidity
        let result = increase_liquidity_quote_a(0, 100, 20, -10, 10, None, None);
        assert_eq!(result.liquidity_delta, 0);
        assert_eq!(result.token_est_a, 0);
        assert_eq!(result.token_est_b, 0);
        assert_eq!(result.token_max_a, 0);
        assert_eq!(result.token_max_b, 0);
    }

    #[test]
    fn test_increase_liquidity_quote_b() {
        // Below range
        let result = increase_liquidity_quote_b(1000, 100, -20, -10, 10, None, None);
        assert_eq!(result.liquidity_delta, 0);
        assert_eq!(result.token_est_a, 0);
        assert_eq!(result.token_est_b, 0);
        assert_eq!(result.token_max_a, 0);
        assert_eq!(result.token_max_b, 0);

        // in range
        let result = increase_liquidity_quote_b(500, 100, 0, -10, 10, None, None);
        assert_eq!(result.liquidity_delta, 1000300);
        assert_eq!(result.token_est_a, 500);
        assert_eq!(result.token_est_b, 500);
        assert_eq!(result.token_max_a, 505);
        assert_eq!(result.token_max_b, 505);

        // Above range
        let result = increase_liquidity_quote_b(1000, 100, 20, -10, 10, None, None);
        assert_eq!(result.liquidity_delta, 1000049);
        assert_eq!(result.token_est_a, 0);
        assert_eq!(result.token_est_b, 999);
        assert_eq!(result.token_max_a, 0);
        assert_eq!(result.token_max_b, 1008);

        // zero liquidity
        let result = increase_liquidity_quote_b(0, 100, 20, -10, 10, None, None);
        assert_eq!(result.liquidity_delta, 0);
        assert_eq!(result.token_est_a, 0);
        assert_eq!(result.token_est_b, 0);
        assert_eq!(result.token_max_a, 0);
        assert_eq!(result.token_max_b, 0);
    }
}
