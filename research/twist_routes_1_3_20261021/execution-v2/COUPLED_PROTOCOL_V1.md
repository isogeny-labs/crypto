# E21–E22 coupled components and projected controls, version 1

Freeze date: 2026-10-23. Commit this protocol, native source and supporting
checks before collecting scientific output. This is a bounded local pilot
of the frozen E21–E22 proposals, not admission of their blocked backlog
cards or completion of either experiment. Preserve every unsupported cell,
rank deficit, error, timeout and partial output.

Use the ten ordinary, odd-prime-order execution-v1 fixtures and their exact
registered curve identities. For P over Fp2 define T=P+Frob(P), U=P−Frob(P).
Inspect nine predicates x(T)+lambda*x(U) in {0,...,h−1}, lambda=1,2,3 and
h=2,4,8, and three predicates x(T)*x(U) in the same sets. Count and exclude
T=O or U=O explicitly; do not assign them affine x-coordinates.

Enumerate every Fp2 point for p=59,211,251. For each other curve use 65,536
public SplitMix64 x-coordinate proposals, selecting a square root sign
with the next public generator word only after a successful lift. Deduplicate
points before measuring sampled fibers. This is a proposal sample, not an
exhaustive or unbiased census. The generator seed is the first 16 hexadecimal
digits, interpreted as a u64, of SHA256 of the UTF-8 string
`twist-followup-20261022-v1|FIXTURE_ID|E21-pool`.

Canonicalize a nonzero trace point by taking the lexicographically smaller
of it and its negative. For every accepted trace record its exact point,
lexicographically least witnessed extension lift with that trace, and the
number of distinct sampled or exhaustively enumerated accepted lifts.
Select bases of 8,12,16 distinct canonical trace points in lexical order;
an insufficient image is unsupported, not replenished from another source.
Independently verify all 36 exhaustive image/fiber tables with PARI/GP by
enumerating the two rational eigenspaces. For sampled tables verify up to
eight concrete lift/trace/predicate witnesses per cell. A zero GP exit or
completion marker is insufficient: use the native hash/error adjudicator.

For relation arities 2 and 3 enumerate every signed tuple of the selected
extension witnesses, take its trace and retain the canonical integer
coefficient row. Independently use direct signed same-base meet-in-the-middle
search on E(Fp) for every target, and require exact equality of the two row
sets. This tests projected relations; it does not exercise an auxiliary
polynomial solver. Each auxiliary cell needs fewer than 2^20 explicit group
additions. Store construction counts, tuple counts, direct-search counts,
row hashes, matrix operations, rank and scalar verification separately.

Generate a target from a nonzero scalar in [1,r−1] by rejection sampling the
first 16 hexadecimal digits of SHA256 of
`twist-followup-20261022-v1|FIXTURE_ID|E21|LABEL|ATTEMPT`.
Accept a u64 word k only when k < u64::MAX−u64::MAX%(r−1), then use scalar
1+k%(r−1). The collector receives only its point. Never provide these
generation scalars to relation search, matrix elimination or diagnostics.
Recover a scalar only if its unit vector lies in the span of verified
factor-base relations, the public generator anchor, and target relations;
verify k*G=Q. Full factor-base rank is not required. Retain recovery failures.

Train on both structural fixtures and all ladder rep-0 fixtures, labels
0–15. Before evaluating any rep-1 target, write SELECTION.json choosing at
most two configurations by, in order: fewest unsupported training workloads,
fewest unrecovered targets, smallest ratio of candidate cold-model unpriced
reference calls to direct-search/anchor calls, then lexical configuration
name. Each unsupported base/arity cell counts as 16 unsupported workloads.
Evaluate only these two on the four rep-1 fixtures, labels 16–31. Record
the other configurations as not evaluated under this fixed rule. This is a
diagnostic selector, not E28's complete-cost selector. Do not generate final
E28 labels 32–63.

For E22 compute exact signed pair multiplicities and additive energy, and
triple multiplicities. Use all rational targets on p=59,211,251 and 32
public E22-domain target labels on larger fields. Deduplicate identical
projected bases across predicate configurations and disclose shared labels.
Compare with 32 public Fisher–Yates bases for each fixture and B=8,12,16,
using all nonzero rational sign classes and seed domain
`twist-followup-20261022-v1|FIXTURE_ID|E22-random|B|SEED`.
Energy and support are exact group calculations, with no discrete-log
coordinates. Repeated-fiber energy scaled by 16 is an explicitly derived
two-copy weighted control; it creates no new distinct base points or rows.

Use a frozen release binary, one thread, 4 GiB address space, and a
3,600-second cap per command, checkpointing per image/profile. Correctness
indices may be cached across repeated bases, but the cold diagnostic model
charges their entire construction to each target, including each image's
shared pool construction. High-level arithmetic calls are unpriced and
incomplete: field primitives, memory, solver work, scalar verification,
sorting and all stages still require E27 accounting. Matrix and anchor
counts remain separately visible. Make no wall-time, calibrated cost,
novelty, NIST-DLP, or asymptotic speedup claim from this pilot.
