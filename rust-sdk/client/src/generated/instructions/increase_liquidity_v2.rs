//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use crate::generated::types::RemainingAccountsInfo;
use borsh::BorshSerialize;
use borsh::BorshDeserialize;

/// Accounts.
#[derive(Debug)]
pub struct IncreaseLiquidityV2 {
      
              
          pub whirlpool: solana_program::pubkey::Pubkey,
          
              
          pub token_program_a: solana_program::pubkey::Pubkey,
          
              
          pub token_program_b: solana_program::pubkey::Pubkey,
          
              
          pub memo_program: solana_program::pubkey::Pubkey,
          
              
          pub position_authority: solana_program::pubkey::Pubkey,
          
              
          pub position: solana_program::pubkey::Pubkey,
          
              
          pub position_token_account: solana_program::pubkey::Pubkey,
          
              
          pub token_mint_a: solana_program::pubkey::Pubkey,
          
              
          pub token_mint_b: solana_program::pubkey::Pubkey,
          
              
          pub token_owner_account_a: solana_program::pubkey::Pubkey,
          
              
          pub token_owner_account_b: solana_program::pubkey::Pubkey,
          
              
          pub token_vault_a: solana_program::pubkey::Pubkey,
          
              
          pub token_vault_b: solana_program::pubkey::Pubkey,
          
              
          pub tick_array_lower: solana_program::pubkey::Pubkey,
          
              
          pub tick_array_upper: solana_program::pubkey::Pubkey,
      }

