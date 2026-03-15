use solana_security_txt::security_txt;
use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount, Transfer};

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "Die Mark Digital",
    project_url: "https://longosbongos.github.io/Investor_App_DMD/",
    contacts: "link:https://t.me/diemarkDigitaloffiziell",
    policy: "https://longosbongos.github.io/DMD-Allocatetd/security.txt",
    preferred_languages: "de,en",
    source_code: "https://github.com/LongosBongos/DMD-Allocatetd",
    source_revision: "main",
    auditors: "none",
    acknowledgements: "https://longosbongos.github.io/DMD-Allocatetd/"
}

const MAX_PRESALE_SUPPLY: u64 = 5_500_000;
const MAX_TOTAL_SUPPLY: u64 = 150_000_000;
const REWARD_INTERVAL: i64 = 60 * 60 * 24 * 90;
const HOLD_DURATION: i64 = 60 * 60 * 24 * 30;
const BASE_REWARD_BPS: u64 = 400;
const STRONG_REWARD_BPS: u64 = 750;
const PENALTY_MIN: u64 = 1000;
const PENALTY_MAX: u64 = 1750;
const MINIMUM_PROTOCOL_OWNER_INIT: u64 = 1 * LAMPORTS_PER_SOL;
const MIN_CONTRIB_LAMPORTS: u64 = LAMPORTS_PER_SOL / 10;
const MAX_CONTRIB_LAMPORTS: u64 = 100 * LAMPORTS_PER_SOL;
const MIN_WL_LAMPORTS: u64 = LAMPORTS_PER_SOL / 10;
const BUY_DAILY_LIMIT: u64 = 10;
const BUY_COOLDOWN_SECONDS: i64 = 60 * 60 * 24 * 2;
const LARGE_BUY_THRESHOLD_LAMPORTS: u64 = 2 * LAMPORTS_PER_SOL;
const LARGE_BUY_EXTRA_FEE_BPS: u64 = 500;
const TREASURY_STRONG_SOL_MIN_LAMPORTS: u64 = 25 * LAMPORTS_PER_SOL;
const TREASURY_MEDIUM_SOL_MIN_LAMPORTS: u64 = 10 * LAMPORTS_PER_SOL;
const VAULT_STRONG_DMD_MIN: u64 = 5_000_000;
const VAULT_MEDIUM_DMD_MIN: u64 = 1_000_000;
const PRICE_MEDIUM_SURCHARGE_BPS: u64 = 500;
const PRICE_STRONG_SURCHARGE_BPS: u64 = 1000;
const SELL_PROTOCOL_OWNER_SHARE_BPS: u64 = 6500;

declare_id!("EDY4bp4fXWkAJpJhXUMZLL7fjpDhpKZQFPpygzsTMzro");

fn autowl_if_needed(buyer_state: &mut Account<BuyerState>, signer_lamports: u64) {
    if !buyer_state.whitelisted && signer_lamports >= MIN_WL_LAMPORTS {
        buyer_state.whitelisted = true;
    }
}

pub fn sol_to_dmd(sol_lamports: u64) -> Result<u64> {
    let num = sol_lamports
        .checked_mul(10_000)
        .ok_or(CustomError::MathOverflow)?;
    Ok(num
        .checked_div(LAMPORTS_PER_SOL)
        .ok_or(CustomError::MathOverflow)?)
}

fn lamports_to_dmd_at_price(sol_lamports: u64, lamports_per_10k: u64) -> Result<u64> {
    require!(lamports_per_10k > 0, CustomError::InvalidPrice);
    let num = (sol_lamports as u128)
        .checked_mul(10_000u128)
        .ok_or(CustomError::MathOverflow)?;
    Ok(num
        .checked_div(lamports_per_10k as u128)
        .ok_or(CustomError::MathOverflow)? as u64)
}

fn bps(v: u64, bps: u64) -> u64 {
    (v as u128 * bps as u128 / 10_000u128) as u64
}

fn whole_dmd_from_units(amount: u64, decimals: u8) -> Result<u64> {
    let factor = 10u128.pow(decimals as u32);
    Ok((amount as u128)
        .checked_div(factor)
        .ok_or(CustomError::MathOverflow)? as u64)
}

fn whole_tokens_to_units(amount_tokens: u64, decimals: u8) -> Result<u64> {
    Ok((amount_tokens as u128)
        .checked_mul(10u128.pow(decimals as u32))
        .ok_or(CustomError::MathOverflow)? as u64)
}

fn compute_dynamic_price_bps(
    vault_token_account_amount: u64,
    mint_decimals: u8,
    treasury_lamports: u64,
) -> Result<u64> {
    let vault_dmd = whole_dmd_from_units(vault_token_account_amount, mint_decimals)?;

    let treasury_bps = if treasury_lamports < TREASURY_MEDIUM_SOL_MIN_LAMPORTS {
        PRICE_STRONG_SURCHARGE_BPS
    } else if treasury_lamports < TREASURY_STRONG_SOL_MIN_LAMPORTS {
        PRICE_MEDIUM_SURCHARGE_BPS
    } else {
        0
    };

    let vault_bps = if vault_dmd < VAULT_MEDIUM_DMD_MIN {
        PRICE_STRONG_SURCHARGE_BPS
    } else if vault_dmd < VAULT_STRONG_DMD_MIN {
        PRICE_MEDIUM_SURCHARGE_BPS
    } else {
        0
    };

    treasury_bps
        .checked_add(vault_bps)
        .ok_or(CustomError::MathOverflow.into())
}

