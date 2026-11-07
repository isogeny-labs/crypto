# Validation of the pq2_rank census (validator, 2026-11-05)

Nothing in this directory is signed. Every figure below is the validator's own
observation and stays an unverified assertion until a CI-signed verdict exists
[pcc:unverified].

| file | what it is | status |
|---|---|---|
| `validator_report.json` | Task A: headline recomputation (`pcc.validator-report/v1`), final pass at `8d9fdc45` | **fail**: 4 of 195 items, all the bounded-degree reading (see "Final pass" below) |
| `replay_certificate.json` | Task B: re-execution, independent closure, msolve replay (`pcc.replay-certificate/v1`), bound to `8d9fdc45` | **verified** (`scalars_verified: true`; 31 run entries, 0 mismatches) |
| `controls.json` | negative controls for the validator's own checkers | 8 of 8 planted defects caught |

The sections from "Binding" to "Not checked" describe the first pass at `f96f4f4c`. Three
reports exist:

* first pass, 2 of 85 items failing: `validator_report.json` at `ea2fde0e`;
* second pass, 3 of 120 failing: the same file at `3cb90671`;
* third pass, 123 of 123 matching: the same file at `8d9fdc45`;
* final pass, binding, 4 of 195 failing: the current file.

The second, third and final passes are described at the end. The 17-entry certificate
from the first pass is the blob of `replay_certificate.json` at `8d9fdc45`.

**Disposition (first pass).** The five headline claims in the dispatch recompute exactly, and the
replay reproduces them [pcc:unverified]. Two other statements in README "Results" are
`insufficient_evidence`, so the validator report is `fail`. Neither is a wrong number
in a results file. Both are sentences the committed artifacts do not support (findings
F1 and F2 below). No producer file was edited.

## Binding

* Validated commit: `f96f4f4c019bc95084b24d9dcb80a10655200cb3` on branch
  `claude/elliptic-curves-graph-weaknesses-fw42m4`. `recompute.py` and `replay_compare.py`
  read every producer file from the git object store at that commit, never from the
  working tree. Every working-tree copy was also hashed: none differed.
* HEAD moved twice during validation, because another session committed `809c95bc`
  (`f4/`, `F4Containment.lean`, partial `results/iter4_n25..27/run.log`) and then
  `36ba42df` (pcc claims for F4Containment). Every validated path is byte-identical at
  both commits. The in-progress iter4_n25..27 runs were neither read nor touched.
* Binary: rebuilt from the committed `src/main.rs` (sha256 `386a7595…dc0d`) with
  `cargo build --release -j 1 --locked --target-dir <scratchpad>/target`, pinned to one
  core. This used a separate target directory so that the binary used by the running
  producer jobs was not rewritten. The fresh binary hashes to
  `a6d1f253f4e538e6bdaeb16cdabfa50784f2ee883f208a551c6668f6ef21f7cc`, byte-identical to
  the producer's `target/release/pq2_rank`. All runs used `cwd` = the pq2_rank directory,
  as in the main sweep.
* Source drift: the sweep log was written by the source of `9ae9c1323b`. The committed
  source (`934c380932`) adds only the `PQ2_DUMP` hook, which draws no random numbers.
  `Cargo.lock` is untracked; the crate has no dependencies.
* Environment: rustc 1.97.0, cargo 1.97.0, Python 3.13.18, numpy 2.5.3, Intel Xeon
  2.10 GHz (pclmulqdq: true), Linux 6.18.44. Every validator process ran under
  `taskset -c 3` (one core).

## A. Recompute (`recompute.py`, validator's own parser)

The script parses `results/run.log` directly, not the JSON. It checks every derived
file row by row against its source log. It recomputes each count.

