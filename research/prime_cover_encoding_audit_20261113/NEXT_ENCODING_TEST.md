# Proposed matched direct-versus-graph cover test

Unexecuted design, not spending authorization or an asymptotic result
[pcc:unverified].

Question: does a graph encoding reduce actual sparse work enough to offset its
extra variables on exactly the same polynomial-cover inputs?

Competing hypotheses: H1, direct degree expansion causes avoidable work; H0,
added variables/critical pairs eliminate the benefit. The frozen round5 source
and degree trace motivate this test; they do not decide it.

## Before a solver run

Implement a native builder using literal saved p, curve, coset, map and target
parameters. Build the direct and graph representations from the same input
object; do not draw new curves/targets by advancing a shared RNG differently.
For cubic N, add x_i-N(g*y_i0)=0 and use the ordinary S3 chain in x_i. Retain
all original tower and partial-sum variables. Proposed m4 counts are22
variables/23 equations and maximum raw degree4; verify these in the actual
builder rather than treat them as a result [pcc:unverified].

The generic graph solution maps are formalized separately. Connect them to
actual polynomial evaluation, verify both directions and unique auxiliary
coordinates, and check rational lifts, signs and partial-sum exceptional charts.
Recover/log a genuine known-answer witness using the frozen native generator;
the original saved planted_ok boolean is insufficient by itself.

## Correctness and cost panel

- Freeze equivalent direct/graph inputs at small depths before the large cell;
  use the identical curve, map, coset and target in each representation.
- Keep satisfiable and naturally unsatisfiable rational targets distinct. A
  planted-only positive comparison does not estimate online relation yield.
- Hold solver version, ordering policy and resources fixed; preserve added
  variable count and all sparse matrix rows/columns/nonzeros. Match outputs.
- Record total solve/output/verification work, not only maximum degree or a
  full-space monomial proxy. Compare the fastest applicable external solver
  on the same polynomial input and host before a fastest claim.
- For a relation-finder experiment, generate targets online with masked Q;
  charge base construction, all attempts, signs, rejected candidates and
  useful independent output. Require an exact final scalar check and matched
  one-target reference under the standing protocol [pcc:unverified].

## Occupancy and sampling

Generate the common curve before independent map streams. Record image size,
usable rational abscissas, collisions and setup rejection costs. The observed
6-versus8 occupancy is one pair of fixed cases, not a generic map advantage.
Use matched-occupancy controls only as explicitly conditioned controls; retain
unconditioned attack-arm draws and charge their setup. Do not divide work by
an assumed number of useful relations.

## Resource and decision rules

No solver/timing job is currently admitted. Restore budget/resource admission
first under the repository run skill. Freeze host inventory, thread policy,
memory/wall caps and panel before execution; perform a bounded feasibility
cell before requesting large memory. Preserve all OOMs/timeouts as censored.
The original cubic needed about190GB according to its producer; added variables
are not a reason to assume that the graph cell fits the same memory.

For timing claims, use the same isolated allocation/resource envelope for both
representations and retain concurrent-process/CPU/memory records. Reusing the
old mixed-concurrency timing as a matched comparison is invalid. W-only
observations remain stage diagnostics; no leaderboard or algorithm claim
without complete validation/replay.

A useful representation gain would justify the next growing-(m,t) panel.
It would not prove sub-exponential ECDLP: that requires a uniform public-input
finder bound with all bit costs, arity symmetries, rationality, useful yield,
whole-call floors and linear algebra included [pcc:unverified].
