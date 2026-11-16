# Balanced S4/S3 prime31 m8 — algebraic input bridge and natural witness controls

**Date:** 2026-11-15. **Standing:** [pcc:unverified]. Source checked into the existing PR, producer-local independent Python arithmetic tested; exact-head Rust compilation/CI and independent PCC replay pending. **This is not an algebraic relation finder and does not demonstrate a cheaper complete ECDLP solver.**

## Curve, inputs and previous work

Reuse the already registered nonsingular ordinary curve over the Mersenne *field* prime **p=2147483647=2^31-1**:
- E: y²=x³+847514846x+2087444724,
- full *group* prime order **n=2147446831**, cofactor 1,
- generator G=(625037715,2100627251) and public Q=(1547953785,1667575595),
- Dickson depth 6 fibre terminal 933677148: 64 raw x-values but **31 actual liftable points**.

The parent experiment [M8_SPLIT_CONTROL.md](M8_SPLIT_CONTROL.md) collected 32 independent affine relation rows from 124 unplanted public masks using a **generic group 4+4 signed hash join**, recovered the public target and counted 3,827,503 inclusive group additions. It was a complete index-calculus *control*, but not an algebraic/efficient oracle. This file does not reclassify that experiment.

The previously identified implementation blocker for a naive S3 m8 chain was **six auxiliary free x-coordinates** versus the packed Rust F4 engine's **MAX_FREE=4**. Instead of modifying the 183kB packed monomial engine or duplicating it, the present bridge uses a higher-degree, lower-free-variable **balanced S4/S3 system**.

## Exact balanced algebraic input

For eight factor-base abscissae x0,...,x7 and the public target R=[a]G+[b]Q, introduce exactly three intermediate abscissae u,v,w and solve

    S4(x0,x1,x2,u) = 0
    S4(x3,x4,x5,v) = 0
    S3(x6,x7,w)    = 0
    S4(u,v,w,x(R)) = 0.

Each xi satisfies the existing depth-6 quadratic Dickson tower and its final top rule. There are 8*6=48 quotient-tower variables plus **three free variables**. The existing src/cryptanalysis/f4_fp_tower.rs::TowerRing supports 64 tower and four free variables: this input fits without touching MAX_FREE or changing source engines.

S4 is the classical quadratic resultant. Write S3(x,y,t)=A t²+B t+C and S3(z,w,t)=D t²+E t+F. Then

    S4(x,y,z,w) = (A F-C D)^2 - (A E-B D)(B F-C E).

This is established Semaev polynomial algebra, **not a novel complexity result**. In the first frozen natural mask the exact producer-local polynomial expansion and independently written Dickson reduction give:

| Equation | Raw terms | Quotient-normal-form terms | Normal-form total degree |
|---|---:|---:|---:|
| S4(x0,x1,x2,u) | 439 | 526 | 9 |
| S4(x3,x4,x5,v) | 439 | 526 | 9 |
| S3(x6,x7,w) | 13 | 16 | 4 |
| S4(u,v,w,xR) | 125 | 125 | 12 |

These are *input support counts*, not Gröbner solving degree, rank, F4 work or speed.

### Semantic preflight actually executed

An independently written local Python EC implementation recomputed the depth-6 base and all **32 recorded unplanted positive-mask witness rows**, using the frozen public transcript in M8_SPLIT_CONTROL.results.json as the witness *source* (so the positives are already known). It checked:

- **96 S4 and 32 S3** polynomial residuals were exactly zero modulo p;
- **32/32** signed-y point-sum identities agree with the full public R, with no intermediate point at infinity;
- changing x(R) to x(R)+1 rejected **32/32** fixture certificates;
- for each of the **32** distinct eight-x tuples, all **256** y-sign assignments were enumerated, producing **256 distinct group sums**. Exactly **one** gave the complete public R, while **two** gave x(R) (the R and -R ambiguity);
- first tuple's (u,v,w,xR)=(803189040,898740481,729813404,903384438).

This producer-local test confirms the *semantic bridge* but is **not** natural-positive F4 witness discovery and is not a signed independent PCC certificate. It does not refute x-only extraneous components or prove that the full ideal can be solved efficiently. The sign-extraction phase must require the complete public y-coordinate and reject wrong-orientation solutions.

## Rust implementation and reproductions

