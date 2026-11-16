# Replay of the chained prime-field transfer results

**Status: unsigned replay, informational only.** This replay was run by an agent
in the same session that dispatched it. It is not a signed validator report
under the validation gate, it carries no `VALIDATION.json`, it creates no pcc
claim, and it does not change the standing of any result. Every result sentence
below is an unverified assertion. [pcc:unverified]

Scope: `RESULTS.md` + `results.json`, `ATLAS.md` + `atlas_census.json` +
`sweep.json`, and `tower/TOWER.md` + `tower/tower.json`, all under
`research/chained_prime_transfer_20261111/`. Nothing outside `replay/` and
`src/bin/replay_check.rs` was edited, committed or pushed. [pcc:unverified]

## 1. Bindings

- Artifact commit: `60ed08cf3838c67a187f3c756c469e9ed27e1df3` (branch
  `claude/ecdlp-chained-transfers-8cpiek`). Producer sources
  (`src/cryptanalysis/chained_transfer/`, the three bins, `ic_boundary.rs`,
  `genus2_fp.rs`, `ecbench/hyper_ic.rs`) were clean against HEAD.
  While the replay ran, HEAD moved to `cb56c20127414200e71c1186385eb3019eaa3f4c`
  through two commits by another agent that only add `tower_ladder/`,
  `tower_formula/` and their bins. `git diff --stat 60ed08cf..HEAD` over every
  replayed artifact and source path is empty, so the artifact set is the same at
  both commits. Details: `ENV.txt`. [pcc:unverified]
- Toolchain: rustc 1.97.0, cargo 1.97.0, Linux 6.18.44 x86_64, 4 CPUs, release
  builds only. Binary hashes and run times are in `ENV.txt`
  (`chained_transfer_bench` 0.4 s, `transfer_atlas run` 1.7 s, `tower_bench`
  2.1 s, and 24 sweeps of 0.01 to 1.3 s each). [pcc:unverified]
- `PROTOCOL.md` matches `frozen.sha256` (`540b2c59...`). [pcc:unverified]
- The producer's bench records bind to dirty trees whose commits do not contain
  the producer code (finding F13). The replay binds the content instead: the
  raw outputs are byte-identical at HEAD. [pcc:unverified]

## 2. Determinism: committed against fresh raw output

All five raw files are byte-identical, and `replay_check diff` reports 0
differing leaf fields on each JSON (`logs/diff_*.log`). `sweep.json` is assembled
from 24 `transfer_atlas sweep` outputs in the committed layout (`[`, records
joined by `,` lines, `]`); the per-target files are in `sweep/`. [pcc:unverified]

| File | sha256 (committed = fresh) | Result |
|---|---|---|
| `results.json` | `fbe6197a79c1508061ea90c1ea764c2c5b76a3ee26ed4380b979d9d1690abc34` | IDENTICAL |
| `atlas_census.json` | `add33216328b4487517cf5b5adc51bcafa484c77cfa689054fdd8285442c2e6d` | IDENTICAL |
| `sweep.json` | `88fa7c9f1eed02673ecd5ff88313c6940718b9c8c11ed495c0f6748e731834c3` | IDENTICAL |
| `tower/tower.json` | `68720849224bde972e9139f89e7e2e2ffe63a27cd48a129ccc1d0df681f0f2eb` | IDENTICAL |
| `tower/tower.svg` | `a0c7120be3da413e513af40e15e09af18cc872ca0e83ab10d630c2998c0b4758` | IDENTICAL |

The committed hashes also equal the `sha256` fields in the two bench records
(`results.json`, `tower.json`). The records mark the raw objects
`"uploaded": false`, so the S3 side was not checked. (`SHA256_COMPARE.txt`.)
[pcc:unverified]

## 3. Headline recomputation (from the raw JSON, own program)

`src/bin/replay_check.rs` recomputes every number below from the JSON. It was
run on the fresh files (`logs/headline_*.log`) and on the committed files
(`logs/headline_*_on_committed.log`), with the same result. The column marked
"Recomputed" is an unverified assertion. [pcc:unverified]

