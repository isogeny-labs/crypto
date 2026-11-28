# Run78 — exact two-leaf support ceiling and asymptotic screening

**Date:** 2026-11-22 PST / 2026-11-23 UTC.  
**Input:** actual unchanged ordinary Prime31 curve from `research/prime31_f4_admission_20261121/RUN72_RESULTS.json`: p=2,147,483,647, subgroup/group order n=2,147,446,831, cofactor one. B=677 distinct abscissae from Run75–77.

## A rigorous combinatorial bound (conditional only on *uniform query distribution*)

For a factor base of B distinct **x-coordinates**, each with at most two curve points ±P_i, the number of unordered choices of two x-coordinate indices with repetition is C(B+1,2). Each pair can produce at most four signed two-point sums ±P_i±P_j, and some diagonal/opposite-sign sums coincide or produce O. Consequently the total number of *distinct nonidentity group targets* represented as **two signed factor-base points** is bounded by

```
N2 <= min(n-1, 4*C(B+1,2)) = min(n-1, 2*B*(B+1)).
```

For independent uniform Q in E(Fp) \ {O}, **without exploiting any conditioning, targeted walks, nonuniform proposal, or known-index planted selection**, one attempt has success probability at most

```
p2 <= 2*677*678/(2_147_446_831-1)
   = 918_012 / 2_147_446_830
   = 0.00042748997887877857.
```

It therefore takes **at least** `1/p2 = 2339.2361` independent uniform group-point queries **in expectation to encounter one hit**, under the stated sampling conditions. This is a cardinality bound, not a heuristic about Groebner bases, and does not assume independence of individual pair outputs (only uniformity/independence of the target draws for the geometric waiting-time interpretation). It is an upper bound on yield, not on rank: a hit need not provide a new rank-independent relation.

**Consequence using *observed stage times* only as a sensitivity extrapolation, not a theorem:** Run77 balanced B677 x-oracle per-query means were 22.506 and 28.752 milliseconds across two GitHub-hosted runners. Multiplying by the *optimistic* uniform-query minimum 2339.24 attempts/hit gives approximately **52.65–67.26 seconds per potential hit**, excluding preparation of a random Q, signed verification, rank, failures, cold setup, memory and end-to-end logarithm recovery. Naively multiplying by B=677 gives **9.90–12.65 hours for 677 hits** and is *not* a measured solver runtime or a valid lower bound for adaptive/nonuniform schemes. The Run77 eight-target corpus deliberately plants three positive cases, so its empirical 3/8 positive fraction must **never** estimate random group-point yield.

The mathematical ceiling already explains why a factor 1.22–1.27 reduction in per-probe time cannot alone establish a faster complete ordinary-prime-field index-calculus solver.

## Four summands / novelty qualification

Replacing two leaves by four **changes relation density** and is the appropriate independent research question. However, Semaev S5 / chained S3 systems still require a nonenumerative extraction method that handles the factor-base constraints and exceptions, produces signed **authentic public RHS rows**, and obeys a full-cost budget. A standard optimistic model for m=4 (uniform unsaturated relation probability Θ(B⁴/n), B independent rows, LA Θ(B²), relation oracle Θ(B^gamma)) yields C(B)≈n B^(gamma-3)+B² with a possible sub-n^1/2 exponent only if gamma<1, under the additional work and saturation qualifications recorded in Run75. This is **not an unconditional algorithm or an asymptotic proof**. A degree-B polynomial input, quadratic circuit, or balanced tree is not itself evidence of gamma<1; the measured Run77 pair extraction is roughly quadratic over this finite range.

**Specific external prior that must now be checked before any naive-method speedup claim:** Yokoyama–Yasuda–Takahashi–Kogure, *Complexity bounds on Semaev's naive index calculus method for ECDLP*, Journal of Mathematical Cryptology 14(1), 460–485 (2020), DOI [10.1515/jmc-2019-0029](https://doi.org/10.1515/jmc-2019-0029). It derives conditional lower costs for Gröbner/PDP in the naive Semaev method **under statistical assumptions**, and argues that naive index calculus cannot beat generic algorithms. This is a severe prior-art screening condition, **not** a proof that no different structured, adaptive, hybrid or non-naive method can work. Also check [Semaev's 2004 original](https://eprint.iacr.org/2004/031) and [Faugère–Huot–Joux–Renault–Vitse, EUROCRYPT 2014](https://doi.org/10.1007/978-3-642-55220-5_3) for summation/symmetrization priority. (The 2020 paper abstracts/HTML and the 2014 bibliographic record were externally retrieved 2026-11-22; no claim to have reverified every proof detail.)

## Next falsification experiment, in existing Rust code

1. Count actual *uniform public* signed two-leaf hits over at least 10^5 fixed-seed Q (batch group shifts), compare with the **ceiling** above, and separately record independent RHS rank; keep planted controls out of the yield estimate. The success fraction must never exceed the combinatorial ceiling except by sampling/statistical noise (a finite sample can exceed the ceiling; use binomial confidence intervals), and every accepted hit must independently verify the point equality.
2. Advance *four-leaf* public-only extraction where it avoids B² signed-pair dictionaries; at B≈64/96/128/192 compare exact F4, chained S3, signed-root, RR9/F3 and selector propagation under equal costs and frozen target cohorts. Record rootfinding negative probes and group-lift exceptions; do not count algebraically planted solutions as random successes.
3. Pipe authenticated relations into the merged Rust target-quotient rank solver, then complete cold benchmark against the strongest project index calculus / shared BSGS / negation-map rho over Prime31 and at least two other ordinary cofactor-one fields. Require source-pinned Rust CI, independent replay, all 1/32/128/1024 scalar results checked, and computation/memory limits. No other PR maintenance after #4901 merged.

**Boundary:** This note establishes an exact screening bound for the *uniform random two-summand* approach and the need to pivot to genuinely different structure. It does not prove an improvement or rule out nonuniform strategies.
