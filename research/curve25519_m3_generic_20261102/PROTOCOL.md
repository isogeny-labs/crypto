# Curve25519-family m = 3 generic decompositions — protocol

Frozen 2026-11-02 before any ecbench data was taken. Amendments are appended
below with their reason and never rewrite this text.

## Question

Does a three-term (m = 3) decomposition oracle, on a smaller projected base,
lower the counted cost S of the generic index calculus on the Curve25519
miniatures (k = 20, 24, 28; `research/curve25519_ic_baseline_20261102`)
below the incumbent (m = 2)? Success criterion (AGENTS.md): a verified,
measured improvement over the incumbent; the gap to matched rho is reported
as distance remaining.

## Arms

Curves: the first curve entry of `c25519-k{20,24,28}.json` of the baseline
(`prime_explicit`, #E = 8r). `N_a` = 32, 81, 203 for k = 20, 24, 28.
Factor base `prime-projected-abscissa:size=N,scan_cap=16N` throughout.

- `rho-neg`: `rho.negation` (matched reference).
- `inc`: incumbent, `N = N_a`, oracle `mitm:negation_folded=1` (m = 2).
- m = 3 candidates at `N = round(f·N_a)`, f in {0.25, 0.35, 0.5, 0.7}
  (k=20: 8, 11, 16, 22; k=24: 20, 28, 40, 57; k=28: 51, 71, 102, 142), two
  oracles each: `mitm:m=3,negation_folded=1` (arms `mitm3-nN`) and
  `ksum:m=3` (arms `ksum3-nN`). `rootfind-mitm:m=3` does not exist in
  `ecbench methods`; `ksum:m=3` is the second m = 3 oracle.

Specs are emitted by `mkspec.sh` (jq) into `specs/`.

## Train

k = 20, 24, 28; 16 fresh public targets, 1 measured round, 1 warm-up,
order `alternate_measured` (one round cannot be balanced under either
order; ecbench warns, accepted), `target_seed` 2026111000 + k, timeout 1800 s per run. Per k the
selected arm is the m = 3 arm with the lowest mean S at w = 0 (in-sample
choice among 8, disclosed). A timed-out or failed arm is reported, and is
not selectable.

## Holdout

32 fresh public targets x 2 measured rounds, 1 warm-up, order `alternate`,
`target_seed` 2026111100 + k. Arms: `rho-neg`, `inc`, `sel` (selected train
arm), `inc-aa` (A/A copy of `inc`), `rho-neg-aa`.

## Decision

Per k, **improvement** only if the paired
`ecbench compare --a inc --b sel` ratio (sel/inc) has its 95% interval below
1 at w = 0 AND `ecbench band --ranks` orders sel < inc at every w in
{0, 0.1, 1} GAE per unpriced native operation (AGENTS.md section 5a).
Otherwise null (interval spans 1 or band order not uniform) or regression
(interval above 1). The A/A ratio inc-aa/inc must contain 1. Relabelling or
accounting changes are not improvements.

## Acceptance

`ecbench verify --replay 8` passes on every session; failures are reported.
Compute budget about 60 min on a shared 4-core host; any arm too slow is
capped by the 1800 s timeout and the cap is recorded.

## Execution route

Local L0, counted units only, frozen binary `/home/user/frozen-bin/ecbench`;
no Iso Lab identity in this container (as the baseline and ratchet
protocols). Not a leaderboard or benchmarks row; no validation-gate evidence.

## Amendment 1 (before holdout, after train)

- The shell rounding `round(0.5*81)` gave N = 41 (not 40) for k = 24; the
  arm names `mitm3-n41`/`ksum3-n41` are authoritative. Train was run with
  `--allow-busy` (the host PSI exceeded 5 with other agents compiling), so
  runs are recorded at L1, not L0; counts, not wall time, are claimed.
- Selection (rule unchanged, mean S at w = 0): k20 `ksum3-n11`,
  k24 `ksum3-n20`, k28 `ksum3-n51`. k24 and k28 selections sit at the
  smallest grid N (grid boundary), disclosed.
