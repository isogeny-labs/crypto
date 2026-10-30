# R3: exact first-degree-fall certificates on the retained n83 grid

The active user goal is to sweep n83 m=2..9, or try n51/53, and identify first or last fall degree. All original curve families remain in scope. This stage uses the exact 48 n83 inputs already generated; it is not a rerun of the failed regularity computation. A bounded certificate stage is progress toward the goal, not completion when exact values remain unresolved.

## Definition and acceptance

Use each generator's own degree. A nonzero polynomial f has a degree fall at d when d is the least degree such that f belongs to the multiplication-closed vector space V_(F,d), and d>deg(f). The earliest such degree is recorded as first_new_degree_fall. For the binary cases certified here, the same witness is also a nontrivial leading-form syzygy at degree 2: trivial Koszul and characteristic-two self syzygies start at degree 4, while no nonzero generator occurs below degree 2. Thus first_fall_degree=2 is justified under the leading-syzygy convention as well. Zero reductions, duplicate input rows and generic rank-deficiency comparisons never count as a degree fall.

Binary and Koblitz: parse every saved input equation, reducing Boolean field equations to zero. Check that no nonzero generator has degree 0 or 1. Compute the ranks of the degree-two input rows and their quadratic parts by exact bit-packed elimination in both pivot orders. A positive rank difference gives an explicit constant combination of quadratic input equations yielding a nonzero affine polynomial. Retain its original equation indices and all result monomials. A separate replay subcommand recomputes that polynomial sum from the saved input and verifies both its degree and the absence of a lower-degree input. This certifies the earliest fall of the entire chain: cubic generators cannot enter below degree 3. The rank difference is the dimension of the lower-degree part of the original quadratic span; it is not the full multiplication-closed V_(F,2) dimension.

Prime: test all lower degrees through 4, preserving the exact interval-selector presentation. Before degree 4 only the monic selector equations b_i^2-b_i occur. Their pairwise coprime leading monomials give a degree-two Groebner basis and no new degree falls. Reduce the quartic leading parts modulo selector squares. Select one monomial per S3 link: four distinct selector bits in the first link, adjacent squared intermediates in each interior link, and the last squared intermediate with two distinct final selector bits in the last link. Exact coefficient extraction gives a diagonal minor, which is checked rather than assumed. A nonzero determinant certifies independence of all degree-four parts and hence no new fall through degree 4. This supplies a lower bound of 5, not an attained first fall. The coefficient extraction sets unused selectors to zero, which preserves the coefficients of the chosen monomials; it is a projection certificate for the full equations, not a smaller curve experiment. Literal formulas, coefficient field, selector definitions and every equation count must match the saved input.

Last fall degree remains null. A first fall is not a last fall, and absence of a fall at one degree does not certify stabilization.

## Inputs, controls and execution

Exactly 48 inputs: 3 families, m=2..9, planted and natural, original root seed 20261027 and balanced ell=floor(83/m). Source inputs are from n83-degree-m2-m9-20261027-v2, pod 25e3489a-612a-4d40-aaa4-93a1b963ff30. Every system hash is checked against the preserved input list. No new targets, randomization or outcome-dependent selection. No timing comparison.

Development controls include a degree-2-to-linear positive example; duplicates, homogeneous quadratics and a zero row as negatives; rejection of malformed field equations and altered witnesses; rejection of unhandled lower-degree inputs; and the exact prime coefficient minors for small selector fixtures at all eight arities. An initial control serializer emitted an empty expression for zero; the failed test log is retained, and zero is now serialized explicitly. No new target fall data was taken before this correction.

One ordinary EKS Spot job, 2 CPUs and 8 GiB, 900-second deadline, no automatic retries. Producer and fresh-process replay run sequentially. Each produced certificate is uploaded immediately. A process interruption remains incomplete and is reconciled before any resubmission. This ordinary command has no qualified CRIU restore. All clocks are diagnostic and wall_valid=false. The prior regularity run's 40-second solver cap does not apply to this different instrument.

## Literature and implementation review

- [Kosters and Yeo, Notes on summation polynomials](https://arxiv.org/html/1503.08001), Corollary 4.11 and Remark 4.12: trace identities can cause a degree-two-to-linear fall; this says nothing about eventual solving degree.
- [Caminata and Gorla, Solving degree, last fall degree, and related invariants](https://arxiv.org/html/2112.05579), Definitions 1.3, 1.5 and 2.6, Theorem 2.8: separate nontrivial syzygies, degree falls and the last fall; a plateau alone does not identify the last fall.
- [Kousidis and Wiemers](https://arxiv.org/html/1906.05594), Definitions 2.1–2.2: first-fall conventions in the Boolean filtered algebra. Their equal-degree span convention must not be applied to a mixed chain merely by taking its largest input degree.
- `src/cryptanalysis/koblitz_groebner.rs` explicitly labels its legacy `first_fall_degree` a rank-deficiency proxy. `examples/dreg_sweep.rs` also disclaims certification. The new instrument does not reuse that proxy as a degree.

## Remaining goal requirements

An exact binary value would complete that cell's first-fall measurement only. A prime lower bound leaves its exact first/last fall unresolved. Continue with a suitable higher-degree method or the authorized n51/53 fallback; do not mark the user goal complete on the strength of these lower bounds.
