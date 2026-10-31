# Preregistration: do the accounting v2 ids and the versioned floor do what PR #2465 says they do?

Frozen before any data of this study was taken (see `git log` for this file's
commit and the sessions' `started_unix_ms`). Written 2026-10-23 for
aburan28/crypto PR #2465 (branch `ecbench/fix-accounting`, head
`7ed39a752b6cd9cd62f015ca9416a8af85e3dc5b`). Classification per the
`controlled-experiment` skill: every claim below is an **exact comparison of
recorded counts** (record fields are equal or not, a ratio of totals is above
or below a number); wall time is **descriptive** and is never a result here.
This study is an **accounting / instrument correction check, not an
algorithmic improvement**: AGENTS.md's success criterion says relabelling and
accounting changes are not improvements, and nothing here is reported as one.

## What the PR claims (the claims under test)

The PR adds method ids with corrected accounting (`rho.cubic_v2`,
`rho.quartic_v2`; Brent cycle detection and lost-walk counters in
`rho.negation_v2` and `rho.signed_frobenius_v2`; the scaled
`rho.signed_frobenius_strong_v2`; one charged final verification in
`bsgs.*_v2`, `kangaroo.vow_v2`, `claw.pair_table_v2`, `ic.large_prime_v2`) and
a versioned automorphism-aware floor (`boundaries.floor_rule`
`automorphisms/v2`: A = 6 on j = 0, 4 on j = 1728). It quotes figures from
scratch runs that were not committed: on the j = 0 check, `rho.cubic_v2` at
1.514x the A = 6 floor against 0.91x for `rho.cubic` on the old A = 2 floor;
and the strong reference, frozen to v2, mean S and set-up share at n = 13, 19,
37: 14.0 -> 2.55 / 84% -> 38%; 2.95 -> 0.68 / 77% -> 31%; 0.293 -> 0.209 /
31% -> 4.5%. The author of this study knows those figures (they are priors).
They are not targets: measured figures are reported as measured, and any
quoted figure that differs from the measured one by more than 15% (relative)
is reported as differing.

## Questions, outcomes, falsifiers

**Q1. j = 0 floor.** On the j = 0 prime curve (y^2 = x^3 + 2 over
F_4194397, r = 1048627, `check-prime-j0-floor-v2.json`), does `rho.cubic_v2`
read at or above 1.0 of its (v2) floor?

* Measure: mean `ratio_to_floor` over the measured runs of each arm (n = 6
  targets x 4 rounds = 24 per arm), and the paired ratio of S with the
  harness's interval (`ecbench compare`, `ecbench table`).
* Outcome O1: `rho-cubic-v2` mean S / floor >= 1.0, with its interval
  excluding values below 1.0. Prior: about 1.5.
* Falsifier: mean below 1.0, or an interval that reaches below 1.0. Then the
  floor rule or the cubic walk is wrong or the set-up charge is not what is
  recorded, and the PR's headline is not supported.
