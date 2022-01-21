use anchor_lang::prelude::*;

declare_id!("CnkEW19aSUzxQMtgQZZKoZdN5mXebg3vGX46Wexm666n");

#[program]
pub mod benchmarking {
    use anchor_lang::solana_program::log::sol_log_compute_units;

    use super::*;

    pub fn test_push(_ctx: Context<Initialize>) -> ProgramResult {
        let default_pubkey = Pubkey::default();
        sol_log_compute_units();
        let mut accounts = Vec::with_capacity(50);

        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        sol_log_compute_units();
        msg!("len {}", accounts.len());
        Ok(())
    }

    pub fn test_extend(_ctx: Context<Initialize>) -> ProgramResult {
        let default_pubkey = Pubkey::default();
        let mut accounts = vec![];
        sol_log_compute_units();
        accounts.extend(
            [50].iter()
                .map(|_| AccountMeta::new_readonly(default_pubkey, false)),
        );
        sol_log_compute_units();
        msg!("len {}", accounts.len());
        Ok(())
    }

    pub fn test_raw_immutable(_ctx: Context<Initialize>) -> ProgramResult {
        let default_pubkey = Pubkey::default();
        sol_log_compute_units();
        let accounts = vec![
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
        ];
        sol_log_compute_units();
        msg!("len {}", accounts.len());
        Ok(())
    }

    pub fn test_raw_mutable_hybrid_push(_ctx: Context<Initialize>) -> ProgramResult {
        let default_pubkey = Pubkey::default();
        sol_log_compute_units();
        let mut accounts = vec![
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
        ];
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        accounts.push(AccountMeta::new_readonly(default_pubkey, false));
        sol_log_compute_units();
        msg!("len {}", accounts.len());
        Ok(())
    }

    pub fn test_raw_mutable_hybrid_extend(_ctx: Context<Initialize>) -> ProgramResult {
        let default_pubkey = Pubkey::default();
        sol_log_compute_units();
        let mut accounts = vec![
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
            AccountMeta::new_readonly(default_pubkey, false),
        ];
        accounts.extend(
            [5].iter()
                .map(|_| AccountMeta::new_readonly(default_pubkey, false)),
        );
        sol_log_compute_units();
        msg!("len {}", accounts.len());
        Ok(())
    }

    pub fn test_msg(_ctx: Context<Initialize>) -> ProgramResult {
        let int_value = 42;
        let string_value = "42";
        sol_log_compute_units();
        msg!("Simple message without variable");
        sol_log_compute_units();
        msg!("Simple message with int32 var {}", int_value);
        sol_log_compute_units();
        msg!("Simple message with string vq {}", string_value);
        sol_log_compute_units();
        msg!("Simple message with 2 var {}, {}", int_value, string_value);
        sol_log_compute_units();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
