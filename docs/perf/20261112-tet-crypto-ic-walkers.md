# ecbench generic walkers (rho.negation, kangaroo.vow, bsgs.textbook): WALL-only

Slug `crypto-ic-walkers`, branch `perf/crypto-ic-walkers`, base `009e796a7a`
(`claude/performance-optimization-parallel-vgenii`). Rules: AGENTS.md section
10 and the audits [`20261110-wall-only-rust-fixes.md`](20261110-wall-only-rust-fixes.md),
[`20261112-rust-deep-audit.md`](20261112-rust-deep-audit.md): every value,
counted unit, RNG draw and record is unchanged. No Python implementation of
any algorithm (Python was used only to aggregate profiler output); no
proof-pinned file touched. Files edited: `src/cryptanalysis/ic_boundary.rs`
(walk paths and their direct helpers: `TunedWalker`, `rho_canon`,
`RhoClasses`/`CountedGroup` hooks, `invmod`) and
`src/cryptanalysis/ecbench/generic.rs` (`kangaroo_with`).

Wall figures are **local, unisolated, [pcc:unverified]** (shared 4-vCPU VM,
load average 1.4-1.8 during the A/B runs, `RAYON_NUM_THREADS=1`,
`report.solve_wall_ns` of `ecbench exec`, min and median of 20-25 runs,
base and candidate alternated). Callgrind Ir and branch-simulator counts are
deterministic for a binary but are **[pcc:unverified]** (not filed as claims;
valgrind's predictor is a model, not the CPU's).

## Profile first (base)

Inputs: `ecbench profile-input --spec docs/perf/20261112-rust-deep-audit/profile-specs/{rho-neg32,kangaroo32,bsgs32}.json --seq 0`, run as `ecbench exec` under Callgrind on a
`nodump.sh` copy ([`20261112-rust-deep-audit/`](20261112-rust-deep-audit/)).
Steps: rho 83.3k group additions, kangaroo 141.4k, bsgs 114.5k. Base wall
per step: rho 169 ns (14.07 ms), kangaroo 140 ns (19.7 ms), bsgs 147 ns
(16.8 ms). A standalone replay of the chord step with the same primitives
(`20261112-rust-deep-audit/microbench/mb.rs` family) gives about 127 ns on
this host, so 15-45 ns a step is outside the group law.

What the base profile showed beyond the audit's table:

- rho walker: Callgrind's branch simulator charges **57,464 mispredictions
  to `TunedWalker::walk`** (of 168k in the whole run, the other 84k being
  one per `add_raw`, the Euclid loop's exit): 41.5k of them at one
  branch, the negation map's "is -P the representative" test (key of -P
  below key of P, 50/50). The compiler kept it a branch, with the
  coefficient negations and the point select hoisted inside it.
- kangaroo: 9 string-keyed `BTreeMap` updates per distinguished point
  (about 5% of Ir), and the point's key mixed twice per step.
- `invmod`: a 64-bit `a % m` before the Euclid and a 32-bit `% m` after
  it, both on the critical path, both no-ops for the reduced operands the
  group law passes.

## Commits

1. `afbb582241` `invmod`: reduced operand skips `a % m`; the final
   `(m - t) % m` / `t % m` is taken only when the value reached `m`
   (`t ≤ m`, equality only for `t = 0` negated and `m = 1`, so the result is
   the same). Existing `invmod_*` tests (3 moduli families, unreduced
   operands, composites) cover it.
2. `0a042f148a` `kangaroo_with`: each kangaroo carries `mix(key(pos))`
   (computed once when the point is reached, not again at the start of its
   next step); `distinguished_points`, `lookups_uncharged`,
   `inserts_uncharged`, `same_kind_collisions` kept in words and written
   once at each exit, only when non-zero (the keys the per-event calls
   created); `steps` is always written, as before.
