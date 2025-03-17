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
pub struct CollectProtocolFeesV2 {
      
              
          pub whirlpools_config: solana_program::pubkey::Pubkey,
          
              
          pub whirlpool: solana_program::pubkey::Pubkey,
          
              
          pub collect_protocol_fees_authority: solana_program::pubkey::Pubkey,
          
              
          pub token_mint_a: solana_program::pubkey::Pubkey,
          
              
          pub token_mint_b: solana_program::pubkey::Pubkey,
          
              
          pub token_vault_a: solana_program::pubkey::Pubkey,
          
              
          pub token_vault_b: solana_program::pubkey::Pubkey,
          
              
          pub token_destination_a: solana_program::pubkey::Pubkey,
          
              
          pub token_destination_b: solana_program::pubkey::Pubkey,
          
              
          pub token_program_a: solana_program::pubkey::Pubkey,
          
              
          pub token_program_b: solana_program::pubkey::Pubkey,
          
              
          pub memo_program: solana_program::pubkey::Pubkey,
      }

impl CollectProtocolFeesV2 {
  pub fn instruction(&self, args: CollectProtocolFeesV2InstructionArgs) -> solana_program::instruction::Instruction {
    self.instruction_with_remaining_accounts(args, &[])
  }
  #[allow(clippy::vec_init_then_push)]
  pub fn instruction_with_remaining_accounts(&self, args: CollectProtocolFeesV2InstructionArgs, remaining_accounts: &[solana_program::instruction::AccountMeta]) -> solana_program::instruction::Instruction {
    let mut accounts = Vec::with_capacity(12+ remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.whirlpools_config,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.whirlpool,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.collect_protocol_fees_authority,
            true
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
            self.token_vault_a,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.token_vault_b,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.token_destination_a,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.token_destination_b,
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
                      accounts.extend_from_slice(remaining_accounts);
    let mut data = borsh::to_vec(&CollectProtocolFeesV2InstructionData::new()).unwrap();
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
 pub struct CollectProtocolFeesV2InstructionData {
            discriminator: [u8; 8],
            }

impl CollectProtocolFeesV2InstructionData {
  pub fn new() -> Self {
    Self {
                        discriminator: [103, 128, 222, 134, 114, 200, 22, 200],
                                }
  }
}

impl Default for CollectProtocolFeesV2InstructionData {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
 pub struct CollectProtocolFeesV2InstructionArgs {
                  pub remaining_accounts_info: Option<RemainingAccountsInfo>,
      }


/// Instruction builder for `CollectProtocolFeesV2`.
///
/// ### Accounts:
///
          ///   0. `[]` whirlpools_config
                ///   1. `[writable]` whirlpool
                ///   2. `[signer]` collect_protocol_fees_authority
          ///   3. `[]` token_mint_a
          ///   4. `[]` token_mint_b
                ///   5. `[writable]` token_vault_a
                ///   6. `[writable]` token_vault_b
                ///   7. `[writable]` token_destination_a
                ///   8. `[writable]` token_destination_b
          ///   9. `[]` token_program_a
          ///   10. `[]` token_program_b
          ///   11. `[]` memo_program
#[derive(Clone, Debug, Default)]
pub struct CollectProtocolFeesV2Builder {
            whirlpools_config: Option<solana_program::pubkey::Pubkey>,
                whirlpool: Option<solana_program::pubkey::Pubkey>,
                collect_protocol_fees_authority: Option<solana_program::pubkey::Pubkey>,
                token_mint_a: Option<solana_program::pubkey::Pubkey>,
                token_mint_b: Option<solana_program::pubkey::Pubkey>,
                token_vault_a: Option<solana_program::pubkey::Pubkey>,
                token_vault_b: Option<solana_program::pubkey::Pubkey>,
                token_destination_a: Option<solana_program::pubkey::Pubkey>,
                token_destination_b: Option<solana_program::pubkey::Pubkey>,
                token_program_a: Option<solana_program::pubkey::Pubkey>,
                token_program_b: Option<solana_program::pubkey::Pubkey>,
                memo_program: Option<solana_program::pubkey::Pubkey>,
                        remaining_accounts_info: Option<RemainingAccountsInfo>,
        __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl CollectProtocolFeesV2Builder {
  pub fn new() -> Self {
    Self::default()
  }
            #[inline(always)]
    pub fn whirlpools_config(&mut self, whirlpools_config: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.whirlpools_config = Some(whirlpools_config);
                    self
    }
            #[inline(always)]
    pub fn whirlpool(&mut self, whirlpool: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.whirlpool = Some(whirlpool);
                    self
    }
            #[inline(always)]
    pub fn collect_protocol_fees_authority(&mut self, collect_protocol_fees_authority: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.collect_protocol_fees_authority = Some(collect_protocol_fees_authority);
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
    pub fn token_destination_a(&mut self, token_destination_a: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.token_destination_a = Some(token_destination_a);
                    self
    }
            #[inline(always)]
    pub fn token_destination_b(&mut self, token_destination_b: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.token_destination_b = Some(token_destination_b);
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
    let accounts = CollectProtocolFeesV2 {
                              whirlpools_config: self.whirlpools_config.expect("whirlpools_config is not set"),
                                        whirlpool: self.whirlpool.expect("whirlpool is not set"),
                                        collect_protocol_fees_authority: self.collect_protocol_fees_authority.expect("collect_protocol_fees_authority is not set"),
                                        token_mint_a: self.token_mint_a.expect("token_mint_a is not set"),
                                        token_mint_b: self.token_mint_b.expect("token_mint_b is not set"),
                                        token_vault_a: self.token_vault_a.expect("token_vault_a is not set"),
                                        token_vault_b: self.token_vault_b.expect("token_vault_b is not set"),
                                        token_destination_a: self.token_destination_a.expect("token_destination_a is not set"),
                                        token_destination_b: self.token_destination_b.expect("token_destination_b is not set"),
                                        token_program_a: self.token_program_a.expect("token_program_a is not set"),
                                        token_program_b: self.token_program_b.expect("token_program_b is not set"),
                                        memo_program: self.memo_program.expect("memo_program is not set"),
                      };
          let args = CollectProtocolFeesV2InstructionArgs {
                                                              remaining_accounts_info: self.remaining_accounts_info.clone(),
                                    };
    
    accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
  }
}

  /// `collect_protocol_fees_v2` CPI accounts.
  pub struct CollectProtocolFeesV2CpiAccounts<'a, 'b> {
          
                    
              pub whirlpools_config: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub whirlpool: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub collect_protocol_fees_authority: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub token_mint_a: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub token_mint_b: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub token_vault_a: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub token_vault_b: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub token_destination_a: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub token_destination_b: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub token_program_a: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub token_program_b: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub memo_program: &'b solana_program::account_info::AccountInfo<'a>,
            }

/// `collect_protocol_fees_v2` CPI instruction.
pub struct CollectProtocolFeesV2Cpi<'a, 'b> {
  /// The program to invoke.
  pub __program: &'b solana_program::account_info::AccountInfo<'a>,
      
              
          pub whirlpools_config: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub whirlpool: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub collect_protocol_fees_authority: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub token_mint_a: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub token_mint_b: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub token_vault_a: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub token_vault_b: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub token_destination_a: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub token_destination_b: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub token_program_a: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub token_program_b: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub memo_program: &'b solana_program::account_info::AccountInfo<'a>,
            /// The arguments for the instruction.
    pub __args: CollectProtocolFeesV2InstructionArgs,
  }

impl<'a, 'b> CollectProtocolFeesV2Cpi<'a, 'b> {
  pub fn new(
    program: &'b solana_program::account_info::AccountInfo<'a>,
          accounts: CollectProtocolFeesV2CpiAccounts<'a, 'b>,
              args: CollectProtocolFeesV2InstructionArgs,
      ) -> Self {
    Self {
      __program: program,
              whirlpools_config: accounts.whirlpools_config,
              whirlpool: accounts.whirlpool,
              collect_protocol_fees_authority: accounts.collect_protocol_fees_authority,
              token_mint_a: accounts.token_mint_a,
              token_mint_b: accounts.token_mint_b,
              token_vault_a: accounts.token_vault_a,
              token_vault_b: accounts.token_vault_b,
              token_destination_a: accounts.token_destination_a,
              token_destination_b: accounts.token_destination_b,
              token_program_a: accounts.token_program_a,
              token_program_b: accounts.token_program_b,
              memo_program: accounts.memo_program,
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
    let mut accounts = Vec::with_capacity(12+ remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.whirlpools_config.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.whirlpool.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.collect_protocol_fees_authority.key,
            true
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
            *self.token_vault_a.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.token_vault_b.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.token_destination_a.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.token_destination_b.key,
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
                      remaining_accounts.iter().for_each(|remaining_account| {
      accounts.push(solana_program::instruction::AccountMeta {
          pubkey: *remaining_account.0.key,
          is_signer: remaining_account.1,
          is_writable: remaining_account.2,
      })
    });
    let mut data = borsh::to_vec(&CollectProtocolFeesV2InstructionData::new()).unwrap();
          let mut args = borsh::to_vec(&self.__args).unwrap();
      data.append(&mut args);
    
    let instruction = solana_program::instruction::Instruction {
      program_id: crate::WHIRLPOOL_ID,
      accounts,
      data,
    };
    let mut account_infos = Vec::with_capacity(13 + remaining_accounts.len());
    account_infos.push(self.__program.clone());
                  account_infos.push(self.whirlpools_config.clone());
                        account_infos.push(self.whirlpool.clone());
                        account_infos.push(self.collect_protocol_fees_authority.clone());
                        account_infos.push(self.token_mint_a.clone());
                        account_infos.push(self.token_mint_b.clone());
                        account_infos.push(self.token_vault_a.clone());
                        account_infos.push(self.token_vault_b.clone());
                        account_infos.push(self.token_destination_a.clone());
                        account_infos.push(self.token_destination_b.clone());
                        account_infos.push(self.token_program_a.clone());
                        account_infos.push(self.token_program_b.clone());
                        account_infos.push(self.memo_program.clone());
              remaining_accounts.iter().for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

    if signers_seeds.is_empty() {
      solana_program::program::invoke(&instruction, &account_infos)
    } else {
      solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
    }
  }
}

/// Instruction builder for `CollectProtocolFeesV2` via CPI.
///
/// ### Accounts:
///
          ///   0. `[]` whirlpools_config
                ///   1. `[writable]` whirlpool
                ///   2. `[signer]` collect_protocol_fees_authority
          ///   3. `[]` token_mint_a
          ///   4. `[]` token_mint_b
                ///   5. `[writable]` token_vault_a
                ///   6. `[writable]` token_vault_b
                ///   7. `[writable]` token_destination_a
                ///   8. `[writable]` token_destination_b
          ///   9. `[]` token_program_a
          ///   10. `[]` token_program_b
          ///   11. `[]` memo_program
#[derive(Clone, Debug)]
pub struct CollectProtocolFeesV2CpiBuilder<'a, 'b> {
  instruction: Box<CollectProtocolFeesV2CpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CollectProtocolFeesV2CpiBuilder<'a, 'b> {
  pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
    let instruction = Box::new(CollectProtocolFeesV2CpiBuilderInstruction {
      __program: program,
              whirlpools_config: None,
              whirlpool: None,
              collect_protocol_fees_authority: None,
              token_mint_a: None,
              token_mint_b: None,
              token_vault_a: None,
              token_vault_b: None,
              token_destination_a: None,
              token_destination_b: None,
              token_program_a: None,
              token_program_b: None,
              memo_program: None,
                                            remaining_accounts_info: None,
                    __remaining_accounts: Vec::new(),
    });
    Self { instruction }
  }
      #[inline(always)]
    pub fn whirlpools_config(&mut self, whirlpools_config: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.whirlpools_config = Some(whirlpools_config);
                    self
    }
      #[inline(always)]
    pub fn whirlpool(&mut self, whirlpool: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.whirlpool = Some(whirlpool);
                    self
    }
      #[inline(always)]
    pub fn collect_protocol_fees_authority(&mut self, collect_protocol_fees_authority: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.collect_protocol_fees_authority = Some(collect_protocol_fees_authority);
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
    pub fn token_destination_a(&mut self, token_destination_a: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.token_destination_a = Some(token_destination_a);
                    self
    }
      #[inline(always)]
    pub fn token_destination_b(&mut self, token_destination_b: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.token_destination_b = Some(token_destination_b);
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
          let args = CollectProtocolFeesV2InstructionArgs {
                                                              remaining_accounts_info: self.instruction.remaining_accounts_info.clone(),
                                    };
        let instruction = CollectProtocolFeesV2Cpi {
        __program: self.instruction.__program,
                  
          whirlpools_config: self.instruction.whirlpools_config.expect("whirlpools_config is not set"),
                  
          whirlpool: self.instruction.whirlpool.expect("whirlpool is not set"),
                  
          collect_protocol_fees_authority: self.instruction.collect_protocol_fees_authority.expect("collect_protocol_fees_authority is not set"),
                  
          token_mint_a: self.instruction.token_mint_a.expect("token_mint_a is not set"),
                  
          token_mint_b: self.instruction.token_mint_b.expect("token_mint_b is not set"),
                  
          token_vault_a: self.instruction.token_vault_a.expect("token_vault_a is not set"),
                  
          token_vault_b: self.instruction.token_vault_b.expect("token_vault_b is not set"),
                  
          token_destination_a: self.instruction.token_destination_a.expect("token_destination_a is not set"),
                  
          token_destination_b: self.instruction.token_destination_b.expect("token_destination_b is not set"),
                  
          token_program_a: self.instruction.token_program_a.expect("token_program_a is not set"),
                  
          token_program_b: self.instruction.token_program_b.expect("token_program_b is not set"),
                  
          memo_program: self.instruction.memo_program.expect("memo_program is not set"),
                          __args: args,
            };
    instruction.invoke_signed_with_remaining_accounts(signers_seeds, &self.instruction.__remaining_accounts)
  }
}

#[derive(Clone, Debug)]
struct CollectProtocolFeesV2CpiBuilderInstruction<'a, 'b> {
  __program: &'b solana_program::account_info::AccountInfo<'a>,
            whirlpools_config: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                whirlpool: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                collect_protocol_fees_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                token_mint_a: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                token_mint_b: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                token_vault_a: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                token_vault_b: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                token_destination_a: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                token_destination_b: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                token_program_a: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                token_program_b: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                memo_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                        remaining_accounts_info: Option<RemainingAccountsInfo>,
        /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
  __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
}