fn effective_price_lamports_per_10k(
    vault: &Account<Vault>,
    vault_config_v2: &Account<VaultConfigV2>,
    vault_token_account_amount: u64,
    treasury_lamports: u64,
) -> Result<u64> {
    let base = if vault_config_v2.manual_price_lamports_per_10k > 0 {
        vault_config_v2.manual_price_lamports_per_10k
    } else {
        vault.initial_price_sol
    };

    require!(base > 0, CustomError::InvalidPrice);

    if !vault_config_v2.dynamic_pricing_enabled {
        return Ok(base);
    }

    let surcharge_bps = compute_dynamic_price_bps(
        vault_token_account_amount,
        vault.mint_decimals,
        treasury_lamports,
    )?;

    let multiplier = 10_000u128
        .checked_add(surcharge_bps as u128)
        .ok_or(CustomError::MathOverflow)?;

    Ok(((base as u128)
        .checked_mul(multiplier)
        .ok_or(CustomError::MathOverflow)?
        .checked_div(10_000u128)
        .ok_or(CustomError::MathOverflow)?) as u64)
}

fn compute_reward_bps(
    buyer_state: &Account<BuyerState>,
    buyer_state_ext_v2: &Account<BuyerStateExtV2>,
    vault: &Account<Vault>,
    vault_config_v2: &Account<VaultConfigV2>,
    vault_token_account_amount: u64,
    treasury_lamports: u64,
) -> Result<u64> {
    let is_first_claim = !buyer_state_ext_v2.first_claim_done && buyer_state.last_reward_claim == 0;
    if is_first_claim {
        return Ok(BASE_REWARD_BPS);
    }

    let strong_reward_tokens = buyer_state
        .total_dmd
        .checked_mul(STRONG_REWARD_BPS)
        .ok_or(CustomError::MathOverflow)?
        .checked_div(10_000)
        .ok_or(CustomError::MathOverflow)?;

    let strong_reward_units = (strong_reward_tokens as u128)
        .checked_mul(10u128.pow(vault.mint_decimals as u32))
        .ok_or(CustomError::MathOverflow)? as u64;

    let price_lamports_per_10k = effective_price_lamports_per_10k(
        vault,
        vault_config_v2,
        vault_token_account_amount,
        treasury_lamports,
    )?;
    let strong_reward_lamports_value = (strong_reward_tokens as u128)
        .checked_mul(price_lamports_per_10k as u128)
        .ok_or(CustomError::MathOverflow)?
        .checked_div(10_000u128)
        .ok_or(CustomError::MathOverflow)? as u64;

    let treasury_buffer_required = strong_reward_lamports_value
        .checked_mul(2)
        .ok_or(CustomError::MathOverflow)?;

    if vault_token_account_amount >= strong_reward_units
        && treasury_lamports >= treasury_buffer_required
    {
        Ok(STRONG_REWARD_BPS)
    } else {
        Ok(BASE_REWARD_BPS)
    }
}

fn require_owner_and_treasury(
    vault: &Account<Vault>,
    vault_config_v2: &Account<VaultConfigV2>,
    protocol_owner_key: Pubkey,
    treasury_key: Pubkey,
) -> Result<()> {
    require_keys_eq!(vault.owner, protocol_owner_key, CustomError::Unauthorized);
    require_keys_eq!(vault_config_v2.treasury, treasury_key, CustomError::InvalidTreasury);
    Ok(())
}

fn compute_sell_penalty_bps(amount_tokens: u64, holding_since: i64, now: i64) -> Result<u64> {
    let base = if amount_tokens >= 100_000 {
        PENALTY_MAX
    } else if amount_tokens >= 25_000 {
        1600
    } else if amount_tokens >= 10_000 {
        1500
    } else if amount_tokens >= 2_500 {
        1250
    } else {
        PENALTY_MIN
    };

    let early_sell_extra = if holding_since > 0 && now - holding_since < HOLD_DURATION {
        250
    } else {
        0
    };

    let final_bps = base
        .checked_add(early_sell_extra)
        .ok_or(CustomError::MathOverflow)?
        .min(PENALTY_MAX);

    Ok(final_bps)
}

fn compute_sell_quote_lamports(amount_tokens: u64, price_lamports_per_10k: u64) -> Result<u64> {
    require!(price_lamports_per_10k > 0, CustomError::InvalidPrice);
    let gross = (amount_tokens as u128)
        .checked_mul(price_lamports_per_10k as u128)
        .ok_or(CustomError::MathOverflow)?
        .checked_div(10_000u128)
        .ok_or(CustomError::MathOverflow)? as u64;

    require!(gross > 0, CustomError::SellAmountTooSmall);
    Ok(gross)
}

fn split_sell_penalty(penalty_lamports: u64) -> Result<(u64, u64)> {
    let protocol_owner_share = (penalty_lamports as u128)
        .checked_mul(SELL_PROTOCOL_OWNER_SHARE_BPS as u128)
        .ok_or(CustomError::MathOverflow)?
        .checked_div(10_000u128)
        .ok_or(CustomError::MathOverflow)? as u64;

    let treasury_retained = penalty_lamports
        .checked_sub(protocol_owner_share)
        .ok_or(CustomError::MathOverflow)?;

    Ok((protocol_owner_share, treasury_retained))
}

