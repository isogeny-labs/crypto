# Protocol: bitop/v1 completeness of the symmetrised oracle, and a Koblitz exponent ladder at one base size

Frozen 2026-10-21, before any data in `sessions/`. Two independent
questions share this directory because they were raised in one task; each
has its own spec files, its own runs and its own decision rule. Neither is
a security claim and neither is an improvement claim (AGENTS.md "Success
criterion": accounting changes are not improvements).

## A. Does the symmetrised oracle price as complete in `bitop/v1`?

**Question.** After PR *ic-acct-7* (counters for the `koblitz-symmetrised`
base build, the oracle's `prepare`, the symmetrised system build and the
degree scan; `src/cryptanalysis/ic_framework/plugins.rs`,
`src/cryptanalysis/koblitz_symmetrised.rs`,
`src/cryptanalysis/koblitz_groebner.rs`, `src/cryptanalysis/ecbench/work.rs`),
does every `ic.pipeline_full` record with `oracle=symmetrised` price with
`complete: true` — nothing in `unpriced`, nothing in `known_uncounted` —
under `ecbench work --counted`?

**Spec.** `specs/koblitz-symmetrised-n17.json`: one curve, `K_1 / F_{2^17}`
(the smallest Koblitz size in the repository's roster whose `x^n − 1` has a
proper Frobenius-invariant subspace containing 1: `n = 13` and `n = 19`
have `2` primitive, so their only invariant subspaces containing 1 are
`F_2` and the whole field; `K_0 / F_{2^17}` has no prime-order subgroup in
the constructor). Divisor `0;1` gives `V` of dimension 9, as in the
library's own symmetrised pipeline test.

**Arms.** `rho-frob` (`rho.signed_frobenius_full`, reference) and
`ic-symmetrised-d9` (`ic.pipeline_full`, `koblitz-symmetrised:divisor=0;1`,
`symmetrised:m=2,divisor=0;1`, `targets=walk`, default engine
`inherited-f4`, Macaulay cap 3, node budget 4096).

**Fixed parameters.** 4 public targets, 2 rounds plus 1 warm-up round,
alternate order, L0 isolation accepted, `timeout_seconds` 3600.

**Procedure.** `ecbench run`, then `ecbench verify --replay 2` (two
measured runs re-executed and required to reproduce every count), then
`ecbench work --counted` over the session.

**Decision rule.** Pass if every counted record of `ic-symmetrised-d9` has
`complete: true` with empty `unpriced` and empty `known_uncounted`, and the
verify receipt reports zero problems. Any record that is not complete
fails the question and the report names the counter or item that blocks
it. The rho arm's completeness is reported, not judged (it is not what
changed). No speed comparison is drawn from this session: four targets at
17 bits support no interval worth stating.

**Replay guarantee.** Every new counter lives in `PhaseRecord.work_native`
or in the solver block's `bitop_*` keys, which replay ignores; `native`
counters are untouched, so committed sessions replay exactly as before.

## B. Exponent of the Frobenius-orbit arms at one base size across n = 13, 19, 23

**Question.** `research/bitop_unit_20261016/specs/koblitz-a0-n{13,19,23,31}.json`
measured each size with its own base size (`columns = 1, 2, 3` for the
compact orbit scan; `dimension = 4, 6, 8` for the negfold subspace), so
each of those sessions supports a constant-only bound at its size. What
exponent does each arm have in `log2 r` when the base size is *held
fixed* across three sizes of one tier?

**Specs.** `specs/koblitz-exponent-n13.json`, `…-n19.json`, `…-n23.json`:
`K_0` over `F_{2^13}`, `F_{2^19}`, `F_{2^23}`. All three fields are at most
32 bits, so they share the `toy` tier of
`src/cryptanalysis/ecbench/bounds.rs::tier_of_bits`, which is what lets
`ecbench bound fit` take the three sessions together (it refuses a fit
whose sizes span tiers). `n = 31` is left out on purpose: `2^31 > 2^30`
would still be `toy`, but its runs are hours, not minutes, and three sizes
already determine a slope with an interval.

**Arms** (identical in every file; the same methods and parameter strings
as the 2026-10-16 specs, only the base size pinned):

| arm | role | method | fixed parameters |
|---|---|---|---|
| `rho-frob` | reference | `rho.signed_frobenius_full` | — |
| `ic-orbit-c2-renew` | candidate | `ic.pipeline_full` | `compact-orbit-scan:columns=2,raw_x_cap=100000000`, `mitm-frobenius-counted:m=2`, `targets=stride-renew` |
| `ic-orbit-c2-walk` | baseline | `ic.pipeline_full` | the same base and oracle, `targets=walk` |
| `ic-negfold-d6-renew` | baseline | `ic.pipeline_full` | `koblitz-standard-subspace:dimension=6`, `pdp3-koblitz:m=2,engine=rootfind-negfold`, `targets=stride-renew` |
| `rho-frob-aa` | control | `rho.signed_frobenius_full` | an A/A copy of the reference |

The one base-size choice is the middle of the three the 2026-10-16 specs
used (`columns = 2`, `dimension = 6`), so the `n = 19` point coincides with
an already-measured configuration and the other two sizes move only `n`.
`max_trials` stays `1000000000`; a fixed base at `n = 23` needs more trials
per relation than a base sized for it, which is the point.

**Fixed parameters.** 16 public targets per size, 2 rounds plus 1
warm-up, alternate order, L0 isolation accepted, `timeout_seconds` 3600;
target and measurement seeds differ per file and are in the files.

**Procedure.** For each file: `ecbench run`, `ecbench verify --replay 2`.
Then, per arm, `ecbench bound fit --dir <the three sessions> --arm <arm>
--unit bitop/v1 --audit <the three receipts>` (one tier, so no `--tier` is
needed; the fit refuses a span of tiers), which reports the constant and
the exponent each with a 95 % bootstrap interval, and `ecbench table` per
session against `rho-frob`.

**Decision rule.** The result of each arm is its fitted exponent in
`log2 r` with its 95 % interval, as `ecbench bound fit` reports it, and
nothing beyond that. In particular:

- no arm is called better or worse than rho from these runs; the table's
  gap to the matched rho is reported as the distance remaining, per row;
- two arms are ordered only if their exponent intervals are disjoint, and
  then only as "steeper" or "flatter" at this base size in this tier;
- an interval that includes the generic `1/2` is reported as consistent
  with generic behaviour at a fixed base, not as evidence of it;
- the A/A control's paired interval must contain 1; if it does not, the
  session is reported as contended and no exponent is stated from it.

**Not run here.** These three specs are frozen, not executed, in this PR
(task scope). The sessions, their reports, graphs and PDFs
(`research-visuals`) belong to the PR that runs them.
