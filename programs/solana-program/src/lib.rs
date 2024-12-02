use anchor_lang::prelude::*;

declare_id!("GnYirbPEkU2cjGsLXF7QReLjNF2JWs5eUF2B4wv6vkgB");

#[program]
pub mod voting_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let vote_account = &mut ctx.accounts.vote_account;
        vote_account.budi_kurniawan_votes = 0;
        vote_account.dewi_kartika_votes = 0;
        vote_account.arif_pratama_votes = 0;
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, candidate: u8) -> Result<()> {
        let vote_account = &mut ctx.accounts.vote_account;

        match candidate {
            1 => vote_account.budi_kurniawan_votes += 1,
            2 => vote_account.dewi_kartika_votes += 1,
            3 => vote_account.arif_pratama_votes += 1,
            _ => return Err(error!(ErrorCode::InvalidCandidate)),
        }

        Ok(())
    }
}

#[account]
pub struct VoteAccount {
    pub budi_kurniawan_votes: u64,
    pub dewi_kartika_votes: u64,
    pub arif_pratama_votes: u64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 24)]
    pub vote_account: Account<'info, VoteAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub vote_account: Account<'info, VoteAccount>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid candidate")]
    InvalidCandidate,
}
