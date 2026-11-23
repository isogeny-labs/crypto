# Degree-seven coefficient maps: development contract

2026-10-30. This is development, not a frozen n83 production run. Preserve all
42 original/baseline/spike40 prime m3–9 source identities and the full n83
Koblitz/general binary/prime m2–9 fixed-base objective. Existing conditional
FFD intervals7–17 remain unchanged until a separate production protocol is
frozen and its certificates are replayed.

## Definitions and argument

Re-read Caminata–Gorla, https://arxiv.org/html/2112.05579, Definitions1.3–1.5
and Section4. FFD, LFD, solving degree and graded regularity are distinct.
The following extension of PRIME_QUADRATIC_DESIGN.md is study-specific.

Modulo the regular sequence of normalized selector squares, a degree-seven
syzygy has cubic coefficients on quartic chain generators. Injectivity of
the complete cubic multiplication map forces those reduced coefficients to
vanish. Mixed selector-square/quartic Koszul syzygies remove the unreduced
coefficients; remaining square-only syzygies are Koszul. Quartic/quartic
Koszul syzygies begin in degree8. Multiplication by a free intermediate
variable excludes lower-degree reduced relations. Thus full injection plus
the source checks would give conditional FFD>=8, not exactFFD or an LFD bound.
Characteristic-p truncation is irrelevant at degree7 for the83-bit prime.

Project each27-selector block onto nine selectors, preserving every
intermediate variable. Every cubic monomial involves at most three local
selector indices, possibly in different blocks. For every i<=j<=k in0..26,
pad {i,j,k} with the smallest missing indices to nine, deduplicate, and use
the same subset in every block. Validate every ordered triple explicitly.
Pair coverage alone is insufficient. A projected kernel does not imply an
original-system kernel.

Seven selectors are unsuitable here: on the terminal square, the product
(b0-b1)(b2-b3)(b4-b5) is a nonzero cubic annihilator. Each difference kills
the sum of its pair; the one remaining selector has square zero. This
control prevents interpreting a deficient seven-selector projection as a
new FFD bound. Nine selectors are a fixed conservative projection choice,
not an empirically selected successful base.

## Complete domain and deterministic ordering

There are n=10m-2 variables and9m selector variables. Cubic normal monomials
are sorted triples a<=b<=c without repeated selector indices. Intermediate
powers and mixed terms remain. The dimension is binomial(n+2,3)-9m*n:
the excluded monomials are selector-square times any variable, and two
different selector-square ideals cannot intersect in degree3. Multiply by
m-1 for the generator-major column count. Output coefficient rows have
degree7. No partial matrix may supply a rank or degree claim.

Retain the existing deterministic column/coefficient/reduction/basis caps and
append-only journal. Every event and result is recomputed during replay.
All cap reasons remain distinct; hard interruption is an unverified prefix.
Do not modify the prior quadratic producer, protocol or artifacts.

## Development controls fixed before execution

- Dense versus sparse ranks for m3,ell1/2/3,p11/101 (six small maps).
- Independent direct six-term quartic expansion for m3/4,ell3/7/9,p11/101.
- Cubic domain enumeration and combinatorial counts for every m3–9.
- Triple-cover validation and a deliberately pair-complete/triple-incomplete
  subset family, plus malformed/reduced-cover controls.
- Explicit seven-selector terminal cubic annihilator.
- Independent dense terminal multiplication test at ell8,p11/101: cubic
  squarefree monomials to degree-five monomials, with full-column-rank check.
- Complete bounded/unbounded equality, all four deterministic caps, replay
  rejection of truncated/extra journals; retain every fixture and log.

Development tests are correctness checks, not n83 observations or timings.
No production entry point is enabled by this development contract. Decide
the complete production resource envelope and freeze source/binary/inputs
only after these controls. Keep failures, drafts, source and format in S3/Git.
No elite-base improvement follows from these normalized templates; the
candidate-base search and filtered LFD obligations remain open.
