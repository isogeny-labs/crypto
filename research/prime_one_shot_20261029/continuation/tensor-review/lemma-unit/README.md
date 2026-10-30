# Exact-rational tapered Hermite lemma unit

2026-10-30 UTC. Native C++17/GMP DEVELOPMENT correctness checks of
AppendixD.4.3-D.4.4, LemmasD.4-D.5 and reconstruction(D.9), in the
OpenAI October25,2026 preprint _The Exponential Time Hypothesis is False_.
This is finite lemma-unit evidence. It does not certify that preprint's
theorem, implement a contraction evaluator, prove an asymptotic bound,
measure performance or provide a cryptanalytic algorithm.

## Observed fixed results

| Check | Retained result |
| --- | --- |
| Parameters | a=1..3;u=1..a;M in{u,u+1};p in{1,3,5} |
| Polynomials |72 cases, two families, four rational coordinates |
| Lift variants |exact;canonical input;2^p_t-perturbed input;2^p_t-perturbed quotient |
| Actual Cauchy-kernel charts |308 charts/4,336 coefficients pass integrality/divisibility |
| Nonintegral ordinary kernel coefficients |266, before shifting to integral local charts |
| Source polynomial chart coefficients |101,280 checked against the hypotheses |
| Active Hermite node instances |552 |
| Inverse-series coefficients |3,624, with exact truncated product identity |
| Node coordinate errors |8,832 retained, all satisfy v2(error)>=p |
| Final coordinate errors |1,152 retained; exact identities and integral residues pass |
| Negative controls |4 expected rejections, including actual corrupted-quotient reconstruction |
| Strict builds and exact repeat |pass; every final record byte-identical on repeat |

For every positive case the code first checks the full polynomial degree,
the known constant vector F(0), and all coefficients of all full charts
F(t+alpha*z), including t=0. The Cauchy family tests genuinely rational
ordinary coefficients whose local charts are integral. These are not
silently replaced by globally integral polynomials.

The code constructs E_t directly from the source product, computes its
inverse by ordinary rational power-series recurrence, and checks the
product identity modulo z^q_t. It computes v2(E_t(0)) from the rational
product and compares it with(D.10), including the j/2^j factor. Each node
contribution is evaluated both by Horner's method and by its explicit
coefficient-weight sum.

Every precision perturbation is checked to be a2^p_t multiple in the
2-local rationals. All errors are retained as exact reduced numerators and
denominators with their exact2-adic valuations; zero is recorded as inf.
The final sums are checked integral and compared modulo2^p with separately
specified known F(0) coordinates. Thus rational cancellation is tested
before reduction; modular equality is not used in place of the exact
unperturbed identity.

The minimum finite error margin observed above p is57 in this fixed panel.
This is a panel diagnostic only. It provides no basis for lowering the
source precision formula or claiming a uniform sharper estimate.

## Frozen protocol and implementation independence

[PROTOCOL.md](PROTOCOL.md) was written and hashed before implementation and
the first execution. It fixes all parameters, families, perturbations,
outputs and controls. [protocol-and-source.sha256](protocol-and-source.sha256)
binds that protocol and the primary PDF/text. Native arithmetic in
[hermite_unit.cpp](hermite_unit.cpp) uses GMP mpq_class/mpz_class.
It includes or invokes no Rust backend, tensor engine, existing
reconstruction implementation or Python research code.

The four coordinates are described in the synthetic basis1,U,V,UV with
U^2+U+1=0 and V^2=0. Reconstruction multipliers are rational scalars, so
the unit acts coordinatewise. It does not implement coefficient-field
multiplication or the global recursion's e=4lambda parameter. This is a
scoped scalar-coordinate lemma check. The same linear identities can be
embedded mathematically in larger permitted coefficient rings; that
observation is not additional executed code or a full-ring implementation.

Primary source read: printedpp59-61, sectionsD.4.3-D.4.4, including the
visually checked(D.10) onp61. Source URL:
https://github.com/openai/math/blob/main/preprints/The-Exponential-Time-Hypothesis-is-False-October-25-2026/paper.pdf .
Local source files are `/workspace/audit/tensor-primary/openai-oct25.pdf`
and `.txt`; their checksums are bound in the protocol manifest. This
records the supplied primary source without asserting its larger claims.

## Controls, review and retained correction

The first three controls reject F=1/2, F=T/2 and excess degree. The final
control uses a=u=M=p=1 and valid F=1, reconstructs from both active nodes,
then adds1 to quotient coefficient2 at node-1. It obtains the actual error
1/128, with valuation-7, and rejects the requested precision/integrality.
This deliberately violates the lift hypothesis and is not a counterexample
to the lemma.

[review.md](review.md) is a separate read-only source/record review. It
found that the first source version computed that last error analytically
without actually injecting the corruption. The72 positive cases already
passed, but that initial version did not execute the promised negative
control path. The implementation was corrected, rebuilt and repeated.
The reviewer checked the correction and found no remaining blocking
fixed-unit correctness or protocol mismatch.

Preserved initial source: `hermite_unit.initial.cpp`; initial records:
`attempt1-records/`; final accepted records: `records/`. The initial and
final build/run logs and exact source/binary hashes are retained in
`build-receipt.txt`. Both strict builds passed; there was no compiler or
runtime failure. The review-driven coverage correction is kept explicitly
rather than overwritten. Review is not an independent full arithmetic
replay or a formal proof certificate.

## Reproduction and raw data

With C++17 and GMP headers/libraries installed:

```sh
bash check.sh /tmp/tensor-lemma-unit-replay
```

The script strictly rebuilds and executes the fixed suite, then compares
all seven final output files byte-for-byte with the retained records.
The exact build is C++17 with-O2,-Wall,-Wextra,-Wpedantic,-Werror and
-lgmpxx,-lgmp. Compiler identity and exits are in `build-receipt.txt`.
Native binaries live outside this source/evidence directory.

`records/cases.tsv` lists every case; `kernels.tsv` lists local kernel
checks; `weights.tsv` retains multiplicities, precisions and exact(D.10)
valuation comparisons. `node-errors.tsv` and `final-errors.tsv` retain
all coordinate errors; `negative.tsv` retains controls; `summary.json`
contains the counters above. No timing is used as a result.

`files.sha256` binds the complete local evidence payload except itself.
The primary-source manifest uses original absolute paths; it documents
provenance and need not be present at those paths when only this unit is
copied. Reproduction of the unit uses no primary PDF at runtime.