| claim | claimed | recomputed |
|---|---|---|
| 1. main sweep instance lines, n range | 119, 8..47 | 119, 8..47 (2 lines at n = 47; none at n ≥ 48) |
| 1. iter3 = k, n ≤ 14 | 21 of 21 | 21 of 21 |
| 1. iter3 = k, n = 16 | 3 | 3 |
| 1. iter3 = 1 < k, n = 15 | 3 | 3 |
| 1. iter3 = 1 < k, 17 ≤ n ≤ 44 | 84 of 84 | 84 of 84 |
| 1. iter3 not computed, n = 45..47 | 8 | 8 |
| 1. `results/pq2_rank.json` / `.csv` equal `run.log`, all 23 fields incl. timings, in order | 119 | 119 / 119 |
| 2. iter3 < k and iter4 = k, `iter4_n17_20` + `iter4_n21_24` | 24 of 24 | 12 + 12 = 24 (iter3 = 1 on all 24) |
| 2. those instances are fresh (parameters shared with the main stream) | 0 | 0 |
| 2. `iter4_n21_24` JSON = its `run.log`; CSV = JSON (both sweeps) | 12; 12, 12 | 12; 12, 12 |
| 3. rerun lines compared / mismatching (17 non-timing fields) | 99 / 0 | 99 / 0 |
| 4. F_size / 2^n′ over the 99 rerun instances: min, max, mean | 0.84, 1.31, 1.021 | 0.84375, 1.3125, 1.02104 |
| 4. same for n ≥ 30 (33 instances): min, max | 0.997, 1.004 | 0.99660, 1.00438 |
| 5. crosscheck degree 3: Python low = Rust iter3; k agree | 5; 5 | 5; 5 (k also equal to N + 1 − rank of [1 \| z] over the dumped Z) |
| README table: d_lin = 4 (20) and > 4 (1) at 8–14; d_iter = 4 at 15; d_lin > 4 at 15–20; > 3 at 21–47 | as listed | all equal |
| README: low2 = 1 everywhere; low3 = 1 for n ≥ 9; N − 7 ≤ k ≤ N | 119; 116; 119 | 119; 116; 119 (N − k ranges over 0..7) |
| **F1.** crosscheck "agrees with Rust … at degree 4" on the five dumped instances | 5 | **0** |
| **F2.** "fails on all 119 instances: d_iter ≥ 3 everywhere" | 119 | **111** |

Rounded claims are compared at the precision the README states. The exact values are
in `recompute.py`'s output.

**F1.** In the committed artifacts, the Rust run on the five dumped instances has
`iter4 = -` on all five. Its command is `… 14,15,16,17,18 1 16 <dir>/run 18 0`, with
iter4_max_n = 0. Python degree 4 was run on only 3 of the 5 instances (n15, n17, n18).
`crosscheck/README.md` itself says the degree-4 column has no Rust counterpart. The
substance is nevertheless true. The validator re-ran that command with iter4 enabled
(`… 18 18`), and on those instances Rust gives iter4 = 13, 15, 15, 17, 17. Those values
equal the crosscheck's Python degree-4 values (3 of 3) and the validator's own closure
(5 of 5). The fix is to commit such an output, or to reword the sentence.

**F2.** The README defines d_iter as the least d with iter_d ≥ k, where iter_d comes
from the degree-d closure. So d_iter ≥ 3 needs iter2 < k. The producer never computed
iter2: `src/main.rs` labels degree 2 using the one-shot `low2`, and low2 ≤ iter2 always.
The claim is established on 111 instances:

* 87 instances with iter3 < k, because the closures are nested and so iter2 ≤ iter3.
* 24 instances with iter3 = k, by the validator's own degree-2 closure, which gives
  iter2 = 1.

The 8 instances at n = 45..47 have no iter2 or iter3 value anywhere. The producer's own
log labels them `d_iter=?`, and its README table shows "–" there. On those 8 the claim
holds only under the code's convention of testing degree 2 with low2 (low2 = 1 < k). The
fix is to say "on the 111 instances with n ≤ 44", or to compute iter2 there.

## B. Replay (`replay_compare.py`)

Every compared instance line is identical on all non-timing fields, round counts
included. `replay_n8_44` (52 min wall on the shared pinned core) covers 111 of the 119 main-sweep lines; the 8 lines at n = 45..47 were not replayed.