#[program]
pub mod dmd_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, initial_price_sol: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let protocol_owner_pk = ctx.accounts.protocol_owner.key();

        let first_init = vault.owner == Pubkey::default();
        if first_init {
            require!(
                ctx.accounts.protocol_owner.lamports() >= MINIMUM_PROTOCOL_OWNER_INIT,
                CustomError::InsufficientProtocolOwnerInit
            );
            vault.owner = protocol_owner_pk;
            vault.presale_sold = 0;
        } else {
            require_keys_eq!(vault.owner, protocol_owner_pk, CustomError::Unauthorized);
        }

        vault.total_supply = MAX_TOTAL_SUPPLY;
        vault.initial_price_sol = initial_price_sol;
        vault.public_sale_active = false;
        vault.mint = ctx.accounts.mint.key();
        vault.mint_decimals = ctx.accounts.mint.decimals;

        let buyer = &mut ctx.accounts.buyer_state;
        buyer.total_dmd = 0;
        buyer.holding_since = Clock::get()?.unix_timestamp;
        buyer.last_reward_claim = 0;
        buyer.last_sell = 0;
        buyer.last_buy_day = 0;
        buyer.buy_count_today = 0;
        buyer.whitelisted = true;

        if first_init && ctx.accounts.mint.supply == 0 {
            let amount_u128 = (MAX_TOTAL_SUPPLY as u128)
                .checked_mul(10u128.pow(ctx.accounts.mint.decimals as u32))
                .ok_or(CustomError::MathOverflow)?;
            let amount = amount_u128 as u64;

            let cpi_ctx = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.protocol_owner_token_account.to_account_info(),
                    authority: ctx.accounts.protocol_owner.to_account_info(),
                },
            );
            token::mint_to(cpi_ctx, amount)?;
        }

        Ok(())
    }

    pub fn toggle_public_sale(ctx: Context<ToggleSale>, active: bool) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        require_keys_eq!(vault.owner, ctx.accounts.protocol_owner.key(), CustomError::Unauthorized);
        vault.public_sale_active = active;
        Ok(())
    }

    pub fn whitelist_add(ctx: Context<WhitelistUpdate>, status: bool) -> Result<()> {
        require_keys_eq!(
            ctx.accounts.vault.owner,
            ctx.accounts.protocol_owner.key(),
            CustomError::Unauthorized
        );
        let buyer = &mut ctx.accounts.buyer_state;
        buyer.whitelisted = status;
        Ok(())
    }

    pub fn auto_whitelist_self(ctx: Context<AutoWhitelistSelf>) -> Result<()> {
        let buyer_state = &mut ctx.accounts.buyer_state;
        let buyer_lamports = ctx.accounts.buyer.lamports();
        require!(buyer_lamports >= MIN_WL_LAMPORTS, CustomError::Unauthorized);
        if !buyer_state.whitelisted {
            buyer_state.whitelisted = true;
            if buyer_state.holding_since == 0 {
                buyer_state.holding_since = Clock::get()?.unix_timestamp;
            }
        }
        Ok(())
    }

    pub fn buy_dmd(ctx: Context<BuyDmd>, sol_contribution: u64) -> Result<()> {
        let clock = Clock::get()?;
        let buyer_state = &mut ctx.accounts.buyer_state;
        let buyer_state_ext_v2 = &mut ctx.accounts.buyer_state_ext_v2;
        let vault = &mut ctx.accounts.vault;

        autowl_if_needed(buyer_state, ctx.accounts.buyer.lamports());
        require_owner_and_treasury(
            vault,
            &ctx.accounts.vault_config_v2,
            ctx.accounts.protocol_owner.key(),
            ctx.accounts.treasury.key(),
        )?;
        require!(
            sol_contribution >= MIN_CONTRIB_LAMPORTS && sol_contribution <= MAX_CONTRIB_LAMPORTS,
            CustomError::ContributionOutOfRange
        );
        if !vault.public_sale_active {
            require!(buyer_state.whitelisted, CustomError::NotWhitelisted);
        }

        require!(
            clock.unix_timestamp >= buyer_state_ext_v2.buy_cooldown_until,
            CustomError::BuyCooldownActive
        );

        let now_day = clock.unix_timestamp / 86_400;
        let last_day = buyer_state.last_buy_day / 86_400;
        if now_day != last_day {
            buyer_state.buy_count_today = 0;
        }
        buyer_state.buy_count_today = buyer_state.buy_count_today.saturating_add(1);

        if buyer_state.buy_count_today > BUY_DAILY_LIMIT {
            buyer_state_ext_v2.buy_cooldown_until = clock
                .unix_timestamp
                .checked_add(BUY_COOLDOWN_SECONDS)
                .ok_or(CustomError::MathOverflow)?;
            return err!(CustomError::BuyDailyLimitExceeded);
        }

        let frequency_fee_bps: u64 = match buyer_state.buy_count_today {
            0..=4 => 0,
            5..=BUY_DAILY_LIMIT => 500,
            _ => 0,
        };

        let size_fee_bps: u64 = if sol_contribution > LARGE_BUY_THRESHOLD_LAMPORTS {
            LARGE_BUY_EXTRA_FEE_BPS
        } else {
            0
        };

        let extra_fee_bps = frequency_fee_bps
            .checked_add(size_fee_bps)
            .ok_or(CustomError::MathOverflow)?;

        let effective_price = effective_price_lamports_per_10k(
            vault,
            &ctx.accounts.vault_config_v2,
            ctx.accounts.vault_token_account.amount,
            ctx.accounts.treasury.lamports(),
        )?;
        let dmd_amount = lamports_to_dmd_at_price(sol_contribution, effective_price)?;

        if !vault.public_sale_active {
            let new_presale = vault
                .presale_sold
                .checked_add(dmd_amount)
                .ok_or(CustomError::MathOverflow)?;
            require!(new_presale <= MAX_PRESALE_SUPPLY, CustomError::PresaleLimitReached);
            vault.presale_sold = new_presale;
        }

        let reward_active = buyer_state.last_reward_claim != 0
            && clock.unix_timestamp - buyer_state.last_reward_claim < REWARD_INTERVAL;
        if !reward_active {
            buyer_state.holding_since = clock.unix_timestamp;
        }

        buyer_state.total_dmd = buyer_state
            .total_dmd
            .checked_add(dmd_amount)
            .ok_or(CustomError::MathOverflow)?;
        buyer_state.last_buy_day = clock.unix_timestamp;

        let extra_fee = bps(sol_contribution, extra_fee_bps);
        let total = sol_contribution
            .checked_add(extra_fee)
            .ok_or(CustomError::MathOverflow)?;
        let protocol_owner_share = total
            .checked_mul(60)
            .ok_or(CustomError::MathOverflow)?
            .checked_div(100)
            .ok_or(CustomError::MathOverflow)?;
        let treasury_share = total
            .checked_sub(protocol_owner_share)
            .ok_or(CustomError::MathOverflow)?;

        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.buyer.to_account_info(),
                    to: ctx.accounts.protocol_owner.to_account_info(),
                },
            ),
            protocol_owner_share,
        )?;
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.buyer.to_account_info(),
                    to: ctx.accounts.treasury.to_account_info(),
                },
            ),
            treasury_share,
        )?;

        let transfer_units = (dmd_amount as u128)
            .checked_mul(10u128.pow(vault.mint_decimals as u32))
            .ok_or(CustomError::MathOverflow)? as u64;

        if transfer_units > 0
            && ctx.accounts.vault_token_account.key() != ctx.accounts.buyer_token_account.key()
        {
            let bump = ctx.bumps.vault;
            let signer_seeds: &[&[u8]] = &[b"vault", &[bump]];
            let cpi_accounts = Transfer {
                from: ctx.accounts.vault_token_account.to_account_info(),
                to: ctx.accounts.buyer_token_account.to_account_info(),
                authority: ctx.accounts.vault.to_account_info(),
            };
            token::transfer(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    cpi_accounts,
                    &[signer_seeds],
                ),
                transfer_units,
            )?;
        }

        Ok(())
    }

    pub fn claim_reward(_ctx: Context<ClaimReward>) -> Result<()> {
        err!(CustomError::LegacyClaimFlowDisabled)
    }

    pub fn set_manual_price(ctx: Context<SetManualPrice>, lamports_per_10k: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        require_keys_eq!(vault.owner, ctx.accounts.protocol_owner.key(), CustomError::Unauthorized);
        require!(lamports_per_10k > 0, CustomError::InvalidPrice);
        vault.initial_price_sol = lamports_per_10k;
        Ok(())
    }

    pub fn transfer_vault_owner(
        ctx: Context<TransferVaultOwner>,
        new_owner: Pubkey,
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        require_keys_eq!(vault.owner, ctx.accounts.protocol_owner.key(), CustomError::Unauthorized);
        require!(new_owner != Pubkey::default(), CustomError::InvalidOwner);
        vault.owner = new_owner;
        Ok(())
    }

    pub fn initialize_vault_config_v2(
        ctx: Context<InitializeVaultConfigV2>,
        treasury: Pubkey,
        manual_price_lamports_per_10k: u64,
        dynamic_pricing_enabled: bool,
        sell_live: bool,
    ) -> Result<()> {
        let vault = &ctx.accounts.vault;
        require_keys_eq!(vault.owner, ctx.accounts.protocol_owner.key(), CustomError::Unauthorized);
        require!(treasury != Pubkey::default(), CustomError::InvalidTreasury);

        let effective_manual_price = if manual_price_lamports_per_10k == 0 {
            vault.initial_price_sol
        } else {
            manual_price_lamports_per_10k
        };
        require!(effective_manual_price > 0, CustomError::InvalidPrice);

        let cfg = &mut ctx.accounts.vault_config_v2;
        cfg.treasury = treasury;
        cfg.manual_price_lamports_per_10k = effective_manual_price;
        cfg.dynamic_pricing_enabled = dynamic_pricing_enabled;
        cfg.sell_live = sell_live;
        Ok(())
    }

    pub fn update_vault_config_v2(
        ctx: Context<UpdateVaultConfigV2>,
        treasury: Pubkey,
        manual_price_lamports_per_10k: u64,
        dynamic_pricing_enabled: bool,
        sell_live: bool,
    ) -> Result<()> {
        let vault = &ctx.accounts.vault;
        require_keys_eq!(vault.owner, ctx.accounts.protocol_owner.key(), CustomError::Unauthorized);
        require!(treasury != Pubkey::default(), CustomError::InvalidTreasury);
        require!(manual_price_lamports_per_10k > 0, CustomError::InvalidPrice);

        let cfg = &mut ctx.accounts.vault_config_v2;
        cfg.treasury = treasury;
        cfg.manual_price_lamports_per_10k = manual_price_lamports_per_10k;
        cfg.dynamic_pricing_enabled = dynamic_pricing_enabled;
        cfg.sell_live = sell_live;
        Ok(())
    }

    pub fn initialize_buyer_state_ext_v2(
        ctx: Context<InitializeBuyerStateExtV2>,
    ) -> Result<()> {
        let ext = &mut ctx.accounts.buyer_state_ext_v2;
        if ext.buy_cooldown_until == 0
            && ext.sell_window_start == 0
            && ext.sell_count_window == 0
            && ext.extra_sell_approvals == 0
            && !ext.first_claim_done
        {
            ext.sell_window_start = Clock::get()?.unix_timestamp;
        }
        Ok(())
    }

    pub fn sell_dmd(_ctx: Context<SellDmd>, _amount: u64) -> Result<()> {
        err!(CustomError::SellTemporarilyDisabled)
    }

    pub fn claim_reward_v2(ctx: Context<ClaimRewardV2>) -> Result<()> {
        let clock = Clock::get()?;
        let buyer = &mut ctx.accounts.buyer_state;
        let buyer_ext = &mut ctx.accounts.buyer_state_ext_v2;
        let vault = &ctx.accounts.vault;
        let vault_cfg = &ctx.accounts.vault_config_v2;

        require_keys_eq!(
            vault_cfg.treasury,
            ctx.accounts.treasury.key(),
            CustomError::InvalidTreasury
        );
        require!(buyer.whitelisted, CustomError::NotWhitelisted);
        require!(
            clock.unix_timestamp - buyer.holding_since >= HOLD_DURATION,
            CustomError::HoldTooShort
        );
        if buyer.last_reward_claim != 0 {
            require!(
                clock.unix_timestamp - buyer.last_reward_claim >= REWARD_INTERVAL,
                CustomError::ClaimTooEarly
            );
        }

        let reward_bps = compute_reward_bps(
            buyer,
            buyer_ext,
            vault,
            vault_cfg,
            ctx.accounts.vault_token_account.amount,
            ctx.accounts.treasury.lamports(),
        )?;

        let reward_tokens = buyer
            .total_dmd
            .checked_mul(reward_bps)
            .ok_or(CustomError::MathOverflow)?
            .checked_div(10_000)
            .ok_or(CustomError::MathOverflow)?;
        require!(reward_tokens > 0, CustomError::RewardTooSmall);

        let units = (reward_tokens as u128)
            .checked_mul(10u128.pow(vault.mint_decimals as u32))
            .ok_or(CustomError::MathOverflow)? as u64;
        require!(
            ctx.accounts.vault_token_account.amount >= units,
            CustomError::InsufficientVaultRewardLiquidity
        );

        let bump = ctx.bumps.vault;
        let signer_seeds: &[&[u8]] = &[b"vault", &[bump]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.vault_token_account.to_account_info(),
            to: ctx.accounts.buyer_token_account.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        };
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                cpi_accounts,
                &[signer_seeds],
            ),
            units,
        )?;

        buyer.total_dmd = buyer
            .total_dmd
            .checked_add(reward_tokens)
            .ok_or(CustomError::MathOverflow)?;
        buyer.last_reward_claim = clock.unix_timestamp;
        buyer_ext.first_claim_done = true;
        Ok(())
    }

    pub fn sell_dmd_v2(ctx: Context<SellDmdV2>, amount_tokens: u64) -> Result<()> {
        let clock = Clock::get()?;
        let vault = &ctx.accounts.vault;
        let vault_cfg = &ctx.accounts.vault_config_v2;

        require_owner_and_treasury(
            vault,
            vault_cfg,
            ctx.accounts.protocol_owner.key(),
            ctx.accounts.treasury.key(),
        )?;
        require!(vault_cfg.sell_live, CustomError::SellTemporarilyDisabled);
        require!(amount_tokens > 0, CustomError::SellAmountTooSmall);

        let buyer_state = &mut ctx.accounts.buyer_state;
        require!(buyer_state.whitelisted, CustomError::NotWhitelisted);
        require!(
            buyer_state.total_dmd >= amount_tokens,
            CustomError::InsufficientBuyerDmdBalance
        );

        let amount_units = whole_tokens_to_units(amount_tokens, vault.mint_decimals)?;
        require!(
            ctx.accounts.buyer_token_account.amount >= amount_units,
            CustomError::InsufficientBuyerDmdBalance
        );

        let price_lamports_per_10k = effective_price_lamports_per_10k(
            vault,
            vault_cfg,
            ctx.accounts.vault_token_account.amount,
            ctx.accounts.treasury.lamports(),
        )?;
        let gross_lamports = compute_sell_quote_lamports(amount_tokens, price_lamports_per_10k)?;
        let penalty_bps =
            compute_sell_penalty_bps(amount_tokens, buyer_state.holding_since, clock.unix_timestamp)?;
        let penalty_lamports = bps(gross_lamports, penalty_bps);
        let buyer_net_lamports = gross_lamports
            .checked_sub(penalty_lamports)
            .ok_or(CustomError::MathOverflow)?;
        require!(buyer_net_lamports > 0, CustomError::SellAmountTooSmall);

        let (protocol_owner_penalty_share, _treasury_retained_share) =
            split_sell_penalty(penalty_lamports)?;
        let total_treasury_out = buyer_net_lamports
            .checked_add(protocol_owner_penalty_share)
            .ok_or(CustomError::MathOverflow)?;

        require!(
            ctx.accounts.treasury.lamports() >= total_treasury_out,
            CustomError::InsufficientTreasuryLiquidity
        );

        let cpi_accounts = Transfer {
            from: ctx.accounts.buyer_token_account.to_account_info(),
            to: ctx.accounts.vault_token_account.to_account_info(),
            authority: ctx.accounts.buyer.to_account_info(),
        };
        token::transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts),
            amount_units,
        )?;

        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.treasury.to_account_info(),
                    to: ctx.accounts.buyer.to_account_info(),
                },
            ),
            buyer_net_lamports,
        )?;

        if protocol_owner_penalty_share > 0 {
            system_program::transfer(
                CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    system_program::Transfer {
                        from: ctx.accounts.treasury.to_account_info(),
                        to: ctx.accounts.protocol_owner.to_account_info(),
                    },
                ),
                protocol_owner_penalty_share,
            )?;
        }

        buyer_state.total_dmd = buyer_state
            .total_dmd
            .checked_sub(amount_tokens)
            .ok_or(CustomError::MathOverflow)?;
        buyer_state.last_sell = clock.unix_timestamp;

        if buyer_state.total_dmd == 0 {
            buyer_state.holding_since = 0;
        }

        Ok(())
    }

    pub fn swap_exact_sol_for_dmd(
        ctx: Context<SwapBuyV1>,
        amount_in_lamports: u64,
        min_out_dmd: u64,
    ) -> Result<()> {
        let clock = Clock::get()?;
        let buyer_state = &mut ctx.accounts.buyer_state;
        let vault = &mut ctx.accounts.vault;

        autowl_if_needed(buyer_state, ctx.accounts.user.lamports());
        require_owner_and_treasury(
            vault,
            &ctx.accounts.vault_config_v2,
            ctx.accounts.protocol_owner.key(),
            ctx.accounts.treasury.key(),
        )?;

        if !vault.public_sale_active {
            require!(buyer_state.whitelisted, CustomError::NotWhitelisted);
        }

        require!(
            amount_in_lamports >= MIN_CONTRIB_LAMPORTS && amount_in_lamports <= MAX_CONTRIB_LAMPORTS,
            CustomError::ContributionOutOfRange
        );

        let effective_price = effective_price_lamports_per_10k(
            vault,
            &ctx.accounts.vault_config_v2,
            ctx.accounts.vault_token_account.amount,
            ctx.accounts.treasury.lamports(),
        )?;
        let dmd_out = lamports_to_dmd_at_price(amount_in_lamports, effective_price)?;
        require!(dmd_out >= min_out_dmd, CustomError::InvalidPrice);

        if !vault.public_sale_active {
            let new_presale = vault
                .presale_sold
                .checked_add(dmd_out)
                .ok_or(CustomError::MathOverflow)?;
            require!(new_presale <= MAX_PRESALE_SUPPLY, CustomError::PresaleLimitReached);
            vault.presale_sold = new_presale;
        }

        let protocol_owner_share = amount_in_lamports
            .checked_mul(60)
            .ok_or(CustomError::MathOverflow)?
            .checked_div(100)
            .ok_or(CustomError::MathOverflow)?;
        let treasury_share = amount_in_lamports
            .checked_sub(protocol_owner_share)
            .ok_or(CustomError::MathOverflow)?;

        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.user.to_account_info(),
                    to: ctx.accounts.protocol_owner.to_account_info(),
                },
            ),
            protocol_owner_share,
        )?;
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.user.to_account_info(),
                    to: ctx.accounts.treasury.to_account_info(),
                },
            ),
            treasury_share,
        )?;

        let units = (dmd_out as u128)
            .checked_mul(10u128.pow(vault.mint_decimals as u32))
            .ok_or(CustomError::MathOverflow)? as u64;

        if units > 0 {
            let bump = ctx.bumps.vault;
            let signer_seeds: &[&[u8]] = &[b"vault", &[bump]];
            let cpi_accounts = Transfer {
                from: ctx.accounts.vault_token_account.to_account_info(),
                to: ctx.accounts.user_dmd_ata.to_account_info(),
                authority: ctx.accounts.vault.to_account_info(),
            };
            token::transfer(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    cpi_accounts,
                    &[signer_seeds],
                ),
                units,
            )?;
        }

        buyer_state.total_dmd = buyer_state
            .total_dmd
            .checked_add(dmd_out)
            .ok_or(CustomError::MathOverflow)?;
        buyer_state.holding_since = clock.unix_timestamp;
        buyer_state.last_buy_day = clock.unix_timestamp;
        buyer_state.buy_count_today = buyer_state.buy_count_today.saturating_add(1);

        Ok(())
    }

    pub fn swap_exact_dmd_for_sol(
        ctx: Context<SwapSellV1>,
        amount_in_dmd: u64,
        min_out_sol: u64,
    ) -> Result<()> {
        let clock = Clock::get()?;
        let vault = &ctx.accounts.vault;
        let vault_cfg = &ctx.accounts.vault_config_v2;

        require_owner_and_treasury(
            vault,
            vault_cfg,
            ctx.accounts.protocol_owner.key(),
            ctx.accounts.treasury.key(),
        )?;
        require!(vault_cfg.sell_live, CustomError::SellTemporarilyDisabled);
        require!(amount_in_dmd > 0, CustomError::SellAmountTooSmall);

        let buyer_state = &mut ctx.accounts.buyer_state;
        require!(buyer_state.whitelisted, CustomError::NotWhitelisted);
        require!(
            buyer_state.total_dmd >= amount_in_dmd,
            CustomError::InsufficientBuyerDmdBalance
        );

        let amount_units = whole_tokens_to_units(amount_in_dmd, vault.mint_decimals)?;
        require!(
            ctx.accounts.user_dmd_ata.amount >= amount_units,
            CustomError::InsufficientBuyerDmdBalance
        );

        let price_lamports_per_10k = effective_price_lamports_per_10k(
            vault,
            vault_cfg,
            ctx.accounts.vault_token_account.amount,
            ctx.accounts.treasury.lamports(),
        )?;
        let gross_lamports = compute_sell_quote_lamports(amount_in_dmd, price_lamports_per_10k)?;
        let penalty_bps =
            compute_sell_penalty_bps(amount_in_dmd, buyer_state.holding_since, clock.unix_timestamp)?;
        let penalty_lamports = bps(gross_lamports, penalty_bps);
        let user_net_lamports = gross_lamports
            .checked_sub(penalty_lamports)
            .ok_or(CustomError::MathOverflow)?;
        require!(user_net_lamports >= min_out_sol, CustomError::SlippageExceeded);
        require!(user_net_lamports > 0, CustomError::SellAmountTooSmall);

        let (protocol_owner_penalty_share, _treasury_retained_share) =
            split_sell_penalty(penalty_lamports)?;
        let total_treasury_out = user_net_lamports
            .checked_add(protocol_owner_penalty_share)
            .ok_or(CustomError::MathOverflow)?;

        require!(
            ctx.accounts.treasury.lamports() >= total_treasury_out,
            CustomError::InsufficientTreasuryLiquidity
        );

        let cpi_accounts = Transfer {
            from: ctx.accounts.user_dmd_ata.to_account_info(),
            to: ctx.accounts.vault_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        token::transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts),
            amount_units,
        )?;

        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.treasury.to_account_info(),
                    to: ctx.accounts.user.to_account_info(),
                },
            ),
            user_net_lamports,
        )?;

        if protocol_owner_penalty_share > 0 {
            system_program::transfer(
                CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    system_program::Transfer {
                        from: ctx.accounts.treasury.to_account_info(),
                        to: ctx.accounts.protocol_owner.to_account_info(),
                    },
                ),
                protocol_owner_penalty_share,
            )?;
        }

        buyer_state.total_dmd = buyer_state
            .total_dmd
            .checked_sub(amount_in_dmd)
            .ok_or(CustomError::MathOverflow)?;
        buyer_state.last_sell = clock.unix_timestamp;

        if buyer_state.total_dmd == 0 {
            buyer_state.holding_since = 0;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_if_needed,
        payer = protocol_owner,
        seeds = [b"vault"],
        bump,
        space = 8 + Vault::SIZE
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        init_if_needed,
        payer = protocol_owner,
        seeds = [b"buyer", vault.key().as_ref(), protocol_owner.key().as_ref()],
        bump,
        space = 8 + BuyerState::SIZE
    )]
    pub buyer_state: Account<'info, BuyerState>,

    #[account(mut)]
    pub protocol_owner: Signer<'info>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub protocol_owner_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BuyDmd<'info> {
    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: Account<'info, Vault>,

    #[account(
        seeds = [b"vault-config-v2", vault.key().as_ref()],
        bump
    )]
    pub vault_config_v2: Account<'info, VaultConfigV2>,

    #[account(
        init_if_needed,
        payer = buyer,
        seeds = [b"buyer", vault.key().as_ref(), buyer.key().as_ref()],
        bump,
        space = 8 + BuyerState::SIZE
    )]
    pub buyer_state: Account<'info, BuyerState>,

    #[account(
        init_if_needed,
        payer = buyer,
        seeds = [b"buyer-ext-v2", vault.key().as_ref(), buyer.key().as_ref()],
        bump,
        space = 8 + BuyerStateExtV2::SIZE
    )]
    pub buyer_state_ext_v2: Account<'info, BuyerStateExtV2>,

    #[account(mut)]
    pub protocol_owner: SystemAccount<'info>,

    #[account(mut)]
    pub treasury: SystemAccount<'info>,

    #[account(
        mut,
        constraint = vault_token_account.mint == vault.mint,
        constraint = vault_token_account.owner == vault.key()
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = buyer_token_account.mint == vault.mint,
        constraint = buyer_token_account.owner == buyer.key()
    )]
    pub buyer_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub buyer: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimReward<'info> {
    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: Account<'info, Vault>,

    #[account(
        mut,
        seeds = [b"buyer", vault.key().as_ref(), buyer.key().as_ref()],
        bump
    )]
    pub buyer_state: Account<'info, BuyerState>,

    pub buyer: Signer<'info>,
}

