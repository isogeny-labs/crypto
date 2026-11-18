# Reproduced prime31 Rust APS single-relation speedup: fresh 24-Q 384-profile confirmation

**Date:** 2026-11-16 UTC. **Standing:** two successfully executed, source-pinned GitHub Actions *developer* runs; all unknown logs independently replayed in Rust; **[pcc:unverified]** pending independently signed/isolated measurement and external index-calculus frontier comparison. No new ECDLP asymptotic or curve-security claim.

## Predeclared experiment, exact sources

Same ordinary cofactor-one curve (no smooth-order shortcut):

    p=2147483647
    E:y²=x³+847514846*x+2087444724
    n=2147446831 prime, cofactor 1
    G=(625037715,2100627251)

The Rust solver is the inherited published APS-style single-S3-relation implementation `prime31_aps_one_relation.rs`. It samples fresh labelled public walk points `R_i=a_iG+b_iQ`, discovers one S3 relation using finite-field quadratics, checks all signed EC point identities and independently validates `[k]G=Q` for computed `k`; test fixture secrets are *caller-side only*. No hidden generic group hash join feeds candidate discovery, and all source/preparation/retries/verification are charged. The algorithm still enumerates finite pair-root support; **it is mathematically generic MITM-equivalent**, despite the APS index-calculus methodology and algebraic implementation.