| # | Headline claim | Verdict | Recomputed [pcc:unverified] | Evidence |
|---|---|---|---|---|
| R1 | IC/rho 9.8x at p=211 | PASS | 4184/428 = 9.776 | `logs/headline_results.log` |
| R2 | IC/rho 26.2x at p=419 | PASS | 9366/357 = 26.235 | same |
| R3 | IC/rho 36.0x at p=821 | PASS | 13313/370 = 35.981 | same |
| R4 | IC/rho 42.9x at p=1619 | PASS | 36363/847 = 42.932 | same |
| R5 | LSQ slope of ln(IC ops) on ln p is about 1.01 | PASS | 1.008082 | same |
| R6 | ratio grows with p | PASS | 9.78, 26.24, 35.98, 42.93 | same |
| R7 | every hop passed, IC recovered the known log in every cell | PASS in replay; the artifact holds booleans only (F14) | rho 12/12 and IC 4/4 recovered the planted x; `[x]G=Q` checked with independent affine code on E1 | `logs/certs_chained.log` |
| A1 | census: 300 curves per p | PASS | 300, 300, 300, 300 | `logs/headline_census.log` |
| A2 | cross-map-zero 300/300 at every p | PASS, and not vacuous | 300/300 each; 14371, 14385, 14392, 14397 of 14400 sample slots were generic fibers, zero held on all, control `phi1_* phi1^* = [2]` held on all | `logs/cross_census.log` |
| A3 | factsDiff = 0 | PASS as a value; zero by construction (F8) | 0, 0, 0, 0 | `logs/headline_census.log` |
| A4 | freeEasy = 0 | PASS as a value; zero by construction (F8) | 0, 0, 0, 0 | same |
| A5 | ATLAS.md census table (same order 7/4/7/3, E1 supersingular 4/2/3/2, anomalous 0) | PASS | all 12 cells match | same |
| A6 | Smart positive control recovers the log on F_43 | PASS (n = 1 instance) | `[43, true]` | same |
| A7 | sweep: 24 reports | PASS | 24 (7 of them have covers = 0) | `logs/headline_sweep.log` |
| A8 | sweep: 5641 covers | PASS, independently recomputed | 5641 | `logs/sweep_independent_maxc600.log` |
| A9 | max distinct complement orders 141 | PASS, independently recomputed | 141 (p=8009, target order 7966) | same |
| A10 | 3479 smaller-prime complements | PASS, independently recomputed | 3479 (3469 under the usual largest-prime-factor, F11) | same |
| A11 | 85 supersingular complements (prose) | PASS | 85 | same |
| A12 | easier_and_reachable total 0 | PASS as a value; zero by definition (F8) | 0 | same |
| A13 | cover_never_carries_log true everywhere | PASS as a value; narrower coverage than the prose (F9) | 24/24 true, 7 of them vacuous | `logs/headline_sweep.log` |
| A14 | `cover-spectrum.svg` / ATLAS.md line 60: "F_1009, order 990: 470/504 smoother" | **FAIL** | 87/504 smoother for order 990 at maxc 1009; 470/504 is the order-998 target (F10) | `logs/sweep_independent_maxc1009_for_svg.log` |
| T1 | IC3/IC2 ranges over 2.3 to 3.4 | PASS | 2.327 to 3.404 | `logs/headline_tower.log` |
| T2 | slope IC genus 2 = 0.85 | PASS | 0.854466 | same |
| T3 | slope IC genus 3 = 1.06 | PASS | 1.064992 (1.06 at 2 d.p., just under the 1.065 rounding edge) | same |
| T4 | slope rho on E = 0.44 | PASS | 0.441507 | same |
| T5 | priced table, 4 primes x 3 weights x 2 genera (24 cells) | PASS | all 24 match TOWER.md to the printed digits | same |
| T6 | genus 3 at least 1.29x rho at 0 GAE | PASS | minimum 1.2912 (p=53) | same |
| T7 | genus 2 is 0.44 to 1.08x rho at 0 GAE | PASS, and it crosses 1, so no rank at 0 GAE | 0.4381 to 1.0760 | same |
| T8 | genus 2 above rho at 0.1 and 1 GAE, 2.1 to 100x | PASS arithmetic; band is one-sided (F12) | minimum 2.087 at 0.1; maximum 100.026 at 1 | same |
| T9 | 3 to 35 homomorphism checks per rung | PASS, recounted from the public cover map | counts identical to `hom_checks_g2/g3` in all 16 cells | `logs/certs_tower.log` |
| T10 | all tower solves recovered the planted log | PASS in replay | rho on E, rho on J(C2), rho on J(C3), hyper_ic on J(C2), hyper_ic on J(C3), all 8 primes | `logs/certs_tower.log` |

## 4. Independent checks beyond the headlines

- **Sweep rebuilt from scratch.** `replay_check sweep-indep` reimplements the
  cover sweep with its own field code (character-sum point counts, cubic
  discriminant, binomial shift, Weierstrass-point test, largest exactly-once
  prime) and matches all 24 reports on every field except
  `cover_never_carries_log` (which needs the maps). [pcc:unverified]