* Also reported (secondary, no falsifier): `rho.cubic` (frozen) on the same
  workloads and seeds under the v2 floor; `rho.cubic` on the main binary under
  the old floor (A = 2, `frozen-j0.json`), which is the reading the review
  gave (prior 0.91x); and the paired ratio `rho.negation_v2` / `rho.cubic_v2`
  (prior near sqrt(3) = 1.73, the ratio of the walk's automorphism counts).
* Floor values: the record carries `floor_s`; the expected values are
  sqrt(pi/12) = 0.5117 (A = 6, v2) and sqrt(pi/4) = 0.8862 (A = 2, first rule);
  a record that carries other values falsifies O1's reading.

**Q2. Strong reference, frozen vs v2.** On the same workloads and the same
per-(workload, round) algorithm seed (spec.rs derives one seed per workload
and round, shared by every arm), does `rho.signed_frobenius_strong_v2`
reduce both the mean S and the set-up share against the frozen
`rho.signed_frobenius_strong`, at n = 13, 19 and 37 (Koblitz a = 0)?

* Measure, over measured runs that verified: mean S (the mean of the
  records' `cost.s`, i.e. `Sum S / n`); set-up share as `Sum(setup +
  automorphism_setup phase gae) / Sum(total_gae)` and, as a second figure,
  the mean of the per-run shares (the PR's `detail.setup_share` definition,
  `methods::setup_share`); the paired ratio of S v2 / frozen and its interval
  (`ecbench compare`). The frozen id records no `detail.setup_share`, so its
  share is computed from its phases by the same formula (derived, labelled).
* Specs: committed `claim-koblitz-strong-v2-n13.json` and `-n19.json`
  (8 public targets, 3 rounds, unchanged) and the new `strong-n37.json` (same
  arms minus the index-calculus arms; 8 public targets, 4 rounds).
* Outcome O2: at each of the three sizes, mean S (v2) < mean S (frozen) with
  the paired ratio's interval excluding 1, and set-up share (v2) < set-up
  share (frozen).
* Falsifier: at any size the ratio's interval includes or exceeds 1, or the
  share is not lower. Reported either way. The sizes are three; no scaling
  claim is made from three sizes.
* Not claimed: that v2 is a faster algorithm. It is a differently sized
  reference whose fixed set-up is a smaller share; the committed table flags
  rows where set-up exceeds 10 % of S or `log2 r` is below the calibrated
  range, and those flags are reported as they appear.

**Q3. One verification more.** For `bsgs.textbook`, `bsgs.interleaved`,
`bsgs.negation`, `kangaroo.vow`, `claw.pair_table` and `ic.large_prime`
against their `_v2` ids, on the same workload and seed, do the counts differ
by exactly the verification phase?

* Specs: `pairs-prime.json` (bsgs x 3, kangaroo; 32 matched pairs each),
  `pairs-koblitz.json` (claw, bsgs.negation; 32 each),
  `pairs-large-prime.json` (ic.large_prime; 16).
* Outcome O3, per matched (workload, round) pair, within 1e-9 relative on
  `cost.total_gae`: (a) `recovered` is equal; (b) every phase of the frozen
  record equals the same-named phase of the v2 record in `adds`, `doubles`,
  `scalar_mults`, `gae` and `native`, except that for `kangaroo` the frozen
  `setup` equals v2 `setup` + v2 `restarts` and for `ic.large_prime` the
  `verification` phases may differ; (c) the v2 record has one more
  `verification` phase (or a larger one for `ic.large_prime`) and
  `v2.total_gae - frozen.total_gae` equals that phase's `gae` (the
  difference for `ic.large_prime`); (d) for bsgs, kangaroo and claw the
  verification phase is one scalar multiplication, `gae = 1.5 log2 r`
  (r = recorded `log2_r`; README section 7).
* Falsifier: any matched pair for which (a), (b), (c) or (d) fails. A frozen
  run that did not verify, or has no v2 partner, is reported, not dropped. The
  claw budget change (the `recover` phase now inside `max_adds`) can differ
  from the frozen id only when a run reaches the budget; a pair where it does
  is reported separately.
* Controls: the A/A arm (`rho-neg-v2-aa`, `rho-frob-v2-aa`, `rho-strong-v2-aa`)
  must equal its reference arm on every count in every pair (negative control:
  the comparison reads equal when nothing changed); the bsgs pairs differing
  by one verification (about 24 GAE at this size) are the positive control (the
  comparison sees a difference of this size). If the A/A arms differ, the
  matching is broken and Q3 and Q4 are invalid, not negative.

**Q4. Frozen ids unchanged.** Are the frozen ids' counts the same on the
branch binary as on the main binary?

* Design: the same spec files (`frozen-prime.json`, `frozen-koblitz.json`,
  `frozen-j0.json`: only frozen ids, so the main binary can run them) run once
  on `./target/release/ecbench` of the branch head and once on a binary built
  from main, then compared record by record on (arm, workload, round, warm-up
  flag).
* Compared fields: `outcome.status`, `outcome.recovered`, `algorithm_seed`,
  `cost.total_gae`, `cost.unpriced`, `phases[].{name, adds, doubles,
  scalar_mults, gae, native}`, `counters`, `run_id`, `boundaries.floor_s` and
  `automorphisms_available/used`. Also `ecbench compare --dir <branch>
  --b-dir <main>` per arm (ratio of S, `same_seeds`).
* Outcome O4: every compared field equal for every matched record, except on
  `frozen-j0` the floor figures (`floor_s`, `ratio_to_floor`,
  `automorphisms_available`, and the new `boundaries.floor_rule`), which are
  expected to change from A = 2 (main) to A = 6 (branch) and are reported.
* Falsifier: any single other difference. Then the PR changes what a frozen id
  counts and the "frozen ids byte-identical" statement is wrong.
* Main binary: `ecbench-base` (sha256
  `a59c4014ff52100ed10c06b5d27a0572f9474d24c137fd18b24798237024f81f`), the
  binary the coordinator built from main `3a1ea541d964d6ef9099300a9ab512d55eaa073b`
  (the PR's merge-base; build log and source tree are in the coordinator's
  scratch, not in this repository, and the binary does not embed its commit,
  so this provenance is the coordinator's word and is stated as such). The
  branch also carries the integrity and runner PRs (#2455 and #2460 stacked
  under it); a difference found would not by itself be attributed to the
  accounting commit.
* Second check: every committed session whose methods are frozen ids and that
  is small enough is replayed by the branch binary (`verify --replay N`);
  replays compare recorded counts, so an identical replay means the frozen
  id still counts what the record says. The set is listed in the report.

**Exploratory (no falsifier).** `rho.negation_v2` against `rho.negation` and
`rho.signed_frobenius_v2` against `rho.signed_frobenius`: the README says the
walks differ only where a cycle longer than the 16-point window occurs; the
fraction of runs with `cycles_longer_than_window > 0` and the S difference
are reported. The section 5a pricing band (`ecbench band`) on `pairs-prime`,
`pairs-koblitz` and `strong-n37`, saved as recorded; arms that name an
uncharged counter without emitting it are lower bounds and are not ranked.

## Specs (frozen; sha256 in `SPEC_SHA256.txt`)

| session | spec | arms x workloads x (rounds + warm-up) |
|:--|:--|:--|
| `j0-v2` | `docs/ecbench/specs/check-prime-j0-floor-v2.json` (committed, unchanged) | 6 x 6 x 4 |
| `j0-frozen-branch`, `j0-frozen-main` | `specs/frozen-j0.json` (new) | 3 x 6 x 4 |
| `smoke-prime-v2`, `smoke-koblitz-v2` | committed `smoke-prime-v2.json`, `smoke-koblitz-v2.json` (unchanged; plumbing only, not used for any outcome) | 9 x 2 x (3+1), 6 x 2 x (3+1) |
| `pairs-prime` | `specs/pairs-prime.json` (new) | 12 x 8 x (4+1) |
| `pairs-koblitz` | `specs/pairs-koblitz.json` (new) | 9 x 8 x (4+1) |
| `pairs-large-prime` | `specs/pairs-large-prime.json` (new) | 3 x 4 x 4 |
| `claim-n13`, `claim-n19` | committed `claim-koblitz-strong-v2-n13.json`, `-n19.json` (unchanged) | 8 x 8 x (3+1) each |
| `strong-n37` | `specs/strong-n37.json` (new) | 5 x 8 x (4+1) |
| `frozen-prime-{branch,main}`, `frozen-koblitz-{branch,main}` | `specs/frozen-prime.json`, `specs/frozen-koblitz.json` (new) | 6 x 8 x (4+1), 4 x 8 x (4+1) |

New spec files are under `specs/`; no committed spec is edited. New specs
use an even number of measured rounds (the committed smoke and claim specs use
3 and the harness warns; their order imbalance is identical for both arms of
every pair and is irrelevant to counts). Committed specs are run exactly as
committed so the committed files are what is tested.

## Run protocol

* Binary: `./target/release/ecbench`, built at head `7ed39a752` (the host
  capsule of every session records `commit` and `binary` sha256
  `e265bbc00a91e8742e50421618c8bb9fe23347b4cba1f1d3dbf14d52954d50b1`);
  the main binary only for the `*-main` sessions.
* Command: `ecbench run --wait --allow-busy --cpus auto --quiet --spec SPEC
  --out sessions/NAME`. The host is a shared 4-vCPU virtual machine with other
  agents building and testing at the same time (load average 3 to 4). Wall
  time is therefore descriptive; the isolation level each record earned is
  reported; operation counts stand at any level (AGENTS.md section 12).
* Order: the sessions of a pair run one after the other, branch before main;
  no ABBA is needed because only counts are compared.
* Audit: `ecbench verify --dir D --replay-all --exit-code` on every session
  with the branch binary; receipts beside the study (`audits/`), never inside
  a session. Then `verify --replay 0 --allow-interrupted --exit-code` on every
  new session (what CI runs).
* Analysis: `ecbench compare`, `ecbench table`, `ecbench band` and `jq` over
  `records.jsonl` (scripts in `analysis/`); Python only plots recorded values
  and renders the PDF. Nothing is edited after a run.
* Dry run: one scratch run of the committed `smoke-koblitz-v2.json` (48
  records, all verified) was made outside the repository to read the record
  layout the `jq` scripts use; none of its numbers is used.

## What does not count

Changing a spec, the matching rule or a tolerance after seeing data; dropping
a run that does not fit; reading a wall-time ratio as a result; reading the
strong v2 reference's lower S as an improvement of any method over rho (it is
a reference sized to r, and a lower mean S at the same walk idea is the
instrument being corrected). Any change is a new, dated amendment, with both
versions reported.

## Scope

Counts on these curves and seeds, on the PR's head. Not shown: any
index-calculus improvement, any claim about sizes between or beyond those
run, any claim about the main binary other than the frozen ids' recorded
counts on these workloads.

## Amendments

**A1 (2026-10-23, after the first batch of sessions had run and before any
analysis of `frozen-large-prime`).** The first batch covered the frozen
`rho.*`, `bsgs.*`, `kangaroo.vow`, `claw.pair_table` and `rho.cubic` ids on
both binaries (Q4) but not the frozen `ic.large_prime`. Added
`specs/frozen-large-prime.json` (same curve, parameters, seeds and arm names as
`pairs-large-prime.json`, frozen ids only) to be run on the branch binary and on
the main binary and compared by the Q4 rule, unchanged. No outcome, falsifier
or earlier spec changes.

**A2 (2026-10-23, after the pair check had run).** Q3 (d) said the verification
phase of the generic v2 ids has `gae = 1.5 log2 r`. The README's wording is
"about `1.5 log2 r`", and the recorded phase is the exact double-and-add count
of the scalar `d` (a whole number of GAE). Outcome (d) as written is therefore
reported as **not met** (it fails for every record), and a replacement (d')
is reported beside it, labelled post hoc: the `verification` phase has
`scalar_mults = 1`, `doubles = bitlength(d) - 1` and `adds = popcount(d) - 1`
for the planted scalar `d` of the workload, and the mean over workloads of
`gae / (1.5 log2 r)` is reported. (d') is not a preregistered test.

**A3 (2026-10-23, after the audits).** `j0-frozen-main` (the frozen ids on the
j = 0 curve, recorded by the main binary under the first floor rule) fails the
branch binary's integrity audit by design: the main binary records
`rho.cubic` as using 6 automorphisms on a curve it records as having 2, and the
branch's audit checks `automorphisms_used <= automorphisms_available`. The
session is therefore kept as a verbatim archive (`archive/`), not as a session
directory the CI audit would pick up, and its audit receipts (the branch
binary's failing one and the main binary's passing one) are committed.

**A4 (2026-10-23, after the first j = 0 sessions and their audits; before the
second j = 0 sessions).** The j = 0 curve of Q1 (y^2 = x^3 + 2 over F_4194397)
is not in `docs/curves/registry.json`, which CI's `names` check
(`scripts/check_curve_names.py`, AGENTS.md section 11) requires of every slug
written in tracked files. Q1 and the `frozen-j0` cross-binary check are
therefore re-run on the registered j = 0 curve `icv1-fp25-t9734-bcfe9f51`
(y^2 = x^3 + 1 over F_23736841, p = 1 mod 3, #E = 23727108 = 12 x 1977259,
r = 1977259 = 1 mod 3, generator from the registry, `ecbench plan` reports
`registered=true`, A = 6). The PR's spec `docs/ecbench/specs/check-prime-j0-floor-v2.json`
(added by the unmerged PR) now names this curve; `specs/frozen-j0.json` names
it too. Outcome O1, its falsifier, the secondary figures and the prior for the
old-floor reading are unchanged, and the floor values are the same (A = 6:
0.5117; A = 2: 0.8862). The first run (r = 1 048 627, 24 runs per arm: `rho.cubic_v2`
S / floor 1.514, frozen `rho.cubic` 0.911 on the old floor) is kept, unedited,
under `archives/fp23-first-run/` and reported beside the second as a replication
on a second curve; the choice of the new curve was made before its data and
the first run's numbers were known when it was made.
