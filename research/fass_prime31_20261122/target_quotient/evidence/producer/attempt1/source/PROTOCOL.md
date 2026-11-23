# Target-dependent FASS quotient: frozen correctness protocol

Date: 2026-11-22 Pacific / 2026-11-23 UTC. Tracking PR: [#4952](https://github.com/isogeny-labs/crypto/pull/4952), branch `research/fass-target-quotient-rank-20261123`, base `f0ca664eca0081a30adfcc79dcf62a32de40d2af`. This file is prospective and is frozen before the first new native qualifier call. Source hashes and the protocol hash belong in each immutable call record.

## Question and hypotheses

Let V be the d-th roots of unity in F_p, p>max(3,2d), and define

    H(x,t) = product_(z in V) S3(x,z,t) mod (x^d-1),
    R_V(t) = product_(z in V) S3(t,z,X),
    G_X(t) = gcd(R_V(t), t^p-t), made monic.

The subject is the coefficient rank and zero support of H after reduction by the target-dependent right polynomial. The generic fixed-subset claim is a written derivation to audit, not an assumption built into acceptance: for nonempty W subset V, k=|W|, the coefficient matrix of H mod monic(R_W) should generically have rank min(d,2k). Here R_W is the product over W, normalized on X not in W. At A=B=X=0 its monic specialization is t^(2k); the first r=min(d,2k) columns select rows {0,d-r+1,...,d-1}. Their proposed determinant is (-1)^(r-1) product_(j=0)^(r-1) binom(2d,j).

The full right polynomial is predicted to have degree 2d when X^d != 1. Since deg_t H <= 2d-1, reduction by that full polynomial should leave H unchanged. This says nothing by itself about reduction by its squarefree rational-root factor G_X.

Competing possibilities for actual target quotients are maximal dimension-limited rank, further target-specific rank loss, and low Boolean zero-support rank despite large coefficient rank. Every observed case is retained. Rank saturation is an observable, never a unit-test assertion. The singular specialization certifies a nonzero algebraic minor of the nonreduced R_W quotient; its radical is t, so it does not directly certify a rank statement about the radical quotient.

## Reuse, scope and prior work

Reuse the exact published field, curve, S3, and polynomial arithmetic in `src/cryptanalysis/semaev_prime_poly.rs` at the recorded base. Reuse or attribute the merged `coupled/native` product and matrix helpers; do not alter the frozen coupled evidence. Place new code only in `target_quotient/native/`, producer evidence in `target_quotient/evidence/producer/`, and separate-role replay in `target_quotient/evidence/independent/`.

This is a bounded native development correctness qualifier, with no timing, operation-performance, randomized-yield, solver-cost, or ECDLP recovery claim. It does not implement the concurrent three-summand signed extractor (#4939 tracking lineage), the four-summand pair-spectrum lane (#4951), or another Fermat-idempotent algorithm. Boolean support is a diagnostic of a known split-algebra representation; forming it is not a free computation.

Sources checked: merged #4921 first-norm rank and circuit work; merged #4936 selector; #4939 tracking status; #4951 prospective pair-spectrum note; #4928/#4941 implicit-index and Fermat-support notes; Petit-Kosters-Messeng, *Algebraic approaches for the Elliptic Curve Discrete Logarithm Problem over prime fields*; Bernstein-De Feo-Leroux-Smith, *Faster computation of isogenies of large prime degree*. The shared KB has no exposed MCP and `scripts/research-kb status` reports no executable. These direct source checks are the recorded fallback; no novelty priority is asserted. Conductor, Lean, Lake and AWS executables are unavailable in this runtime. An isolated fresh clone and disjoint paths avoid concurrent worktree mutation.

## Fixed fixtures and target selection

- F97 ordinary curve A=2, B=3; d in {1,2,3,4,6,8,12,16}.
- Prime31 p=2147483647, A=847514846, B=2087444724; d in {1,2,3,6,7,9,14,18,21,31,42,63}. These are the existing project coefficients; this qualifier does not recertify the full group order.
- For each ordinary curve and d: the first three non-2-torsion affine points encountered by scanning public integer x=0,1,...,1023, using the smaller of the two square roots for Y. Also include the first liftable selector root as an explicit X-in-V target when one exists. If a target repeats between categories, retain the categories and document any deduplication.
- F97 adds the public 2-torsion target (30,0).
- Rank-one control: F97, A=-1, B=0, d=2, targets (0,0), (1,0), and the first ordinary affine scan point.
- Cusp controls: A=B=X=0 for every d; use the unique positive k in {1,max(1,floor(d/2)),d}. Choose W as the first k sorted roots. This is solely a polynomial-minor control, not a valid elliptic target.
- Empty-subset control: R_empty=1, quotient remainder zero. Affine S3 does not encode a target at infinity; explicitly mark that chart outside the qualified finite-target calls.

No secret scalar, DLP-dependent input, random target generator, or d=1302 run is used.

## Observables and acceptance checks

For each target record p,A,B,d,X,Y, target category, liftable-selector count, 2-torsion/degree-drop flags, deg R_V, deg G_X, multiplicity loss, coefficient ranks before/after reduction, nonzero coefficient counts, algebraic zero-support count, and Boolean support rank. If the implementation also restricts rows to liftable left coordinates, label those dimensions and ranks separately. Algebraic zeros on all V are not signed curve relations.

Required correctness comparisons:

1. Build H in the cyclic x quotient and compare scalar evaluations with direct products of S3 factors.
2. Compute G_X using published modular powering and polynomial gcd. Independently reconstruct its monic product from the union of roots of the degree-at-most-two S3 factors. Exhaustively check this root set over F97.
3. Compare H and its canonical G_X remainder at every x in V and every root of G_X. Compare remainder construction with independent per-x product/reduction and interpolation, or an equivalent independent coefficient reconstruction.
4. Check that coefficient-matrix rank equals evaluation-matrix rank over the distinct split grids. Record Boolean support rank separately.
5. For nonexceptional X^d != 1, check deg R_V=2d and literal equality H mod R_V=H. At degree drops retain actual degrees instead of applying that assertion.
6. Check the proposed cusp R_W quotient columns and selected minor, including k<d/2; check the empty quotient. Keep cusp-radical and nonreduced results distinct.
7. Exercise the nonsingular rank-one curve, selected 2-torsion targets, and X-in-V cases without rejecting a genuine low-rank observation.

Any failed equality or source-binding check fails the qualifier and is retained. An ordinary rank below min(d,deg G_X) is a result to investigate, not a failed assertion. Exact finite-fixture comparisons do not establish a universal theorem or a probability bound for conditionally selected curve families.

## Resource and evidence bounds

Use Rust only for mathematical computation, correctness checks and replay. Shell is thin orchestration; Python may handle document rendering and repository metadata only. Copy the qualifier and dependencies to a separate immutable execution snapshot before running. Do not replace a pre-call manifest. Preserve compiler identity, Cargo.lock, source SHA-256, build command/stdout/stderr/exit status, binary pre-call and post-call hashes, raw JSONL, and every failed attempt.

Bound each local development invocation to one process, 2 GiB working memory and 60 seconds. Builds use one job. A timeout or memory failure is recorded; do not silently raise limits or change the fixture grid. One corrected retry is allowed only after a diagnosed implementation/build defect, with a new immutable attempt path and a documented change. An unchanged timed-out command is not retried. No paid compute is invoked by this protocol.

The separate-role reviewer checks source and mathematical scope, then replays the frozen qualifier under the same bounds and compares raw outputs. This is same-host, unsigned reproduction. If a fixed-degree evaluated identity certificate is feasible with the existing native identity checker, retain its exact expression, seed, source binding, fresh-point replay and tamper controls; it cannot discharge the generic theorem. Root-library Cargo validation is a distinct gate and must not be inferred from this standalone qualifier.

## Reporting and stopping criterion

Complete this component when the written theorem has an independent mathematical audit, the fixed fixture qualifier and separate-role replay have actual retained outcomes, exceptions are classified, and a source-linked report, proof map, explanatory figure and PDF are published in the tracking continuation. A formal theorem remains pending until a concrete Lean toolchain/dependency route exists. Any signed PCC standing is obtained from the repository verifier; written proofs and local checks remain unverified assertions until then. [pcc:unverified]

The next algorithmic decision uses the observed distinction between coefficient rank and root support. A general root-finding lower bound, solver speedup, arbitrary-selector result, and full ECDLP conclusion are outside this component.
