# Pre-registration: an invariant guess on the second link, in place of the raw guess on x₃

Frozen 2026-11-05 before any run of the new arm.

## Question

At (22, 8) with three guesses, the best measured guess set is
**S3** = {`a₇+b₇`, `c₇`, `e₁[0]`}. It beat a raw guess on the pair
(research/dreg_invariant_guess_more_20261029, #2883). A second invariant
guess on the pair in place of `c₇` did not help
(research/dreg_invariant_pair_guess_20261029, #2885).

The second link `S₃(e₁, x₃, x(R))` is symmetric in its two unknown
arguments, so `e₁ + x₃` is invariant under swapping them, as `s = x₁ + x₂`
is for the first link. **Question: does fixing a coordinate of `e₁ + x₃`
(arm L3), in place of S3's raw `c₇`, suffice on more draws than S3?**

- **Arm L3:** `x₁₅ := x₇ + c` (`a₇ + b₇`), `(e₁ + x₃)₂₀ = h`, `x₂₄ = g`
  (`e₁[0]`).
  - The second guess is `x₄₄ := h + Σ_{t ∈ T₂₀} x_{16+t}`, where
    `T₂₀ = {t : v_t has bit 20}` is read from each draw's basis
    (`--link2 20=h`).
  - Branch `b`: `c = b & 1`, `h = (b >> 1) & 1`, `g = (b >> 2) & 1`.
- **Why coordinate 20.** k = 21 is unusable: T₂₁ is empty on draw 6, which
  would make the guess a raw `e₁` guess there. The rule fixed before any
  closure data is the largest k with |T_k| ≥ 2 on every registered draw,
  which is k = 20. |T₂₀| ranges from 2 to 6.
- **Comparison.** L3 differs from S3 in exactly one guess, `(e₁+x₃)₂₀`
  instead of `c₇`, and both leave 43 occurring variables (136,698 degree-4
  columns).

## Draws and comparator (all committed)

- **Draws.** The same 12 unsatisfiable (22, 8) draws as #2885:
  - 0, 3, 5, 6 from `research/dreg_invariant_guess_20261029/draws/m3-22-8.jsonl`;
  - 8, 11, 14, 15, 17, 24, 25, 27 from
    `research/dreg_invariant_guess_more_20261029/draws/m3-22-8-more.jsonl`.
- **Comparator S3.** S3's committed records in those studies' `runs/`.
  Closure outcomes are exact, so S3 is not re-run.
- **Instrument identity (N).** `dreg_linear_guess` has a new build here
  (multi-source `--sum`, `--link2`; `runs/binary.sha256`). Its S3 run on
  draw 5 branch 0 must equal the committed
  `research/dreg_invariant_guess_20261029/runs/S3.d5.b0.jsonl` on every scan
  field except `secs`.

## Exactness check, run before this commit

- `--check` is true on 24 sampled L3 systems (branches 0 and 7 of all 12
  draws), each with 43 occurring variables.
- A build with the multi-source point generator deliberately broken (one
  source dropped) gives `false`.

## Measurement (`run.sh`)

- Each run is `dreg_linear_guess --to 4 --max-cols 140000 --check`, under
  `prlimit --as=15e9 timeout 7200`, one process at a time.
- Every attempt is kept: an empty earlier attempt is moved to `runs/attempts/`
  before a retry.
- **Early stop:** a draw stops at its first branch that saturates degree 4
  without refuting.
- **Order:** N first, then L3 branch by branch across the 12 draws.

## Validity

- **V1:** every L3 record's draw and guesses match its name, including the
  resolved second-link sources against the draw's basis. `identity_check` is
  true.
- **V2:** N matches the committed record.
- **Every draw is decided** for both arms.

## Verdict rules (12 draws)

A draw is *discordant* when exactly one of L3 and S3 suffices.

- With k_L discordant draws where L3 suffices and k_S where S3 suffices
  (d = k_L + k_S), use a **two-sided** exact sign test:
  `p = min(1, 2·P(Bin(d, ½) ≥ max(k_L, k_S)))`.
- **L3_better:** p ≤ 0.05 and k_L > k_S.
- **S3_better:** p ≤ 0.05 and k_S > k_L.
- **No detected difference:** anything else, including d = 0.
- Descriptive: each arm's suffice count, and, where both fail, which fails
  first.

## Predictions (before any L3 run)

- In #2885, replacing `c₇` with a second guess on `s` made things worse.
- If what matters is that each link has one coordinate of its invariant
  fixed, L3 should do at least as well as S3. If it is x₃'s dimension, L3 is
  worse.
- P(L3_better) ≈ 0.2, P(no detected difference) ≈ 0.55, P(S3_better) ≈ 0.25.

## What it would mean for ECC2K-130

- **L3_better:** one invariant guess per link is a rule that extends along the
  chain, and m = 4 would be the next test.
- **S3_better:** the raw guess on the closing summand is what carries the
  second link.
- In every case: a stage-level reading of the guess-then-degree-4 oracle's
  guess choice. No `S`, no rho ratio, no leaderboard row. Results are cited as
  unverified assertions until a pcc verdict exists (AGENTS.md, "Sabotage is
  assumed").

## Scope

m = 3, one cell, 12 draws, the 4-core 16 GB container, not isolated.

- Iso Lab EKS and the S3 resume checkpoint are not reachable from it.
- Closure outcomes are exact algebra; timings are indicative only.
- Conductor is not installed.
