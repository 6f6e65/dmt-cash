# CASH — A Proof-of-Work Native Monetary Layer

## Abstract
CASH is a digital asset issued by Bitcoin miners via proof-of-work.  
All units are minted in coinbase outputs.  
Total supply: 2 100 000 000 000.  
Issuance halves every 210 000 blocks.  
A spend-throttle reduces emissions when usage exceeds 100 M CASH per 2016 blocks.  
No pre-mine. No committees. No new rules.

## 1. Issuance
- **Activation**: block 929 929  
- **Initial reward**: 5 000 000 CASH per block  
- **Halving**: every 210 000 blocks  
- **Recipient**: miners, pro-rata by coinbase satoshi value (excluding OP_RETURN)  

After activation, user mints are ignored.  
Issuance follows:  
`base_k = 5 000 000 >> floor((k - 929929) / 210000)`

## 2. Hard Cap
`total_minted <= 2 100 000 000 000`  
When cap is reached: `issuance_k = 0`

## 3. Spend-Throttle (EMA)
For block `k >= 929929`:
Spend_raw = Σ (outputs with 0.5% burn) in last 2016 blocks
Spend_ema = 0.9 * prev_ema + 0.1 * Spend_raw
m_k = max(0.25, 1 - 0.5 * Spend_ema / 100 000 000)
issuance_k = floor(base_k * m_k)


- **Burn address**: `1BitcoinEaterAddressDontSendf59kuE`  
- **Min transfer to count**: 100 CASH  

## 4. Miner Distribution
reward_i = issuance_k * (sats_i / total_coinbase_sats)


- Optional marker: `OP_RETURN "DMT-CASH-REWARD"`  
- Fallback: equal split  

## 5. Governance
Two inscriptions:
| ID | Content |
|----|---------|
| 109906847 | Reward Parent: activation, halving |
| 109907649 | Policy: cap, throttle, burn |

Any node can replay from block 929929 and verify state.

## 6. Timeline
- First 210 000 blocks: 1 050 000 000 000 CASH  
- Full cap at m_k=1: ~84 years  
- With throttle: 100–300+ years  

## 7. Summary
CASH is mined, not printed.  
It scales with Bitcoin.  
It self-regulates via usage.  
It lives in Bitcoin's data layer.  

From block 929929, every Bitcoin block mints CASH.

*CASH v1.0 — 2025 — Open specification. Reproducible from Bitcoin blocks and inscriptions.*
