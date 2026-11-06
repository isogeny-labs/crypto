# Normalized-square kernel: proof plan and controlled experiment

The candidate is W_delta=B_delta^2/rho_delta, where rho_delta=B_delta(delta).
It preserves source node values, and a unit rho_delta makes a nonzero
crossing's valuation double. This is the specific component under investigation,
not a completed ECDLP algorithm or a certified novelty claim. Producer claim
records: PCC-20261106-4e89a0 (node identity), PCC-20261106-94216f (conditional
valuation), PCC-20261106-0c6563 (fixed development controls). All currently
have standing unverified pending trusted signed verdicts. [pcc:unverified]

## Proof plan

Proof update2026-11-06: the [normalizer follow-up](lean-normalizer/README.md)
now derives the unit property from actual punctured products on every shifted
dyadic grid, then proves pointwise valuation doubling without the earlier
unspecified-unit hypothesis. The finite-product/factorial bridge is included.
The actual kernels now also have all-node preservation and entrywise origin
identity theorems, covering every valid packet size and real slot.
Chart-content, uniform mode bounds and recursive cost are separate remaining
obligations. Claims bd8ff1 and82eac6 have standing unverified pending trusted
signed verdicts. [pcc:unverified]

1. **Specify the algorithmic claim.** Classical prime-field ECDLP with explicit
   subgroup/input assumptions, correctness/success convention and bit cost
   exp(o(b)), b=ceil(log2 p). Charge preprocessing, failed attempts, recursive
   tensor entries, recovery and verification. The user's novelty requirement
   remains an additional prior-art obligation.
2. **Establish the actual kernel invariants.** For L=2^a and the existing
   puncture polynomial B_delta, write W_delta=rho_delta*ell_delta^2.
   At a source node B_delta is either0 orrho_delta, giving the exact node
   identity, including origin I. The proposed degree is4L-2 versus2L-1.
   Prove rho_delta is a2-adic unit and all chart coefficients are integral;
   the exact reciprocal is (-1)^delta*binom(2L-1,L+delta)/binom(2L-1,L).
   Lucas parity is relevant because2L-1 has all binary digits1. This exact
   Cauchy-specific bridge is not part of the current generic Lean lemmas.
3. **Prove the crossing representation.** The experiment checks two complete
   representations: Newton binomial moments and per-cell modular Smith
   factors. Every entry must reconstruct. Extract a uniform bound over ALL
   required levels, including basis construction, description and precision.
   A sampled Smith decomposition is not a free uniform basis oracle.
4. **Charge recovery and recursion.** The old3ML multiplicity lower bound
   does not cover degree M(4L-2). Re-derive the Hermite budget and precision
   losses (for example, test h>=ceil(4M/u), rather than assume the old h).
   Prove child shrink and total recurrence using the actual new mode counts.
   Linear weight growth with unchanged family density would not by itself
   change the existing square-root partition exponent.
5. **Finish a faithful Lean theorem and external checks.** Only after the
   construction and total recurrence exist, connect the tensor routine to
   the ECDLP reduction, prove solver correctness and sub-exponential total
   cost, and perform novelty review. Do not replace these obligations by a
   hypothetical fast solver, finite benchmark, or assumed conclusion. [pcc:unverified]

Steps2--4 are explicit research obligations, not established global results.
[pcc:unverified]

## Experiment and decision rules

[PROTOCOL.md](PROTOCOL.md) and protocol.json fix L4,8,16,32; all active
nonzero nodes; zeta=-1,1,3; q8,12,16,24. There are1344 input cells, two
factorization methods, and one identical replay. The native checker emits
the full mode-weight histograms and arbitrary-precision tuple counts for
each original/candidate pair. It tracks both Smith basis changes and
reconstructs the entire matrix; Newton coefficients reconstruct each exact
rational row. Every loss and tie stays in the output.

A lower within-cell tuple count is a component observation. It can support
a search for a uniform weight theorem but cannot establish an ECDLP runtime.
If changes are confined to constant weight shifts, test the resulting
recurrence honestly rather than fit an improved exponent. If a more favorable
uniform family bound emerges, prove it symbolically and price the new
Hermite/basis costs before an end-to-end comparison. [pcc:unverified]

## Actual execution status

**Full panel: NOT RUN.** Backend exception is pending. EKS is the repository
default; the session has no EKS MCP/AWS caller or live shared-budget readback.
The available GitHub workflow builds a fixed ecbench bundle, not this custom
C++ workload. No unrelated integration fixture was submitted as a substitute.
The user has been asked whether to authorize the bounded local exact panel;
no absence of reply is treated as approval.

**Development controls: executed.** Strict C++20/GMP build and fixed units
pass: normalized node5 remains5; unnormalized25 is rejected; normalizer second
difference is exactly -4862/6435 with valuation1; the fixed matrix
[[2,4],[6,0]] has Smith weights1,2; weights2,3 give5 exponent tuples below6.
The normalizer control disproves the naive claim that its bth Newton
coefficient always has valuation>=b, already at b2. It does not disprove a
different weighted factorization. [pcc:unverified]

The first strict compile failed on misleading indentation. Its exact source
and diagnostic are preserved; splitting the statements fixes the warning
without changing arithmetic. The first proof-gate invocation ran before
source staging and returned FAIL_no_sources; it is not counted as a proof.
The actual staged-source gate reports PASS,3 declarations including a
generated instance proof,0 trusted. See lean/ for exact formal scope and
verification receipts. Official fresh replay status is recorded after exit.
[pcc:unverified]

The component lemmas prove (i) w*w/r=w for r!=0 and w=0 orr, and (ii)
v2(w*w/r)=2v2(w) for nonzero w,r with v2(r)=0. They do not assert that the
normalizer is a unit, that a cheaper factorization exists, or that an ECDLP
algorithm has been constructed. [pcc:unverified]

Sources checked: pinned manuscript and earlier reports in
[sources.md](../prime_common_kernel_20261030/sources.md), original D.4
crossing/mode argument, and the atlas normalized-square/Mahler/Newton query.
KB unavailable: no MCP tools; tracked CLI wrapper status failed127. No absence
or priority conclusion follows from that fallback.
