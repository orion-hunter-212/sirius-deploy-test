use anchor_lang::prelude::*;

declare_id!("zX6qgNnfs9eTPh4oYqfon8a8hunLESweeDhWgttA8hs");

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