| run (`validation/<id>/`) | command (cwd = pq2_rank) | reference | lines | mismatches |
|---|---|---|---|---|
| `replay_n8_20` | `pq2_rank 8,…,20 3 20 validation/replay_n8_20/pq2_rank 44 16` (dispatch) | run.log lines 1–39 | 39 | 0 |
| `replay_n8_20_dump` | same arguments with `PQ2_DUMP` set | run.log lines 1–39 | 39 | 0 |
| `replay_n8_24_dump` | `pq2_rank 8,…,24 3 20 … 44 16` with `PQ2_DUMP` | run.log, n ≤ 24 | 51 | 0 |
| `replay_n8_44` | `pq2_rank 8,…,44 3 20 … 44 16` | run.log, n ≤ 44 | 111 | 0 |
| `replay_n8_44` JSON | the same run | `rerun_n8_40/pq2_rank.json`: instance parameters, F_size, all values | 99 (its 12 rows at n = 41..44 have no counterpart) | 0 |
| `iter4_n17_20_replay` | producer's `COMMAND.txt`, out prefix moved, `PQ2_DUMP` set | `results/iter4_n17_20/pq2_rank.json` (23 non-timing fields) | 12 | 0 |
| `iter4_n21_24_dump` | `pq2_rank 21,22,23,24 3 0 … 24 0` (iter4 off) | `results/iter4_n21_24/pq2_rank.json` (20 non-iter4 fields) | 12 | 0 |
| `dump_n15_17` | `PQ2_DUMP=… pq2_rank 15,16,17 1 0 … 17 17` (dispatch) | the validator's closure, below | 3 | 0 |
| `dump_n14_18` | the crosscheck's dump command with iter4 on (`… 18 18`) | `crosscheck/n1{4..8}_i0.json` | 5 dumps byte-identical | 0 |

**Independent closure (`closure_check.py`, written from scratch).** For each dumped
instance it does the following, without the producer's coefficient formulas:

1. Checks the modulus (Rabin irreducibility test), b ≠ 0, rank(basis) = n′, and that
   x_R lifts to a curve point. It also checks S3(x(P), x(Q), x(P+Q)) = 0 using its own
   group law.
2. Evaluates S3(x1, x2, x_R) at all 2^N assignments. It requires the algebraic normal
   form of each output bit to equal the dumped equation, monomial for monomial, and the
   zero set to equal the dumped Z.
3. Computes k as N + 1 − rank[1 | z] over Z, and rho from the derived ANF.
4. Computes the one-shot low2, low3 and low4.
5. Computes the degree-2, 3 and 4 closures by worklist saturation in F2[x] (monomials
   not reduced by x² = x, field equations as generators). This is a different
   algorithm from both the Rust rounds and the crosscheck's rounds.

It then checks invariants. Every closure basis vector must vanish on every z in Z. The
low part must not exceed k. A closedness pass re-multiplies basis vectors (all of them
at d ≤ 3, 400 sampled at d = 4) and requires each product to reduce to zero.

| instances | source of the Rust values | degrees | values compared | mismatches | failed invariants |
|---|---|---|---|---|---|
| 39 main sweep, n = 8..20 | replayed lines = run.log | low2–4, iter2–3, iter4 (n ≤ 16) | 429 | 0 | 0 |
| 12 main sweep, n = 21..24 | replayed lines = run.log | low2–3, iter2–3 | 132 | 0 | 0 |
| 3 dispatch dumps, n = 15..17 | `dump_n15_17` | low2–3, iter2–4 | 33 | 0 | 0 |
| 5 crosscheck instances | `dump_n14_18` | low2–4, iter2–4 | 55 | 0 | 0 |
| 12 `iter4_n17_20` | producer JSON | low2–3, iter2–4 | 132 | 0 | 0 |
| 12 `iter4_n21_24` | producer JSON | low2–3, iter2–4 | 132 | 0 | 0 |

The validator's closure dimensions and `one_in_span` also match `crosscheck/closure_d3.txt`
and `closure_d4.txt` (32 values). Across all of these instances iter2 = 1 = low2, so the
producer's low2 shortcut changes no label where it could be checked.

**|F′| recount.** Factor-base points were recounted with the validator's numpy GF(2^n)
for the 75 rerun instances with n ≤ 32, as 1 + 2·#{x ∈ V∖0 : Tr(x + a + b/x²) = 0}. The
recount equals the producer's F_size on 75 of 75. The instances at n = 33..40 rest on
the Rust replay alone.

**Controls (`controls.py`).** The baseline passes. Each of the following planted defects
is caught:

* a flipped monomial in a dumped equation (the ANF check fails);
* a solution dropped from Z (the Z check fails);
* a producer iter3 off by one (a value mismatch is reported);
* a planted generator outside the ideal (the closure stops vanishing on Z).

The log parser reads a known line exactly, sees a changed value, and rejects a line
with a missing field.

## Other observations

