use anchor_lang::prelude::*;

declare_id!("Hs18JoJZYm9VGA38fJVwfAbKGCkv3ssGVwByTiD4Fb6");

#[program]
pub mod contribpage {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_items = 0;
        Ok(())
    }

    pub fn add_item(
        ctx: Context<AddItem>,
        name: String,
        ammount: u64,
        message: String,
        size: u64,
        color: String,
        coordx: u64,
        coordy: u64,
        timestamp: u64,
    ) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;
        let mut name_string = name.to_string();
        if name_string == "" {
            name_string = "Anonymous".to_string();
        }

        let item = ItemStruct {
            owner: *user.to_account_info().key, // The address of the user who submitted.
            name: name_string, // The name of the user.
            ammount: ammount, // The ammount donated by the user.
            message: message.to_string(), // Message of the crystal
            size: size, // The size of the crystal.
            color: color.to_string(), // The color of the crystal.
            coordx: coordx, // The x coordinate of the crystal.
            coordy: coordy, // The x coordinate of the crystal.
            timestamp: timestamp, // The timestamp of submission.
            id: base_account.total_items, // ID of crystal.
        };
    
        base_account.item_list.push(item);
        base_account.total_items += 1;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct AddItem<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

#[account]
pub struct BaseAccount {
    pub total_items: u64,
    pub item_list: Vec<ItemStruct>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub owner: Pubkey, // The address of the user who submitted.
    pub name: String, // The name of the user who submitted.
    pub ammount: u64, // The ammount the user donated.
    pub message: String, // Message of the crystal.
    pub size: u64, // The size of the crystal.
    pub color: String, // The color of the crystal.
    pub coordx: u64, // The x coordinate of the crystal.
    pub coordy: u64, // The x coordinate of the crystal.
    pub timestamp: u64, // The timestamp of submission.
    pub id: u64, // ID of crystal.
}
