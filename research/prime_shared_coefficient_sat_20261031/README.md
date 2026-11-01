# Prime-field shared-coefficient solver discriminator

## Outcome

This subtree tests the **odd-arity shared-divisor-coefficient system with
quotient/remainder membership** as one canonical predicate across exact
enumeration, F4, CDCL/CNF, CNF-XOR and finite-field SMT.

The hardened
[31 October lead review](../prime_ic_lead_review_20261031/REPORT.md) ranks the
hashed-factor-base Boolean-tensor route first and instantiates the rank-two
practical solver experiment as a fixed-arity-three chained-`S3` CDCL(T)/`F_p`
screen. This quotient/remainder presentation is a representation candidate in
that shared-coefficient mechanism family, not yet a production exponent
screen. Its exact quadratic input, growing-arity witnesses and complete tiny
panel make it a useful E0/E1 discriminator without confusing solvers that see
different mathematical predicates.

The implemented E0 gate passes:

- the historical `m=3,5,7`, `t=4` selected-chart witnesses satisfy exactly
  `35`, `60`, and `85` generated equations over F17;
- corrupting `a0` is rejected in every fixture;
- for every one of the 18 affine targets on
  `y^2=x^3+2x+2` over F17, an algebraic coefficient enumerator and an
  algorithmically separate elliptic-curve group/interpolation oracle produce
  the same projected coefficient-model set;
- the census is 6 SAT targets, 12 UNSAT targets, and 6 models in each
  implementation, with 18/18 exact set matches.

This is development correctness, not solver evidence. E0 ran no external
solver. Subsequent preparation built and validated pinned solver toolchains,
but no external solver has seen a study instance, no timing was collected, and
no index-calculus speedup is claimed.

## Claim ledger

| Statement | Status | Evidence or obligation |
|---|---|---|
| The selected-chart remainder construction has a deterministic quadratic IR | supported for the implemented construction | strict generator/parser plus legacy fixture conformance |
| The arity-three F17 predicate represents the intended rational, distinct-root decompositions | supported on the complete fixed toy panel | 18/18 exact projected coefficient-model-set matches against an algorithmically separate same-author group/interpolation oracle |
| SAT/XOR beats F4 or enumeration | unknown | E1 exporter equivalence has not run; no performance screen is admitted |
| The mechanism has exponent-changing scaling | unknown | requires a prospectively frozen fixed-`m=3` complete-cost panel with `e<0.5` and `a<1.5` |
| The mechanism improves relation collection or ECDLP | unknown | requires E4 relation extraction, matrix construction, log replay and complete cost accounting |
| A finite solver panel proves a subexponential algorithm | out of scope | requires a family theorem and a complete asymptotic cost proof |

## Construction under test

For odd `m=2h-1`, use a monic polynomial `A` of degree `h`, a polynomial `C`
of degree at most `h-2`, and a monic root polynomial `U` of degree `m`. For an
affine target `R=(r_x,r_y)`, impose

```text
A(r_x) - r_y C(r_x) = 0
A(X)^2 - (X^3+aX+b) C(X)^2 = (X-r_x) U(X).
```

Membership in the split `2^t`-power coset is represented by shared remainder
coefficients:

```text
R_0 = X
R_{j-1}^2 = Q_j U + R_j       for 1 <= j <= t
R_t = c.
```

Bezout equations enforce `gcd(U,C)=1` and `gcd(U,U')=1`; one inverse equation
enforces `U(r_x) != 0`. Every generated input equation has total degree at most
two. All field elements are canonical decimal strings in the JSON IR, variable
and monomial order is deterministic, and malformed or trailing input is
rejected.

The native tool in [`native`](native/) emits:

- strict canonical JSON (`prime-shared-coefficient.ir/v1`);
- msolve prime-field polynomial input;
- cvc5 `QF_FF` input;
- ordinary CNF and CNF-XOR from one deterministic functional Boolean circuit;
- strict msolve rational-parametrization parsing with Fp extraction and
  canonical full-assignment replay;
- fail-closed, hash-pinned SAT and msolve correctness-panel runners;
- the exhaustive E0 F17 census.

The Boolean lowering and a fail-closed external-solver runner are implemented
and frozen in [E1_PROTOCOL.md](E1_PROTOCOL.md). The fixed study panel has not
yet run. Its CaDiCaL/CNF and CryptoMiniSat/CNF-XOR projected model sets and
checked UNSAT proofs must agree with the exact oracle before any performance
row is admissible. The separate
[comparator protocol](E1_COMPARATOR_PROTOCOL.md) freezes an msolve 0.11.0
base-field cross-check; it is also unrun. cvc5 1.4.2 is proof-ineligible for
finite-field UNSAT under the checked-proof policy and remains optional for SAT
model cross-checking only. Its exact source/reference register is
[`e1-comparator-inputs.sha256`](e1-comparator-inputs.sha256).

## Experiments

The staged protocol is in [PROTOCOL.md](PROTOCOL.md), and the machine-readable
status is in [experiment-matrix.json](experiment-matrix.json).

