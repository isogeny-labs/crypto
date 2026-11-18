# EXP-R31-H: exact discriminant control for the S4 rational triple projection (2026-11-15)

**Standing [pcc:unverified].** Ordinary prime-order curve over `p=2147483647`,
`a=847514846`, `b=2087444724`, `#E(Fp)=2147446831`,
`G=(625037715,2100627251)`. This is an algebraic identity and a
classification control, **not** a new subgeneric index-calculus oracle,
complete public-target recovery, or a cold speed result.

## Why this control was added

EXP-R31-G used 9,455 rational quadratic S3 solves to construct all 17,980
distinct triple-abscissa roots for the 31-point depth-six Dickson base. Its
field-only root production was a real stage implementation result, but
eliminating fixed S3 factors is not the same as solving the target-dependent
polynomial ideal. A particularly important confounder is that both source
abscissae are already liftable to fully known curve points. We check whether
that rational S3 projection merely recovers signed ordinary group sums.

### Exact algebraic statement

Write `f(T)=T^3+aT+b`. Regard the standard Semaev cubic
`S3(x,z,U)` as a polynomial in `U`:

```
A2 = (x-z)^2
B1 = -2[(x+z)(xz+a)+2b]
C0 = (xz-a)^2 - 4b(x+z)
S3(x,z,U) = A2 U^2 + B1 U + C0.
```

A polynomial-ring expansion (no probabilistic or heuristically sampled
assumption) yields

```
B1^2 - 4 A2 C0 = 16 f(x) f(z).
```

For actual curve points `P=(x,y)` and `Q=(z,w)`,
`f(x)=y^2` and `f(z)=w^2`. Therefore, for distinct `x,z` and
nonzero `yw`, the two roots in Fp are exactly

```
U_± = [ -B1 ± 4 y w ] / [2 (x-z)^2].
```

These are `x(P+Q)` and `x(P-Q)` as an **unordered** pair.
The finite-field rationality and double-root exceptions can be checked
directly. This identity is ordinary EC/Semaev prior art, **not claimed
novel**; its specific application is an adversarial control on our projection.

By iterating the identity with the ordinate of each intermediate point,
the roots of the specialised three-point `S4` are exactly abscissae of
`±P_i ±P_j ±P_k` modulo global sign, excluding exceptional finite cases.
Thus, when both ordinates and intermediate lifts are supplied, the
`S3 → S4` root projection has a direct generic group-decomposition
implementation. Its finite-field arithmetic alone is not evidence of
a non-generic relation oracle or improved ECDLP complexity.

**Scope qualification:** This does *not* prove a symbolic quotient solver
cannot do something better. A target-dependent ideal with implicit factor-base
constraints may avoid enumerating known liftable points; that is a separate
algorithmic question requiring measured natural extraction.

## Exact prime31 control and reproducibility

New code:
[`examples/prime31_s3_discriminant_gate.rs`](examples/prime31_s3_discriminant_gate.rs).
It uses the repository's actual Rust `Arithmetic`, `Base`, `PRIME31`
and group law, with separately implemented small finite-field formulas.
The method rejects diagonal, invalid ordinate, double-root, collision and
infinite-point cases instead of silently losing witnesses.

The deterministic all-pair/all-triple source-domain fixture expects:

| Observable | Registered value |
|---|---:|
| Canonical Dickson points | 31 |
| Pairs with i<j | 465 |
| Triples with i<j<k | 4495 |
| S3 calls on intermediate signed pairs | 8990 |
| Total rational S3 quadratic calls | 9455 |
| Distinct output abscissae | 17980 |
| Little-endian 32-bit root corpus SHA-256 | `a71a99940794aead48642690575a37be72ef9994a93c9bbace47b538307d126d` |
| Independent FNV1a-64 root corpus | `cc15d719e3eaf8c2` |

A separate producer-local Python evaluation of the entire 465+8990-domain,
using the explicit discriminant formula, independently calculated finite
group additions and the existing deterministic fibre, reproduced 17,980
distinct roots and the SHA-256 above on 2026-11-15. These are **not**
signed PCC-verification claims. Rust source was committed before exact-head
compilation and remains execution-unverified until CI for that source passes.

To run after an admitted checkout:

```sh
cargo +1.100.0 test --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_s3_discriminant_gate
cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_s3_discriminant_gate
```

The group-addition operation counter in this control measures **post hoc
semantic validation plus acquisition of intermediate point ordinates**.
It is not included in the earlier finite-field-only S4 projection timer and
cannot be treated as a free oracle. Do not rank the two as full algorithms.

## Prior leads and exclusions checked before this edit

