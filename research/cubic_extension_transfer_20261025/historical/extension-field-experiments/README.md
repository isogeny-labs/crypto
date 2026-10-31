# ECDLP over extension fields: reproducible experiments

**Latest continuation:** [results from all four research tracks](research_lead/FOUR_TRACK_RESULTS.md).
Certified sources can require two degree-three steps in total; equal
endomorphism rings do not determine quotient coverage. The reusable transfer
now has obstruction fallback, stronger divisor recovery, and line and
double-large-prime target solvers. Complete small-field attacks remain slower
than source Pollard rho. The extra-step bound, universal chart coverage, and
novelty remain open.

The [previous polynomial-transfer stage](research_lead/NEXT_RESULTS.md) is
retained with its original small examples.

This suite covers the three requested questions: low-genus transfers, trace-zero
subgroups, and useful intermediate fields. It also reproduces the numerical
isogeny in Tian's published cubic example and screens the original binary GHS
genus criterion. All scalars and points are synthetic.

The executed results are in [results/standard/report.html](results/standard/report.html).
That standalone report links raw JSON, CSV, the executed parameter plan, and SVG
figures. Nothing is uploaded or published by the experiment runner.

## Run

Tested with SageMath 10.6. With Sage installed:

```sh
sage -python run_experiments.py --preset standard
sage -python -m unittest discover -s tests -v
```

Alternatively, `./run.sh --preset standard` uses a Docker image pinned by digest.
The image is large; download it before running in an offline environment:

```sh
docker pull sagemath/sagemath:10.6
./run.sh --preset standard
```

Available presets:

```sh
./run.sh --preset quick --output results/quick
./run.sh --preset standard --replicates 3 --output results/repeated
./run.sh --preset sweep --replicates 3 --timeout 300 --output results/sweep
./run.sh --preset standard --family tian_cubic_rational_map
./run.sh --preset standard --plan-only
```

`quick` still includes the three Tian cases. The standard plan contains 38 cases.
Each case has a deterministic seed derived from its parameters, replicate number,
and master seed. Cases run in separate processes with a time limit. Every error,
timeout, and ineligible input is recorded; unexpected outcomes give a nonzero
runner exit status. The deliberate degree-three input at p=7 has no eligible
large odd subgroup under this experiment's cutoff and is expected to be
ineligible.

## Experiments and acceptance criteria

| Family | Inputs | What is constructed or measured | Required evidence |
|---|---|---|---|
| Quadratic cover | F_(p^2), plus F_(p^4)/F_(p^2) and F_(p^6)/F_(p^3) | An explicit genus-two curve, elliptic quotient, pullback, and divisor-class trace | Genuine extension-field j-invariant; nonzero image; exact prime subgroup order; homomorphism checks; same recovered scalar |
| Cubic cover | F_(p^3)/F_p and F_(p^6)/F_(p^2) | A genus-three hyperelliptic special-family cover and pullback-and-trace map | Rational-function map identity; nonsingular target; subgroup preservation; matching source/target BSGS recovery |
| Tian kernel | Published E/F_(523^3), r=143069629 | The 27-element 3-torsion kernel on the Weil restriction | Rank three, isotropy, Frobenius stability, no coordinate factor, kernel degree coprime to r; reject an axis kernel |
| Tian images | Published non-hyperelliptic plane quartic over F_523 | Actual divisor classes S,T | S nonzero, rS=0, T=1987S; recover 1987 within the explicitly stated interval [0,4095] |
| Tian rational map | Same published isogeny | All six rational functions, pullback image divisors, sum of Frobenius conjugates | Match the two independently constructed published image classes; verify subgroup order and linearity |
| Trace-zero structure | p=7,11; extension degrees 2 through 6 | Kernel of 1+pi+...+pi^(n-1), prime subgroup, Frobenius eigenvalue | Exact cardinalities; trace zero; lambda^n=1 mod r; scalar recovery; direct kernel enumeration for small fields |
| Trace-zero relations | Degrees 3 and 5 | Restricted factor base, signed decompositions, relation matrix | Full rank; solve for all factor-base logs and the target scalar; check every recovered log |
| Intermediate-field controls | n=4,d=2 and n=6,d=2 or 3 | Ordinary trace through a proper intermediate field | Trace transitivity; preserved base subgroup; extension subgroup killed when r does not divide the intermediate group order |
| Binary GHS screen | q=4; relative degrees 2 through 6 | F_2-rank of the Artin–Schreier coefficient space, candidate genus | Full coefficient orbit; direct character-count genus agrees with the GHS rank formula |

### 1. Explicit low-genus transfers

**Quadratic construction.** Over k choose a nonsquare c and let s^2=c in its
quadratic extension K. Take

    f(x) = x^6 + a x^5 + b x^4 + h x^3 + cb x^2 + c^2 a x + c^3,

with h chosen so f(1)=0. Reject singular curves and quotients with j-invariant in
k. The involution (x,y) -> (c/x, s^3 y/x^3) produces an elliptic quotient E/K.
The code computes its coefficients and the covering map explicitly. The branch
point (1,0) supplies an odd-degree model for ordinary Mumford arithmetic.

For a point P on E, pull back [P-O], then add its conjugate to obtain a point in
Jac(C)(k). The test r*Phi(P)=0 and Phi(P)!=0 proves injectivity on the chosen
prime-order subgroup. The unit tests check the map identity as rational
functions and check every scalar in one small subgroup.