The new [Rust example](examples/prime31_m8_balanced_f4_input.rs) reuses the existing native Arithmetic, Base, PRIME31, and the **original** src/cryptanalysis/f4_fp_tower.rs TowerRing::from_raw and eval. It constructs the four *actual quotient-normal-form* polynomial inputs, replays each of the 32 natural positive witnesses, checks the complete group equality and all 256 orientations of each leaf-x tuple, and fails closed if support/certificate statistics drift. It does **not** invoke the expensive F4 reduction yet and does not use the previous generic meet-in-middle to find new rows.

The [separate Python replay](replay_balanced_m8_s4.py) recomputes the same checks independently of Rust and keeps the original transcript immutable. The focused .github/workflows/prime31-algebraic-ic.yml now runs both source checks on pushes. The native workflow is a development correctness gate, not an independent signed replay, benchmark, or PCC promotion.

    cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_m8_balanced_f4_input
    python3 research/prime31_ic_20261115/native/replay_balanced_m8_s4.py

The selected producer runtime has no cargo/rustc, so only separately implemented **Python exact-field, EC, and symbolic-poly checks** were actually executed here. There were no reported GitHub commit workflow runs when queried. Do not claim the Rust example compiled, that F4 solved a positive or negative query, or that measured cold cost improved.

## Prior leads and novelty constraints checked

| Existing work | What we take from it |
|---|---|
| [PKM prime-field paper (2016)](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf) | Polynomial tower factor bases and summation systems are published prior art; lower-degree map composition is not automatically fast |
| [crypto #4091](https://github.com/isogeny-labs/crypto/pull/4091) and [#4275](https://github.com/isogeny-labs/crypto/pull/4275) | Rust PKM algebraic solver is now merged on main; planted-small-system dense membership often beats tower but generic enumeration still won; do not extrapolate to natural m8 |
| [crypto #4077](https://github.com/isogeny-labs/crypto/pull/4077) | Frozen natural-positive/negative m8 all-call cost protocol and gamma/delta useful-rank gate gamma < 3+delta/2 |
| [crypto #4228](https://github.com/isogeny-labs/crypto/pull/4228) | S3 quotient work with honest construction cost and degree accounting; don't confuse S4's input degree with solve degree |
| [M3 projective audit](M3_PROJECTIVE_ROW_AUDIT.md) | Raw collisions may represent the same relation; projective dedup then incremental independent row rank is mandatory |
| [cryptanalysis #1536](https://github.com/isogeny-labs/cryptanalysis/pull/1536), [crypto #3877](https://github.com/isogeny-labs/crypto/pull/3877), [autoresearcher #3305](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3305) | Auxiliary-map and chained-transfer alternatives remain separate scalar-blind cost gates; no confirmed cheaper complete prime-field route imported |

KB status/search and Conductor were unavailable in this tool environment, so this bounded repository/literature review is **not** exhaustive. No novelty claim is made.

## Next falsifiable experiment: actual algebraic m8 oracle

1. Confirm the Rust bridge's exact-head CI, semantic replay, term counts and 256-orientation checks.
2. Rebase/integrate with *current merged main* PKM dense/tower tooling. Submit the four S4/S3 quotient-normal-form equations to the existing Rust F4 solver under explicit degree, matrix-entry, memory and timeout caps. Capture full negative, censored and naturally positive cases on the **same 124 public masks**, not an all-positive transcript-only fit. Extract **every** real finite-field solution and exact signs; no hidden-log or generic hash-join oracle fallback.
3. Preregister an equal-input comparison against the six-free S3 chain via a compatible generic F4 backend, and the existing dense and tower membership formulations. Balanced S4 may reduce free variables but increase regularity/solving degree. Stop it if positive witness extraction fails or costs are above generic enumeration; count every failure.
4. Deduplicate equivalent signed rows, measure rank increments instead of raw witness totals; charge construction, factor-base selection, all failed calls, witness lifting/orientation, sparse LA, target verification, total memory and cold wall/GAE.
5. Repeat two held-out larger sizes, fit complete-work gamma jointly with independent useful-output exponent delta, and compare complete scalar recovery against same-host **optimized negation-map rho** plus external Gröbner tooling before claiming a cheaper solver.

**Decision:** worth an inexpensive bounded algebraic solver attempt because it removes a real input-representation blocker *without rewriting F4*. Whether the higher-degree S4 system is solvable cheaply is unknown; the strongest measured earlier complete prime31 IC control remains slower than the generic baselines. [pcc:unverified]