3. `6fdc0e59dd` rho walk: the negation canonicalisation selects point and
   coefficients with `cmov` (`pick_word`, inline asm on x86-64, `if` elsewhere;
   `CountedGroup::select_elt`, default `select_unpredictable`, overridden for
   `PrimePoint`; `RhoClasses::negation_rule` gates the fast path to
   `NegationClasses` with the walker's `r > 2`, so `r = 2` and every other
   class take the old code); the look-ahead test is one branch on the
   conjunction (all operands pure); the short-cycle window's any-equal scan
   uses AVX2 when detected. Tests added:
   `negation_canonicalisation_by_selects_equals_the_branches` (3 curves,
   about 2k points each plus infinity, `y = 0`, edge coefficients, against the
   branchy `canon` through a wrapper class), `any_key_equals_matches_a_scan`,
   `pick_word_is_the_select`.

## Binaries and Callgrind Ir (child `exec`, one thread)

`cargo build --release --bin ecbench -j2`, `CARGO_INCREMENTAL=0`, rustc
1.97.0, same flags for every row. sha256 in
[`20261112-tet-crypto-ic-walkers/binaries.sha256`](20261112-tet-crypto-ic-walkers/binaries.sha256).

| binary | content | rho.negation 32-bit | kangaroo.vow 32-bit | bsgs.textbook 32-bit |
|---|---|---:|---:|---:|
| base | `009e796a7a` | 57,107,116 | 60,017,478 | 46,323,829 |
| c2 | commits 1, 2 and a first rho-select form (`select_unpredictable`) | 56,868,787 | 55,536,319 | 45,534,052 |
| c3 | + mask-arithmetic select, AVX2 scan | 55,841,964 | 55,540,451 | 45,523,867 |
| c4 (final) | + `cmov` select = HEAD | 56,406,336 (-1.2%) | 55,543,345 (-7.5%) | 45,512,379 (-1.7%) |

Ir hardly moves for rho (the `cmov` costs a few instructions back); the
effect is in mispredictions and wall:

| rho.negation 32-bit, Callgrind branch simulator | `walk` mispredictions | whole run |
|---|---:|---:|
| base | 57,464 | 168,109 |
| c2 (select form the compiler turned back into a branch) | 55,942 | 166,738 |
| c3 (mask arithmetic: folded to a select, then a branch) | 51,712 | 167,558 |
| c4 (asm `cmov`) | 8,641 | 124,714 |

Wall (ms per solve, min / median, 3 alternations of 20 runs, base then c4;
load 1.4-1.8):

| arm | base | c4 |
|---|---|---|
| rho.negation 32-bit | 14.05/14.39, 14.06/16.03, 14.12/15.82 | 12.78/13.37, 12.80/13.00, 12.86/14.60 (-9%) |
| kangaroo.vow 32-bit | 19.75/21.45, 19.66/20.38, 19.65/20.01 | 18.09/19.81, 18.00/20.11, 17.99/18.66 (-8%) |
| bsgs.textbook 32-bit | 16.82/18.34, 16.70/18.66, 16.88/18.70 | 16.52/17.44, 15.57/17.78, 16.06/18.18 (-4%) |

Earlier alternations (c2 against base, 25 runs): rho 14.0 to 13.6 min,
kangaroo 19.7 to 18.1, bsgs 16.8 to 15.9 (these three carry `invmod` and
`kangaroo_with`, not the cmov), so `invmod` is the bsgs and most of the rho
gain before the cmov, the kangaroo change is the kangaroo gain. Per-commit
walls were not taken in isolation (each build is 12-17 minutes): attribution
is by which kernel a commit can touch.

## Identity

- `ecbench exec` on the three profile inputs, base vs c4: `.report`
  identical after dropping `*_ns`, `*_ms`, `wall*` keys (`norm.sh`; sha of
  the normalised report rho `dfcafae655da`, kangaroo `b86b3c4d19ab`, bsgs
  `2c98e96b6487`). Also identical for c2 and c3.
- Round-1 gates `docs/perf/20261110-wall-only-rust-fixes/gate-{prime,koblitz,tensor}.json`,
  base vs c4, `norm-records.sh` hashes equal: prime 36 records
  `b92aac8e6846767d`, koblitz 16 `0edf450ee11696a1`, tensor 4
  `1462ea7ff3921940`; `ecbench verify --replay-all` on the c4 prime
  session: 36/36 replays reproduced.