impl IncreaseLiquidityV2 {
  pub fn instruction(&self, args: IncreaseLiquidityV2InstructionArgs) -> solana_program::instruction::Instruction {
    self.instruction_with_remaining_accounts(args, &[])
  }
  #[allow(clippy::vec_init_then_push)]
  pub fn instruction_with_remaining_accounts(&self, args: IncreaseLiquidityV2InstructionArgs, remaining_accounts: &[solana_program::instruction::AccountMeta]) -> solana_program::instruction::Instruction {
    let mut accounts = Vec::with_capacity(15+ remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new(
            self.whirlpool,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program_a,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program_b,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.memo_program,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.position_authority,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.position,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.position_token_account,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_mint_a,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_mint_b,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.token_owner_account_a,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.token_owner_account_b,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.token_vault_a,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.token_vault_b,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.tick_array_lower,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.tick_array_upper,
            false
          ));
                      accounts.extend_from_slice(remaining_accounts);
    let mut data = borsh::to_vec(&IncreaseLiquidityV2InstructionData::new()).unwrap();
          let mut args = borsh::to_vec(&args).unwrap();
      data.append(&mut args);
    
    solana_program::instruction::Instruction {
      program_id: crate::WHIRLPOOL_ID,
      accounts,
      data,
    }
  }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
 pub struct IncreaseLiquidityV2InstructionData {
            discriminator: [u8; 8],
                              }

impl IncreaseLiquidityV2InstructionData {
  pub fn new() -> Self {
    Self {
                        discriminator: [133, 29, 89, 223, 69, 238, 176, 10],
                                                                          }
  }
}

impl Default for IncreaseLiquidityV2InstructionData {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
 pub struct IncreaseLiquidityV2InstructionArgs {
                  pub liquidity_amount: u128,
                pub token_max_a: u64,
                pub token_max_b: u64,
                pub remaining_accounts_info: Option<RemainingAccountsInfo>,
      }


/// Instruction builder for `IncreaseLiquidityV2`.
///
/// ### Accounts:
///
                ///   0. `[writable]` whirlpool
          ///   1. `[]` token_program_a
          ///   2. `[]` token_program_b
          ///   3. `[]` memo_program
                ///   4. `[signer]` position_authority
                ///   5. `[writable]` position
          ///   6. `[]` position_token_account
          ///   7. `[]` token_mint_a
          ///   8. `[]` token_mint_b
                ///   9. `[writable]` token_owner_account_a
                ///   10. `[writable]` token_owner_account_b
                ///   11. `[writable]` token_vault_a
                ///   12. `[writable]` token_vault_b
                ///   13. `[writable]` tick_array_lower
                ///   14. `[writable]` tick_array_upper
#[derive(Clone, Debug, Default)]
pub struct IncreaseLiquidityV2Builder {
            whirlpool: Option<solana_program::pubkey::Pubkey>,
                token_program_a: Option<solana_program::pubkey::Pubkey>,
                token_program_b: Option<solana_program::pubkey::Pubkey>,
                memo_program: Option<solana_program::pubkey::Pubkey>,
                position_authority: Option<solana_program::pubkey::Pubkey>,
                position: Option<solana_program::pubkey::Pubkey>,
                position_token_account: Option<solana_program::pubkey::Pubkey>,
                token_mint_a: Option<solana_program::pubkey::Pubkey>,
                token_mint_b: Option<solana_program::pubkey::Pubkey>,
                token_owner_account_a: Option<solana_program::pubkey::Pubkey>,
                token_owner_account_b: Option<solana_program::pubkey::Pubkey>,
                token_vault_a: Option<solana_program::pubkey::Pubkey>,
                token_vault_b: Option<solana_program::pubkey::Pubkey>,
                tick_array_lower: Option<solana_program::pubkey::Pubkey>,
                tick_array_upper: Option<solana_program::pubkey::Pubkey>,
                        liquidity_amount: Option<u128>,
                token_max_a: Option<u64>,
                token_max_b: Option<u64>,
                remaining_accounts_info: Option<RemainingAccountsInfo>,
        __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl IncreaseLiquidityV2Builder {
  pub fn new() -> Self {
    Self::default()
  }
            #[inline(always)]
    pub fn whirlpool(&mut self, whirlpool: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.whirlpool = Some(whirlpool);
                    self
    }
            #[inline(always)]
    pub fn token_program_a(&mut self, token_program_a: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.token_program_a = Some(token_program_a);
                    self
    }
            #[inline(always)]
    pub fn token_program_b(&mut self, token_program_b: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.token_program_b = Some(token_program_b);
                    self
    }
            #[inline(always)]
    pub fn memo_program(&mut self, memo_program: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.memo_program = Some(memo_program);
                    self
    }
            #[inline(always)]
    pub fn position_authority(&mut self, position_authority: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.position_authority = Some(position_authority);
                    self
    }
            #[inline(always)]
    pub fn position(&mut self, position: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.position = Some(position);
                    self
    }
            #[inline(always)]
    pub fn position_token_account(&mut self, position_token_account: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.position_token_account = Some(position_token_account);
                    self
    }
            #[inline(always)]
    pub fn token_mint_a(&mut self, token_mint_a: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.token_mint_a = Some(token_mint_a);
                    self
    }
            #[inline(always)]
    pub fn token_mint_b(&mut self, token_mint_b: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.token_mint_b = Some(token_mint_b);
                    self
    }
            #[inline(always)]
    pub fn token_owner_account_a(&mut self, token_owner_account_a: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.token_owner_account_a = Some(token_owner_account_a);
                    self
    }
            #[inline(always)]
    pub fn token_owner_account_b(&mut self, token_owner_account_b: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.token_owner_account_b = Some(token_owner_account_b);
                    self
    }
            #[inline(always)]
    pub fn token_vault_a(&mut self, token_vault_a: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.token_vault_a = Some(token_vault_a);
                    self
    }
            #[inline(always)]
    pub fn token_vault_b(&mut self, token_vault_b: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.token_vault_b = Some(token_vault_b);
                    self
    }
            #[inline(always)]
    pub fn tick_array_lower(&mut self, tick_array_lower: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.tick_array_lower = Some(tick_array_lower);
                    self
    }
            #[inline(always)]
    pub fn tick_array_upper(&mut self, tick_array_upper: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.tick_array_upper = Some(tick_array_upper);
                    self
    }
                    #[inline(always)]
      pub fn liquidity_amount(&mut self, liquidity_amount: u128) -> &mut Self {
        self.liquidity_amount = Some(liquidity_amount);
        self
      }
                #[inline(always)]
      pub fn token_max_a(&mut self, token_max_a: u64) -> &mut Self {
        self.token_max_a = Some(token_max_a);
        self
      }
                #[inline(always)]
      pub fn token_max_b(&mut self, token_max_b: u64) -> &mut Self {
        self.token_max_b = Some(token_max_b);
        self
      }
                /// `[optional argument]`
#[inline(always)]
      pub fn remaining_accounts_info(&mut self, remaining_accounts_info: RemainingAccountsInfo) -> &mut Self {
        self.remaining_accounts_info = Some(remaining_accounts_info);
        self
      }
        /// Add an additional account to the instruction.
  #[inline(always)]
  pub fn add_remaining_account(&mut self, account: solana_program::instruction::AccountMeta) -> &mut Self {
    self.__remaining_accounts.push(account);
    self
  }
  /// Add additional accounts to the instruction.
  #[inline(always)]
  pub fn add_remaining_accounts(&mut self, accounts: &[solana_program::instruction::AccountMeta]) -> &mut Self {
    self.__remaining_accounts.extend_from_slice(accounts);
    self
  }
  #[allow(clippy::clone_on_copy)]
  pub fn instruction(&self) -> solana_program::instruction::Instruction {
    let accounts = IncreaseLiquidityV2 {
                              whirlpool: self.whirlpool.expect("whirlpool is not set"),
                                        token_program_a: self.token_program_a.expect("token_program_a is not set"),
                                        token_program_b: self.token_program_b.expect("token_program_b is not set"),
                                        memo_program: self.memo_program.expect("memo_program is not set"),
                                        position_authority: self.position_authority.expect("position_authority is not set"),
                                        position: self.position.expect("position is not set"),
                                        position_token_account: self.position_token_account.expect("position_token_account is not set"),
                                        token_mint_a: self.token_mint_a.expect("token_mint_a is not set"),
                                        token_mint_b: self.token_mint_b.expect("token_mint_b is not set"),
                                        token_owner_account_a: self.token_owner_account_a.expect("token_owner_account_a is not set"),
                                        token_owner_account_b: self.token_owner_account_b.expect("token_owner_account_b is not set"),
                                        token_vault_a: self.token_vault_a.expect("token_vault_a is not set"),
                                        token_vault_b: self.token_vault_b.expect("token_vault_b is not set"),
                                        tick_array_lower: self.tick_array_lower.expect("tick_array_lower is not set"),
                                        tick_array_upper: self.tick_array_upper.expect("tick_array_upper is not set"),
                      };
          let args = IncreaseLiquidityV2InstructionArgs {
                                                              liquidity_amount: self.liquidity_amount.clone().expect("liquidity_amount is not set"),
                                                                  token_max_a: self.token_max_a.clone().expect("token_max_a is not set"),
                                                                  token_max_b: self.token_max_b.clone().expect("token_max_b is not set"),
                                                                  remaining_accounts_info: self.remaining_accounts_info.clone(),
                                    };
    
    accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
  }
}

