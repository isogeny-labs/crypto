# L5 kernel split: where the counted linear stage's multiplications go

Lane key `l5-kernel-split`, 2026-11-19. Triage of lead L5
(`scout-graph-ic/LEADS.md` §L5) before any work on descent-finder candidate
C4 (`descent-finder/CANDIDATES.md`, "Linear stage reuse across residuals").
Every number here is a producer observation, **[pcc:unverified]**. No pcc
claim was registered. No paid compute, no remote side effects. This is a
local recount of counted work on the reconstructed Run50 stream. It makes no
timing claim and changes no algorithm, so it is not an improvement.

## Question

The counted linear stage on the Run50 stream (seed 389, 3,415 residuals) is
653,094,605 `mac`, 37,864,800 `mul` and 44,582,227 `add`
(`complete-accounting/runs/recount-seed389-n3415.json`). How do these split
between the kernels of `zero_dim_la::fast_stage`?

The kernels are:
- normal-form (NF) reductions that build `M_{x_n}` and the read functionals;
- the transposed Krylov sequence (sparse-FGLM-style, `2δ` products);
- Berlekamp–Massey;
- `x^p mod f`;
- gcd;
- the read-out;
- the candidate evaluation.

## Rule (L5, applied exactly as written)

- If Krylov + BM > 70% of the `mac`s: demote C4 to "≤ 1% of descent" and close it.
- If NF > 50%: test border-basis direct read-off from the replayed reduced basis.
- Otherwise: state the split and leave C4 open.

## Method

The work is in Rust. Shared `src/` and the complete-accounting files are not
edited.

- **The patch.**
  [`kernel-split/zero_dim_la.kernel_split.patch`](kernel-split/zero_dim_la.kernel_split.patch)
  is 202 lines. It applies on top of complete-accounting's counted
  `zero_dim_la.rs` (SHA-256 `f0c50f5f…`). The build copies that file and
  patches the copy. The patched source (`25a2a816…`, 71,116 bytes) stays in
  scratch, over the 64 KiB artifact limit; its hash is in
  `runs/RAW.sha256`.
- **Tagging.**
  - A thread-local "current kernel" is set by RAII `KScope` guards in
    `fast_inner` and `roots_in_fp`.
  - `lin()` is the single funnel through which every `LinOps` count already
    passes. It now also adds each count to the current kernel's bucket.
  - So every `mul`, `add`, `mac`, `word`, `probe`, `rng` and every
    reconciliation counter lands in exactly one bucket.
  - Bucket 0 (`unscoped`) catches anything outside a guard.
- **Recording.** `fast_stage` stores the call's per-kernel difference in a
  new `FastStats::kernel_ops: [LinOps; 10]` field. Nothing else in
  `FastStats` or the stage's outputs changes.
- **Buckets.**

  | Bucket | Covers |
  |---|---|
  | `setup` | seed draw, staircase, monomial index, basis normalization |
  | `nf` | `NfTable::nf` for every column of `M` and the read functionals, plus the row scans |
  | `krylov` | the random functional `w`, the read functionals and `M^T w` |
  | `bm` | Berlekamp–Massey |
  | `xpow_mod_f` | `ModF::new`, `x_pow_p` and `−x` |
  | `gcd` | `poly_gcd` |
  | `ring_split` | `PolyRing::roots` |
  | `readout` | the `Q` chain, `ψ`, coordinate divisions and the final sort/dedup |
  | `candidate_eval` | `eval_counted` |

- **The recount binary.**
  [`kernel-split/kernel_split_recount.rs`](kernel-split/kernel_split_recount.rs)
  is a copy of complete-accounting's `linear_stage_recount.rs`. It keeps the
  checks (a)–(d) unchanged and adds:
  - **(e)** Per residual, the sum of the per-kernel tallies equals the call's
    `LinOps` field by field, and `unscoped` is zero.
  - **(f)** Per residual, each legacy `FastStats` phase counter equals its
    kernel group's net products (`mac + mul + legacy_overcount + ring_muls −
    mul_outside_legacy`). This check is independent of the tagging. The
    groups are:
    - `nf_muls` ↔ setup + nf
    - `krylov_muls` ↔ krylov
    - `bm_muls` ↔ bm
    - `roots_muls` ↔ xpow + gcd + ring
    - `readout_muls` ↔ readout + candidate
  - Per-kernel bitop/v1 prices, using `charge_lin`'s mapping and
    `jv_bitops::price`.
- **The differential test.**
  [`kernel-split/conservation_test.rs`](kernel-split/conservation_test.rs)
  is complete-accounting's 108-system test with (e) and the Krylov part of (f)
  added.

## Commands

```
df -h /                      # 13G free before building
cd research/extension_ic_graph_20261115/leads-20261118/l5-kernel-split
SCRATCH=<scratchpad>/l5 CARGO_TARGET_DIR=/home/user/cargo-target-round2 \
  RUSTUP_TOOLCHAIN=1.98 CARGO_INCREMENTAL=0 bash build/build.sh > build/build.log
