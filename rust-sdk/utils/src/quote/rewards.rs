use core::ops::Shl;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use crate::{CollectRewardsQuote, U128};

/// Calculate rewards owed for a position
///
/// # Paramters
/// - `tick_current_index`: The current tick index
/// - `tick_lower_index`: The lower tick index of the position
/// - `tick_upper_index`: The upper tick index of the position
/// - `pool_liquidity`: The liquidity of the pool
/// - `position_liquidity`: The liquidity of the position
/// - `growth_global_1`: The global growth for reward token 1
/// - `growth_global_2`: The global growth for reward token 2
/// - `growth_global_3`: The global growth for reward token 3
/// - `reward_growth_outside_1_lower`: The reward growth outside the lower tick for reward token 1
/// - `reward_growth_outside_2_lower`: The reward growth outside the lower tick for reward token 2
/// - `reward_growth_outside_3_lower`: The reward growth outside the lower tick for reward token 3
/// - `reward_growth_outside_1_upper`: The reward growth outside the upper tick for reward token 1
/// - `reward_growth_outside_2_upper`: The reward growth outside the upper tick for reward token 2
/// - `reward_growth_outside_3_upper`: The reward growth outside the upper tick for reward token 3
/// - `reward_emissions_per_second_1`: The reward emissions per second for reward token 1
/// - `reward_emissions_per_second_2`: The reward emissions per second for reward token 2
/// - `reward_emissions_per_second_3`: The reward emissions per second for reward token 3
/// - `growth_inside_checkpoint_1`: The growth inside the checkpoint for reward token 1
/// - `growth_inside_checkpoint_2`: The growth inside the checkpoint for reward token 2
/// - `growth_inside_checkpoint_3`: The growth inside the checkpoint for reward token 3
/// - `amount_owed_1`: The amount owed for reward token 1
/// - `amount_owed_2`: The amount owed for reward token 2
/// - `amount_owed_3`: The amount owed for reward token 3
/// - `reward_last_updated_timestamp`: The timestamp when the rewards were last updated
///
/// # Returns
/// - `CollectRewardsQuote`: The rewards owed for the 3 reward tokens.
#[allow(clippy::too_many_arguments)]
#[cfg_attr(feature = "wasm", wasm_bindgen(js_name = collectRewardsQuote, skip_jsdoc))]
pub fn collect_rewards_quote(
    tick_current_index: i32,
    tick_lower_index: i32,
    tick_upper_index: i32,
    pool_liquidity: U128,
    position_liquidity: U128,
    growth_global_1: U128,
    growth_global_2: U128,
    growth_global_3: U128,
    reward_growth_outside_1_lower: U128,
    reward_growth_outside_2_lower: U128,
    reward_growth_outside_3_lower: U128,
    reward_growth_outside_1_upper: U128,
    reward_growth_outside_2_upper: U128,
    reward_growth_outside_3_upper: U128,
    reward_emissions_per_second_1: U128,
    reward_emissions_per_second_2: U128,
    reward_emissions_per_second_3: U128,
    growth_inside_checkpoint_1: U128,
    growth_inside_checkpoint_2: U128,
    growth_inside_checkpoint_3: U128,
    amount_owed_1: U128,
    amount_owed_2: U128,
    amount_owed_3: U128,
    reward_last_updated_timestamp: i32,
) -> CollectRewardsQuote {
    let current_timestamp = 0; // TODO: get current timestamp
    let timestamp_delta = (current_timestamp - reward_last_updated_timestamp).unsigned_abs();

    let reward_emissions_per_second_1: u128 = reward_emissions_per_second_1.into();
    let reward_emissions_per_second_2: u128 = reward_emissions_per_second_2.into();
    let reward_emissions_per_second_3: u128 = reward_emissions_per_second_3.into();

    let position_liquidity: u128 = position_liquidity.into();
    let pool_liquidity: u128 = pool_liquidity.into();

    let mut reward_growth_1: u128 = growth_global_1.into();
    let mut reward_growth_2: u128 = growth_global_2.into();
    let mut reward_growth_3: u128 = growth_global_3.into();

    if pool_liquidity != 0 {
        let reward_growth_delta_1 = reward_emissions_per_second_1
            .wrapping_mul(timestamp_delta as u128)
            .wrapping_div(pool_liquidity);
        reward_growth_1 += <u128>::try_from(reward_growth_delta_1).unwrap();

        let reward_growth_delta_2 = reward_emissions_per_second_2
            .wrapping_mul(timestamp_delta as u128)
            .wrapping_div(pool_liquidity);
        reward_growth_2 += <u128>::try_from(reward_growth_delta_2).unwrap();

        let reward_growth_delta_3 = reward_emissions_per_second_3
            .wrapping_mul(timestamp_delta as u128)
            .wrapping_div(pool_liquidity);
        reward_growth_3 += <u128>::try_from(reward_growth_delta_3).unwrap();
    }

    let mut reward_growth_below_1: u128 = reward_growth_outside_1_lower.into();
    let mut reward_growth_below_2: u128 = reward_growth_outside_2_lower.into();
    let mut reward_growth_below_3: u128 = reward_growth_outside_3_lower.into();

    let mut reward_growth_above_1: u128 = reward_growth_outside_1_upper.into();
    let mut reward_growth_above_2: u128 = reward_growth_outside_2_upper.into();
    let mut reward_growth_above_3: u128 = reward_growth_outside_3_upper.into();

    if tick_current_index < tick_lower_index {
        reward_growth_below_1 = reward_growth_1.wrapping_sub(reward_growth_below_1);

        reward_growth_below_2 = reward_growth_2.wrapping_sub(reward_growth_below_2);

        reward_growth_below_3 = reward_growth_3.wrapping_sub(reward_growth_below_3);
    }

    if tick_current_index > tick_upper_index {
        reward_growth_above_1 = reward_growth_1.wrapping_sub(reward_growth_above_1);

        reward_growth_above_2 = reward_growth_2.wrapping_sub(reward_growth_above_2);

        reward_growth_above_3 = reward_growth_3.wrapping_sub(reward_growth_above_3);
    }

    let reward_growth_inside_1 = reward_growth_1
        .wrapping_sub(reward_growth_below_1)
        .wrapping_sub(reward_growth_above_1);

    let reward_growth_inside_2 = reward_growth_2
        .wrapping_sub(reward_growth_below_2)
        .wrapping_sub(reward_growth_above_2);

    let reward_growth_inside_3 = reward_growth_3
        .wrapping_sub(reward_growth_below_3)
        .wrapping_sub(reward_growth_above_3);

    let reward_growth_inside_checkpoint_1: u128 = growth_inside_checkpoint_1.into();
    let reward_growth_inside_checkpoint_2: u128 = growth_inside_checkpoint_2.into();
    let reward_growth_inside_checkpoint_3: u128 = growth_inside_checkpoint_3.into();

    let reward_growth_delta_1 = reward_growth_inside_1
        .wrapping_sub(reward_growth_inside_checkpoint_1)
        .wrapping_mul(position_liquidity);

    let reward_growth_delta_2 = reward_growth_inside_2
        .wrapping_sub(reward_growth_inside_checkpoint_2)
        .wrapping_mul(position_liquidity);

    let reward_growth_delta_3 = reward_growth_inside_3
        .wrapping_sub(reward_growth_inside_checkpoint_3)
        .wrapping_mul(position_liquidity);

    let amount_owed_1: u128 = amount_owed_1.into();
    let amount_owed_2: u128 = amount_owed_2.into();
    let amount_owed_3: u128 = amount_owed_3.into();

    let reward_owed_1 = amount_owed_1.shl(64) + reward_growth_delta_1.shl(64);
    let reward_owed_2 = amount_owed_2.shl(64) + reward_growth_delta_2.shl(64);
    let reward_owed_3 = amount_owed_3.shl(64) + reward_growth_delta_3.shl(64);

    // TOOD: remove transfer fee deduction

    CollectRewardsQuote {
        reward_owed_1,
        reward_owed_2,
        reward_owed_3,
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_rewards_quote() {
        let quote1 = collect_rewards_quote(
            0,
            -100,
            100,
            100000u128.into(),
            50000u128.into(),
            1000000u128.into(),
            2000000u128.into(),
            3000000u128.into(),
            500000u128.into(),
            1000000u128.into(),
            1500000u128.into(),
            2000000u128.into(),
            2500000u128.into(),
            3000000u128.into(),
            100u128.into(),
            200u128.into(),
            300u128.into(),
            500000u128.into(),
            1000000u128.into(),
            1500000u128.into(),
            100u128.into(),
            200u128.into(),
            300u128.into(),
            0,
        );

        assert_eq!(
            quote1.reward_owed_1,
            340282365076264057937093853202723373056
        );
        assert_eq!(
            quote1.reward_owed_2,
            340282364615095457939029470173678534656
        );
        assert_eq!(
            quote1.reward_owed_3,
            340282364153926857940965087144633696256
        );
    }

    #[test]
    fn test_collect_rewards_quote_2() {
        let quote2 = collect_rewards_quote(
            0,
            -100,
            100,
            100000u128.into(),
            50000u128.into(),
            1000000u128.into(),
            2000000u128.into(),
            3000000u128.into(),
            500000u128.into(),
            1000000u128.into(),
            1500000u128.into(),
            2000000u128.into(),
            2500000u128.into(),
            3000000u128.into(),
            100u128.into(),
            200u128.into(),
            300u128.into(),
            500000u128.into(),
            1000000u128.into(),
            1500000u128.into(),
            100u128.into(),
            200u128.into(),
            300u128.into(),
            100,
        );

        assert_eq!(quote2.reward_owed_1, 50000);
        assert_eq!(quote2.reward_owed_2, 100000);
        assert_eq!(quote2.reward_owed_3, 150000);
    }
}