- **Chained rows re-solved.** `replay_check certs` rebuilds the four instances
  with the producer's deterministic search, reproduces every committed count
  (rho on E1, E1', J(C), IC ops, trials, relations), recovers the planted `x`
  with rho on all three groups (5 to 38 walk steps against caps of 14,200 to
  80,200), and checks the E1 side with independent affine arithmetic:
  `[r]G = O`, `[x]G = Q`, `#E1` and `#E2` by character sum. `#J(C)` from the
  library L-polynomial point count equals `#E1 * #E2` at all four sizes, and
  `r || #J`. 0 attempts were measured and then discarded before the accepted
  instance (the skipped ones were pre-measurement refusals: 9, 27, 2, 1).
  [pcc:unverified]
- **Group law sanity on J(C).** `[#J]DG = 0` and 300 random triples per size
  (associativity, commutativity, inverse, doubling, validity) all hold.
  [pcc:unverified]
- **The solved instance is equivalent to the transported one.** For each size
  `phi*(G) = [lambda] DG` and `phi*(Q) = [lambda] DT` with lambda = 19, 34, 5,
  192, so the surrogate instance the bench solves carries the same log as the
  pull-back (F5). [pcc:unverified]
- **Tower rebuilt.** `replay_check tower-certs` rebuilds the 8 instances,
  checks the E side with independent code, `[r]D = 0`, `[#J]D = 0`, `T = [x]D`,
  `r || #J` and Hasse-Weil on both rungs, recounts the homomorphism checks, and
  re-solves with rho and `hyper_ic` (one seed each) against the planted log.
  80 checks, 0 failures. [pcc:unverified]
- **Rho scatter (40 seeds, informational).** The committed run has one seed per
  cell and no spread. Over 40 seeds the three rho columns have a standard
  deviation of 15 to 24 percent of their mean, and the means of E1', J(C) over E1
  are 1.06/0.99, 0.98/0.97, 1.03/1.03 and 0.94/0.90. So "no rho advantage, within
  scatter" holds; at p=1619 the J(C) mean is 0.90x E1, about 2 standard errors of
  the difference, which is a constant-factor wobble in a set-up dominated count
  (F7), not a change in walk length. With the 40-seed rho mean as denominator the
  four IC/rho ratios are about 10.4, 22.3, 37.9 and 55.2. [pcc:unverified]

## 5. Code-review findings

Severity is the effect on the headline, not on the code. File and line refer to
the tree at `60ed08cf`. [pcc:unverified]

**F1 (accounting asymmetry, medium): the IC column is overstated by 8 to 13
percent.** `scalar_counted` charges `ops += 1` for
`acc = c.add(&acc, &base)` even when `acc` is the identity
(`src/cryptanalysis/chained_transfer/measure.rs:250-266`, add at 256), once per
nonzero scalar, so about 2 per trial (`measure.rs:217-221`). `Genus2Curve::add`
returns at once on the identity (`src/cryptanalysis/genus2_fp.rs:657-663`).
The rho reference does not charge it, because `CountedGroup::mul` starts from
`acc = p` (`src/cryptanalysis/ic_boundary.rs:1377-1391`). Replayed exactly with
the IC's own xorshift: 484, 1051, 1717 and 3167 trivial ops of 4184, 9366,
13313 and 36363 (11.6, 11.2, 12.9, 8.7 percent). Corrected IC/rho: 8.64, 23.29,
31.34, 39.19; corrected slope 1.019. The conclusion (IC well above rho, ratio
growing) does not change; the printed ratios are too high. [pcc:unverified]

**F2 (emission gap, medium): linear-algebra cost is computed and dropped.**
`IcOutcome.la_field_ops` is filled (`measure.rs:121-122, 239-246`) but `Row` has
no field for it (`src/bin/chained_transfer_bench.rs:96-114, 185-202`).
`PROTOCOL.md` says linear algebra is "counted separately", and RESULTS.md calls
the IC figure a lower bound without the count. Replay: 14,633, 51,976, 164,828
and 603,721 field multiplications. Priced at 0 / 0.1 / 1 GAE each, IC/rho_src is
9.8 / 13.2 / 44.0, 26.2 / 40.8 / 171.8, 36.0 / 80.5 / 481.5 and 42.9 / 114.2 /
755.7. This widens the gap. [pcc:unverified]

**F3 (no solve check on the rho columns, low in effect): `rho_on_ec` and
`rho_on_jac` return only `group_ops`.** They drop `RhoResult.recovered` and
`verified` (`measure.rs:94-105`), so the bench never asserts that any of the
three rho columns solved the planted log; a run that hit the `200*r` step cap
would still print a cost. The replay shows all 12 rho runs solved (5 to 38 steps).
`tower_bench` does check (`src/bin/tower_bench.rs:58, 63, 67`). The committed
`results.json` holds no certificate for the rho solves. [pcc:unverified]

**F4 (low): the IC linear algebra is handed the answer.** `solve_linear`
receives `target_is_x(...)` (BSGS, uncharged) and accepts a candidate only if it
equals that value (`measure.rs:237, 361`). A wrong elimination returns `None`,
never a wrong log, so `ic_recovered_ok` is not independent evidence that the
elimination alone is right. The replay certified the answer separately with
`[k]DG = DT`. [pcc:unverified]

**F5 (low): the solved instance is not the transported one.**
`chained_transfer_bench.rs:161-170` verifies the chain on `(gen, target)` in
E1, then solves a fresh `jac_instance(...)` pair `(DG, DT = [x]DG)`. The
replay shows the two are equivalent at all four sizes (section 4), but nothing
in the code ties them. [pcc:unverified]

**F6 (low): failed attempts are silently dropped.** The search loop keeps the
first row with `ic_recovered_ok && chain_verified` and prints nothing about
rejected rows (`chained_transfer_bench.rs:282-302`). None was rejected here.
[pcc:unverified]

**F7 (medium for reading the ratio): the rho reference is dominated by
set-up.** With `dp_bits = (bits/2).saturating_sub(3)` every point is
distinguished at r = 109, 113 and 71, and each walk start costs two scalar
multiplications; the jump table costs 32 more
(`src/cryptanalysis/ic_boundary.rs:1433, 1443-1453`). Rho on E1 is 357 to 847
operations for 6 to 38 steps, an `S = ops/sqrt(r)` of 34 to 44 (28 to 53 over
all three rho columns) against an asymptotic 1.3. RESULTS.md does not say so
(TOWER.md does). The ratio to this
rho understates the gap to a tuned rho, and a single seed moves it by 20
percent (section 4). [pcc:unverified]

**F8 (three zero columns carry no evidence, medium):** `factsDiff`, `freeEasy`
and `easier_and_reachable` cannot be nonzero. `node_facts` computes `anomalous`,
`supersingular`, `largest_prime` and the embedding degree from `(p, #E)` alone
(`src/cryptanalysis/chained_transfer/atlas.rs:95-109`). `same_order_facts_differ`
and `free_easier_node` are tested only when `#E1 = #E2`, where those facts are
equal (`src/bin/transfer_atlas.rs:85-96`; `atlas.rs:160-165`). In the sweep,
`isog` means `e2.order == target.order`, so `strictly_easier` is false
(`atlas.rs:265-279`). ATLAS.md's "adversarial catch" repaired a definition, and
the resulting 0 is a consequence of that definition, not a measurement. The
values reproduce, and they should not be cited as evidence about reachable
nodes; the reachable set itself is an assumption stated in
`atlas.rs:156-162`. [pcc:unverified]

**F9 (low): `cover_never_carries_log` covers less than the prose says.** It
starts `true`, is tested only for the first 40 covers of each report, with 16
point samples and the E1 to E2 direction only (`atlas.rs:280-282`). The 7
zero-cover reports are `true` vacuously. ATLAS.md attaches it to all 5,641
covers. The separate census in section 3 (A2) does test both directions at 99.8
percent coverage. [pcc:unverified]

**F10 (documentation error, medium): `cover-spectrum.svg` and ATLAS.md line
60-61 report a wrong smoother count for the order-990 target.** The figure says
"target E1 order 990 ... 470 of 504 covers are smoother" (`cover-spectrum.svg:4,
16`). Independent recomputation at the figure's own range (maxc = 1009) gives,
for order 990: 504 covers, 63 orders, 22 isogenous (all three match the
figure) and 87 smoother. 470 smoother is the count for the order-998 target
(`{11,2,6,1}`, largest prime 499). `sweep.json` (maxc 600) agrees with the
independent code: 50 of 305. The SVG looks hand-written (no generator in
`src/`, `scripts/`, `tools/` or `.github/`) and uses maxc 1009, not the
committed sweep's 600. [pcc:unverified]

**F11 (low): "smoother" means "smaller largest exactly-once prime".**
`largest_prime_factor_sqfree` ignores primes whose exponent is above 1
(`atlas.rs:57-77`). With the usual largest prime factor the 3,479 becomes
3,469. [pcc:unverified]

**F12 (band is one-sided, medium for the 0.1 and 1 GAE columns):**
`hyper_ic` emits seven uncharged counters (`linalg_mulmods`, `linalg_inversions`,
`oracle_fp_muls`, `factor_base_fp_muls`, `factor_base_lookups`,
`canonicalisations`, `step_draws`; `src/cryptanalysis/ecbench/hyper_ic.rs:907-927`).
`tower_bench` keeps only `linalg_mulmods*` (`tower_bench.rs:74-81`) and prices
it on the IC arms only. `rho_reference` emits no unpriced counters at all
(`ic_boundary.rs:1536-1537`). Under AGENTS.md section 5a every arm lists every
unpriced counter and all arms are priced at the same weights; at weight 0 the
columns are exact group-operation ratios (T6, T7 stand), but at 0.1 and 1 both
sides are lower bounds, so "genus 3 is above rho at every price" and the 2.1 to
100x range are not established by section 5a. TOWER.md names only the
smoothness oracle. [pcc:unverified]

**F13 (reproducibility binding, low):** `benchmarks/records/20261111T0540Z-chained-prime-transfer.json`
records commit `12439ac5` with `commit_dirty: true`, and that commit has no
`src/bin/chained_transfer_bench.rs`; the tower record names `ecd04f4d`, dirty,
which contains neither `tower.rs` nor `tower_bench.rs`. `atlas_census.json`,
`sweep.json` and the 24 per-target sub-runs have no record. The
`transfer-cover-sweep` workflow and the `cover-spectrum.svg` generator that
ATLAS.md refers to are not in `src/`, `scripts/`, `tools/` or `.github/`, so
the assembly of `sweep.json` and the figure cannot be replayed from the repo
(this replay assembled `sweep.json` by the committed layout, which came out
byte-identical). All raw objects in both records are `"uploaded": false`.
[pcc:unverified]

**F14 (no certificate in the artifacts, medium for the validator vocabulary):**
`results.json` and `tower.json` hold `ic_recovered_ok` and `all_recovered`
booleans, with no generator, target, divisor or claimed log. A reader of the
artifacts alone cannot re-verify any solve; only this replay's re-execution
did. No pcc claim references this directory. [pcc:unverified]

## 6. Not checked, or checked only partly

- No S3 object was fetched (records mark raw data as not uploaded).
- No independent Cantor implementation was written. The J(C) arithmetic and the
  genus-3 `HCurve` arithmetic are the library's; they are checked by internal
  consistency, by `#J` from separate point counts, by `[#J]D = 0`, by Hasse-Weil,
  and by recovering planted logs.
- The adversarial-agent workflow narrative in ATLAS.md, "a second run under the
  record reproduced every count", and the TOWER.md interpretation that the 3x
  genus ratio comes from `1/g!` are not checkable from the artifacts.
- `cargo test` was not run (disk and the dispatch rules).
- No timing, isolation or wall-clock claim exists in these artifacts, and none
  was assessed. m = 83 and ECC2K-130 disclosure rules do not apply (prime fields
  only).
- The remaining `hyper_ic` counters (F12) cannot be priced, because
  `tower.json` does not carry them.
- The `transfer-cover-sweep` workflow definition and the figure generator were
  not found in the paths searched (`src/`, `scripts/`, `tools/`, `.github/`); a
  repo-wide search timed out on this large tree, so they may exist elsewhere.

## 7. Indicative outcome (not a signed disposition)

In the validator vocabulary, and only as an informal reading: the raw outputs
reproduce byte for byte; all headline numbers recompute (with the caveats F1,
F7, F8, F9, F11); one documentation claim fails (A14); and the artifacts alone
carry no solve certificate (F14), so a signed report would be `insufficient_evidence`
until a certificate-bearing artifact and a pcc claim exist, even though the
replay's own re-verification passed. [pcc:unverified]

## 8. Reproduce

```
cargo build --release --bin chained_transfer_bench --bin transfer_atlas --bin tower_bench --bin replay_check
D=research/chained_prime_transfer_20261111/replay
target/release/chained_transfer_bench run --json $D/results.json
target/release/transfer_atlas run --json $D/atlas_census.json
target/release/tower_bench run --json $D/tower/tower.json --svg $D/tower/tower.svg
# 24 sweeps: p in 1009 2003 4001 8009, six targets, --maxc 600 (see ENV.txt)
target/release/replay_check diff|results|census|sweep|sweep-indep|tower|cross-census|certs|tower-certs ...
```

`src/bin/replay_check.rs` sha256 and the binary hashes are in `ENV.txt`.
[pcc:unverified]
