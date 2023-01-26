use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;


declare_id!("EWPA2mbmKLNLXLmr3676DtZFYJnjp7iaVXB2NQZ3BYxB");

#[program]
pub mod nl_sol {

    use super::*;

    pub fn init_nl(ctx: Context<InitNl>) -> ProgramResult {
        let nl_account = &mut ctx.accounts.nl_account;
        let genesis_post_account = &mut ctx.accounts.genesis_post_account;
        let authority = &mut ctx.accounts.authority;

        nl_account.authority = authority.key();
        nl_account.current_post_key = genesis_post_account.key();

        Ok(())
    }

    pub fn signup_user(ctx: Context<SignupUser>, name: String, avatar: String) -> ProgramResult {
        let user_account = &mut ctx.accounts.user_account;
        let authority = &mut ctx.accounts.authority;

        user_account.name = name;
        user_account.avatar = avatar;
        user_account.authority = authority.key();

        Ok(())
    }

    pub fn update_user(ctx: Context<UpdateUser>, name: String, avatar: String) -> ProgramResult {
        let user_account = &mut ctx.accounts.user_account;

        user_account.name = name;
        user_account.avatar = avatar;

        Ok(())
    }

    /*The post is created, Now we can let the client know that the post is created. 
    The client can fetch the post and render it into the UI. Anchor provides a handy
     feature of emitting an event,We can emit an event like post-created. 
     Before emitting an event, We need to define it. */

    pub fn create_post(ctx: Context<CreatePost>, title: String, content: String) -> ProgramResult {
        let nl_account = &mut ctx.accounts.nl_account;
        let post_account = &mut ctx.accounts.post_account;
        let user_account = &mut ctx.accounts.user_account;
        let authority = &mut ctx.accounts.authority;

        post_account.title = title;
        post_account.content = content;
        post_account.user = user_account.key();
        post_account.authority = authority.key();
        post_account.pre_post_key = nl_account.current_post_key;

        nl_account.current_post_key = post_account.key();

        emit!(PostEvent {
            label: "CREATE".to_string(),
            post_id: post_account.key(),
            next_post_id: None
        });

        Ok(())
    }

    pub fn update_post(ctx: Context<UpdatePost>, title: String, content: String) -> ProgramResult {
        let post_account = &mut ctx.accounts.post_account;

        post_account.title = title;
        post_account.content = content;

        emit!(PostEvent {
            label: "UPDATE".to_string(),
            post_id: post_account.key(),
            next_post_id: None
        });

        Ok(())
    }

    pub fn delete_post(ctx: Context<DeletePost>) -> ProgramResult {
        let post_account = &mut ctx.accounts.post_account;
        let next_post_account = &mut ctx.accounts.next_post_account;

        next_post_account.pre_post_key = post_account.pre_post_key;

        emit!(PostEvent {
            label: "DELETE".to_string(),
            post_id: post_account.key(),
            next_post_id: Some(next_post_account.key())
        });

        Ok(())
    }

    pub fn delete_latest_post(ctx: Context<DeleteLatestPost>) -> ProgramResult {
        let post_account = &mut ctx.accounts.post_account;
        let nl_account = &mut ctx.accounts.nl_account;

        nl_account.current_post_key = post_account.pre_post_key;

        emit!(PostEvent {
            label: "DELETE".to_string(),
            post_id: post_account.key(),
            next_post_id: None
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitNl<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 32 + 32 + 32)]
    pub nl_account: Account<'info, NlState>,
    #[account(init, payer = authority, space = 8 + 32 + 32 + 32 + 32 + 8)]
    pub genesis_post_account: Account<'info, PostState>, //To create LinkedList we initialize the 
    //                       blog account with the very first post so, we can link it to the next post.
    #[account(mut)]
    pub authority: Signer<'info>,  //program signer is a creator of the blog.
    pub system_program: Program<'info, System>,  //required by the runtime for creating the account.

}

#[derive(Accounts)]
pub struct CreatePost<'info> {
    #[account(init, payer = authority, space = 8 + 50 + 500 + 32 + 32 + 32 + 32 + 32 + 32)]
    pub post_account: Account<'info, PostState>,
    #[account(mut, has_one = authority)]
    pub user_account: Account<'info, UserState>,
    #[account(mut)]
    pub nl_account: Account<'info, NlState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdatePost<'info> {
    #[account(
        mut,
        has_one = authority,
    )]
    pub post_account: Account<'info, PostState>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeletePost<'info> {
    #[account(
        mut,
        has_one = authority,
        close = authority,
        constraint = post_account.key() == next_post_account.pre_post_key
    )]
    pub post_account: Account<'info, PostState>,
    #[account(mut)]
    pub next_post_account: Account<'info, PostState>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteLatestPost<'info> {
    #[account(
        mut,
        has_one = authority,
        close = authority
    )]
    pub post_account: Account<'info, PostState>,
    #[account(mut)]
    pub nl_account: Account<'info, NlState>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct SignupUser<'info> {
    #[account(init, payer = authority, space = 8 + 40 + 120  + 32)]
    pub user_account: Account<'info, UserState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateUser<'info> {
    #[account(
        mut,
        has_one = authority,
    )]
    pub user_account: Account<'info, UserState>,
    pub authority: Signer<'info>,
}

#[event]
pub struct PostEvent {
    pub label: String,
    pub post_id: Pubkey,
    pub next_post_id: Option<Pubkey>,
}

#[account]
pub struct NlState {
    pub current_post_key: Pubkey,
    pub authority: Pubkey,
}

#[account]
pub struct UserState {
    pub name: String,
    pub avatar: String,
    pub authority: Pubkey,
}

#[account]
pub struct PostState {
    title: String,
    content: String,
    user: Pubkey,
    pub pre_post_key: Pubkey,
    pub authority: Pubkey,
}