  /// `increase_liquidity_v2` CPI accounts.
  pub struct IncreaseLiquidityV2CpiAccounts<'a, 'b> {
          
                    
              pub whirlpool: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub token_program_a: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub token_program_b: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub memo_program: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub position_authority: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub position: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub position_token_account: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub token_mint_a: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub token_mint_b: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub token_owner_account_a: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub token_owner_account_b: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub token_vault_a: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub token_vault_b: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub tick_array_lower: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub tick_array_upper: &'b solana_program::account_info::AccountInfo<'a>,
            }

/// `increase_liquidity_v2` CPI instruction.
pub struct IncreaseLiquidityV2Cpi<'a, 'b> {
  /// The program to invoke.
  pub __program: &'b solana_program::account_info::AccountInfo<'a>,
      
              
          pub whirlpool: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub token_program_a: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub token_program_b: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub memo_program: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub position_authority: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub position: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub position_token_account: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub token_mint_a: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub token_mint_b: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub token_owner_account_a: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub token_owner_account_b: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub token_vault_a: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub token_vault_b: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub tick_array_lower: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub tick_array_upper: &'b solana_program::account_info::AccountInfo<'a>,
            /// The arguments for the instruction.
    pub __args: IncreaseLiquidityV2InstructionArgs,
  }

impl<'a, 'b> IncreaseLiquidityV2Cpi<'a, 'b> {
  pub fn new(
    program: &'b solana_program::account_info::AccountInfo<'a>,
          accounts: IncreaseLiquidityV2CpiAccounts<'a, 'b>,
              args: IncreaseLiquidityV2InstructionArgs,
      ) -> Self {
    Self {
      __program: program,
              whirlpool: accounts.whirlpool,
              token_program_a: accounts.token_program_a,
              token_program_b: accounts.token_program_b,
              memo_program: accounts.memo_program,
              position_authority: accounts.position_authority,
              position: accounts.position,
              position_token_account: accounts.position_token_account,
              token_mint_a: accounts.token_mint_a,
              token_mint_b: accounts.token_mint_b,
              token_owner_account_a: accounts.token_owner_account_a,
              token_owner_account_b: accounts.token_owner_account_b,
              token_vault_a: accounts.token_vault_a,
              token_vault_b: accounts.token_vault_b,
              tick_array_lower: accounts.tick_array_lower,
              tick_array_upper: accounts.tick_array_upper,
                    __args: args,
          }
  }
  #[inline(always)]
  pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(&[], &[])
  }
  #[inline(always)]
  pub fn invoke_with_remaining_accounts(&self, remaining_accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
  }
  #[inline(always)]
  pub fn invoke_signed(&self, signers_seeds: &[&[&[u8]]]) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
  }
  #[allow(clippy::clone_on_copy)]
  #[allow(clippy::vec_init_then_push)]
  pub fn invoke_signed_with_remaining_accounts(
    &self,
    signers_seeds: &[&[&[u8]]],
    remaining_accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]
  ) -> solana_program::entrypoint::ProgramResult {
    let mut accounts = Vec::with_capacity(15+ remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new(
            *self.whirlpool.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program_a.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program_b.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.memo_program.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.position_authority.key,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.position.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.position_token_account.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_mint_a.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_mint_b.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.token_owner_account_a.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.token_owner_account_b.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.token_vault_a.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.token_vault_b.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.tick_array_lower.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.tick_array_upper.key,
            false
          ));
                      remaining_accounts.iter().for_each(|remaining_account| {
      accounts.push(solana_program::instruction::AccountMeta {
          pubkey: *remaining_account.0.key,
          is_signer: remaining_account.1,
          is_writable: remaining_account.2,
      })
    });
    let mut data = borsh::to_vec(&IncreaseLiquidityV2InstructionData::new()).unwrap();
          let mut args = borsh::to_vec(&self.__args).unwrap();
      data.append(&mut args);
    
    let instruction = solana_program::instruction::Instruction {
      program_id: crate::WHIRLPOOL_ID,
      accounts,
      data,
    };
    let mut account_infos = Vec::with_capacity(16 + remaining_accounts.len());
    account_infos.push(self.__program.clone());
                  account_infos.push(self.whirlpool.clone());
                        account_infos.push(self.token_program_a.clone());
                        account_infos.push(self.token_program_b.clone());
                        account_infos.push(self.memo_program.clone());
                        account_infos.push(self.position_authority.clone());
                        account_infos.push(self.position.clone());
                        account_infos.push(self.position_token_account.clone());
                        account_infos.push(self.token_mint_a.clone());
                        account_infos.push(self.token_mint_b.clone());
                        account_infos.push(self.token_owner_account_a.clone());
                        account_infos.push(self.token_owner_account_b.clone());
                        account_infos.push(self.token_vault_a.clone());
                        account_infos.push(self.token_vault_b.clone());
                        account_infos.push(self.tick_array_lower.clone());
                        account_infos.push(self.tick_array_upper.clone());
              remaining_accounts.iter().for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

    if signers_seeds.is_empty() {
      solana_program::program::invoke(&instruction, &account_infos)
    } else {
      solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
    }
  }
}

