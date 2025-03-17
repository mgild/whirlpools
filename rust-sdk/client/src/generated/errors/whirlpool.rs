//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use num_derive::FromPrimitive;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum WhirlpoolError {
    /// 6000 - Enum value could not be converted
    #[error("Enum value could not be converted")]
    InvalidEnum = 0x1770,
    /// 6001 - Invalid start tick index provided.
    #[error("Invalid start tick index provided.")]
    InvalidStartTick = 0x1771,
    /// 6002 - Tick-array already exists in this whirlpool
    #[error("Tick-array already exists in this whirlpool")]
    TickArrayExistInPool = 0x1772,
    /// 6003 - Attempt to search for a tick-array failed
    #[error("Attempt to search for a tick-array failed")]
    TickArrayIndexOutofBounds = 0x1773,
    /// 6004 - Tick-spacing is not supported
    #[error("Tick-spacing is not supported")]
    InvalidTickSpacing = 0x1774,
    /// 6005 - Position is not empty It cannot be closed
    #[error("Position is not empty It cannot be closed")]
    ClosePositionNotEmpty = 0x1775,
    /// 6006 - Unable to divide by zero
    #[error("Unable to divide by zero")]
    DivideByZero = 0x1776,
    /// 6007 - Unable to cast number into BigInt
    #[error("Unable to cast number into BigInt")]
    NumberCastError = 0x1777,
    /// 6008 - Unable to down cast number
    #[error("Unable to down cast number")]
    NumberDownCastError = 0x1778,
    /// 6009 - Tick not found within tick array
    #[error("Tick not found within tick array")]
    TickNotFound = 0x1779,
    /// 6010 - Provided tick index is either out of bounds or uninitializable
    #[error("Provided tick index is either out of bounds or uninitializable")]
    InvalidTickIndex = 0x177A,
    /// 6011 - Provided sqrt price out of bounds
    #[error("Provided sqrt price out of bounds")]
    SqrtPriceOutOfBounds = 0x177B,
    /// 6012 - Liquidity amount must be greater than zero
    #[error("Liquidity amount must be greater than zero")]
    LiquidityZero = 0x177C,
    /// 6013 - Liquidity amount must be less than i64::MAX
    #[error("Liquidity amount must be less than i64::MAX")]
    LiquidityTooHigh = 0x177D,
    /// 6014 - Liquidity overflow
    #[error("Liquidity overflow")]
    LiquidityOverflow = 0x177E,
    /// 6015 - Liquidity underflow
    #[error("Liquidity underflow")]
    LiquidityUnderflow = 0x177F,
    /// 6016 - Tick liquidity net underflowed or overflowed
    #[error("Tick liquidity net underflowed or overflowed")]
    LiquidityNetError = 0x1780,
    /// 6017 - Exceeded token max
    #[error("Exceeded token max")]
    TokenMaxExceeded = 0x1781,
    /// 6018 - Did not meet token min
    #[error("Did not meet token min")]
    TokenMinSubceeded = 0x1782,
    /// 6019 - Position token account has a missing or invalid delegate
    #[error("Position token account has a missing or invalid delegate")]
    MissingOrInvalidDelegate = 0x1783,
    /// 6020 - Position token amount must be 1
    #[error("Position token amount must be 1")]
    InvalidPositionTokenAmount = 0x1784,
    /// 6021 - Timestamp should be convertible from i64 to u64
    #[error("Timestamp should be convertible from i64 to u64")]
    InvalidTimestampConversion = 0x1785,
    /// 6022 - Timestamp should be greater than the last updated timestamp
    #[error("Timestamp should be greater than the last updated timestamp")]
    InvalidTimestamp = 0x1786,
    /// 6023 - Invalid tick array sequence provided for instruction.
    #[error("Invalid tick array sequence provided for instruction.")]
    InvalidTickArraySequence = 0x1787,
    /// 6024 - Token Mint in wrong order
    #[error("Token Mint in wrong order")]
    InvalidTokenMintOrder = 0x1788,
    /// 6025 - Reward not initialized
    #[error("Reward not initialized")]
    RewardNotInitialized = 0x1789,
    /// 6026 - Invalid reward index
    #[error("Invalid reward index")]
    InvalidRewardIndex = 0x178A,
    /// 6027 - Reward vault requires amount to support emissions for at least one day
    #[error("Reward vault requires amount to support emissions for at least one day")]
    RewardVaultAmountInsufficient = 0x178B,
    /// 6028 - Exceeded max fee rate
    #[error("Exceeded max fee rate")]
    FeeRateMaxExceeded = 0x178C,
    /// 6029 - Exceeded max protocol fee rate
    #[error("Exceeded max protocol fee rate")]
    ProtocolFeeRateMaxExceeded = 0x178D,
    /// 6030 - Multiplication with shift right overflow
    #[error("Multiplication with shift right overflow")]
    MultiplicationShiftRightOverflow = 0x178E,
    /// 6031 - Muldiv overflow
    #[error("Muldiv overflow")]
    MulDivOverflow = 0x178F,
    /// 6032 - Invalid div_u256 input
    #[error("Invalid div_u256 input")]
    MulDivInvalidInput = 0x1790,
    /// 6033 - Multiplication overflow
    #[error("Multiplication overflow")]
    MultiplicationOverflow = 0x1791,
    /// 6034 - Provided SqrtPriceLimit not in the same direction as the swap.
    #[error("Provided SqrtPriceLimit not in the same direction as the swap.")]
    InvalidSqrtPriceLimitDirection = 0x1792,
    /// 6035 - There are no tradable amount to swap.
    #[error("There are no tradable amount to swap.")]
    ZeroTradableAmount = 0x1793,
    /// 6036 - Amount out below minimum threshold
    #[error("Amount out below minimum threshold")]
    AmountOutBelowMinimum = 0x1794,
    /// 6037 - Amount in above maximum threshold
    #[error("Amount in above maximum threshold")]
    AmountInAboveMaximum = 0x1795,
    /// 6038 - Invalid index for tick array sequence
    #[error("Invalid index for tick array sequence")]
    TickArraySequenceInvalidIndex = 0x1796,
    /// 6039 - Amount calculated overflows
    #[error("Amount calculated overflows")]
    AmountCalcOverflow = 0x1797,
    /// 6040 - Amount remaining overflows
    #[error("Amount remaining overflows")]
    AmountRemainingOverflow = 0x1798,
    /// 6041 - Invalid intermediary mint
    #[error("Invalid intermediary mint")]
    InvalidIntermediaryMint = 0x1799,
    /// 6042 - Duplicate two hop pool
    #[error("Duplicate two hop pool")]
    DuplicateTwoHopPool = 0x179A,
    /// 6043 - Bundle index is out of bounds
    #[error("Bundle index is out of bounds")]
    InvalidBundleIndex = 0x179B,
    /// 6044 - Position has already been opened
    #[error("Position has already been opened")]
    BundledPositionAlreadyOpened = 0x179C,
    /// 6045 - Position has already been closed
    #[error("Position has already been closed")]
    BundledPositionAlreadyClosed = 0x179D,
    /// 6046 - Unable to delete PositionBundle with open positions
    #[error("Unable to delete PositionBundle with open positions")]
    PositionBundleNotDeletable = 0x179E,
    /// 6047 - Token mint has unsupported attributes
    #[error("Token mint has unsupported attributes")]
    UnsupportedTokenMint = 0x179F,
    /// 6048 - Invalid remaining accounts
    #[error("Invalid remaining accounts")]
    RemainingAccountsInvalidSlice = 0x17A0,
    /// 6049 - Insufficient remaining accounts
    #[error("Insufficient remaining accounts")]
    RemainingAccountsInsufficient = 0x17A1,
    /// 6050 - Unable to call transfer hook without extra accounts
    #[error("Unable to call transfer hook without extra accounts")]
    NoExtraAccountsForTransferHook = 0x17A2,
    /// 6051 - Output and input amount mismatch
    #[error("Output and input amount mismatch")]
    IntermediateTokenAmountMismatch = 0x17A3,
    /// 6052 - Transfer fee calculation failed
    #[error("Transfer fee calculation failed")]
    TransferFeeCalculationError = 0x17A4,
    /// 6053 - Same accounts type is provided more than once
    #[error("Same accounts type is provided more than once")]
    RemainingAccountsDuplicatedAccountsType = 0x17A5,
    /// 6054 - This whirlpool only supports full-range positions
    #[error("This whirlpool only supports full-range positions")]
    FullRangeOnlyPool = 0x17A6,
    /// 6055 - Too many supplemental tick arrays provided
    #[error("Too many supplemental tick arrays provided")]
    TooManySupplementalTickArrays = 0x17A7,
    /// 6056 - TickArray account for different whirlpool provided
    #[error("TickArray account for different whirlpool provided")]
    DifferentWhirlpoolTickArrayAccount = 0x17A8,
    /// 6057 - Trade resulted in partial fill
    #[error("Trade resulted in partial fill")]
    PartialFillError = 0x17A9,
    /// 6058 - Position is not lockable
    #[error("Position is not lockable")]
    PositionNotLockable = 0x17AA,
    /// 6059 - Operation not allowed on locked position
    #[error("Operation not allowed on locked position")]
    OperationNotAllowedOnLockedPosition = 0x17AB,
}

impl solana_program::program_error::PrintProgramError for WhirlpoolError {
    fn print<E>(&self) {
        solana_program::msg!(&self.to_string());
    }
}

impl<T> solana_program::decode_error::DecodeError<T> for WhirlpoolError {
    fn type_of() -> &'static str {
        "WhirlpoolError"
    }
}