| Stage | Question | Current status | Decisive output |
|---|---|---|---|
| E0 | Is the canonical predicate internally and independently correct on tiny complete domains? | complete | exact set equality, legacy conformance, mutation rejection |
| E1 | Do JSON, msolve, QF_FF, CNF and CNF-XOR preserve the same projected models? | SAT and msolve subgates frozen, not run; cvc5 proof-ineligible | exhaustive model-set equivalence; independent checked certificates for every accepted UNSAT |
| E2 | Can this representation be integrated into an admissible fixed-`m=3` complete-cost protocol? | blocked | semantic-object decision, source-bound `F_p` theory, setup/query ledger, power and resource receipts |
| E3 | At fixed `m=3`, can one proof-eligible CDCL(T)/`F_p` bundle achieve natural-query `e<0.5` and reusable-setup `a<1.5`? | parent protocol is a hardened draft; this subtree is not admitted | complete `s=6..14` natural panel plus separately reported planted/UNSAT controls |
| E4 | Does an advancing bundle improve a complete relation-collection pipeline? | conditional | verified relations, matrix rank/log replay and end-to-end cost |
| E5 | Can an observed mechanism be proved for the family? | conditional | a theorem bounding solver work or a certified structural invariant |

The earlier variable-arity `(m,t)=(3,3),(5,4),(7,5),(9,6)` E3 draft and its
`0.85/0.95` screening thresholds were withdrawn before any performance data.
They do not implement the complete index-calculus cost bridge. Any later
performance amendment must hold `m=3` fixed, use actual `log2(p)/3`, account
separately for reusable setup and online-plus-verification work, and apply the
parent protocol's strict `a<1.5` and `e<0.5` gates on a complete natural panel.

## Why this is the SAT representation worth checking

Prior direct Semaev bit-blasts and prime-field QF_FF coordinate encodings were
poor, but they did not test this shared-coefficient predicate. Here the same
`U` coefficients occur in every remainder layer. That repeated incidence is a
specific propagation hypothesis worth checking at E1. It becomes a
performance hypothesis only after a prospective amendment either proves
projected-model equivalence to the parent's fixed-`m=3` semantic object or
registers this as a separate representation with the same complete-cost
accounting.

The comparison object is the complete **representation plus solver bundle**.
It would be invalid to attribute a win to CDCL alone, or to compare SAT on the
selected chart with F4 on a larger repeated-root chart.

## Sources and prior work checked

- Petit--Kosters--Messeng, *Algebraic Approaches for the Elliptic Curve
  Discrete Logarithm Problem over Prime Fields* (PKC 2016), sections 3.1--3.5,
  4.2 and 5; pinned PDF SHA-256
  `a53030a6cde720c354aebfb65b1eb01e07766ce2d26e543d77f2516c0a49ef5b`
  in the [source register](../prime_one_shot_20261029/SOURCES.json).
- The [one-shot construction report](../prime_one_shot_20261029/README.md),
  prototype exports and continuation reviews, including the rationality and
  repeated-root chart cautions.
- The [prime-field lead review](../prime_ic_lead_review_20261031/README.md) and
  its [formal draft protocol](../prime_ic_lead_review_20261031/protocol.draft.json).
- The [prior SAT/SMT design](../sat_smt_prime_ic_design_20261030/DESIGN.md),
  which rules out merely repeating the direct Semaev representation.
- Official msolve input/output and genericity-handling source, cvc5
  finite-field proof behavior, CaDiCaL DIMACS/LRAT documentation and
  CryptoMiniSat CNF-XOR/XLRUP documentation were checked for exporter,
  parametrization and certificate obligations. The pinned development
  toolchains passed non-study smoke checks; none was benchmarked or run on E0
  study inputs.

No novelty claim is made for divisor coefficients, quotient/remainder
membership, SAT, XOR reasoning, F4, or PKM factor bases. The open contribution
is a fair exact discriminator and, only if it survives, a scalable mechanism
and complete index-calculus cost proof.

## Reproduction

With Rust 1.78 or newer:

```bash
cd research/prime_shared_coefficient_sat_20261031/native
cargo test --release --locked
cargo run --release --locked -- e0-panel
cargo run --release --locked -- emit 17 2 2 3 4 1 0 6 json
cargo run --release --locked -- emit 17 2 2 3 4 1 0 6 msolve
cargo run --release --locked -- emit 17 2 2 3 4 1 0 6 qf-ff
cargo run --release --locked -- emit 17 2 2 3 4 1 0 6 cnf
cargo run --release --locked -- emit 17 2 2 3 4 1 0 6 cnf-xor
```

The study runners are deliberately not included in the ordinary reproduction
block. They require the exact frozen binaries, a 40-hex source revision, a
fresh output directory, and a successful repository checkpoint gate. See
[E1_PROTOCOL.md](E1_PROTOCOL.md) and
[E1_COMPARATOR_PROTOCOL.md](E1_COMPARATOR_PROTOCOL.md) for their only
admissible invocations.

See [VALIDATION.md](VALIDATION.md) for the exact checks and environment
limitations of this handoff.
