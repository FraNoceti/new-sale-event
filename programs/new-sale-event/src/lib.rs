use anchor_lang::prelude::*;
// use anchor_spl::token::{self, Burn, Token, Mint, TokenAccount};

declare_id!("7hKa7TtHSFRFo66qW2zQ3vFTLs84AWWzsEUerqy8drJS");

#[program]
pub mod test_credit_sale_program {
    use super::*;

    pub fn record_sale(
        _ctx: Context<RecordSale>,
        credit_id: Pubkey,
        client_id: Pubkey,
        amount: u64,
        price: u64,
    ) -> Result<()> {
        let timestamp = Clock::get()?.unix_timestamp;
        
        msg!(
            "Sale recorded: Credit ID: {}, Client ID: {}, Amount: {}, Price: {}",
            credit_id,
            client_id,
            amount,
            price
        );
        emit!(SaleEvent {
            credit_id: credit_id.to_bytes(),
            client_id: client_id.to_bytes(),
            amount,
            price,
            timestamp,
        });

        Ok(())
    }

    // Modified function to retire and effectively burn tokens
    pub fn retire_token(
        _ctx: Context<RetireToken>,
        credit_id: Pubkey,
        client_id: Pubkey,
        amount: u64,
    ) -> Result<()> {
        let timestamp = Clock::get()?.unix_timestamp;

        msg!(
            "Token retired: Client ID: {}, Credit ID: {}, Amount: {}",
            client_id,
            credit_id,
            amount
        );

        // Perform the burn operation to effectively remove tokens from circulation
        // token::burn(
        //     CpiContext::new(
        //         ctx.accounts.token_program.to_account_info(),
        //         Burn {
        //             mint: ctx.accounts.credit_mint.to_account_info(),
        //             from: ctx.accounts.credit_account.to_account_info(),
        //             authority: ctx.accounts.authority.to_account_info(),
        //         },
        //     ),
        //     amount,
        // )?;

        // Emit a RetireEvent for tracking the burn in logs
        emit!(RetireEvent {
            credit_id: credit_id.to_bytes(),
            client_id: client_id.to_bytes(),
            amount,
            timestamp,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct RecordSale<'info> {
    pub user: Signer<'info>,
}

// Modified context struct for retire_token to include token accounts
#[derive(Accounts)]
pub struct RetireToken<'info> {
    pub user: Signer<'info>,
    
    // #[account(mut)]
    // pub credit_account: Account<'info, TokenAccount>, // Token account holding credits
    
    // #[account(mut)]
    // pub credit_mint: Account<'info, Mint>, // Mint account of the token

    // pub authority: Signer<'info>, // Authority to approve the burn
    
    // pub token_program: Program<'info, Token>, // Reference to the Token Program
}

// Event to log sales
#[event]
pub struct SaleEvent {
    pub credit_id: [u8; 32],
    pub client_id: [u8; 32],
    pub amount: u64,
    pub price: u64,
    pub timestamp: i64,
}

// Event to log retirements
#[event]
pub struct RetireEvent {
    pub credit_id: [u8; 32],
    pub client_id: [u8; 32],
    pub amount: u64,
    pub timestamp: i64,
}
