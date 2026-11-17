# Paired optimization qualification

Same dedicated-host/isolation/output requirements as PROTOCOL.md. One
ephemeral c7i.2xlarge, on-demand, TTL2h, no kept instance. Compile both
sources on the same host/toolchain before timing. Restore the five exact
baseline files from `frozen-baseline/`, build and retain the baseline
binary, restore candidate sources, then build the candidate. Semaev batching
is disabled in this round. The frozen
copies come from `1340a6176` and have separate digests. Source and binary
hashes plus both build logs accompany the results.

Kernel candidate: single-write and vector tables enabled; baseline has
single-write enabled and vector tables absent. Five A/A and five A/B pairs
for the two degree4 matrix calls and the affected complete-root/decomposition
arms. Complete roots: solve-f4/f5, inherited, generalized-native.
Decomposition: decomp-f4/f5/f6 and decomp-generalized. Four fixtures n17/n23,
planted/random, seed230; matrix pairs use the primary and n17 fixtures.
Fresh degree4 n23 matrix seeds231/232 have five additional paired calls.

Each round also measures the candidate complete-root references (Gray,
wide Gray, Moebius, Monica, generic GB), F6-pairs and both Semaev oracle
references, to display the final same-host comparison. The complete-set and
decomposition boundaries stay separate. As in the initial continuation,
all cold Semaev costs include freshly prepared lookup structures, and the
existing generic-basis50-second budget is retained.

Ablations measure the same candidate executable with vector tables disabled
for matrix-F4/F5 and generalized scalar Gray forced for the generalized
root and decomposition calls. This distinguishes ISA dispatch from the
borrowing/evaluation changes. Every attempt and refusal is retained; three
bounded attempts per sample, selecting the first fully admissible one.
The same matrix rounds also run the frozen baseline with single-write off
(`legacy`) to qualify the original optimization separately on this host.
Allow60 seconds after compilation for CPU-pressure moving averages to decay;
this does not change any admission threshold.

Five-pair exact3125-resample percentile bootstrap, A/A noise threshold
max(2%, twice median absolute A/A deviation), and output equality determine
faster/slower/neutral/inconclusive. The matrix candidate remains opt-in.
Scalar portability, already supported vector ISAs, GF(2^83), all original
separator coordinates, budgets and exact truncated counts retain their
correctness obligations. Do not use a fast empty answer from an unsupported
shape, infer native speed from Ir, or pool this host with the initial host.
