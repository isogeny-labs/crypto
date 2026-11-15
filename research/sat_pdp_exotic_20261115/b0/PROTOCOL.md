# B0 — re-measured PDP SAT baseline: protocol

**Status: FROZEN** at the commit recorded in `FREEZE.sha256` (hashes of this
file, `protocol.json`, `analyze.py` and `run_rows.sh`), after instrument checks
at ℓ = 5 and 6 (all arms, 0 disagreements, 0 unverified models), none of which
is a row; an ℓ = 7 fast-arm check (oracle, native, lin, lin-nolex) was running
at the freeze and is reported in the session log as a check, not a row. Rows
are ℓ = 7…10 taken after the freeze commit. Everything in the parent
`PLAN.md` §2 applies; this file fixes the details the plan left open.

## Question

On uniform point-decomposition targets, how much work does each SAT arm spend
relative to the two exact yardsticks at ℓ = 7…10: `R = conflicts / T_enum`
against the pairs-and-solve oracle (`T_enum = 2^ℓ(2^ℓ+1)/2`), and
`R₃ = conflicts / C(2^ℓ, 3)` against the triple count (the survey's "one
conflict per candidate tuple" reading, `R₃ ≈ 1`); and does any arm's `R₃` fall
with ℓ?

Dry-run finding that fixed this framing (ℓ = 5, n = 14, 2026-11-15, not a
row): native median `R = 10.9`, `R₃ = 1.16`; `2^ℓ/3 = 10.7`. The incumbent SAT
arm is already `≈ 2^ℓ/3` behind the pair oracle because it refutes one triple
per conflict while the oracle refutes `2^ℓ` triples per pair by root-finding.

## Instance family (fixed)

- Curve `y² + xy = x³ + x² + 1` over `F_{2^n}`, `a = b = 1`, `n = 3ℓ − 1`,
  irreducible polynomial `find_irreducible_sparse(n)` (recorded per row as
  `irr_low_terms`).
- Factor base: abscissae `1 … 2^ℓ − 1` of the subspace `⟨1, z, …, z^{ℓ−1}⟩`,
  both points per abscissa.
- `m = 3`, symmetrised Semaev `S′₄`, Weil descent to GF(2) (`semaev_sat.rs`,
  `XorEncoding::Native` for the native and CMS arms, `XorEncoding::Cnf` for
  CaDiCaL), lexicographic symmetry breaking on in every SAT arm.
- Targets per rung: 16 **uniform** (random abscissa of a curve point with
  `x_R ≥ 2^ℓ`, i.e. outside the base) and 4 **planted** (`plant_nondegenerate`:
  `R = P₁ + P₂ + P₃` with distinct base abscissae, `x_R ≥ 2^ℓ`). RNG
  `StdRng::seed_from_u64(20261115 ^ (n << 32) ^ (ℓ << 16))`, uniform targets
  drawn first, then planted.
- Rungs: ℓ ∈ {7, 8, 9, 10} (n = 20, 23, 26, 29). ℓ = 5, 6 are the dry run and
  are reported as such, not as rows.

## Arms (same targets, same budget, one thread each)

| id | instrument | role |
|:--|:--|:--|
| oracle | `SubspaceOracle::low_order(ℓ, 1).decompose` (pairs-and-solve) | verdict and `T_enum`; `R ≡ 1` |
| native | `sat.rs` CDCL + Gauss–Jordan XOR, native parity rows, x-variable branch priority | incumbent |
| native-aa | native with `scramble_saved_phases(seed ^ 0xA5A55A5A ^ idx)` | A/A arm (the solver's only seed knob) |
| cms | CryptoMiniSat 5.8.0 (crates.io, `third_party/cryptominisat-rs-patches`), XOR-DIMACS of the native encoding, 1 thread | external XOR-aware reference |
| cadical | CaDiCaL 3.0.1 (`cadical-sys 0.8.0`), CNF of the Tseitin-XOR encoding | plain-CDCL control |
| chain-p12 | repository chained-S₃ encoder (`binary_semaev_chain_sat.rs`, AND-gate products, native XOR rows, `x₁ ≤ x₂ ≤ x₃`), branch priority on `x₁, x₂` bits; both identity patterns | existing chain implementation (ℓ ≤ 7 only; 4·10⁶ budget) |
| lin | **new solver** `examples/pdp_lin_cdcl.rs`: CDCL + XOR engine with field-product gates that linearise when one operand block is assigned; chained S₃ with full-width `t`; `x₁ ≤ x₂` lex; priority on `x₁, x₂` bits | candidate |
| lin-nolex | `lin` without the lex constraint | candidate control (pair-ordering effect) |

Conflict budget: 4·10⁶ per target per arm at ℓ ≤ 9; 1.6·10⁷ at ℓ = 10. A
budgeted-out solve is `unknown` and is reported censored, never imputed and never
counted in a median.

## Metrics

- **Primary** `R₃ = conflicts / C(2^ℓ, 3)` (survey yardstick, incumbent ≈ 1) and
  `R = conflicts / T_enum`, `T_enum = 2^ℓ(2^ℓ + 1)/2` (the pairs `X₁ ≤ X₂` the
  oracle scans on a refutation; the oracle's own `pairs` counter is recorded per
  target and equals `T_enum` on every refutation). `R_pairs` with `C(2^ℓ, 2)`
  is recorded beside them. Medians over the **decided uniform** targets of a
  rung; planted targets are controls, not rows. A SAT arm reaches the pair
  oracle's work only at `R = 1`, i.e. `R₃ ≈ 3/2^ℓ`.
- **Secondary**: decisions, propagations (and XOR propagations for native),
  `word_ops` for native, wall ms per target (descriptive only on a shared
  host), encoding size (vars, clauses, XOR rows).
- **Verdict agreement**: a SAT answer must decode to abscissae in the subspace
  whose `S′₄` value is zero; an UNSAT answer must match an oracle refutation.

## Controls

- Negative population: the uniform targets.
- Positive control: planted targets; every arm must return a SAT model that
  passes the `S′₄` check (within budget). A planted miss invalidates that arm at that rung.
- A/A: `native` vs `native-aa`; its spread in `R` is the noise floor every
  between-arm difference is read against.
- Same instrument: one binary (sha256 recorded), one `b0-ext-solvers` binary,
  one budget, one thread, same targets in the same order; arm order within a
  target is fixed (oracle, native, native-aa, cms, cadical), which is harmless
  for counted metrics and declared for the wall-time columns.

## Predictions (written before any ℓ ≥ 7 row)

1. `native` median `R₃ ∈ [0.9, 1.3]` at every rung, with no downward trend in
   ℓ (the survey's "one conflict per candidate tuple"); equivalently
   `R ≈ 2^ℓ/3`.
2. `cms` median `R₃` within `[0.5, 5] ×` the native median (ℓ = 5 dry run: 3.3×,
   with a wide per-target spread, 2× to 20×).
3. `cadical` median `R₃` within `[1, 3] ×` the native median (ℓ = 5: 1.6×);
   `cadical` and `cms` are not ordered by a prediction.
4. A/A relative spread `|R_native − R_aa| / R_native` median below 0.05.
5. Every planted target decided SAT by every arm within budget, each model passing the `S′₄` check.
6. Zero verdict disagreements with the oracle.
7. `lin` median `R ∈ [1.0, 1.7]` at every rung, trending to ≈ 1.5 as ℓ grows
   (ℓ = 5 dry run 1.20, ℓ = 6 check 1.46): for a pair whose quadratic in `t`
   has no root the engine refutes in one conflict, for a pair with two roots
   each root's `x₃` fails separately, and about half the pairs have roots, so
   ≈ 1.5 conflicts per ordered pair; `lin-nolex ∈ [1.5, 2] × lin`. The new
   solver spends about one conflict per `(x₁ ≤ x₂, t-root)` and no fewer. Its
   learnt-clause priority-width histogram shows whether any clause excludes more
   than one pair; the prediction is that conflicts never fall below the oracle's
   pair count on a refutation (`R ≥ 1` on every uniform UNSAT target).
8. `chain-p12` median `R` between 100 and 300 at ℓ = 7 (ℓ = 5: 160).

If (1) fails downward (`R₃ < 0.9` at any rung), the instrument is inspected
before anything else: count `T_enum` and the conflict counter definitions
against the solver source, then re-run. Only a reproducible `R₃ < 0.5` at two
consecutive rungs ℓ ≥ 8 is read as the survey's falsifier firing (its
`0.5 · T_enum` is stated against the triple count there).

## Invalidation

- A SAT model that fails verification invalidates that arm's row at that
  rung (bug, not result).
- A disagreement with the oracle on any target invalidates the rung for the
  disagreeing arm until explained.
- A planted miss: see controls.
- An export/import mismatch (CMS or CaDiCaL verdict disagreeing with native on
  the same target) is first checked against the oracle, which decides.

## Execution

- Instrument: `cargo build --release --example b0_pdp_sat_baseline` and
  `cargo build --release` in `tools/b0-ext-solvers` (after
  `third_party/cryptominisat-rs-patches/prepare.sh`).
- Counted metrics are host-independent and are taken locally in the cloud
  container (shared host, L0); wall-time columns are descriptive until an
  isolab-eks L2 run repeats the rungs. The bench record is opened before the
  first ℓ ≥ 7 row; raw JSONL goes to S3 by sha256.
- Validator: independent build, replay of ℓ = 8 and 9, field-by-field compare.

## Amendments

- **AM1 (2026-11-15T04:50Z, before any ℓ ≥ 8 data; stopping rule only).** `cms`
  and `cadical` run at ℓ = 7 only: the ℓ = 7 row run measured about 25 min of
  wall for one CryptoMiniSat refutation on this host, so ℓ ≥ 8 would exceed
  24 h per rung. Rungs ℓ = 8, 9, 10 run oracle, native, native-aa, lin,
  lin-nolex, and rungs may run concurrently on the same frozen binaries
  (counted metrics unaffected). Budget unchanged; the incumbent is therefore
  expected to be censored at ℓ = 9, 10 and that is reported as censoring.
  Hashes after the amendment: `FREEZE-am1.sha256`.

- **AM2 (2026-11-15T04:50Z, after the first ℓ = 7 target; chain-p12 only).** In
  the ℓ = 7 row run `chain-p12` censored at 4·10⁶ conflicts on target 0 after
  1282 s (R > 484, outside its predicted 100–300), which made the rung ≈ 11 h and
  kept the bench record open (an open record fails the repository's `records`
  check for every PR). The run was stopped after its first complete target; that
  row is kept unchanged as `rows/l7-run1-chain-p12-partial.jsonl`; the rung is
  rerun without `chain-p12`, and `analyze.py` merges rows of one rung by target
  (asserting the same `x_R`). `chain-p12` at ℓ = 7 is therefore 1 target,
  censored, and B5 reads "not supported, censored". Other arms' counted metrics
  are deterministic per target and unaffected. Hashes: `FREEZE-am2.sha256`.

- **AM3 (2026-11-15T06:00Z, before any ℓ = 10 data; ℓ = 10 only).** The frozen
  budget 1.6·10⁷ is below the incumbent's deterministic cost at ℓ = 10 (one
  conflict per triple, C(1024, 3) = 1.78·10⁸), so `native` and `native-aa` would be
  censored by construction at ≈ 40 machine-hours of cost and no information.
  ℓ = 10 runs oracle, lin, lin-nolex; the native arms are reported as censored by
  construction. ℓ = 9 (4·10⁶ < C(512, 3) = 2.2·10⁷) is run as frozen and its
  native censoring is observed. Hashes: `FREEZE-am3.sha256`.

- **AM4 (2026-11-15T06:05Z, execution only).** The session harness stops a
  background command after 2 h, which will cut the ℓ = 7 rerun and the ℓ = 8 run
  short. A detached watcher reruns a rung whose process ended with fewer than
  20 rows, with the identical frozen binary, arguments and budget, into
  `rows/l<ℓ>-run<k>.jsonl`; `analyze.py` merges by target (AM2). Counts are
  deterministic per (target, arm), as the ℓ = 7 target-0 check showed. No data
  is affected; some targets are computed twice. Hashes: `FREEZE-am4.sha256`.

## Inadmissible after the freeze

Changing budget, seeds, target counts, symmetry-breaking setting, encoding, or
rung list; re-running a censored target with a larger budget; reading a
planted target as a row; reading wall time on the shared host as a cost.
