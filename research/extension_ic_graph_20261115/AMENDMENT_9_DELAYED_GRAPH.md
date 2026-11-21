# Amendment 9: delayed modular products on the quotient graph

Frozen before new kernel timing,16November2026. Rust only; local backend
explicitly authorized. Producer protocol; [pcc:unverified].

Scalar graph experiment11finished on7cac567a4: all36systems/864batches match
complete dense outputs and pass A/A. Every sparse solve in the first measured
batch succeeds at its first projection, no dense fallback. Sparse/dense means
0.998–1.390; two near-ties below1have intervals crossing1, and no accepted upper
interval is below1. Keep this negative/tied outcome. Partial counted walk work
is about200kproducts/call, not a native wall profile. [pcc:unverified]

Next change preserves that scalar solver and introduces an explicit
F4KrylovDelayed mode. It changes graph dot-product arithmetic only. For
D<=64, normalized coefficients/vectors<p and1<p<2^20, every destination sum
is at most64*(p-1)^2<2^46. Accumulate unsigned products and reduce once per
vertex. Check dimension/modulus/coefficient bounds before choosing this path;
random projections and every output vector are normalized. Outside the guard,
use the scalar kernel. Preserve each semantic multiplication/addition hook
and counted field product; wall time prices changed native division work.
No new root algorithm or basis/NF shortcut. Exact recurrence/Hankel/original
polynomial/cubic checks and charged dense/oracle fallback stay active.

Prior art reviewed before implementation: native f4_fp.rs reduce_lazy_by and
reduce_pairs_blocked bounded accumulators; official LinBox-team FFLAS-FFPACK
fflas_fgemv.inl delayed/LazyTag MaxDelayedDim, normalization, generic fallback
and blocked reduction (lines226–324 of downloaded source).
https://raw.githubusercontent.com/linbox-team/fflas-ffpack/master/fflas-ffpack/fflas/fflas_fgemv.inl
SHA2569d2b4579095e7ba5799d9a4dbd04726ae3e9dbea7647382a3f3161e02f0cd970.
Sparse-FGLM algebra remains Faugere–Mou arxiv1304.1238section3.1.
KB MCP absent; wrapper status and exact delayed-dot-product query both fail
with no executable. This is established arithmetic engineering, no novelty
or asymptotic improvement assertion. No external source code is copied.

Stage12is an explicit kernel ablation on the same36public systems as11:
p67/127/1039,seeds83/89/97,four residuals each, scalar_a/delayed_a/scalar_b/
delayed_b, reverse odd rounds,8calls/batch,1retained warmup+5measured rounds.
All complete canonical outputs must equal the dense qualifier, including empty
sets. Scalar/delayed counted multiplications and quotient dimension must be
identical in addition to A/A signatures and intra-batch consistency. Both
median A/A ratios must lie within reciprocal1.10and1.10. Deterministic10000
paired-round bootstrap; upper95percentintervalbelow1for a phase gain.
All864batches/start markers, failures and incomplete cases retained; no retries.
Full basis/border/matrix/extraction/failed attempts inside compared engine
phase. Shared symbolic/input/qualification outside and reported. Native F4
degree24,no per-call deadline; CPU0,8GiBvirtual memory,900swhole process.
Publish source/spec, copy/hash binaries/checkpoint/runtime/host before calls.

If12has no invalid complete outputs, is not process-censored, and at least6of
its12largest-field cases have eligible upper intervals below1, run stage13
on fresh seeds101/103/107against the dense solver, same primes/residual count/
batch/rounds/budget/controls. Otherwise retain12and revise from its observed
mechanism instead of running13. Stage13uses dense_a/delayed_a/dense_b/delayed_b;
semantic product equality is not required across different extraction algorithms.
Again keep every case, no unchanged rerun for better timing controls. Stage13
is still the complete coordinate engine, not full IC. Only after that new
phase evidence supports viability will full IC protocols be frozen against
dense, prepared S3 and matched rho, with all end-to-end preparation charged.
No ratios combined across experiments or inferred pipeline/parity gains.

Planned create-new directories:
/workspace/ic-local-runs/local-s4-delayed-v-scalar-20261116-12
/workspace/ic-local-runs/local-s4-delayed-v-dense-20261116-13
Both unstarted at this amendment. Canonical schema unavailable(encryptedVLT1),
no standard lab admission, strongest-external comparison or signed independent
verdict asserted. Raw/checkpoints retain authorized local custody. [pcc:unverified]