* `results/iter4_n17_20/` has no `run.log`, only JSON and CSV. Its rows were checked
  against the validator's replay, not against a log.
* The README table in "Main sweep" and the "Reading" bullet disagree about d_iter at
  n = 45..47: the table shows "–", the bullet says ≥ 3. This is the same issue as F2.
* Process: the validator's first attempt at the `iter4_n17_20` replay stopped after two
  instances. It wrote no stderr, the kernel log shows no OOM, and the producer jobs were
  unaffected; the cause is not known. The validator deleted that partial output (its own
  directory, two instance lines never compared) and re-ran the command cleanly.
  `replay_n8_44` and the closure checks ran concurrently on the same pinned core.

## Not checked

* **Signing.** The validator cannot sign. `pcc verify` needs a verdict signed by CI
  (`ci-main`), so these files carry no standing.
* **F4 containment.** The claim that F4's output lies in the degree-D closure, which is
  what makes d_iter a bound on F4, is a separate Lean claim (`F4Containment.lean`,
  committed in `809c95bc` during this validation). It was not reviewed. The Lean
  theorems cited in the README (`SolvingDegreeBound`, `BooleanNullstellensatz`,
  `SubspaceCriterion`) were not reviewed either.
* **Interpretation.** What the counts mean for Petit–Quisquater Assumption 2 or the
  Kousidis–Wiemers reading was not assessed.
* **Coverage of the independent implementation.** It covers n ≤ 24 (N ≤ 24). iter3 at
  n = 25..44 rests only on a replay of the same Rust code. n = 45..47 was not replayed:
  Z enumeration takes 3–6 minutes per instance there.
* **Z completeness.** Z was checked independently only on the brute-forced instances
  (N ≤ 24).
* **Timings** were not compared.
* **Producer code.** The crosscheck's `naive_closure.py` and `log_to_json.py` were not
  executed. Their committed outputs were compared instead.
* **Reporting contract.** The one-table/ratio/rho-reference rules and the §5a band do
  not apply: this is a degree census, not a cost comparison.
* **Repository run procedures.** The resume checkpoint, session log, isolab-eks
  routing, bench records and research-visuals were not done. The dispatch restricted
  writes to this directory and specified local execution on one core.

## Second pass: the corrected text (commit `ea2fde0e`)

The producer corrected F1 and F2 in `5f12533a`. That commit edits `pq2_rank/README.md`,
`crosscheck/README.md`, the census `README.md`, `report.typ`, `WEAKNESS_TRIAGE.md`,
`WEAKNESSES.md`, `weakness.json` and corpus edge R4-013, and re-renders two corpus PDFs.
The producer then committed this directory as `ea2fde0e`.

The coordinator first asked for a recompute against the uncommitted working tree. The
validator does not validate a moving working tree, so it ran only a non-binding preview.
The corrections were committed while the preview ran, and the binding report was then
made from that commit:

    python3 -I -B validation/recompute.py --commit ea2fde0e4ded41a08ed5d966c96f7383d61ab802 --claims corrected

Checks before binding:

* The two commits contain only the files listed above (plus in-progress `f4/` and
  `iter4_n25/26` outputs).
* The committed copies of this directory are byte-identical to what the validator wrote.
  The 8 deliverables match the hashes reported at handback, and all 39 output hashes
  recorded in `replay_certificate.json` match the committed files.
* The producer files the certificate read are unchanged at `ea2fde0e`, so the
  certificate stands for that commit.
* In `--claims corrected` mode, every *claimed* number is parsed from the document text
  itself, so a changed or missing phrase fails its item. The validator's own evidence is
  read from the commit and pinned: the iter2 values in `closure_replay_n8_20.json` and
  `closure_replay_n21_24.json`, and `closure_dump_n14_18.json` with its dumps.

