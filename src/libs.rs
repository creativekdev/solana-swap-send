use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
    program_error::ProgramError,
    sysvar::{rent::Rent, Sysvar},
};
use spl_token::{instruction::transfer, state::Account as TokenAccount};
use spl_token_swap::instruction::swap;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    
    // Accounts
    let user_account = next_account_info(accounts_iter)?;  // User who initiates the swap
    let source_token_account = next_account_info(accounts_iter)?;  // Source token account for swap
    let destination_token_account = next_account_info(accounts_iter)?;  // Destination token account for swap
    let merchant_token_account = next_account_info(accounts_iter)?;  // Merchant account to receive tokens
    let token_program = next_account_info(accounts_iter)?;  // Token program account
    let swap_program = next_account_info(accounts_iter)?;  // Swap program account (SPL Token Swap)

    msg!("Starting token swap...");

    // Perform the token swap using the Jupiter API (pseudo-code for demonstration purposes).
    let swap_instruction = swap(
        &spl_token_swap::id(),
        &source_token_account.key,
        &destination_token_account.key,
        &user_account.key,
        &swap_program.key,
        instruction_data,  // Jupiter swap instructions are provided here
    )?;
    
    invoke_signed(
        &swap_instruction,
        &[source_token_account.clone(), destination_token_account.clone(), swap_program.clone()],
        &[],  // Signers (if needed)
    )?;
    
    msg!("Token swap completed. Sending tokens to merchant account...");

    // Transfer swapped tokens to the merchant account
    let transfer_instruction = transfer(
        &spl_token::id(),
        &destination_token_account.key,
        &merchant_token_account.key,
        &user_account.key,
        &[],
        1_000_000,  // Replace with actual amount
    )?;

    invoke(
        &transfer_instruction,
        &[
            destination_token_account.clone(),
            merchant_token_account.clone(),
            user_account.clone(),
            token_program.clone(),
        ],
    )?;

    msg!("Tokens transferred to merchant successfully.");

    Ok(())
}