- [#4188](https://github.com/isogeny-labs/crypto/pull/4188):
  m=3 projective-rank duplicates, m=8 raw support and bounded algebraic F4
  oversize, pair S3 elimination and S4 field projection. Current continuation.
- [#4077](https://github.com/isogeny-labs/crypto/pull/4077):
  full-call `γ,δ` useful-rank gate, not raw witness counts.
- [#4228](https://github.com/isogeny-labs/crypto/pull/4228):
  certified quotient solver and S3 resultant path; prospective symbolic
  target-dependent competitor, not yet measured on these S4 systems.
- [#4275](https://github.com/isogeny-labs/crypto/pull/4275):
  dense versus tower F4 experiments; planted positives do not replace
  natural-target oracle throughput.
- [#3877](https://github.com/isogeny-labs/crypto/pull/3877),
  [cryptanalysis #1536](https://github.com/isogeny-labs/cryptanalysis/pull/1536),
  [autoresearcher #3305](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3305):
  transfers, auxiliary IC and lead atlas respectively; no checked-in
  corresponding prime31 cold algebraic win was established by this review.
- Petit–Kosters–Messeng, PKC 2016,
  [DOI](https://doi.org/10.1007/978-3-662-49387-8_1), and Diem,
  *On the discrete logarithm problem in elliptic curves*,
  [publisher](https://www.cambridge.org/core/journals/compositio-mathematica/article/on-the-discrete-logarithm-problem-in-elliptic-curves/59B877810708C90F6287972486A5BF0C).
  Both precede this technique. The S3 roots/EC group law identity is
  classical; the finite-instance classification check is not a novel
  complexity breakthrough.
- Shared research KB MCP, local wrapper, Conductor, exact-head Rust toolchain
  and EKS/checkpoint admission unavailable in this runtime. Review falls
  back to these bounded primary repository sources. No absence/novelty
  inference from an inaccessible KB.

## Next experiments (falsifiable and directly on this curve)

**H1 — genuine symbolic target intersection (highest priority).**
Use #4228's quotient/subresultant machinery or a mature independently checked
CAS to solve the actual target-dependent central
`S4(u,v,w,x_R)=0` *modulo* three fixed factor-base projection
constraints `K3(u)=0`, `K3(v)=0`, `H2(w)=0`.
Begin with a reduced selected factor base and fresh planted-positive,
independently negative and natural masked targets, then restore B=31.
Retain exact subresultants, certificates, input-membership checks,
nonunit/positive/repeated-root failures and all charged construction.
**Pass** requires natural positive *extraction without a table of group
sums or supplied intermediate ordinates*. A group- or x-table join is
explicitly the separate generic-decomposition control.

**H2 — useful-rank and full cold recoveries.** After H1 succeeds, collect
linear relations for `R=aG+bQ` on 12 preregistered public prime31
targets (four held out), with unknown `log_G Q` inaccessible to the oracle.
Deduplicate projective signed rows, count actual modular rank increments,
fail/censor the original attempts, replay all EC equations and recover each
scalar. Charge initialization, all unsuccessful algebraic calls,
certificates, extraction, linear algebra and scalar verification.
Compare strongest optimized Rust rho, folded rho if applicable, and
BSGS on the same CPU and input panel, with `S` at 0/0.1/1 GAE.

**H3 — stopping/crossover test.** The current 17,980-squarefree arm
projection is an expensive expansion; demand a degree/matrix/memory
estimate and a small-instance scaling trace of the genuine target
intersection before investing in B=135 or 256. The old unstructured
cross-product would enumerate `17980^2*930=300650772000` tuples
per target; that is not a mathematical lower bound on symbolic quotient
work. Stop a proposed `K3×K3×H2` enumerator before spending time on
it. Record negative results and do not reopen the same lead without an
explicit cost-changing mechanism.

The acceptance criterion remains a **complete, independently replayed
Rust prime31 algebraic index-calculus DLP recovery cheaper than the
strongest cold matched rho**. This control makes the absence of a generic
group table and the use of the actual target-dependent equations an
explicit, testable requirement. No claimed completion, rho win, spend,
EKS run or leaderboard change follows from this audit.

## Executed Rust development gate (2026-11-15, [pcc:unverified])

[GitHub Actions run 43735784837](https://github.com/isogeny-labs/crypto/actions/runs/43735784837),
job `132877986877`, passed all steps on exact source/workflow commit
`6a6bdff372e9d2c3fa26cef4b2e4989eee018b6b`.
The new Rust example test passed `1/1`; the release binary returned:

```json
{
  "pairs": 465,
  "triple_s3_calls": 8990,
  "rational_quadratic_calls": 9455,
  "distinct_triple_abscissae": 17980,
  "root_fnv1a_le32": "cc15d719e3eaf8c2",
  "ec_group_additions_post_root_identity": 18910,
  "classification": "generic decomposition; ordinate inputs and EC law reveal signed sums",
  "target_dependent_polynomial_solver": false,
  "cold_ECDLP_solve": false
}
```

Exact original output and source-run identification are preserved in
[`S3_DISCRIMINANT_CI_EVIDENCE_20261115.json`](S3_DISCRIMINANT_CI_EVIDENCE_20261115.json).
The 18,910 counted group additions are the **semantics-control** computation
(930 pair additions plus 17,980 triple additions); the check does not establish
that this amount is minimal or that it is a cheap complete ECDLP pipeline.
The complete focused CI job passed including inherited source reuse and
focused Clippy; it does not establish a passing full root-release library
suite. This is developer execution evidence **only**, not a signed PCC or
independent scientific validator receipt.