| statement (document) | claimed | recomputed |
|---|---|---|
| agrees with Rust on five dumps at degree 3 (pq2 README, crosscheck README, report.typ) | 5 | 5 |
| Python degree 4 reaches k on the three instances it was run on | 3 | 3 |
| the Rust run on those dumps had degree 4 disabled | 5 | 5 (iter4 = `-`, command ends `18 0`) |
| validator ran both degrees on all five and found agreement | 5 | 5 (dumps byte-identical to `crosscheck/`) |
| crosscheck README table, all cells | 5 rows | 5 rows match |
| validator's Rust degree 4 matches the Python degree-4 column | 3 | 3 |
| literal Assumption 2 fails on the instances with n ≤ 44 (pq2 README, census README, report.typ, triage) | 111 | 111 (every n ≤ 44 instance) |
| via iter3 < k / via validator iter2 / iter2 value / open at n = 45..47 | 87 / 24 / 1 / 8 | 87 / 24 / 1 / 8 |
| census README: n = 15 and 17..44 with iter3 = 1 | 87 | 87 |
| report.typ, triage, corpus node: 17 ≤ n ≤ 44 with iter3 = 1 | 84 | 84 |
| degree-4 sweeps reach k; iterated degree exactly 4 (report.typ) | 24 | 24 |
| corpus node: low2 = 1 on every instance | 119 | 119 |
| R4-013: solver degree exceeds 2 on the n ≤ 44 instances | 111 | 111 (lower bound, under the containment property) |
| **R4-013 (also in `WEAKNESSES.md`, `weakness.json`): solver degree "exactly 4 on 24 fresh instances at n = 17 to 24"** | 24 | **0** |

**F3 (new).** R4-013's subject is the solver degree. The census measures the iterated
closure degree. d_iter = 4 exactly on the 24 fresh instances gives solver degree ≥ 4,
under the containment property, but no upper bound. No artifact runs a solver on those
24 instances: `f4/run_msolve.py` ran msolve on the main-sweep dumps (`f4/dumps`), not on
the degree-4 sweeps, and `f4/` is in progress and was not validated. The clause
pre-dates the correction, but it sits in the corrected sentence. The census's own scope
contradicts it: "lower bounds on F4's maximal step degree" (pq2 README, corpus node).
Supported wording would be "the iterated closure degree is exactly 4 (solver degree ≥ 4
under the containment property) on 24 fresh instances at n = 17 to 24". The two
re-rendered corpus PDFs were not checked.

All 120 items match except these three. `replay_certificate.json` and `controls.json` are
unchanged; re-running the controls against the modified `recompute.py` gives a
byte-identical `controls.json`.

## Third pass: R4-013 corrected (commit `3cb90671`)

`3cb90671` rewrites R4-013 in `lean-round4-20261104.json`, `WEAKNESSES.md` and
`weakness.json` to "the iterated closure degree is exactly 4 (solver degree >= 4 under
the containment property) on 24 fresh instances at n = 17 to 24", and re-renders the two
corpus PDFs. The same commit holds the second-pass files, byte-identical to what the
validator wrote (`validator_report.json` `631f8c66…`, `recompute.py` `096be45f…`,
`README.md` `7451a4ed…`). No census data, results, source or crosscheck file changed.
`replay_certificate.json` therefore stands at this commit.

    python3 -I -B validation/recompute.py --commit 3cb9067133968daba8340c6cd4e2a46819ee6672 --claims corrected

R4-013's degree-4 clause is now checked in whichever wording a document carries. Run at
`ea2fde0e`, the script still reproduces the second-pass items exactly (names, claimed
and recomputed values), apart from a longer note on two items. At `3cb90671` the new
clause gives, in each of the three files:

| item | claimed | recomputed |
|---|---|---|
| iterated closure degree exactly 4 on fresh instances (iter3 < k, iter4 = k) | 24 | 24 |
| solver degree ≥ 4 under containment on fresh instances (iter3 < k) | 24 | 24 |

**Result: pass, 123 of 123 items, 37 files pinned, no working-tree differences.** No
sentence in the checked text remains unsupported. Remaining conditions, which do not
fail the report:

* R4-013's first clause ("the solver degree exceeds the first fall degree 2 on the 111
  instances with n <= 44") states no condition, unlike its second clause. As an F4
  bound it holds only under the containment property, which the corpus node's scope
  states. For the one-shot Caminata–Gorla solving degree it holds on all 119 instances
  via low2 < k and the Lean `SubspaceCriterion`. Neither Lean claim was reviewed here.
* `pq2_rank/README.md` and `report.typ` still call containment "not formalised", while
  `F4Containment.lean` now carries pending pcc claims. That wording is conservative, not
  unsupported.
* Not checked: `f4/` (msolve still running; held for a later dispatch), the
  re-rendered `attack-graph.pdf` and `overview.pdf`, and signing.

