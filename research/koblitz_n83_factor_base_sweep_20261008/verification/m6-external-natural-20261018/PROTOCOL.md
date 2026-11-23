# Primary N83 natural-query external XOR-SAT access test

This bounded rung tests whether CryptoMiniSat can return a fully checkable
six-summand, four-base/two-residual relation from the same exposed public
synthetic `a=0` N83 query used in the hosted [100,000-conflict Gray/hash
comparison](../hosted-m6-highcap-20261018/PROTOCOL.md). It holds fixed the
K600 sequential base, the two K600 Gray-prefix and hash-derived pool objects,
the public corpus and coefficients, equal-domain-run ordering, and identity
mask zero. The three compressed point-defined factor bases are already in
`s3://crypto-autoresearcher/factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/a0/objects/`.

The new `export-cross-pilot-canonical` worker mode uses the native search's
exact pre-solve pattern builder, with the same 150,000-variable and
12,000,000-domain-clause caps. It reimports and validates each archived S3
object, checks the separate public corpus, reconstructs the same coefficients
and group target, and creates a new XOR-DIMACS file. Its output records the
input hashes, target, source hash, formula hash and circuit shape. Exporting
one mask does not cover the full 16-mask six-summand search.

Each arm gets one CryptoMiniSat 5.14.7 process with one thread, a
5,000,000-conflict cap and a 600-second solver wall cap. Both arms use the
same binary SHA-256 and settings. The outer runner records exact raw stdout,
stderr, formula and export bytes, process wall and peak RSS. A SAT model must
pass independent full CNF/XOR evaluation, all five S3 nodes over the original
F₂⁸³, exact signed-orbit point membership, and the six-point target sum.
One-pattern UNSAT is local to that mask; a conflict or wall cap is censored.

These are local L0 solver-access measurements on an exposed query. Full
factor-base construction, repeated natural-target yield, useful rank,
large-prime graph closure, linear algebra, target extraction, and paired rho
are outside this rung. The charge for selection purposes is conservatively
the full 300-second export and 630-second outer solver allowances, with a
30-second input and replay allowance, **960 seconds per attempted arm**, plus
build, validation, and all earlier setup and
failed work in the overall campaign. `selected_best_total_runtime`
remains null regardless of a single favorable local observation.
