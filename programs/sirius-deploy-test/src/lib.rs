use anchor_lang::prelude::*;

declare_id!("DvWEH2ZJnfEXsRZEcGPGnS5PsVxyRzcDAsYR6jrYXmhs");

#[program]
pub mod sirius_deploy_test {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
