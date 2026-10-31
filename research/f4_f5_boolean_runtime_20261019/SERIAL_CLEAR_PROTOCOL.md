# Follow the retained one-worker F4 profile

Observed source: `records/f4-candidate-hot-functions.txt`, from the corrected
iteration1 Callgrind batch. The one-worker path still enters Rayon parallel
iterator plumbing for large row-clearing work. This is a candidate overhead,
not a measured native speed claim. Earlier vector-table results were neutral.

Hypothesis: opt-in `KIC_GF2_SERIAL_CLEAR=1` uses the existing serial iterator
when the current Rayon pool has exactly one worker. Leave multiworker pools,
row order, selected table entries, XOR work and default behavior unchanged.
Read the switch once per process, following the existing GF2 switch contract.
Reuse the existing varied-rank RREF/echelon/reference tests in separate
processes with the switch disabled and enabled before native measurement.

One dedicated ephemeral c7i.2xlarge, TTL90m, full checkout, on-demand with
same-type Spot fallback only if launch capacity refuses. Compile once, keep
executable/source/host manifests,60-second cooldown. Same unchanged isolation
and bounded first-eligible-of-three admission. No local native timing.

In the same binary toggle serial clearing0/1. Keep single-write/vector table
switches1 in both arms so this is an isolated incremental change. For each
F4/F5 matrix, n17 seed230 and n23 seeds230/231/232, use five baseline A/A
pairs and five interleaved A/B pairs at RAYON_NUM_THREADS=1. Also use five
A/A and A/B pairs at four workers on n17/n23 seed230 as a negative control:
the new dispatch must retain the original multiworker path. Reserve a full
physical core for one worker, and two physical cores with their SMT siblings
for the four-worker control. Record those actual CPU sets, not an assumed
topology. Verify identical raw/canonical outputs, rank, XOR and profile work.

Analysis reuses the exact five-pair3125-resample CI and max(2%, twice median
absolute A/A noise) gate. Missing pairs are inconclusive, failures retained.
Do not pool hosts or multiply earlier gains. This round measures matrix
calls, not complete recursive root solves; no whole-solver improvement is
inferred. Final18-arm solver costs remain scoped to their earlier exact
source/flags. The new switch stays opt-in; report neutral or slower outcomes.
