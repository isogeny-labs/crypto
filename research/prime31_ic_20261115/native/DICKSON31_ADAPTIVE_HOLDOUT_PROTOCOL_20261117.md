# Frozen protocol: prime31 Dickson31 adaptive 8A membership holdout

Date: 2026-11-17 UTC. Owner PR #4615. Status: prospectively specified, before new holdout outcomes. `[pcc:unverified]`.

## Scope and provenance

**Actual ordinary cofactor-one curve** E/F_p: p=2147483647, a=847514846, b=2087444724, n=2147446831, G=(625037715,2100627251); verify with native `check_curve`. Use `Base::build(PRIME31,6)` to recover the **actual** 31-point Dickson depth-six factor base, *not* first-31 rational abscissae.

Reuse the existing Rust signed 4A index `FourSet::build` / `recover` and native point arithmetic in `research/prime31_ic_20261115/native/examples/prime31_s9_b31_half_sum_oracle.rs`. The 4A+4A group oracle is an **explicit generic MITM control**, *not* a new non-generic polynomial relation extractor or an unknown-scalar ECDLP solver. The main program and historical 24-natural/16-planted fixture must stay unchanged; add an isolated test module.

## Prospective holdout and cost accounting

Use exactly **96 fresh public synthetic points** Q=[k]G with reproducible SplitMix64 state `0x533942484f4c4431`, independent of the earlier NATURAL_SEED; generate k in 1..n-1 by `1 + splitmix64(&mut state) % (n-1)`, discard duplicate k, and never pass k to `FourSet::recover`. This modulo sampler is deterministic and has slight bias; do not describe it as mathematically exact uniform sampling.

For each Q attempt public residuals R=Q+[a]G, in fixed order `a=0,1,2,...,8`, using native `Arithmetic::scalar` for the public shift and `Arithmetic::add` for the target combination. Stop at first exact eight-leaf support, or record all nine misses. Recovered witnesses MUST verify independently inside `FourSet::recover` with original-curve addition; negatives MUST exhaust the entire ordered 4A set. Record every attempted shift, 4A probes, group additions, elapsed nanoseconds, and the precise returned signed word, including missing trials. Include two separately identified **legacy known-miss controls** with caller-only scalars 1610011808 and 1264623843; require a=0 nonmembership to reproduce the prior 24-Q result, then attempt the identical public shifts.

Emit JSON from the isolated test for 96 holdouts plus two legacy controls. Assert invariants and no unsupported claim about population coverage, relation rank, base logarithms, ECDLP completion, novel non-generic complexity or comparison speed. A successful bounded retry means only the tested transformed residual had a signed 8A representation. A complete algebraic IC still needs **sign-independent** RR9/S9 extraction, independent relations, base-log LA, individual k recovery and full charging of failed attempts.

Do not change seed, N, shift cap, original curve/base, cohort or oracle after observing output. Failures are reportable and must not be hidden.

## Validation and comparator

Source-specific workflow: pinned Rust 1.100, immutable native library from `1df714645aa683cf975bc76fcacb9842cb4780fd`, overlay only the appended exact test source, run 40 native library tests and targeted adaptive test with `--nocapture`. Upload log/JSON; check exact source and byte digest if obtainable. We have not run this test as of preregistration. No EKS authentication is available in this connector-only environment; GitHub-hosted CI is a development fallback, not a calibrated benchmark or PCC signature. No pretense of compute authorization or leaderboard promotion.

The fully charged 1,024-Q **historical** comparison is merged [#4532](https://github.com/isogeny-labs/crypto/pull/4532): mixed IC 2221.474ms versus matched cached generic BSGS 170.113ms. Those numbers are producer records, not measurements from this protocol. Separately owned [#4596](https://github.com/isogeny-labs/crypto/pull/4596) remains responsible for batched native IC versus shared BSGS, and [#4604](https://github.com/isogeny-labs/crypto/pull/4604) owns sparse D9 Wiedemann; neither is duplicated.

## Lead review and next actionable experiments

Previous exact B31 4A controls: `native/S9_B31_FOUR_PLUS_FOUR_ORACLE_ARITHMETIC_RESULT_20261116.md`, `native/M8_RATIONAL_QUOTIENT_BRIDGE_20261116.md` (H31/K961, no extracted R-dependent S5 roots), `rr9/RR9_B8_COST_GATE_RESULT_20261117.md` and PRs #4513, #4532, #4550, #4581, #4612. Cross-repo negative/triage: `crypto-autoresearcher` #3301/#3305, `cryptanalysis` #1536. Prior prime-field index calculus: Petit–Kosters–Messeng (PKC 2016 DOI 10.1007/978-3-662-49387-8_1); Amadori–Pintore–Sala (FFA 2018 DOI 10.1016/j.ffa.2018.01.009). Knowledge-base MCP and CLI unavailable here; no novelty-absence claim.

**P0**: sign-free public-target Dickson31 `H^8 mod M(A,V,R)=0` / S9 root extractor, *without* 4A point table, as an actual Rust F4/quotient search. Reuse #4228/#4349 and verify all positive/negative branches incl repeated roots and identity; report matrix costs, memory and certificates.

**P1**: feed true algebraic relations to the existing Rust modular LA, solve complete blinded prime31 logarithms, including adaptive misses, and compare fully charged total against same-1024Q shared BSGS, best tuned rho and external prime-field IC. No generic 4A membership substitution in the candidate arm.

**P2**: compare x-only against the *existing* x/y pilot `XY_COORDINATE_PROJECTION_PILOT_20261115.md` only after preserving B31 point identity; y-only fibres add 34 unwanted points to the 31 original labels.

**Retire** B8 fixed signed support for end-to-end work (256/n relation probability); do not re-run blind D5/D6 Macaulay grids or seven-of-eight quotients that simply reconstruct sign enumeration.