- Extra gate ([`gate-extra.json`](20261112-tet-crypto-ic-walkers/gate-extra.json)):
  16 arms (`bsgs.interleaved`, `bsgs.interleaved_v2`, `bsgs.negation_v2`,
  `bsgs.negation_table`, `bsgs.textbook_v2`, `kangaroo.vow_v2`,
  `rho.negation_v2`..`_v6`, `rho.negation_chained`, `rho.frozen_reference`,
  `rho.multitarget_pooled` batch 4, `rho.negation_lanes_v1` lanes 4, `rho.plain`)
  on prime_search 16, 22 and 28 bits, 2 targets: 96 records,
  `40f5cb2e845d9c2e` for base and c4, 96/96 replays reproduced. Arms that
  cannot run on a prime curve (`rho.cubic`, `rho.negation_strong`) are not in it.

## Tests

`cargo test --release --lib` on the final tree, by filter:
`ic_boundary::tests` 63 passed (4 ignored), `ecbench::generic::` 7,
`ecbench::methods::` 75, `invmod` 4, `kangaroo` 7 (1 ignored), `bsgs`, `rho`
and `endo_rho`: all pass except two tests that read files this sparse
worktree does not check out (`certicom_dlp::device_tests`, "tracked
gpu/certicom source must be materialized"; `ecbench::band::tests::pricing_table_work_...`,
missing `research/curve25519_f1_targetwalk_20261108/sessions/k20-s0/records.jsonl`):
not the code under change. Not run: `cargo clippy`, the full `--lib` run, the
CI debug-profile gate.

## Problems and reverted ideas

- The `c1`/incremental experiment is void: the binary copied as `c1` (first
  rho-select commit, built with `CARGO_INCREMENTAL=1`) is byte-identical to
  the unchanged-source incremental control (same sha256, same 57.4k
  mispredictions), so those two rows are not reported above; incremental
  builds were dropped (disk: the incremental cache grew to 3.6 GB and one
  build died on `No space left on device`).
- **`select_unpredictable` / mask arithmetic** for the negation select:
  compiled back into the branch (c2, c3 above); kept only as the non-x86
  fallback shape. The asm `cmov` is what removed the branch.
- Branch-free `invmod` variants and Barrett `mulmod` were not tried again:
  the audit measured them (binary GCD slower, Barrett about 5% at best and
  needs a `PrimeCurve` field).
- Standalone replay: `invmod` change alone about 1.5% of a chord step.

## Deferred (would change counted work or leave scope)

- Batched/Montgomery-trick inversion across the independent baby steps or
  lanes of BSGS and rho: needs one charged inversion per addition to stay
  true to what is performed, or a recount (METHOD/COUNTER).
- `remember` (6.5% Ir): the ring stores a 40-byte state per step, read only on
  a short-cycle hit; a lazy store would need the walk to replay on a hit.
- The Euclid loop's exit misprediction (one per `add_raw`, 83k per run) and
  the three dependent `div`s of `mulmod` per addition remain; they are the
  floor of this step (about 127 ns here).
- `RhoClasses::canon` could return the representative's key, saving one
  `key` per step (about 1%); it changes every class implementation.
- The same cmov treatment for `CubicClasses`/`QuarticClasses`/endomorphism
  canonicalisations (lanes walk, IC relation walks): other files and arms.

## Reproduce

```bash
cargo build --release --bin ecbench -j2            # base and HEAD, same flags
ecbench profile-input --spec docs/perf/20261112-rust-deep-audit/profile-specs/rho-neg32.json --seq 0 > rho.in
RAYON_NUM_THREADS=1 ecbench exec < rho.in | jq '.report.solve_wall_ns'
docs/perf/20261112-rust-deep-audit/nodump.sh BIN BIN.nd
valgrind --tool=callgrind --branch-sim=yes --callgrind-out-file=cg BIN.nd exec < rho.in
callgrind_annotate --show=Ir,Bcm cg
```
