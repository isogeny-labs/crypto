# Resume: crypto @ codex/ghs-strong-rho-results-20261114

## Goal

Establish, with admitted end-to-end measurements, whether the historical
plane-quartic index-calculus result on the restricted GHS-weak
`E0/F_{q^3}` family remains below a newly optimized, same-target strong
Pollard-rho control. Reconcile the comprehensive ECBench frontier report and
generated leaderboard only if the validation gates permit it.

## Lead / hypothesis

Historical N4-v2 evidence places its best double-large-prime IC arm far below
the older serial negation rho at large q. PR #4171 added a materially stronger
rho implementation: six-product cubic multiplication, negation quotient,
32 partitions, eight lockstep walks and Montgomery batch inversion. The
frozen hypothesis is in the study protocol; this round tests whether the
restricted-family exception survives that stronger comparator, without
generalizing it to generic or standard curves.

## Spec

`research/quartic_ghs_strong_rho_20261114/PROTOCOL.md` plus prospective
duration-only amendment
`research/quartic_ghs_strong_rho_20261114/ROUND_A.md`. Grid q = 4,051 through
4,194,271; seeds 1,2,3; `strong-b8` primary; exact historical IC targets;
five repetitions through q = 262,111 and three thereafter; weights 0,0.1,1;
10,000 clustered bootstrap resamples; strongest same-target reference and
all conjunctive admission gates retained.

## State

Implementation/protocol PR #4171 merged at
`352dd51da0f7519e25e32256213cad90c246d486`; required checks passed. No primary
or paid round has run. The selected runtime has no AWS/checkpoint identity;
live budget headroom and all-tenant pool state are unavailable. Historical
archive SHA-256 is
`aac9a25ce50ac5d23895f0dba8f907fb223aaac32a2a41391bc590e3c6bcca42`.
The existing frontier/leaderboard is unchanged.

## Next steps

1. Finish the round-A launch package, analyzer/validator, problem record and
   report/PDF update in this worktree.
2. Run repository validation, commit, push, run `check-experiment-git`, and
   open a normal PR with auto-merge.
3. After merge, from an approved operator context, record live STS, Budgets,
   quota, all-tenant pool, and active-Job receipts. Stop if any gate fails.
4. Publish the exact custom image by digest, commit/push its four request
   manifests, checkpoint again, and submit low/mid/high/top with zero retries.
5. Retrieve raw S3 objects, verify manifests, run independent analysis/replay,
   create bench/PCC records, and update report/leaderboard only if admitted.

## Runs in flight

None. Optional CI jobs on already merged PR #4171 are monitored separately;
they are not scientific runs.

## Stop and success conditions

Hard all-attempt ceiling USD 5, at most 8 requested vCPU and 8 GiB concurrently
in low+mid then high+top waves (16 vCPU/16 GiB across all four Jobs), zero
automatic retries, deadlines 3,600/3,600/7,200/10,800 seconds for
low/mid/high/top. Stop before spending on unknown/stale budget, pool conflict,
checkpoint/Git/source/image mismatch, or missing raw-result custody. A complete
measurement requires every primary row correct, exact repetitions/targets,
batch eight, rho steps/sqrt(ell) in [0.75,1.15], replay match, independent
recompute, and full artifact manifests. H1 succeeds on the measured grid only
if every size is admitted and every 95% upper IC/rho bound is below one at all
three weights; otherwise report the frozen rejected/limited/unresolved verdict.

## Open questions

Whether an approved operator identity with current budget/pool visibility is
available, and whether the optimized rho records remain inside the preregistered
step band at every size.