#   (fetch_frozen.sh: read-only `git show fcda1e22:…`; assemble.py: the
#    complete-accounting mini-build plus this lane's patch; cargo -j 1 --offline)
/home/user/cargo-target-round2/release/kernel_split_recount \
  ../../runpod-deferred-pair-20261117-50/public.json 389 3415 \
  runs/kernel-split-seed389-n3415.json
```

The build passed:
- the lib tests (9 passed, 1 ignored);
- the conservation test (108 systems: 107 with roots, 1 give-up);
- the binary build.

`Cargo.lock` `3d1cb733…` is the same as complete-accounting's. The recount
took 11.6 s on one thread of a shared host (indicative only). Hashes are in
`runs/RAW.sha256`. The research-kb lookup was unavailable (no executable, no
MCP tools; `runs/kb-receipt.log`). This lane makes no literature or novelty
claim.

## Results (`runs/kernel-split-seed389-n3415.json`)

Per-kernel counts on the reconstructed Run50 stream:

| Kernel | mac | % of mac | mul | add | field_mul (mul+mac) | % field_mul | word | probe | rng | bitops | % of linear stage | % of descent |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| krylov | 558,028,320 | **85.44** | 0 | 0 | 558,028,320 | 80.76 | 0 | 0 | 219,704 | 758,932,576,256 | 80.49 | 17.41 |
| xpow_mod_f | 79,696,764 | 12.20 | 218,288 | 6,945,970 | 79,915,052 | 11.57 | 0 | 0 | 0 | 109,559,069,380 | 11.62 | 2.51 |
| bm | 14,060,537 | **2.15** | 14,485,296 | 14,482,342 | 28,545,833 | 4.13 | 0 | 0 | 0 | 38,821,948,860 | 4.12 | 0.89 |
| gcd | 0 | 0 | 14,594,636 | 14,158,618 | 14,594,636 | 2.11 | 0 | 0 | 0 | 19,792,022,620 | 2.10 | 0.45 |
| setup | 0 | 0 | 6,805,753 | 0 | 6,805,753 | 0.98 | 22,055,295 | 218,552 | 3,415 | 9,805,563,038 | 1.04 | 0.22 |
| readout | 1,308,984 | 0.20 | 231,800 | 235,209 | 1,540,784 | 0.22 | 4,463 | 0 | 0 | 2,096,195,042 | 0.22 | 0.05 |
| nf | **0** | **0** | 0 | 8,433,176 | 0 | 0 | 6,368,815 | 4,592,230 | 0 | 1,981,508,960 | 0.21 | 0.05 |
| candidate_eval | 0 | 0 | 1,529,027 | 326,912 | 1,529,027 | 0.22 | 0 | 0 | 0 | 1,923,201,770 | 0.20 | 0.04 |
| ring_split | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| unscoped | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| **total** | **653,094,605** | 100 | **37,864,800** | **44,582,227** | 690,959,405 | 100 | 28,428,573 | 4,810,782 | 223,119 | **942,912,085,926** | 100 | 21.63 |

How to read the table:
- The descent denominator is this stream's descent with the counted linear
  stage, 4,358,996,976,408 bitops. That is the production arm before C1.
- `ring_split` shows 0 because its `PolyRing` products are hooked separately,
  outside `LinOps`. Its 586,008 `ring_muls` are a reconciliation counter and
  are not charged here. The same holds for `inv_mod`.
- The `nf` row is zero `mac`, and so zero `mul` as well. On this stream the
  basis is reduced, so the tail of each basis element is already standard. So
  every NF used for `M_{x_n}` is either a unit or a negated tail. The NF cost
  is 8.4M negations, 6.4M divisibility words and 4.6M probes. The legacy
  `nf_muls` agrees: it is 0 on the stream.

## Conservation check

All checks passed:
- **Per-kernel sum against the total.** The sum over kernels equals the
  counted `LinOps` total field by field: mac 653,094,605, mul 37,864,800,
  add 44,582,227, word 28,428,573, probe 4,810,782, rng 223,119,
  mul_outside_legacy 8,358,644, legacy_overcount 3,409 and ring_muls
  586,008. That holds for the stream aggregate and for each of the 3,415
  residuals (check (e): 0 failures). `unscoped` is 0.
- **Against complete-accounting's recorded run.** This run's `lin_ops` is
  identical to `complete-accounting/runs/recount-seed389-n3415.json`. So are
  its descent-with-counted-stage, pre-patch, pipeline, harness-counted and
  harness-frozen tallies. Check (a)–(d) failures: 0. Foreign hooks: 0.
  Admitted: true.
- **Priced.** The per-kernel bitops sum to 942,912,085,926. That equals the
  linear-stage charge and complete-accounting's recorded increment exactly.
- **Legacy cross-check (f).** It is exact for each group on the stream:
  - nf: 0 = 0
  - krylov: 558,028,320 = 558,028,320
  - bm: 28,545,833 = 28,545,833
  - roots: 95,088,877 = 95,088,877
  - readout: 1,527,148 = 1,527,148

## Verdict on C4

Krylov + BM = 85.44% + 2.15% = **87.60% of the linear stage's `mac`s, above
70%.** That is 84.9% of its `field_mul`.

**L5's rule therefore demotes C4 to "≤ 1% of descent" and closes it.** The
measured ceiling is lower than that. C4 replays the normal-form pattern. Even
if it removed the whole `nf` kernel plus the whole `setup` kernel (staircase,
index and basis normalization), it would remove 11.79e9 bitops. That is 1.25%
of the linear stage and **0.27% of this stream's descent**. Against a
post-C1 descent about 1.7x smaller, it would be about 0.45%. The NF kernel
performs no multiplications on this stream, so the replay has no `mac`s to
remove.

The NF > 50% branch does not apply (NF is 0%). Border-basis direct read-off
is not recommended by this rule.

These observations are not part of the rule and are not recommendations:
- The linear stage's multiplications sit in the transposed Krylov products:
  163,405 `mac` per residual, 17.4% of descent.
- `x^p mod f` comes next, at 2.5% of descent.

Any future linear-stage lead would have to reduce those. This lane makes no
claim about whether that is possible.

## Limits

- **Counted work, not timing.** This is one reconstructed stream (p = 1009,
  Run50 seed 389), replayed locally. It is not a recorded Run50 measurement.
  It uses the hybrid build boundary described in `complete-accounting`.
- **Bucket boundaries.**
  - `setup` holds the basis normalization (6.8M products the legacy formula
    never counted). It could be read as NF preparation; the C4 ceiling above
    includes it.
  - The sort/dedup words (4,463) are in `readout`.
- **Not covered.** The dense fallback stage (5 fallbacks) and the hooked
  `inv_mod`/`PolyRing` work are outside `LinOps`, so they are outside this
  split. Both were already hooked before the complete-accounting patch.

## Files

- `README.md`: this file.
- `build/build.sh`, `assemble.py`, `fetch_frozen.sh`, `Cargo.toml.in`: copies
  of complete-accounting's build scripts. They point at this lane and apply
  the patch, with toolchain 1.98, `-j 1` and `CARGO_INCREMENTAL=0`.
- `build/build.log`: the build and test output, with source and binary hashes.
- `kernel-split/zero_dim_la.kernel_split.patch`: the per-kernel tallies.
- `kernel-split/kernel_split_recount.rs`: the recount driver with checks (e)
  and (f) and pricing.
- `kernel-split/conservation_test.rs`: the 108-system differential test with
  (e) and (f).
- `runs/kernel-split-seed389-n3415.json`: the full result (6.7 KB).
- `runs/kernel-split-seed389-n3415.time`: the wall time.
- `runs/RAW.sha256`: hashes, including the scratch-only patched source and
  the binary.
- `runs/kb-receipt.log`: the research-kb lookup attempt.