## Final pass: msolve F4, degree 4 to n = 27, round-5 documents (commit `8d9fdc45`)

    python3 -I -B validation/recompute.py --commit 8d9fdc45be3678dc98b1044e9d8250aca1073ce7 --claims final
    python3 -I -B validation/replay_compare.py --commit 8d9fdc45be3678dc98b1044e9d8250aca1073ce7

**Binding.**

* `8d9fdc45` contains the third-pass files byte-identical to what the validator wrote.
* `src/` is unchanged since `f96f4f4c`; the binary hash is unchanged (`a6d1f253…`).
* msolve is `0.6.5-1build2`, `/usr/bin/msolve`, sha256 `c2722288…`.
* The tree was clean and nothing else was running. Validator processes used at most two
  cores (`taskset -c 2` and `-c 3`).

**Result: `validator_report.json` fail, 4 of 195 items; `replay_certificate.json` verified (31 run entries, 0 mismatches).**
Every number in the new text recomputes exactly. The four failing items are all the same
interpretive claim, the bounded-degree reading (F4 below).

### What the validator re-derived

| check | result |
|---|---|
| main stream n = 8..34 regenerated with the census arguments | 81 lines equal `results/run.log` and `f4/dump_run.log` |
| `f4/dumps` byte-identical to the validator's regeneration | 81 of 81 |
| dumped equations equal the Weil descent of S3 (validator's own field arithmetic) | 81 of 81: brute-force ANF over 2^N for n ≤ 24; exact weight-≤2 interpolation and 500 random points for n = 25..34 |
| validator's degree-3 closure = Rust iter3, main stream | n ≤ 34 now (adds 30 instances at n = 25..34, all iter2 = iter3 = 1) |
| msolve replay with the validator's own input writer and output parser | 69 of 81 main-sweep instances (all n ≤ 28; one each at n = 29..34); steps, maximal step degree, basis size and linear count equal the producer's rows on every one |
| msolve quotient dimension = \|Z\| and linear elements = k, basis vanishes on Z | on every replayed instance (independent check that Z is complete) |
| msolve degree = census d_iter where exact (n ≤ 16) | 27 of 27 (3 on 24, 4 on 3) |
| msolve maximal step degree, n = 17..34 | 4 on every replayed instance |
| fresh n = 25..27 regenerated; parameters and non-iter4 values equal `results/iter4_n25..27` | 9 of 9 |
| validator's degree-4 closure on the fresh n = 25..27 instances | iter3 = 1 < k and iter4 = k on 9 of 9 (with the earlier 24: 33 of 33 to n = 27) |
| msolve on the fresh n = 25..27 instances (not claimed by the producer) | degree 4, quotient dimension = \|Z\|, linear = k on 9 of 9 |

### Headline items from the documents (claimed parsed from the text, recomputed from the commit)

| statement (where) | claimed | recomputed |
|---|---|---|
| fresh n = 25..27 same outcome; d_iter = 4 on 33 of 33 fresh (pq2 README, census README, report.typ, triage) | 9; 33 / 33 | 9; 33 / 33 |
| msolve on the main sweep's own instances, n = 8..34 | 81 | 81 |
| dump log equals `results/run.log` | 81 | 81 |
| linear elements in msolve's reduced basis = k | 81 of 81 | 81 of 81 |
| msolve degree = d_iter where exact (n ≤ 16) | 27 of 27 | 27 of 27 |
| census d_iter > 3 and msolve below 4 | 0 | 0 |
| msolve maximal step degree n = 17..34 (all documents, MEAS-PQ2-MSOLVE-F4) | 4 on all 54 | 4 on all 54 |
| MEAS-PQ2-MSOLVE-F4: degree 3 on 24 and 4 on 3 of the 27 with n ≤ 16 | 24, 3, 27 | 24, 3, 27 |
| R5-003: never below the census bound; matches where exact | 0; 27 | 0; 27 |
| truncated row recorded 0 linear; rerun gives k and takes precedence | 0; yes | 0; yes (`msolve_all.json` = precedence merge on 81 of 81) |
| three instances rerun after a restart | 3 | 3 per-instance rerun files (the restart itself is not verifiable) |
| R4-013 (3 copies): under containment F4's degree exceeds 2 on the n ≤ 44 instances | 111 | 111 |
| R5-002 (3 copies), Direction bullet: degree one above the Kousidis–Wiemers 3, constant over n = 17..34 | 1; 1 | 1; 1 |
| Lean README: round-5 pcc claim records present | 2 | 2 |
| **pq2 README Direction: "This is evidence for the bounded-degree premise … not against it"** | supported | **not supported** |
| **R5-002 (`lean-round5-20261107.json`, `WEAKNESSES.md`, `weakness.json`): "the bounded-degree premise holds on this range"** | supported | **not supported** |

### F4: the bounded-degree reading is an overreach

The measurement is sound: msolve's maximal step degree is 4 on every instance from
n = 17 to 34. The reading drawn from it is not supported, for three reasons.

1. The repository's own record of this family shows the next step. The corpus node
   `MEAS-GB-KW-TABLE1` (`docs/ecdlp-hardness-corpus/lit-groebner.json`) gives Kousidis–Wiemers'
   Magma data for the same construction: m = 2, random curves, random n′-dimensional
   factor-base subspaces. Their highest step degree is 4 for n ≤ 36 and 5 for n = 37..48.
   `GAP-GB-SUMMATION-DREG-GROWTH` and `WEAKNESS_TRIAGE.md` ("degree 4→5 at n = 36→37") cite
   the same step. The msolve window stops at n = 34. It agrees with Kousidis–Wiemers where
   they overlap, and it cannot see the rise they report three steps later.
2. Within the measured range the degree did grow. On these same instances msolve gives 3
   at n ≤ 14 and n = 16, and 4 at n = 15 and from n = 17. "Does not grow" holds only on the sub-window
   after that step.
3. A two-fold window in n cannot tell a bounded degree from a slowly growing one. The
   repository's fall-degree study prefers a log₂ n fit on its m = 2 data
   (`research/fall_degree_bounds_20261008/RESULTS.md`). A premise about all n cannot
   "hold on a range" (R5-002). The pq2 README's own Reading bullet makes the matching
   point: "a finite range cannot refute an o(1) statement".

**On "consistent with the bounded-degree premise"** (census README, `report.typ`,
`WEAKNESS_TRIAGE.md`). Read strictly, it is true of the n = 17..34 window, but empty: no
finite table is inconsistent with a bounded degree. Without the Kousidis–Wiemers step
beside it, it reads as support, so in context it is an overreach. These sentences are
flagged, not failed: their literal content is supported.

A fair wording for all of them: "msolve's degree is 4 on every instance from n = 17 to
34, one above the Kousidis–Wiemers value 3, and matches their Magma value 4 for n ≤ 36.
They report 5 from n = 37, so a window ending at n = 34 does not decide whether the
degree stays bounded."

**Not checked in this pass:**

* `F4Containment.lean` and whether msolve fits the abstract F4 model; the documents state
  the latter as a reading, not a theorem.
* msolve on 12 main-sweep instances with n ≥ 29 (two each at n = 29..34). Their
  degree, linear count and the completeness of Z rest on the producer's rows and the Rust
  enumeration.
* The re-rendered PDFs, SVGs and DOT graphs, and the producer's session log.
* The pcc standing of the round-5 claims (only their records' presence).

## Reproduce

From `research/ecdlp_weakness_census_20261104/pq2_rank` (each step pinned to one core):

    python3 -I -B validation/recompute.py      # first pass (f96f4f4c, original claims)
    python3 -I -B validation/recompute.py --commit ea2fde0e4ded41a08ed5d966c96f7383d61ab802 --claims corrected
                                               # second pass
    python3 -I -B validation/recompute.py --commit 3cb9067133968daba8340c6cd4e2a46819ee6672 --claims corrected
                                               # third pass
    python3 -I -B validation/recompute.py --commit 8d9fdc45be3678dc98b1044e9d8250aca1073ce7 --claims final
                                               # final pass (binding); writes validator_report.json
    python3 -I -B validation/msolve_check.py ...  # msolve replays; exact commands in replay_certificate.json
    python3 -I -B validation/replay_compare.py --commit 8d9fdc45be3678dc98b1044e9d8250aca1073ce7
    python3 -I -B validation/controls.py       # writes controls.json
    python3 -I -B validation/replay_compare.py # writes replay_certificate.json (needs the run directories)

The run directories hold every command's `run.log`, JSON, CSV and dumps. The exact
commands and the SHA-256 of every output are in `replay_certificate.json`.
