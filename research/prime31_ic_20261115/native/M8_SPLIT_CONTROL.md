# Ordinary prime31 m=8: complete 4+4 index-calculus control

**2026-11-15 · [pcc:unverified].** This finite-size experiment reuses the existing Rust `Arithmetic`, `Base`, `check_curve` and `PRIME31` adapters. The original producer arithmetic was a separate temporary C++ diagnostic; the checked-in candidate is **Rust**. Its 4+4 oracle is explicit *generic group* meet-in-the-middle, **not F4 or a novel non-generic solver**. Exact-head Rust compilation and independent signed PCC replay are pending.

## Curve, factor base and algebra

Curve: p=2147483647=2^31-1, E: y²=x³+847514846x+2087444724, prime order n=2147446831, cofactor 1, G=(625037715,2100627251), public Q=(1547953785,1667575595). The verifier-only scalar 661068045 is not available to the collector or linear algebra. The existing depth-6 Dickson Base::build selects terminal 933677148 and **B=31 liftable points** from 64 raw x-coordinates.

Partition the ordered basis into 15 and 16 points. Enumerate signed sums of four *distinct* indices on each side: 16*C(15,4)=21,840 left sums, all distinct on the fixed curve; 16*C(16,4)=29,120 right templates. For each natural public mask R=[a]G+[b]Q, b nonzero, hash-join R minus right-four-sums against left-four-sums, and verify the group identity. Every hit yields the affine row

\[ \sum_i \varepsilon_i\ell_i-bd=a\pmod n,\qquad \ell_i=\log_G(P_i),\ d=\log_G(Q). \]

This gives 31 unknown factor-base logs plus the unknown target log. The independent rank, not the raw number of hits, determines recovery.

## Measured finite-instance producer diagnostic

| Metric | Value |
|---|---:|
| Natural masked queries / queries with witnesses | 124 / 26 |
| Raw signed 4+4 equations | 32 |
| Independent affine rank / unknowns | **32 / 32** |
| Left table setup group additions | 203,840 |
| Public mask group additions | 11,352 |
| Group membership lookup additions | 3,610,880 |
| Group additions before final verification | 3,826,072 |
| Final 31 basis-point + target verification additions | 1,431 |
| **Inclusive producer group-addition total** | **3,827,503** |

The separately authored producer-local Python verifier replayed all 32 elliptic-curve equations, independently row-reduced to rank 32, recovered all 31 basis logs and checked [661068045]G=Q. Transcript: [M8_SPLIT_CONTROL.results.json](M8_SPLIT_CONTROL.results.json); verifier: [replay_m8_split_control.py](replay_m8_split_control.py); Rust reproduction: [examples/prime31_m8_split_ic.rs](examples/prime31_m8_split_ic.rs). No source-controlled C++ solver is offered as the attack. Scratch C++ O3 measurements on this host: 1.48 s / 3,524 KiB, and on an equivalent later run 1.31 s / 3,492 KiB; these **are not** Rust or matched rho timings. The first reported 3,774,896 modular inversion calls included 32 scalar-order pivots and omitted final verification; never present them as EC-only or complete inversion counts. Full GAE and F4 work are not priced.

The naive uniform heuristic predicts 21,840*29,120/n≈0.296 representations per target and Poisson positive probability ≈0.256; actual positive frequency was 26/124≈0.210. No exponent is inferred from one finite-size cell.

## What an algebraic replacement must achieve

Replacing this *whole* group hash-join and signed-table setup with a genuine m8 algebraic PKM oracle leaves 11,352 public-mask additions plus 1,431 final group-verification additions on this fixed 124-call trace. If the same 32 independent rows can be recovered, the historical same-curve BSGS mean of 60,524 additions leaves an **optimistic ceiling** (60,524-11,352-1,431)/124 ≈ **385 group-addition-equivalents per complete call**, *before* factor-base construction, F4 building, witness extraction, linear algebra, additional failures or RAM. The earlier Floyd-rho 183,586 additions is not an optimized rho baseline. This is a discriminating **cost target**, not evidence that an F4 m8 oracle can meet it; an algebraic oracle's natural witness distribution may differ.