- Pilot preregistration: [APS_UNEQUAL_PROFILE_PROTOCOL_20261116.md](APS_UNEQUAL_PROFILE_PROTOCOL_20261116.md), prior to program source and pilot CI.
- **Exploratory pilot:** [Actions #43817341546](https://github.com/isogeny-labs/crypto/actions/runs/43817341546), job `133132277587`, source **`fbf312c09005080f3032d4f2232a7cbd41c2bea1`**. [Full source-exact 24×4 raw JSON](APS_UNEQUAL_PROFILE_EXACT_CI_20261116.json), [pilot analysis](APS_UNEQUAL_PROFILE_PILOT_CI_20261116.md). 96/96 complete recovered target scalars.
- **Fresh validation preregistration**: [APS_UNEQUAL_PROFILE_384_CONFIRM_PROTOCOL_20261116.md](APS_UNEQUAL_PROFILE_384_CONFIRM_PROTOCOL_20261116.md), committed **before** new 24 Q source/CI execution.
- **Actual fresh confirmation:** [Actions #43817946682](https://github.com/isogeny-labs/crypto/actions/runs/43817946682), job `133134181765`, source **`42014773f0b79ff23e3ad2fd697cb03341a1802e`**. [Full source-exact 24×3 raw JSON](APS_UNEQUAL_PROFILE_384_FRESH_CI_20261116.json). 72/72 complete verified unknown scalars; printed `passes_preregistered_development_gate=true`.

## Measured full-cold results — no partial-oracle denominator

| Same-host Rust arm | Pilot: 24 Q, cold total | Fresh confirmation: 24 new Q, cold total | Fresh successes |
|---|---:|---:|---:|
| Equal `[1536,1536,1536]` control | **1,335.138 ms** | **1,111.688 ms** | 24/24 |
| Asymmetric `[384,384,16384]` | **258.271 ms** | **278.384 ms** | 24/24 |
| Asymmetric `[384,384,16384]` A/A repeat | not in pilot | **276.236 ms** | 24/24 |
| Asymmetric `[256,256,32768]` | 380.740 ms | not selected | 24/24 (pilot) |
| Asymmetric `[256,256,32768]` A/A repeat | 369.338 ms | not selected | 24/24 (pilot) |

**Fresh holdout 384 speedup = 1,111,687,738 / 278,384,243 = 3.99336× fully cold.** 384 wins 21/24 matched fresh targets (pilot 22/24). Fresh 384 A/A total is 276,235,958 ns, **0.7717% lower** than its first arm, passing the predeclared <=5% drift bound. Total fresh S3 quadratic evaluations: equal **9,291,264**, 384 **877,824** (10.58× fewer). Fully charged source-generation group additions: equal **259,153**, 384 **560,739** (2.16× more). Six-round caps were paid; all arms completed all 24 Q. This *reproduces* a substantial source/match-specific complete-algorithm optimization, not merely a component microbenchmark.

The pilot's 384 profile was selected *from* the exploratory arms; the fresh panel was frozen with a new distinct `Q` PRNG seed `0x434f4e465f333834`, its selection held fixed, order alternated, and a direct 384 A/A repeated arm included. The fresh confirmation cleared all preregistered gates: 24/24 correct, >=20% cold total reduction, <=5% A/A drift, and >=18/24 per-Q wins (observed 21/24). It is development evidence, not independently isolated L2/PCC confidence analysis. Source stdout is frozen in both JSON records, including every individual Q, attempts, signed scalar and source-operation counter.

## Mathematical explanation, restrictions, and prior-art audit

The S3 root computation for the first two lists costs approximately `A*B` pair quadratics. A lookup in the third abscissa list has success chance growing with its cardinality `C`; larger C requires more public labelled points and group additions. Equal `T=1536` is not optimal in this Rust finite group at this source/field cost ratio. The unequal method trades **a larger public point source** for far fewer expensive finite-field S3 square-root queries. Additive-walk point dependencies mean Poisson relation probabilities are only hypotheses, not used to certify success.

Prior existing research (not novelty):
- Amadori–Pintore–Sala, *On the discrete logarithm problem for prime-field elliptic curves* (FFA 2018, DOI 10.1016/j.ffa.2018.01.009), established single-relation IC methodology; we reuse it.
- Petit–Kosters–Messeng, *Algebraic approaches for ECDLP over prime fields* (PKC 2016, DOI 10.1007/978-3-662-49387-8_1), separate prime-field polynomial IC baseline.
- `research/aps_cost_leads_20261106/PROTOCOL.md` *already* includes unequal-profile APS experiments on smaller primes; this is not a first unequal-list technique.
- #4383 equal-size T1024 improvement **reversed** on fresh targets; this motivated selecting/confirming independently. #4391 existing APS additive walk and Mersenne-fold arithmetic are reused. #4373 chained S3 m8, #4188 ordinary finite support, and #4391 anchored m6 returned negative/matrix-censored controls. #4228/#4349 certified quotient remains limited for wide m8; don't extrapolate.
- The classical symmetric Semaev program is already implemented in `isogeny-labs/cryptanalysis/suite/src/cryptanalysis/symmetrized_semaev.rs` after Faugère–Gaudry–Huot–Renault (J. Cryptology 2014), so no Semaev/Vieta novelty claimed.
- Shared research-KB tools and local wrapper/EKS are unavailable in this connector-only runtime; GitHub primary notes plus verified published metadata are the bounded fallback, not proof of novelty or priority.

**Scope boundary:** the new **complete APS-derived Rust implementation** is substantially cheaper than our previous complete APS Rust implementation on fresh prime31 heldouts. This is not a completed full-cost comparison against mature PKM, APS, msolve, Magma, optimized negation-rho, or an independently reproduced non-generic algorithm. A large per-run wall advantage here does not imply an improved asymptotic ECDLP complexity or reduced security of real-world curves.

## Next novel/non-generic lead and falsifiable gates

1. **Explicit symmetric m8 quotient (highest-priority mathematics):** for the 31 admitted prime31 factor-base abscissae set `S`, let `H(X)=∏_{x∈S}(X-x)`. Represent the unordered eight-leaf multiset by `f(T)=T^8-e_1T^7+⋯+e_8`, with exact membership condition `f|H^8` (or `f|H` for distinct leaves). The radical finite solution set has `binom(38,8)=48,903,492` points vs `31^8=852,891,037,441` ordered leaf assignments; these are mathematical counts, not root-solving runtimes. Reuse the prior symmetric Semaev code and certify a target-dependent `S9`/balanced-resultant formula in the elementary coefficients. Root recovery must preserve repeated leaves, cancellation and identity and verify the original signed full point. **No generic group-sum lookups may propose roots.**
2. **Small-B first:** compare B4/B8 complete rational root sets, original-ideal certificates and actual end-to-end extraction costs against the inherited F4 and #4349 selected-root worker. A finite dimension smaller on paper is not itself a speed improvement.
3. **B31 natural heldouts and frontier:** after actual algebraic m8 roots, show natural relation yield, independent rank or APS-style single relation, full unknown-log recovery, all failed/censored calls, fully cold PKM/APS/native F4/certified quotient and external CAS on the same original input. Optimized rho is the separate security baseline. Independently signed PCC and at least one other ordinary curve required before novel complexity/security claims.
4. **Engineering follow-up:** preserve the 384 profile as opt-in; run L2 isolated randomized-order A/A and expand fresh target population before changing the production default or a leaderboard. Avoid treating any partial F4 timing as a complete solver denominator.

## Reproduction

`cargo +1.100.0 test --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_aps_one_relation`

`cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_aps_one_relation -- --unequal-confirm`

`cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_aps_one_relation -- --unequal-384-confirm`

The exact measured output depends on the CI runner and source pin. Rust code/tests and full raw evidence are in this PR; no Python/C++ solver or paid GPU job was used.
