use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Serialize, Deserialize)]
struct RewardParent {
    block: u64,
    schedule: Schedule,
}

#[derive(Debug, Serialize, Deserialize)]
struct Schedule {
    initial_per_block: u64,
    epoch: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct PolicyManifest {
    cap: Cap,
    throttle: Throttle,
}

#[derive(Debug, Serialize, Deserialize)]
struct Cap {
    s_max: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Throttle {
    window_blocks: u64,
    target: u64,
    beta: f64,
    min_multiplier: f64,
    burn_required: Burn,
}

#[derive(Debug, Serialize, Deserialize)]
struct Burn {
    rate: f64,
    burn_addr: String,
}

#[derive(Default)]
struct CashState {
    total_minted: u64,
    spend_window: VecDeque<u64>,
    prev_spend_ema: f64,
}

fn main() {
    println!("DMT-CASH Indexer v0.1 — Verifying from block 929929");

    let reward = RewardParent {
        block: 929929,
        schedule: Schedule {
            initial_per_block: 5_000_000,
            epoch: 210_000,
        },
    };

    let policy = PolicyManifest {
        cap: Cap { s_max: 2_100_000_000_000 },
        throttle: Throttle {
            window_blocks: 2016,
            target: 100_000_000,
            beta: 0.5,
            min_multiplier: 0.25,
            burn_required: Burn {
                rate: 0.005,
                burn_addr: "1BitcoinEaterAddressDontSendf59kuE".to_string(),
            },
        },
    };

    let mut state = CashState::default();

    for height in 929929..=930028 {
        let issuance = compute_issuance(height, &reward, &policy, &mut state);
        println!("Block {} | Issuance: {} CASH | Total: {}", height, issuance, state.total_minted);
    }
}

fn compute_issuance(height: u64, reward: &RewardParent, policy: &PolicyManifest, state: &mut CashState) -> u64 {
    if state.total_minted >= policy.cap.s_max {
        return 0;
    }

    let epochs = (height - reward.block) / reward.schedule.epoch;
    let base = reward.schedule.initial_per_block >> epochs;

    let spend_k: u64 = state.spend_window.iter().sum();
    let spend_ema = if state.prev_spend_ema == 0.0 {
        spend_k as f64
    } else {
        0.90 * state.prev_spend_ema + 0.10 * spend_k as f64
    };
    state.prev_spend_ema = spend_ema;

    let m = (1.0 - policy.throttle.beta * spend_ema / policy.throttle.target as f64)
        .max(policy.throttle.min_multiplier);

    let issuance = ((base as f64) * m).floor() as u64;
    let final_issuance = (state.total_minted + issuance).min(policy.cap.s_max) - state.total_minted;

    state.total_minted += final_issuance;

    state.spend_window.push_back(50_000_000);
    if state.spend_window.len() > policy.throttle.window_blocks as usize {
        state.spend_window.pop_front();
    }

    final_issuance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_halving() {
        let reward = RewardParent { block: 929929, schedule: Schedule { initial_per_block: 5_000_000, epoch: 210_000 } };
        let policy = PolicyManifest { 
            cap: Cap { s_max: u64::MAX }, 
            throttle: Throttle { 
                window_blocks: 1, target: 1, beta: 0.0, min_multiplier: 1.0, 
                burn_required: Burn { rate: 0.0, burn_addr: "".to_string() } 
            } 
        };
        let mut state = CashState::default();

        assert_eq!(compute_issuance(929929, &reward, &policy, &mut state), 5_000_000);
        assert_eq!(compute_issuance(929929 + 210_000, &reward, &policy, &mut state), 2_500_000);
    }
}