#[derive(Accounts)]
pub struct SellDmd<'info> {
    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: Account<'info, Vault>,

    #[account(
        mut,
        seeds = [b"buyer", vault.key().as_ref(), buyer.key().as_ref()],
        bump
    )]
    pub buyer_state: Account<'info, BuyerState>,

    pub buyer: Signer<'info>,
}

#[derive(Accounts)]
pub struct ToggleSale<'info> {
    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: Account<'info, Vault>,
    pub protocol_owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct WhitelistUpdate<'info> {
    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: Account<'info, Vault>,
    pub buyer: SystemAccount<'info>,
    #[account(
        init_if_needed,
        payer = protocol_owner,
        seeds = [b"buyer", vault.key().as_ref(), buyer.key().as_ref()],
        bump,
        space = 8 + BuyerState::SIZE
    )]
    pub buyer_state: Account<'info, BuyerState>,
    #[account(mut)]
    pub protocol_owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AutoWhitelistSelf<'info> {
    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: Account<'info, Vault>,
    #[account(
        init_if_needed,
        payer = buyer,
        seeds = [b"buyer", vault.key().as_ref(), buyer.key().as_ref()],
        bump,
        space = 8 + BuyerState::SIZE
    )]
    pub buyer_state: Account<'info, BuyerState>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferVaultOwner<'info> {
    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: Account<'info, Vault>,
    pub protocol_owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetManualPrice<'info> {
    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: Account<'info, Vault>,
    pub protocol_owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct InitializeVaultConfigV2<'info> {
    #[account(seeds = [b"vault"], bump)]
    pub vault: Account<'info, Vault>,

    #[account(
        init_if_needed,
        payer = protocol_owner,
        seeds = [b"vault-config-v2", vault.key().as_ref()],
        bump,
        space = 8 + VaultConfigV2::SIZE
    )]
    pub vault_config_v2: Account<'info, VaultConfigV2>,

    #[account(mut)]
    pub protocol_owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateVaultConfigV2<'info> {
    #[account(seeds = [b"vault"], bump)]
    pub vault: Account<'info, Vault>,

    #[account(
        mut,
        seeds = [b"vault-config-v2", vault.key().as_ref()],
        bump
    )]
    pub vault_config_v2: Account<'info, VaultConfigV2>,

    pub protocol_owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct InitializeBuyerStateExtV2<'info> {
    #[account(seeds = [b"vault"], bump)]
    pub vault: Account<'info, Vault>,

    #[account(
        mut,
        seeds = [b"buyer", vault.key().as_ref(), buyer.key().as_ref()],
        bump
    )]
    pub buyer_state: Account<'info, BuyerState>,

    #[account(
        init_if_needed,
        payer = buyer,
        seeds = [b"buyer-ext-v2", vault.key().as_ref(), buyer.key().as_ref()],
        bump,
        space = 8 + BuyerStateExtV2::SIZE
    )]
    pub buyer_state_ext_v2: Account<'info, BuyerStateExtV2>,

    #[account(mut)]
    pub buyer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimRewardV2<'info> {
    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: Account<'info, Vault>,

    #[account(
        seeds = [b"vault-config-v2", vault.key().as_ref()],
        bump
    )]
    pub vault_config_v2: Account<'info, VaultConfigV2>,

    #[account(
        mut,
        seeds = [b"buyer", vault.key().as_ref(), buyer.key().as_ref()],
        bump
    )]
    pub buyer_state: Account<'info, BuyerState>,

    #[account(
        mut,
        seeds = [b"buyer-ext-v2", vault.key().as_ref(), buyer.key().as_ref()],
        bump
    )]
    pub buyer_state_ext_v2: Account<'info, BuyerStateExtV2>,

    #[account(
        mut,
        constraint = vault_token_account.mint == vault.mint,
        constraint = vault_token_account.owner == vault.key()
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = buyer_token_account.mint == vault.mint,
        constraint = buyer_token_account.owner == buyer.key()
    )]
    pub buyer_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub treasury: SystemAccount<'info>,

    pub buyer: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct SellDmdV2<'info> {
    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: Account<'info, Vault>,

    #[account(
        seeds = [b"vault-config-v2", vault.key().as_ref()],
        bump
    )]
    pub vault_config_v2: Account<'info, VaultConfigV2>,

    #[account(
        mut,
        seeds = [b"buyer", vault.key().as_ref(), buyer.key().as_ref()],
        bump
    )]
    pub buyer_state: Account<'info, BuyerState>,

    #[account(
        mut,
        constraint = vault_token_account.mint == vault.mint,
        constraint = vault_token_account.owner == vault.key()
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = buyer_token_account.mint == vault.mint,
        constraint = buyer_token_account.owner == buyer.key()
    )]
    pub buyer_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub treasury: Signer<'info>,

    #[account(mut)]
    pub protocol_owner: SystemAccount<'info>,

    pub buyer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SwapBuyV1<'info> {
    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: Account<'info, Vault>,

    #[account(
        seeds = [b"vault-config-v2", vault.key().as_ref()],
        bump
    )]
    pub vault_config_v2: Account<'info, VaultConfigV2>,

    #[account(
        init_if_needed,
        payer = user,
        seeds = [b"buyer", vault.key().as_ref(), user.key().as_ref()],
        bump,
        space = 8 + BuyerState::SIZE
    )]
    pub buyer_state: Account<'info, BuyerState>,

    #[account(
        mut,
        constraint = vault_token_account.mint == vault.mint,
        constraint = vault_token_account.owner == vault.key()
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = user_dmd_ata.mint == vault.mint,
        constraint = user_dmd_ata.owner == user.key()
    )]
    pub user_dmd_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub protocol_owner: SystemAccount<'info>,

    #[account(mut)]
    pub treasury: SystemAccount<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SwapSellV1<'info> {
    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: Account<'info, Vault>,

    #[account(
        seeds = [b"vault-config-v2", vault.key().as_ref()],
        bump
    )]
    pub vault_config_v2: Account<'info, VaultConfigV2>,

    #[account(
        mut,
        seeds = [b"buyer", vault.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub buyer_state: Account<'info, BuyerState>,

    #[account(
        mut,
        constraint = vault_token_account.mint == vault.mint,
        constraint = vault_token_account.owner == vault.key()
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = user_dmd_ata.mint == vault.mint,
        constraint = user_dmd_ata.owner == user.key()
    )]
    pub user_dmd_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub treasury: Signer<'info>,

    #[account(mut)]
    pub protocol_owner: SystemAccount<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct VaultConfigV2 {
    pub treasury: Pubkey,
    pub manual_price_lamports_per_10k: u64,
    pub dynamic_pricing_enabled: bool,
    pub sell_live: bool,
}
impl VaultConfigV2 {
    pub const SIZE: usize = 32 + 8 + 1 + 1;
}

