# Preserve cheap refutations on small systems

The initial native source0acfc55b7f improves the original16/22-variable
complete-root calls (n23 planted F4/F5 paired ratios10.937/10.914), but its
full ecbench standard-subspace panel regresses. Candidate/baseline pipeline
wall ratios are2.267/2.195 at n17 and6.574/6.215 at n23, with40 admitted
pairs per engine and replay audits OK. Every before/after query/answer/
relation-count check agrees. These regressions remain evidence, not wins.

For the n23 target0/round1, F4 executes135075 solver calls in both arms.
Baseline uses79417986 word XORs; the all-width top-degree policy uses
3000248688. Only17 branches split and424 Gray assignments occur. The
extra degree-three root matrices, not tail enumeration, explain this
specific deterministic-work increase. Native timings remain separate.

Before editing the next candidate, freeze the change: retain the ascending
degree ladder for original systems below16 variables, while retaining all
prepared packing/layout/linear-tail policies. Original16/22-variable cases
keep their existing top-degree schedule. No degree cap, solver budget,
output cap, field, public target, replay or admission rule is relaxed.
The bounded quadratic tail remains exactly the same after root reduction.
The corrected policy has distinct identity-bound names adaptive-prepared
and adaptive-quadratic-tail. The original prepared/quadratic-tail names
retain their all-width scheduling for exact historical replay. Candidate
oracle parameters in new full specs change only that policy name; target,
basis, rounds, budgets, reference and A/A methods stay identical.

Local screening checks the explicit degree ladder and exact old/new roots,
plus both original fixture widths' deterministic counters against frozen
source71c4c99d85. A small full-pipeline screen uses the same standard bases
and target0, one warmup/one measured round, baseline/candidate F4/F5, L0.
It is correctness/work screening only, never an admitted wall result.
Require verified outputs, exact baseline/candidate query/rank/answer checks,
no exhaustion, and reduced small-system word work versus the failed policy.
Preserve all failures and all previous source contracts and run reports.

After passing, freeze source/binary hashes and rerun the entire original
native32-pair/72-cost-cell panel, including holdouts and four-worker
controls, and the unchanged11-arm full ecbench specs on fresh c7i hosts.
Use the same native A/A/paired gates and ecbench replay/admission rules.
The already running source71c4c99d85 jobs become iteration2 evidence and
are never relabelled as measurements of this scheduling correction.

The initial local work screen preceded policy versioning. Its16 sealed L0
records and original audit remain unchanged; frozen-adaptive-screen contains
the exact guarded source used. Weekly CI replays those two sessions with
that frozen source, while all versioned sessions use the current binary.
