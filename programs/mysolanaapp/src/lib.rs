use anchor_lang::prelude::*;

declare_id!("6Wa9tqiXbJoy7aYQRgKNi5vRPX7x9FDUb2sWz9XEe2Ff");

#[program]
mod mysolanaapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let copy = data.clone();
        base_account.data = data;
        base_account.data_list.push(copy);
        Ok(())
    }

    pub fn update(ctx: Context<Update>, data: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let copy = data.clone();
        base_account.data = data;
        base_account.data_list.push(copy);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 64 + 64)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[account]
pub struct BaseAccount {
    pub data: String,
    pub data_list: Vec<String>,
}

// #[program]
// mod mysolanaapp {
//     use super::*;

//     pub fn create(ctx: Context<Create>) -> ProgramResult {
//         let base_account = &mut ctx.accounts.base_account;
//         base_account.count = 0;
//         Ok(())
//     }

//     pub fn increment(ctx: Context<Increment>) -> ProgramResult {
//         let base_account = &mut ctx.accounts.base_account;
//         base_account.count += 1;
//         Ok(())
//     }
// }

// // Transaction instructions
// #[derive(Accounts)]
// pub struct Create<'info> {
//     #[account(init, payer = user, space = 16 + 16)]
//     pub base_account: Account<'info, BaseAccount>,
//     #[account(mut)]
//     pub user: Signer<'info>,
//     pub system_program: Program <'info, System>,
// }

// // Transaction instructions
// #[derive(Accounts)]
// pub struct Increment<'info> {
//     #[account(mut)]
//     pub base_account: Account<'info, BaseAccount>,
// }

// // An account that goes inside a transaction instruction
// #[account]
// pub struct BaseAccount {
//     pub count: u64,
// }