# Independent prime-field S3 arithmetic validation

This is a stronger check for item13 on the previously recorded S3 prime
population, not a new experiment on S4/S5 or binary formulas. Implement
field/group arithmetic separately in Rust, then compare every aggregate
against the earlier C++implementation. The expected C++data have already
been read; this is disclosed known-answer replication, not a blinded
discovery. Both local Rust and a separate standard GitHub-hosted runner
are planned. No separate-host success is asserted before actual job output.
[pcc:unverified]

Scope is all nonsingular short models y²=x³+ax+b over p5,7,11,13,17,19,
and every ordered base-field x-coordinate triple. The reference records
942models,4188162tuples and244236zeros; its file is pinned by SHA256.
Rust directly evaluates S3 and independently constructs the relation table
from signed point sums in Fp². It shares no arithmetic helper with the
earlier C++code or the corpus's old Rust validator. Exact aggregate equality
is the criterion; both false/missing counters must be0. [pcc:unverified]

Quadratic representation is u+v*z with z²=d. Choose the first nonsquare
d≥2 by exhaustive base-field tests. Check every nonzero extension inverse,
every base-field square lift, every input/sum point's curve equation, and
all nonsingular-model counts. Repeated coordinates and 2-torsion follow
the actual affine formulas and are included. No group table is assumed.
[pcc:unverified]

Lean foundation proves actual AdjoinRoot irreducibility/dimension/cardinality,
that all base elements acquire square roots in a quadratic extension of
an odd finite field, and the Mathlib point-sum/S3 oracle equivalence. A
kernel-computed control has54zeros for all6short models over F3.
These mathematical statements do not prove Rust compilation, execution
attestation, or physical host independence. They preserve those boundaries
rather than replacing them with axioms. [pcc:unverified]

Preflight is only p3 and the larger held-out p23, outside the six-field full
Rust population. Raw outputs, source/binary hashes and budget clocks are
retained. No full Rust call precedes the frozen JSON record and its commit.
Build/metadata syntax checks are ordinary preflight validation. No statistical
power, random seed, warmup, performance claim, A/A variance or sampled null
population is involved; both arms evaluate a complete known-answer population.
[pcc:unverified]

Compile from the same frozen source separately on each host; compiler,
source/binary digest and OS/kernel/CPU are recorded before native execution.
Binary digests cannot be fixed for an unvisited host, so those protocol
fields are null and explicitly measured. Source, build, driver, comparator,
workflow and known-answer bytes are frozen. One local call and one manual
remote job; no outcome-based retry. Compilation is capped60s, native call30s,
remote job5minutes. An API/eligibility rejection is not-completed remote
validation and is retained rather than rerouted through a trusted verifier.
[pcc:unverified]

Use only standard ubuntu24.04 public-repository GitHub runners, which
official billing documentation says remain free. Repository visibility
was checked public. Workflow is manual-only under AGENTS.md: no PR trigger,
EKS, larger/GPU runner, secrets, cache or artifact upload. Remote evidence
is the public job log. Do not dispatch if the repository is private or the
runner choice/budget contract changes. If GitHub requires the new workflow
on main, retain that actual blocker; do not weaken another workflow to
bypass it. Approved local checkpoints apply to local files. [pcc:unverified]

Sources checked (KB): unavailable; use existing corpus source/Lean review.
The original Semaev ePrint PDF access remains403, not newly read. Primary
formula/pair-sum proof is the pinned S3Roots/S3Completion formalization;
the full mathematical equivalence retains characteristic and curve-domain
conditions. Finite-field square criterion and AdjoinRoot basis APIs are
from pinned Mathlib. Official billing source is recorded in provenance.
Canonical corpus promotions and signed execution standing remain outside
this unsigned replication. [pcc:unverified]
