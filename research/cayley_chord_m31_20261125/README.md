# Cayley-product M31 signed chord extractor and sampling bound

Development evidence recorded 2026-11-25; repository publication in [PR #5232](https://github.com/isogeny-labs/crypto/pull/5232). See [the visual report](report/REPORT.pdf). Standing: **producer evidence / PCC-unverified**. [pcc:unverified]

## Result

Implemented a signed two-summand Cayley-constrained extractor in Rust and checked its complete output sets against a separately authored symmetric-Semaev implementation on two established ordinary M31 fixtures. All **2,818 cases matched**, including **2,048 unplanted selector cases**, **384 withheld known-positive pairs**, repeated summands, tangent cases, the linear specialization and the identity torus value. All returned pairs passed independent curve-equation, point-addition and direct Fp² Cayley-product checks. The exact same final executable reproduced all case bytes and summary bytes in a second run.

A new double-counting argument identifies the right comparison for candidate selection. For a fixed negation-closed signed base B of size b and uniformly distributed nonzero targets in a group of odd order r, every target-independent distribution of Cayley-product labels has expected unordered B+B candidate count at most b/(r−1) per label. Direct sampling of P in B and testing R−P in B succeeds with probability (b−1)/(r−1). Static Gaussian-smooth label selection therefore cannot by itself give an asymptotic improvement in this average candidate-yield metric. Target-dependent selection, cheaper extraction arithmetic and different relation architectures remain open. [pcc:unverified]

These are a component correctness result and a conditional counting theorem. No elapsed-time comparison, natural factor-base yield experiment, complete DLP recovery, formal Lean certificate or signed PCC verdict is claimed. Standing: **producer evidence / PCC-unverified**. The independent reference and mathematical review were authored by separate workers in this session; that is not a signed external validation.

## 1. Signed extraction through the chord slope

Work on E: y²=x³+a*x+b over Fp, with p=M31=2147483647. Let R=(w,yR) be an affine target with yR nonzero. Write

    C(x) = (x+i)/(x−i),       i² = −1,
    c = C(x1) C(x2).

Since p is 3 modulo 4, x±i are never zero for x in Fp. For c != 1 define

    lambda = i(1+c)/(1−c),
    c = (lambda−i)/(lambda+i).

Then, for s=x1+x2 and q=x1*x2, the constraint is exactly

    q = 1 − lambda*s.

The earlier run first eliminated to a quadratic in s and subsequently recovered ordinate signs. The implementation here instead uses the chord through −R:

    y = m(x−w) − yR.

Substituting this line into the curve equation and dividing by the known factor x−w leaves a quadratic with

    s = m²−w,
    q = w*m²+w²+a+2*m*yR.

Consequently the supplied Cayley constraint is a quadratic directly in the chord slope:

    (w+lambda)m² + 2*yR*m + (w²+a−1−lambda*w) = 0.                 (1)

Its discriminant divided by four simplifies to

    d = w*lambda² + (1−a)*lambda + w+b.                           (2)

For each slope root, solve

    X² − s*X + q = 0,                                           (3)

and recover each ordinate without a further square root:

    yj = m(xj−w) − yR.                                          (4)

Equations (1) and (3) are the two root stages. There can be two slope roots and up to two coordinate roots for each, hence at most four ordered affine output pairs. “Two quadratic stages” does not mean only two square-root evaluations on every call: the second polynomial may be evaluated for each of two slopes, so the generic upper bound is three square-root calls. No separate ordinate square roots are required.

### Completeness and exceptions

Every affine pair P+Q=R with R != O has a nonvertical chord or tangent through −R. Its slope satisfies (1), and its coordinates satisfy (3), including repeated intersections. Conversely, the roots of those polynomials reconstruct the remaining two intersections of that line with E; their sum is R with the signs fixed by (4). The harness checks the full point equations explicitly.

- If lambda=−w, (1) is linear with coefficient 2*yR != 0. The rational map extended to O also has identity-summand decompositions at this label; this interface deliberately returns only affine summands.
- If c=1, the constraint is s=0. Solve m²=w and use q=2w²+a+2*m*yR, then (3) and (4).
- A repeated root is emitted once. P=Q is retained when 2P=R.
- Identity targets, zero-ordinate targets, noncanonical input coordinates/lambda and off-curve targets are rejected. Nonsingularity is a caller precondition, certified for both fixtures.

For the primary curve a=−3, b=21, (1) and (2) become

    (w+lambda)m² + 2*yR*m + w²−4−lambda*w = 0,
    d = w*lambda² + 4*lambda + w+21.

This is an exact algebraic specialization of ordinary chord/Semaev relations. No novelty claim is made for the underlying group law or summation polynomial.

## 2. Exact bound for target-independent product-label sampling

Let B be a negation-closed subset of G\{O}, where G has odd order r. Write b=|B|=2m, and let U be its m distinct Cayley x-labels. The Cayley map is injective on finite x-coordinates. For any c define

    m_c = #{(u,v) in U² : u*v=c} = |U intersect c*U^(−1)| <= m,
    d_c = #{u in U : u²=c}.

For nonzero R, let A_R(c) count distinct unordered signed pairs {P,Q}, allowing P=Q, with P,Q in B, P+Q=R and label c. These are candidate relations before duplicate removal across queries or independent-rank filtering.

There are 4*m_c ordered signed point pairs over the ordered x-label pairs. Exactly 2*d_c of them are opposite pairs with zero sum. Of the remaining ordered pairs, 2*d_c are diagonal pairs P=Q. Passing to unordered pairs therefore gives

    sum_(R != O) A_R(c) = ((4*m_c−2*d_c)+2*d_c)/2 = 2*m_c <= b.  (5)

Let q(c) be any label distribution independent of the particular target R. It may use fixed field, curve, base, Gaussian factorization and preprocessing data. For R uniform on G\{O},

    E[A_R(c)] = sum_c q(c)*2*m_c/(r−1) <= b/(r−1).               (6)

For the direct comparator, assume B is nonempty (so b >= 2), draw P uniformly in B, compute Q=R−P and test Q in B. For each fixed P, exactly b−1 elements Q of B give R != O, because Q=−P is excluded. Thus

    Pr[direct trial succeeds] = (b−1)/(r−1).                     (7)

The ratio of the upper bound in (6) to (7) is b/(b−1), tending to one. This rules out an asymptotic gain in this particular average candidate-count metric from static product-label selection alone. It is not a runtime lower bound. In particular it does not cover target-dependent/adaptive selection, nonuniform target populations, cheaper field/membership arithmetic, more summands, other valid relation forms, or independent-rank effects.

### Deterministic identity check

For each M31 fixture, use B={±[j]G:1<=j<=32}. The harness enumerates this small base, not the whole curve. Both fixtures produced 528 distinct product labels and satisfied (5) for every label. Each had 4,032 ordered nonzero-sum pairs and 2,048 unordered nonzero-sum pairs; the largest label count in these particular bases was four. These are finite correctness checks, not a natural relation-yield benchmark or a general distribution claim.

## 3. Fixture identity and validation results

| Field/parameter | Primary p31-subexp fixture | Secondary native algebraic fixture |
|---|---:|---:|
| p | 2147483647 | 2147483647 |
| a | 2147483644 (−3) | 847514846 |
| b | 21 | 2087444724 |
| Group and subgroup order r | 2147487637 | 2147446831 |
| Cofactor | 1 | 1 |
| Gx | 1391443431 | 625037715 |
| Gy | 177979044 | 2100627251 |
| Main targets | 64 | 64 |
| Paired selector cases | 1409 | 1409 |

Both field/order primalities, nonsingularity, generator equations, unreduced [r]G=O and [(r−1)]G=−G were checked. Prime r and the integer Hasse bound certify the whole group order. The fixtures are distinct; they are not substituted for one another or pooled as the same instance.

| Final development check | Result |
|---|---:|
| Complete candidate/reference set matches | 2818 / 2818 |
| Unplanted supplied-selector cases | 2048 |
| Controlled/exceptional cases | 770 |
| Withheld known-positive pairs recovered | 384 / 384 |
| Empty result sets, matched by both implementations | 1513 |
| Returned ordered pair occurrences across all cases | 3408 |
| Maximum ordered pair count | 4 |
| Linear first-equation cases | 128 |
| Identity-selector cases | 128 |
| Repeated first-discriminant cases | 2 |
| Repeated coordinate-discriminant cases | 128 |
| Rejected malformed input cases | 8 |
| Distinct deterministic child seeds | 24 |
| Second-run byte replay | identical cases and summary |

The 3,408 pair occurrences include controlled cases and may repeat across queries. They are not 3,408 independent factor-base relations. The extractor was never given a hidden target scalar or the expected pair. Known-positive cases are explicitly separated from unplanted selectors.

`reference.rs` was authored by a different worker before it saw `extractor.rs`. It uses independently implemented field/group arithmetic, solves the symmetric Semaev equation, takes ordinate square roots and filters all signs. The candidate reuses the upstream native arithmetic excerpt for square roots and group validation. This is separately authored cross-checking, with shared mathematical assumptions, not formal verification. [pcc:unverified]

## 4. Provenance, reproduction and limits

Run `./replay.sh` after setting RUSTC to a compatible Rust compiler if necessary. The implementation was compiled with Rust 1.100.0, edition 2021, optimization enabled, warnings denied, one code-generation unit and one linker thread. No external Cargo dependencies are required.

`SOURCE.json` pins crypto commit `eb55e93110676c91213d91d6c8d00489311a4bd8`, the full upstream native library hash, the fixture-source hash and the exact arithmetic adaptation. Only the Serialize marker was removed from the extracted source's derive lists. Both repository source files were clean against that commit. The package is a standalone qualification component, not an integrated production-crate change.

The frozen protocol and pre-execution amendment preserve a seed-derivation correction: plain XOR could cancel low-valued fixture/replicate identifiers, so hierarchical mixing was fixed before generating any fixture. All 24 resulting streams are explicitly distinct. Modulo sampling is adequate for this deterministic correctness panel and is not asserted to be an unbiased statistical design.

The first build failed when the default parallel linker threw `Resource temporarily unavailable` under the declared memory cap. A redacted derivative of its log is preserved; `publication/packaging.json` records the original and derivative hashes. Restricting the linker to one thread resolved that build issue. An initial formatter changed whitespace in the arithmetic excerpt; the exact source-bound excerpt was restored before any execution. The successful final source and compiler records are preserved, along with both run outputs. No successful default-parallel build is claimed.

The original November 25 execution phase made no GitHub changes and launched no paid computation. This PR publishes that completed component. The original source review used exact repository files, the current journal, related PRs and primary papers because the research-KB and Conductor tools were unavailable. No signed PCC record was created. The original 535.9x and 14.24-second claims remain source-unverified and played no role in this qualification.

## Publication and evidence custody

The Rust algorithms, upstream snapshots, frozen protocol (archived byte-for-byte in `publication/original-protocol.md`), compiler/source receipts and scientific outputs retain their original bytes. This publication removes private scheduling metadata and internal service history from the narrative; it records the original failed-link log hash and publishes a clearly named path-redacted derivative. `publication/original-manifest.sha256` describes the original handoff, not the curated publication; `PUBLICATION.sha256` inventories the committed study files. The root `PROTOCOL.md` adds only a publication-standing tag to the archived protocol. `publication/packaging.json` explains every deliberate omission or adaptation. [pcc:unverified]

The two 521,012-byte case streams are identical. Their artifact manifest retains their exact common SHA-256 and size. The existing S3-free recovery backend supplies a deterministic gzip copy through `.artifacts/recovery/`; no S3 upload is claimed. The repository's normal artifact reader checks size and SHA-256 before installation. These records remain individually addressable because the replay and source receipts name them. [pcc:unverified]

From the repository root, hydrate and replay with:

```sh
python3 tools/artifacts/artifacts.py pull research/cayley_chord_m31_20261125/evidence --source git
cd research/cayley_chord_m31_20261125
sha256sum -c PUBLICATION.sha256
./replay.sh
```

`replay.sh` also hydrates the two case files if they are absent. Set `RUSTC` to the Rust 1.100.0 compiler used in the frozen run when an exact toolchain replay is required. The harness uses no external crates. The publication replay is a packaging check of the same frozen inputs, not a new sample or timing experiment. [pcc:unverified]

### Publication checks

A fresh Rust 1.100.0 build of this repository-facing package completed the existing replay script. Its two case streams and two summaries matched each other and the original frozen outputs byte-for-byte. Scoped Rust formatting, artifact hydration and edited-prose checks passed. See `publication/validation.json` and the captured replay output. These remain local producer observations. [pcc:unverified]

The root `cargo test --release --lib --offline` gate was attempted separately. The first attempt lacked sparse path dependencies; after they were materialized, a bounded single-job attempt reached its 55-second cap during dependency compilation. No root-library test pass is claimed. No production library source is changed by this PR. [pcc:unverified]

### Proof map and formalization status

The extraction and counting proofs above are written arguments. No Lean certificate, `identity.certificate/v1` record, or signed PCC verdict is supplied. Lean and Lake executables were unavailable in the publication runtime, so no theorem build, axiom audit or fresh kernel check was performed. The repository currently pins Lean 4.35.0; no compatible dependency checkout was available for this component. [pcc:unverified]

| Written result | Hypotheses and current evidence | Open formal obligation |
|---|---|---|
| Cayley selector equivalence | Odd characteristic, -1 nonsquare, finite coordinates, the stated Cayley convention; direct extension-field output checks | Model the quadratic extension and prove denominator nonvanishing, norm-one membership and both directions for c != 1 and c = 1 |
| Chord factorization and half-discriminant | On-curve affine R; README equations (1)-(4); finite exact arithmetic assertions | Generic ring factorization with the on-curve hypothesis and the two algebraic substitutions |
| Extractor soundness and completeness | Nonsingular curve, yR != 0, canonical public inputs, affine summands; 2,818 complete-set comparisons | Connect polynomial intersections to the elliptic group law, including tangencies; prove complete linear/quadratic root enumeration and its correspondence to the Rust outputs |
| Four-output bound | At most two slopes and two ordered coordinate assignments per slope; special branches handled | Prove all branches and duplicate removal respect the bound, rather than assuming generic degree |
| Exact counting identity (5) | Finite odd-order group, negation-closed B excluding O, labels with precisely the +/- fibers; per-label checks on two 64-point bases | Finite fibers, fixed-point-free negation, opposite-pair removal, unordered-pair quotient and diagonal correction |
| Static-label average bound (6) | Uniform nonzero target and a normalized nonnegative label law independent of the target | Finite weighted sum/probability model with explicit independence |
| Direct comparator and ratio (7) | Nonempty B, hence b >= 2; uniform P in B and uniform nonzero target | Success-count proof and positive-denominator ratio argument |
| Two fixture identities | Native primality, nonsingularity, unreduced scalar and Hasse checks | A checked certificate tying the exact fixtures to the formal elliptic group |

The existing polynomial identity checker works over 2^61-1 and does not establish M31 root completeness, elliptic-group equivalence or the finite counting theorem. A certificate for a cross-multiplied algebraic identity would leave those obligations open; no substitute certificate is presented as their proof. Before a future Lean edit, apply the repository's `lean-instance-gate`, reuse the appropriate Mathlib/crypto-lean structures, and pin the actual dependency revision. [pcc:unverified]

Canonical performance graphs and leaderboards are unchanged because this component supplies no measured speed comparison or validated recovery result. The two figures in the report explain the extraction flow and the frozen correctness-panel composition. [pcc:unverified]

## 5. Next decisive work

The extractor is ready for a bounded integration into candidate-selection experiments. The next candidate must state what information about R it uses to predict a useful product label, or explain an arithmetic/membership saving large enough to compensate for its cost. Include direct P-in-B / R−P membership sampling as a comparator; beating uniform torus labels alone is insufficient.

An admissible experiment must freeze the existing M31 fixture, public targets, signed base, cost caps, deterministic independent seed panel and matched resources. Charge every norm trial, factorization, failed label, quadratic solve, membership test, duplicate and rank operation. Inspect useful independent rank or a target-determining nonzero coefficient, and then complete solving cost. Do not infer an advantage from coordinate roots alone.

Resume by reading this README, `PROTOCOL.md`, `SOURCE.json`, `REVIEW.md` and `evidence/result-run1.json`. Reuse the completed correctness gate; do not rerun it merely to recreate the same report.

**Original development impact score: 5/10.** Converted an unexecuted construction into a source-bound Rust component with exact cross-implementation checks, and established a conditional bound that eliminates a misleading candidate-selection comparison. Target-dependent useful relation enrichment remains open.

## Sources checked

1. Igor Semaev, *Summation polynomials and the discrete logarithm problem on elliptic curves* (2004), Theorem 1 and its chord derivation. Primary-author paper mirrored at https://iacr.steepath.eu/2004/031-Summationpolynomialsandthediscretelogarithmproblemonellipticcurves.pdf . The ePrint endpoint returned HTTP 403; the full mirrored paper was read.
2. Aurore Guillevic, *Computing Individual Discrete Logarithms Faster in GF(p^n) with the NFS-DL Algorithm*, https://arxiv.org/abs/1505.07553 . Bibliographic/abstract check only in this iteration; no new smoothness theorem is attributed to that abstract.
3. crypto PR #5049, https://github.com/isogeny-labs/crypto/pull/5049 : existing distinction between a p+1 selector, torus arithmetic and the unresolved elliptic extraction step. Read-only source reuse; no terminal PR maintenance.
4. cryptanalysis PR #1972, https://github.com/isogeny-labs/cryptanalysis/pull/1972 : prior rational-height base investigation and equal-mean counting results. Its reported timings are historical and were not repeated here; the stronger scope claims in its PR body are not assumed as universal runtime theorems.
5. Exact arithmetic source: https://github.com/isogeny-labs/crypto/blob/eb55e93110676c91213d91d6c8d00489311a4bd8/research/prime31_ic_20261115/native/lib.rs .
6. Exact primary fixture source: https://github.com/isogeny-labs/crypto/blob/eb55e93110676c91213d91d6c8d00489311a4bd8/src/cryptanalysis/ic_boundary.rs . [pcc:unverified]

7. Related prior Cayley-coset audit: [cryptanalysis PR #1979](https://github.com/isogeny-labs/cryptanalysis/pull/1979). Different p=15359 Sage/msolve panel; this publication makes no comparative timing or novelty claim against it.