**Cubic construction.** Let K/k be cubic, q=|k|, and alpha generate K/k. Use

    E: Y^2 = X (X-alpha) (X-alpha^q).

A rational conic parametrization produces A(t) in K[t] and u(t),v(t) in k[t]
with A(t)^2=u(t)+v(t)alpha. Then

    C: w^2 = -u(t)v(t),
    X = -u(t)/v(t),
    Y = -w A(t) A(t)^q / v(t)^2.

Here deg(u)=4, deg(v)=3; a squarefree degree-seven polynomial certifies genus
three. The map identity is verified exactly before any point experiments.
Pullback and a three-term trace give the subgroup transfer. With k=F_(p^2),
this supplies a genuine F_(p^6)-to-F_(p^2) Jacobian example.

This is an overlapping-branch GHS-style special family with full rational
2-torsion. It is distinct from Tian's prime-order construction. It demonstrates
the geometry used in a cover attack, not the complete Joux–Vitse computation.

**Tian reproduction.** The six published functions are stored as numerical
coefficient arrays in `tian_data.json`. Their provenance and source checksum are
included. The optional extractor regenerates those arrays from the public arXiv
TeX source using SymPy; the experiments need neither a download nor SymPy parsing.
To regenerate the numerical data explicitly:

```sh
curl -L https://arxiv.org/src/2012.07173 -o sources/tian_source.tar
tar -xOf sources/tian_source.tar Cover_attack.tex > sources/tian.tex
sage -python extract_tian_data.py
```

The target plane quartic is independently checked for smoothness. To avoid
Sage's costly general normalization, the implementation supplies certified
integral bases through Sage 10.6's private integral-basis hook. This is justified
because both relevant affine patches of the smooth plane quartic are normal,
and the projection polynomial has constant nonzero leading coefficient. The
bases are 1,y,y^2,y^3 and, in the inverted patch, 1,y/x,(y/x)^2,(y/x)^3. Genus,
ideal/divisor arithmetic, equality, and scalar checks are still computed by Sage.
No genus, scalar relation, or DLP answer is installed as a cached result.

The six-function experiment represents conjugate summands over F_(523^3).
Their trace is mathematically F_523-rational and is matched against both
published F_523 image divisors. This is evaluation of one supplied isogeny;
there is no general isogeny constructor for arbitrary new curves.

### 2. Trace-zero experiments

The exact trace-zero order is #E(F_(p^n))/#E(F_p). A sampled point is projected
into a prime-order subgroup and verified. The minimum field of the **point** is
also recorded: a point stored in F_(p^6) can already be defined over F_(p^3).

The relation experiment actually solves linear equations for unknown factor-base
logs and the target log. The unknown target scalar is not supplied to the solver.
However, relation generation uses bounded exhaustive signed decompositions,
not the Gröbner-basis algorithm or exact symmetric-coordinate factor base of
Gorla–Massierer. Its timings establish the mechanism and its cost at toy size;
they do not reproduce that paper's asymptotic improvement.

### 3. Intermediate-field and binary controls

Ordinary point trace is not a substitute for a Jacobian transfer. If a selected
prime r divides the ambient curve order but not the intermediate curve order,
the trace of the entire r-subgroup is zero. These are expected negative controls.
Base-field subgroups with r coprime to the trace multiplier survive, providing
positive controls on the same curve.

The original binary GHS screen uses ordinary curves y^2+xy=x^3+b. If m is the
F_2-rank of the vectors (1,(sqrt(b))^(q^i)), the associated candidate cover has
genus 2^(m-1), or one less if (1,0) lies in that span. The code independently
counts Artin–Schreier characters having both simple poles to obtain that genus.
This is a geometry/eligibility screen only, not a binary divisor-map or DLP
implementation. A low candidate genus alone does not prove subgroup retention.

## Interpreting costs

Source and target cover experiments use the **same r** and BSGS on both sides.
They record group additions, memory-table size, source/target timings, transfer
cost, and total experiment time. Transfer cost includes measured generator
search and the two point images. Construction, counting, and validation work is
included in the total, not in the reported transfer-only comparison.

The Tian source BSGS searches its full prime-order subgroup. The target Jacobian
replay searches only [0,4095]. Those runtimes are not comparable; the target
experiment is excluded from matched timing plots. No fixed-degree p-exponent
or cryptographic-size speedup is inferred from these toy experiments.

`degree_bounds.svg` is explicitly theoretical. It displays n/2 and 2-2/n as
reference exponents, ignoring degree-dependent costs. It is not fitted to data.

## Sources

- Song Tian, [Cover attacks for elliptic curves with prime order](https://arxiv.org/abs/2012.07173), arXiv v1 (2020), published 2023. The numerical reproduction uses Section 3.4 of v1.
- Elisa Gorla and Maike Massierer, [Index Calculus in the Trace Zero Variety](https://arxiv.org/abs/1405.1059), 2015.
- Antoine Joux and Vanessa Vitse, [Cover and Decomposition Index Calculus](https://eprint.iacr.org/2011/020), EUROCRYPT 2012.
- Pierrick Gaudry, Florian Hess, Nigel Smart, [Constructive and Destructive Facets of Weil Descent on Elliptic Curves](https://www.kantmath.de/publications/papers/weil_descent.pdf), 2002.
- Claus Diem, [On the discrete logarithm problem in class groups of curves](https://doi.org/10.1090/S0025-5718-2010-02281-1), 2011.

The suite contains no copied implementation from these papers. Published
numerical mathematical data is identified in its corresponding experiment.
