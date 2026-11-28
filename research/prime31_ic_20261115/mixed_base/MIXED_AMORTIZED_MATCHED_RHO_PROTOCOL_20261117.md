# Prime31 index calculus 1024Q vs source-identical optimized native rho — frozen same-run comparison

Date: 2026-11-17 UTC. This is a **separate** prospective comparison registered **after** successful target-independent native Rust IC source-specific CI #44024919023; no comparator results existed when frozen. A descriptive same-runner development result is not a hardware-isolated or independently PCC-certified best-rho victory. Do not use the earlier 24Q rho timing as matched evidence.

## Inputs and source

Run the immutable source `1df714645aa683cf975bc76fcacb9842cb4780fd`, Python workflow patch only **renaming** existing `prime31_aps_one_relation.rs` main to avoid two `main` functions; directly use its original `neg_rho_cold_policy`, `Arithmetic`, Jacobian batch, distinguished point handling, restart watchdog and counted cold times from the existing Rust source, without restubbing or changing any rho algorithm. New suffix source is appended as an additional wrapper only. The same pinned native Rust example `prime31_mixed_base_amortized_ic.rs` yields the exact previously frozen 1024 public Q input, full cold setup and per-Q descents in its JSON. Comparator reads public Q from this record; caller-only k is checked after each rho function returns, never passed to that function. Existing same-run IC is executed once with full setup, 1024 correct all.

## Frozen rho controls

- Target count 1024. Same Q and same time clock/runner as the IC batch.
- Arm0: source original `neg_rho_cold_policy(q,seed,32,4,2000000,512,false)`: 32-lane **unfolded** Jacobian rho, distinguished bits4.
- Arm1: source original `neg_rho_cold_policy(q,seed ^ 0x464f4c445f333231,32,4,2000000,512,true)`: 32-lane **negation folded** Jacobian rho, same watchdog.
- Arm0/arm1 execution order rotated by `target_index % 2`.
- `rho_seed=0x52484f5f414d4f52 ^ i*0xd1b54a32d192ed03`, wrapping 64-bit; deterministic. A/A repeat of arm0 for the first 64 public targets, with the same seed, after the two primary arms.
- Every rho attempt has the frozen original **2,000,000-step cap**; no truncation, retry, deletion or hiding of capped samples. On a cap, record its original cold time and status, and mark the full-speed comparison **censored/incomplete**, not winner.
- Audit the caller-only k and replay `[log]G=Q` after each rho arm, outside its internal cold timer. The rho source itself already charges an independent candidate `[log]G=Q` verification inside cold wall. Preserve all 1024 rho records and 64 A/A with per-Q cost and counters in raw JSON.
- Report total charged IC setup + all descents, unfolded rho total, folded rho total and complete counts, descriptive ratios, paired wins and A/A drift. Strongest external WZ and **shared-precomputation BSGS** still require isolated separate study; no new asymptotic or nongeneric claim.

## Acceptance

No runtime claim unless compiled source-specific Rust 1.100 CI passes all 40 library and 3 index-calculus tests, original 1024/1024 IC unknown target recovery, all rho arms 1024/1024 *or explicit censoring*, and 1024 caller/curve checks per arm. Retain unchanged failed runs and no arbitrary target/seed tuning after observing timings.

Prior: [IC source-specific 1024Q result](MIXED_AMORTIZED_IC_1024Q_RESULT_20261117.md), classic generic rho, Wiener–Zuccherato original negative-map literature, DOI 10.1007/3-540-48892-8_15, original native Rust #4454 and #4486. No independent PCC.