#[account]
pub struct BuyerStateExtV2 {
    pub buy_cooldown_until: i64,
    pub sell_window_start: i64,
    pub sell_count_window: u8,
    pub extra_sell_approvals: u8,
    pub first_claim_done: bool,
}
impl BuyerStateExtV2 {
    pub const SIZE: usize = 8 + 8 + 1 + 1 + 1;
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub total_supply: u64,
    pub presale_sold: u64,
    pub initial_price_sol: u64,
    pub public_sale_active: bool,
    pub mint: Pubkey,
    pub mint_decimals: u8,
}
impl Vault {
    pub const SIZE: usize = 32 + 8 + 8 + 8 + 1 + 32 + 1;
}

#[account]
pub struct BuyerState {
    pub total_dmd: u64,
    pub last_reward_claim: i64,
    pub last_sell: i64,
    pub holding_since: i64,
    pub last_buy_day: i64,
    pub buy_count_today: u64,
    pub whitelisted: bool,
}
impl BuyerState {
    pub const SIZE: usize = 8 + 8 + 8 + 8 + 8 + 8 + 1;
}

#[error_code]
pub enum CustomError {
    #[msg("Du bist nicht auf der Whitelist.")]
    NotWhitelisted,
    #[msg("Claim zu früh.")]
    ClaimTooEarly,
    #[msg("Zu früh verkauft.")]
    SellTooEarly,
    #[msg("Nicht lange genug gehalten.")]
    HoldTooShort,
    #[msg("Presale Limit erreicht.")]
    PresaleLimitReached,
    #[msg("Nicht berechtigt.")]
    Unauthorized,
    #[msg("Protocol Owner Initialisierung zu niedrig.")]
    InsufficientProtocolOwnerInit,
    #[msg("Arithmetischer Überlauf.")]
    MathOverflow,
    #[msg("Beitrag außerhalb des erlaubten Bereichs (0.1–100 SOL).")]
    ContributionOutOfRange,
    #[msg("Ungültiger Preis.")]
    InvalidPrice,
    #[msg("Ungültige Treasury.")]
    InvalidTreasury,
    #[msg("Ungültiger neuer Owner.")]
    InvalidOwner,
    #[msg("Buy-Cooldown ist aktiv.")]
    BuyCooldownActive,
    #[msg("Tageslimit für Buys überschritten. 2-Tage-Cooldown aktiviert.")]
    BuyDailyLimitExceeded,
    #[msg("Sell ist bis zum finalen Upgrade deaktiviert.")]
    SellTemporarilyDisabled,
    #[msg("Der alte Claim-Flow ist deaktiviert. Nutze nur claim_reward_v2.")]
    LegacyClaimFlowDisabled,
    #[msg("Reward ist zu klein für eine sinnvolle Auszahlung.")]
    RewardTooSmall,
    #[msg("Vault hat nicht genug DMD für die Reward-Auszahlung.")]
    InsufficientVaultRewardLiquidity,
    #[msg("Treasury hat nicht genug SOL für die Sell-Auszahlung.")]
    InsufficientTreasuryLiquidity,
    #[msg("Buyer hat nicht genug DMD für diesen Sell.")]
    InsufficientBuyerDmdBalance,
    #[msg("Sell-Menge ist zu klein.")]
    SellAmountTooSmall,
    #[msg("Min-Out / Slippage-Bedingung nicht erfüllt.")]
    SlippageExceeded,
}