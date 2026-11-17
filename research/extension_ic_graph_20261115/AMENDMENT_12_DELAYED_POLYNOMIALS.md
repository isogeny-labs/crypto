# Amendment 12: delayed coefficient accumulation in S3

Review and frozen plan, 16 November 2026, before any new source or timing.
Rust only, explicitly authorized local backend. [pcc:unverified]

Stage16 finished on f33d904b7: all540executions and exact paired streams,
seven controlled direct comparisons, three upper intervals below1 and four
inconclusive cases, plus two rejected comparisons. Preserve this mixed outcome;
do not promote a uniform baseline or infer a density change. All9diagnostic
targets finish at the first repeated decomposable abscissa. The next candidate
changes multiplication in the tiny polynomial workspace only. [pcc:unverified]

Sources checked (KB): no MCP loaded; wrapper status and two distinct queries
(delayed modular convolution; negative pseudo-remainder/narrow overhead) fail
with no executable. Receipts /tmp/ic-poly-delayed-kb-{status,search1,search2}.log.
Fallback: current prepared S3 source, Amendment9 and retained stages12/13,
native bounded F4 accumulators, and the official LinBox-team FFLAS-FFPACK
fgemv LazyTag implementation, lines226–324. Read its MaxDelayedDim bound,
input normalization, blocked reductions and generic fallback. Retrieved again:
https://raw.githubusercontent.com/linbox-team/fflas-ffpack/master/fflas-ffpack/fflas/fflas_fgemv.inl
SHA2569d2b4579095e7ba5799d9a4dbd04726ae3e9dbea7647382a3f3161e02f0cd970,
local /tmp/ic-poly-fflas-primary.inl. Established arithmetic engineering,
no novelty claim or external code copied. Pseudo-division identity/reference
remain in Amendment11. Conductor unavailable(command not found).

For normalized coefficients in [0,p),1<p<2^20 and degree<=4, each convolution
output has at most5products. Its unsigned sum is bounded by5*(p-1)^2<2^43,
so native u64 accumulation cannot overflow. Retain the original workspace
degree bound. Guard modulus and coefficient normalization before choosing
delayed arithmetic; outside that guard call the unchanged scalar multiplication.
Reduce once per output coefficient. Keep
one semantic multiplication/addition hook per product and identical ring
product counts. Preserve pseudo-remainder steps, final monic normalization,
root RNG, exceptional scans and every group/scalar check. No general field,
rho, sampler, relation-density or graph-elimination change.

Introduce explicit DelayedPseudoRemainderPairOnly / delayed-pseudo-pair;
old prepared and pseudo variants/defaults remain. Differential tests include
maximum coefficients at the guard boundary, zero/degree drops, unsupported
modulus/noncanonical-coefficient fallback, and exact full graph/pivot/sampler
relation streams with identical semantic product counts to pseudo-pair.

Frozen stage17: primes127/523/1039, fresh seeds157/163/167; oracle order
delayed-pseudo-pair,prepared-pair,pseudo-pair. Declare direct paired comparisons
prepared/pseudo, pseudo/delayed and prepared/delayed, with exact stream/residual/
group/scalar equality; additionally compare semantic products for pseudo/delayed.
810executions(135warmup,675measured), one warmup plus five paired rounds,
pivot_a/graph/pivot_b/rho_a/rho_b reversed on odd rounds. R-adding-long,
small-base ceil(|F|^(2/3)), at most2auxiliary variables,500000residual cap,
CPU0,8GiBvirtual memory,900seconds/process, zero retries. Every setup/restart/
failed test/elimination/replay inside cold wall; shared supplied public fixture
generation outside. No compiler activity during timing. Synthetic Fp^3 fixtures
isolate this existing extension family absent from the standard panel.

Both median A/A ratios within reciprocal1.10and1.10; deterministic10000draw
paired-round bootstrap. Primary unit is full cold elapsed time. Retain every
comparison, failure, censor and rejected control. Upper interval below1with
both controls passing is a local engineering gain only. No composition of
old ratios, unchanged retries, strongest-reference, tier or exponent claim.
The reference is existing local cubic rho; complete pricing and independent
signed replay/recompute remain absent. [pcc:unverified]

Publish clean source/spec/manifest before the single call, and preserve copied
binaries/checkpoint/host/argv/hash receipts. Create-new directory:
/workspace/ic-local-runs/local-delayed-pseudo-pair-20261116-17
Plan delayed-pseudo-pair-plan.json, manifest validation/delayed-poly-source.sha256.
Exact start markers identify censored arms; partial-arm wall stays unknown.
No new timing started at freezing. No encrypted canonical schema admission.
