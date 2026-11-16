# Certified quotient-algebra lead: development protocol

Status: development, not a timing run. No new runtime, solving-degree or
regularity result is asserted `[pcc:unverified]`.

Prior work: the study's exact Semaev identities and projection certificates;
research/jv_cover_sparse_fglm_20261018/README.md (existing native FGLM stage,
engineering classification and stage/end-to-end distinction); Sage/Singular
change-of-order documentation (FGLM requires a zero-dimensional input basis).
The repo KB wrapper and Conductor are unavailable in this environment.
Quotient linear dependence and change of order are standard techniques;
this lead claims no novelty.

Hypothesis: a certified shared-vertex univariate H(t), followed by unit linear
subresultants over Fp[t]/H, can produce a rational univariate representation
without a general Buchberger solve. Linear dependence of monomials in that
quotient can then recover the complete reduced grevlex basis. All discovery,
construction, proof expansion and final checks must be charged if timed.

For a reduced pair f=A*y²+B*y+C and g=D*y²+E*y+F, the row A*g-D*f is
(A*E-B*D)*y+(A*F-C*D). Invert the y coefficient only when its extended-Euclidean
GCD with H is 1; preserve the inverse identity. Reduce modulo the certified H
to obtain y-r(t) with an exact original-input membership certificate.
Propagate only across recognized edges with one known endpoint. No witness,
root enumeration, discrete logarithm, reference-basis oracle or free scout.

Before quotient conversion, require every original input to reduce to zero
under H and the coordinate rewrite rules. Their certificates prove the reverse
containment, establishing equality of ideals. A zero divisor, disconnected
graph, unsupported dimension or exhausted proof budget is an explicit failure,
not a division, saturation, guessed coordinate or successful empty basis
`[pcc:unverified]`.

Enumerate monomials in ascending grevlex order. Independent quotient vectors
become standard monomials; dependencies become monic relations, with exact
membership extracted through the certified rewrite rules. Skip multiples of
existing leading monomials. Require the declared quotient dimension, a reduced
monic final basis, original-input membership, every input normal form and every
S-polynomial. Compare exact bases against the existing reference in development
cases. Do not infer correctness from dimension or witnesses alone.

Development fixtures: the existing three-/four-variable generator at seed 7,
on primes 2147483647 and 9007199254740881. Additional synthetic algebra tests
must cover nonunits, forged/missing coordinate rules and valid quotient
dependencies. No new holdout seeds are generated in this development stage.
Reject inputs outside 3–8 variables, at most 32 equations, total degree at most
4 and 32 terms per equation. Cap quotient degree at 8, generated relation degree
at that dimension, sparse polynomial terms at 5000, expanded proof terms at
50000 and substitution reductions at 100000 per call. These are guardrails,
not an assertion of uniform runtime or memory bounds for arbitrary inputs.

No performance acceptance is tested here. Before timing, freeze a separate
matched protocol against the strongest existing native route and supported
external solvers, including construction and equal correctness work. Require
the S3 checkpoint and local isolation gates, which are currently blocked by
missing checkpoint upload transport. No cloud launch. Do not reuse old weak
baseline ratios as a new result, and do not modify past raw evidence.