/// Instruction builder for `IncreaseLiquidityV2` via CPI.
///
/// ### Accounts:
///
                ///   0. `[writable]` whirlpool
          ///   1. `[]` token_program_a
          ///   2. `[]` token_program_b
          ///   3. `[]` memo_program
                ///   4. `[signer]` position_authority
                ///   5. `[writable]` position
          ///   6. `[]` position_token_account
          ///   7. `[]` token_mint_a
          ///   8. `[]` token_mint_b
                ///   9. `[writable]` token_owner_account_a
                ///   10. `[writable]` token_owner_account_b
                ///   11. `[writable]` token_vault_a
                ///   12. `[writable]` token_vault_b
                ///   13. `[writable]` tick_array_lower
                ///   14. `[writable]` tick_array_upper
#[derive(Clone, Debug)]
pub struct IncreaseLiquidityV2CpiBuilder<'a, 'b> {
  instruction: Box<IncreaseLiquidityV2CpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> IncreaseLiquidityV2CpiBuilder<'a, 'b> {
  pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
    let instruction = Box::new(IncreaseLiquidityV2CpiBuilderInstruction {
      __program: program,
              whirlpool: None,
              token_program_a: None,
              token_program_b: None,
              memo_program: None,
              position_authority: None,
              position: None,
              position_token_account: None,
              token_mint_a: None,
              token_mint_b: None,
              token_owner_account_a: None,
              token_owner_account_b: None,
              token_vault_a: None,
              token_vault_b: None,
              tick_array_lower: None,
              tick_array_upper: None,
                                            liquidity_amount: None,
                                token_max_a: None,
                                token_max_b: None,
                                remaining_accounts_info: None,
                    __remaining_accounts: Vec::new(),
    });
    Self { instruction }
  }
      #[inline(always)]
    pub fn whirlpool(&mut self, whirlpool: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.whirlpool = Some(whirlpool);
                    self
    }
      #[inline(always)]
    pub fn token_program_a(&mut self, token_program_a: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.token_program_a = Some(token_program_a);
                    self
    }
      #[inline(always)]
    pub fn token_program_b(&mut self, token_program_b: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.token_program_b = Some(token_program_b);
                    self
    }
      #[inline(always)]
    pub fn memo_program(&mut self, memo_program: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.memo_program = Some(memo_program);
                    self
    }
      #[inline(always)]
    pub fn position_authority(&mut self, position_authority: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.position_authority = Some(position_authority);
                    self
    }
      #[inline(always)]
    pub fn position(&mut self, position: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.position = Some(position);
                    self
    }
      #[inline(always)]
    pub fn position_token_account(&mut self, position_token_account: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.position_token_account = Some(position_token_account);
                    self
    }
      #[inline(always)]
    pub fn token_mint_a(&mut self, token_mint_a: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.token_mint_a = Some(token_mint_a);
                    self
    }
      #[inline(always)]
    pub fn token_mint_b(&mut self, token_mint_b: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.token_mint_b = Some(token_mint_b);
                    self
    }
      #[inline(always)]
    pub fn token_owner_account_a(&mut self, token_owner_account_a: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.token_owner_account_a = Some(token_owner_account_a);
                    self
    }
      #[inline(always)]
    pub fn token_owner_account_b(&mut self, token_owner_account_b: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.token_owner_account_b = Some(token_owner_account_b);
                    self
    }
      #[inline(always)]
    pub fn token_vault_a(&mut self, token_vault_a: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.token_vault_a = Some(token_vault_a);
                    self
    }
      #[inline(always)]
    pub fn token_vault_b(&mut self, token_vault_b: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.token_vault_b = Some(token_vault_b);
                    self
    }
      #[inline(always)]
    pub fn tick_array_lower(&mut self, tick_array_lower: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.tick_array_lower = Some(tick_array_lower);
                    self
    }
      #[inline(always)]
    pub fn tick_array_upper(&mut self, tick_array_upper: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.tick_array_upper = Some(tick_array_upper);
                    self
    }
                    #[inline(always)]
      pub fn liquidity_amount(&mut self, liquidity_amount: u128) -> &mut Self {
        self.instruction.liquidity_amount = Some(liquidity_amount);
        self
      }
                #[inline(always)]
      pub fn token_max_a(&mut self, token_max_a: u64) -> &mut Self {
        self.instruction.token_max_a = Some(token_max_a);
        self
      }
                #[inline(always)]
      pub fn token_max_b(&mut self, token_max_b: u64) -> &mut Self {
        self.instruction.token_max_b = Some(token_max_b);
        self
      }
                /// `[optional argument]`
#[inline(always)]
      pub fn remaining_accounts_info(&mut self, remaining_accounts_info: RemainingAccountsInfo) -> &mut Self {
        self.instruction.remaining_accounts_info = Some(remaining_accounts_info);
        self
      }
        /// Add an additional account to the instruction.
  #[inline(always)]
  pub fn add_remaining_account(&mut self, account: &'b solana_program::account_info::AccountInfo<'a>, is_writable: bool, is_signer: bool) -> &mut Self {
    self.instruction.__remaining_accounts.push((account, is_writable, is_signer));
    self
  }
  /// Add additional accounts to the instruction.
  ///
  /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
  /// and a `bool` indicating whether the account is a signer or not.
  #[inline(always)]
  pub fn add_remaining_accounts(&mut self, accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]) -> &mut Self {
    self.instruction.__remaining_accounts.extend_from_slice(accounts);
    self
  }
  #[inline(always)]
  pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed(&[])
  }
  #[allow(clippy::clone_on_copy)]
  #[allow(clippy::vec_init_then_push)]
  pub fn invoke_signed(&self, signers_seeds: &[&[&[u8]]]) -> solana_program::entrypoint::ProgramResult {
          let args = IncreaseLiquidityV2InstructionArgs {
                                                              liquidity_amount: self.instruction.liquidity_amount.clone().expect("liquidity_amount is not set"),
                                                                  token_max_a: self.instruction.token_max_a.clone().expect("token_max_a is not set"),
                                                                  token_max_b: self.instruction.token_max_b.clone().expect("token_max_b is not set"),
                                                                  remaining_accounts_info: self.instruction.remaining_accounts_info.clone(),
                                    };
        let instruction = IncreaseLiquidityV2Cpi {
        __program: self.instruction.__program,
                  
          whirlpool: self.instruction.whirlpool.expect("whirlpool is not set"),
                  
          token_program_a: self.instruction.token_program_a.expect("token_program_a is not set"),
                  
          token_program_b: self.instruction.token_program_b.expect("token_program_b is not set"),
                  
          memo_program: self.instruction.memo_program.expect("memo_program is not set"),
                  
          position_authority: self.instruction.position_authority.expect("position_authority is not set"),
                  
          position: self.instruction.position.expect("position is not set"),
                  
          position_token_account: self.instruction.position_token_account.expect("position_token_account is not set"),
                  
          token_mint_a: self.instruction.token_mint_a.expect("token_mint_a is not set"),
                  
          token_mint_b: self.instruction.token_mint_b.expect("token_mint_b is not set"),
                  
          token_owner_account_a: self.instruction.token_owner_account_a.expect("token_owner_account_a is not set"),
                  
          token_owner_account_b: self.instruction.token_owner_account_b.expect("token_owner_account_b is not set"),
                  
          token_vault_a: self.instruction.token_vault_a.expect("token_vault_a is not set"),
                  
          token_vault_b: self.instruction.token_vault_b.expect("token_vault_b is not set"),
                  
          tick_array_lower: self.instruction.tick_array_lower.expect("tick_array_lower is not set"),
                  
          tick_array_upper: self.instruction.tick_array_upper.expect("tick_array_upper is not set"),
                          __args: args,
            };
    instruction.invoke_signed_with_remaining_accounts(signers_seeds, &self.instruction.__remaining_accounts)
  }
}

#[derive(Clone, Debug)]
struct IncreaseLiquidityV2CpiBuilderInstruction<'a, 'b> {
  __program: &'b solana_program::account_info::AccountInfo<'a>,
            whirlpool: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                token_program_a: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                token_program_b: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                memo_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                position_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                position: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                position_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                token_mint_a: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                token_mint_b: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                token_owner_account_a: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                token_owner_account_b: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                token_vault_a: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                token_vault_b: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                tick_array_lower: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                tick_array_upper: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                        liquidity_amount: Option<u128>,
                token_max_a: Option<u64>,
                token_max_b: Option<u64>,
                remaining_accounts_info: Option<RemainingAccountsInfo>,
        /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
  __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
}