## Previous leads checked before choosing this control

| Existing line | Evidence and implications |
|---|---|
| Ordinary prime31 Rust m2 | [native README](README.md), reused `src/cryptanalysis/f4_fp_tower.rs` and `sig_fp_tower.rs`; development tests, no measured full prime31 algebraic cold solve |
| PKM sparse m8 gate | [crypto #4077](https://github.com/isogeny-labs/crypto/pull/4077) and [rank-aware exponent model](../../norm_branch_rank_pilot_20261115/PKM_PARTIAL_OUTPUT_GATE.md): measure complete-call γ and **useful independent-rank yield δ**, necessary optimistic γ<3+δ/2 |
| Tower and dense F4 | [crypto #4091](https://github.com/isogeny-labs/crypto/pull/4091) and [#4275](https://github.com/isogeny-labs/crypto/pull/4275); dense won on small planted-positive systems but enumeration remained cheaper; neither is a natural-prime31 m8 result |
| Earlier m3 control | [M3_PROJECTIVE_ROW_AUDIT.md](M3_PROJECTIVE_ROW_AUDIT.md); depth-9 B256 achieves rank 255 after 4,051,363 templates, and [M3_RANK_EARLY.results.json](M3_RANK_EARLY.results.json) records 2,094,275 partial additions; new m8 trades **lower memory** for more point additions in this generic representation |
| Pair S3 and quotient algebra | [crypto #4228](https://github.com/isogeny-labs/crypto/pull/4228) provides component preprocessing, not complete m8 target recovery |
| Auxiliary/transfer and previous lead atlas | [cryptanalysis #1536](https://github.com/isogeny-labs/cryptanalysis/pull/1536), [crypto #3877](https://github.com/isogeny-labs/crypto/pull/3877), [crypto-autoresearcher #3305](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3305) and [ECDLP attack matrix](https://github.com/isogeny-labs/cryptanalysis/blob/main/suite/docs/ECDLP_ATTACK_MATRIX.md); no source here demonstrates a cheaper complete public scalar-preserving route |

PKM 2016 is established prior art: [Petit–Kosters–Messeng, PKC 2016](https://doi.org/10.1007/978-3-662-49387-8_1). Literature review included Diem's extension-field index-calculus work; it does not establish a cheaper ordinary prime-field solution. The shared research-KB MCP/wrapper, Conductor, and authenticated EKS/PCC independent signing route were not available here. No absence or novelty claims are made.

## Next preregistered measurements

**A — Natural m8 oracle:** use the *existing Rust* `src/cryptanalysis/pkm_smooth_tower.rs::algebraic_decompose_dense` or generalized F4 on the exact B31 abscissae and frozen public Q, with at least 124 identical natural queries and held-out masks; no planted rows in attack fits. Record decided positive, decided negative, censored, exact witnesses, duplicate projective rows, rank increment, field operations, F4 degree, peak RSS and total cost. The current packed native tower has MAX_FREE=4 and cannot directly encode the six free abscissae of chained m8; that implementation gap must be closed and tested rather than claiming m8 was run.

**B — Real speed gate:** require successful complete scalar recovery using algebraic extraction only (no generic hash-join fallback). Measure all setup, failed invocations, polynomial operations, matrix and individual logs, verify [d]G=Q, compare against the best same-host optimized negation-map rho and external F4/GB tooling. Repeat on two larger held-out sizes before fitting γ and δ; passing an optimistic exponent gate is not an observed speedup.

**C — Structural controls:** compare Dickson depth-5/6/7, their *actual* liftable B, and a cardinality-matched ordinary x-base; charge factor-base selection and record both projective and rank yield. Do not treat the current deterministic single-split probability as a scaling law.

Development reproduction (not a performance experiment):

```sh
cargo +1.100.0 test --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_m8_split_ic
python3 research/prime31_ic_20261115/native/replay_m8_split_control.py
```

Full Rust reproduction, only after execution and checkpoint admission:

```sh
cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_m8_split_ic
```

Keep #4188 open and do not promote leaderboard/cost claims without independent PCC verification. [pcc:unverified]
