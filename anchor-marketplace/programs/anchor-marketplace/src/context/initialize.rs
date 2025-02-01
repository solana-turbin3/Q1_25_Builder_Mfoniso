use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::{error::MarketPlaceError, state::MarketPlace};

#[derive(Accounts)]
#[instruction (name:String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init, 
        payer=admin, 
        seeds=[b"marketplace", name.as_str().as_bytes()], 
        bump, 
        space=MarketPlace::INIT_SPACE
    )]
    pub marketplace: Account<'info, MarketPlace>,

    #[account(
    seeds=[b"treasury", marketplace.key().as_ref()],
    bump,
    )]
    pub treasury: SystemAccount<'info>,


    #[account(
    init,
    payer=admin,
    seeds= [b"rewards", marketplace.key().as_ref()],
    bump,
    mint::decimals=6,
    mint::authority=marketplace,
    mint::freeze_authority=marketplace,
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl <'info> Initialize<'info>{


    pub fn initialize(&mut self, name: String, fee: u16, bumps: &InitializeBumps)->Result<()>{
    

    require!(name.len()>0 && name.len()<=4+32, MarketPlaceError::NameTooLong);

    self.marketplace.set_inner(MarketPlace{
        admin: self.admin.key(),
        bump:self.marketplace.bump,
        name, 
        fee,
        treasury_bump: bumps.treasury,
        reward_bump: bumps.rewards_mint,
    });

    Ok(())
    }
}
