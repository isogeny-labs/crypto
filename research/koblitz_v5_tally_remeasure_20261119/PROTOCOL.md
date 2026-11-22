# Protocol: v5 with its jump-index tally against strong on the small Koblitz curves (2026-11-19)

Frozen before any measured data. Class: **reference measurement** (existing methods; the only source change is a native
counter, no IC change, no security claim). Every number is `[pcc:unverified]` until validated.

## 1. Question

The owner decision of 2026-11-19 (`research/koblitz_rho_reference_20261115/DECISION.md`) makes
min(`rho.signed_frobenius_strong`, `rho.signed_frobenius_v5`) the matched rho reference for Koblitz rows below r = 2^38, on
condition that v5 first tallies its per-step jump-index selection as strong tallies its partition hash
(`partition_hashes_uncharged`, one per step). With that tally in place: what is v5 / strong per curve at w = 0, 0.1 and 1 on
every Koblitz curve with a leaderboard row below 2^38, and where does the ratio cross 1?

## 2. Source change and binary

Source commit `61e4022114` (built from a scratch worktree whose tracked sources equal that commit: `git diff 61e4022114 -- src Cargo.toml Cargo.lock crates` empty) adds the tally (`ic_boundary::TunedWalker::walk` counts one jump-index selection per step;
ecbench's `rho.signed_frobenius_v5` reports it as `partition_hashes_uncharged`, listed in `cost.unpriced`; no other method's
report changes; the replay audit compares a pre-tally v5 record without the counter). Unit test
`v5_jump_index_tally_adds_one_counter_equal_to_steps_and_changes_nothing_else`: the tallied report equals the pre-tally report
(recovered scalar, phases, group operations, GAE, every other counter) plus that counter, which equals `steps`.

Binary: `ecbench` built from that commit in a clean sparse scratch worktree (own CARGO_TARGET_DIR, CARGO_BUILD_JOBS=1, under the
CPU lock), sha256 `92ae9054d2c791c796fc9e2f1b01c53d2bc1584f6661c34e59957576003044cb`; `run.sh` refuses any other. Run with `--cpus none --allow-busy` (L0), launched through
`tools/benchrec.py run`.

## 3. Design

| arm | method | role |
|:--|:--|:--|
| rho-strong, rho-strong-aa | rho.signed_frobenius_strong (twice) | current reference; A/A control |
| rho-v5, rho-v5-aa | rho.signed_frobenius_v5 (twice, with the tally) | candidate reference; A/A control |

Curves (a, n): (0,13), (1,17), (0,19), (0,23), (1,23), (1,29), (0,31), (0,37): every Koblitz curve with a leaderboard row
below 2^38 (`research/koblitz_rho_reference_20261115/README.md` section 2; n23 in both the a = 0 and a = 1 variants). A curve
whose ecbench slug differs from the leaderboard's is reported with its slug, not dropped. One session per curve: 48 public
targets, 3 rounds, 1 warm-up, order `alternate`. Fresh target seeds 20261119601-20261119608 (unused in the repository at
freeze), measurement seed = target seed + 1000. Specs: `make_specs.py` -> `specs/`.

## 4. Pricing, statistics, reads

`ecbench verify --replay-all` on every session (0 problems required). `ecbench band --ranks`. `ecbench compare --weight w
--resamples 4000 --seed 20261002`, paired by target; interval = the wider of compare's two 95% intervals. `analyze.py`.

- R1 soundness: every measured solve of every arm verified; every v5 record (warm-ups included) carries
  `partition_hashes_uncharged` equal to its `steps` and lists it in `cost.unpriced`; both A/A ratios contain 1 at every weight.
  Any failure is reported and the curve's ratio is not used for the reference.
- R2 (primary, descriptive): v5 / strong with its interval per curve and weight; order "v5 below strong" iff the upper end < 1,
  "v5 above" iff the lower end > 1, else unresolved.
- R3 crossover: per weight, the log2 r where v5 / strong reaches 1, by linear interpolation of ln(ratio) between adjacent
  measured curves sorted by log2 r; "not reached" if every ratio is below 1. (Above 2^38 the reference stays strong by the
  decision whatever R3 finds; R3 checks that the 2^38 boundary is not inside a region where v5 is already dearer.)
- R4 consistency (no gate): at w = 0 the tally adds nothing, so v5 / strong should agree with the 2026-11-15 study on the
  curves both measured (n17, n23 a=1, n29, n31, n37) within their intervals; disagreements are reported.

Use of the result: the per-curve ratio is context only for leaderboard rows whose evidence has no in-session v5 (never applied
as a cross-session multiplier); rows whose evidence measured v5 in-session use min(strong, v5) from their own session.

## 5. Non-claims

No IC improvement. Changing the matched reference is an accounting change (AGENTS.md, "Relabelling and accounting changes are
not improvements"). L0 shared virtual machine; counted GAE are deterministic, wall time is not used.

## 6. Instrument check before freezing (not data)

One smoke session (a = 0, n = 13, 2 targets from seed 777001, arm rho-v5 only, scratch only, never analysed) confirmed the
pinned binary writes `partition_hashes_uncharged` = `steps` (8 = 8, 12 = 12), lists it in `cost.unpriced`, and passes
`verify --replay-all`. No strong arm was run and no ratio was computed.
