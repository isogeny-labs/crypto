# EXP-R31-P — stop/go gate for a genuinely non-generic prime31 ECDLP

Date: 2026-11-15. Scientific standing: proposed Rust experiments, **not** implemented/verified complete m8 ECDLP or new complexity theorem. Preserve Rust-only implementation requirement.

## Why the current *complete* Rust algebraic solver is not the target

At source `ad8349cc9be584914d223082b7890d9309c2b902` [Actions #43776311207](https://github.com/isogeny-labs/crypto/actions/runs/43776311207) used one ordinary cofactor-one p=2^31-1 point Q with known answer used **only after extraction**. S3 root-index relation/LA recovered 32 logs independently and solved Q, but was **46.200742075s cold** versus **0.023613920s** for plain Rust BSGS, a loss by **1956.50×**. S3 roots of (x_i,x_j,U) list generic pair-sum coordinates, so the fast algebraic index just reproduces a group MITM table. Replacing group additions with quadratic-root arithmetic is not novel non-generic index calculus. Freeze it as a **negative methodological baseline**; do not optimize it merely to advertise a stage speedup. See `PRIME31_RUST_COMPLETE_LOG_COST_GATE_20261115.md`.

## Specific next non-generic experiment: algebraic eight-summand elimination

Carry forward the existing source `native/examples/prime31_m8_balanced_f4_input.rs` with its actual **target-dependent balanced S4/S3 F4** input and 16-public-mask panel. Input only public target Q, factor-base defining polynomial and the curve equation; **never** pass known ordinates, scalar labels or a precomputed point-sum table to algebraic discovery.

A balanced coordinate-only system can be expressed schematically as
```text
S3(x1,x2,u)=0, S3(x3,x4,v)=0, S3(x5,x6,w)=0, S3(x7,x8,z)=0
S3(u,v,L)=0, S3(w,z,R)=0, S3(L,R,x_Q)=0,
f_B(x_i)=0  (i=1,...,8)
```
with the eight Dickson/tower factor-base membership constraints and their original-ideal certificates. (Variables L,R,u,v,w,z and the x_i represent sign-ambiguous affine intermediates; the S3 equations project signed sums and must be **verified with full Q and exceptional identity branches after algebraic extraction**. The model is not assumed radical or reduced; all multiplicity/extraneous-component checks must be explicit.)

**Do not** eliminate the system by precomputing the full four-sum support (up to 618885 total signed four-term targets) or by enumerating all B31 factor-base four-tuples; that is generic MITM. Instead test local quotient reduction of each two-summand block followed by **target-dependent** S4/S3 interface constraints. Implementation should reuse actual native optimized F4 and certified relation/check APIs in `crates/certified-relations` and `src/cryptanalysis/f4_fp.rs` from [#4349](https://github.com/isogeny-labs/crypto/pull/4349) stacked on [#4228](https://github.com/isogeny-labs/crypto/pull/4228). Do not vendor a copy of the certified library or rewrite their pinned PCC ledger. Work can be integrated after their stack lands, or on a fresh branch from its head with retained parent check-in.

## Prior leads reviewed before prioritization

| Lead | Current evidence | Decision |
| --- | --- | --- |
| m4 factorized short-quartic resultant (#4188) | 181.93x stage-only vs naive Horner at B16; **natural target coverage ceiling 0.02882%** at B31 | Engineering stage achievement, but cannot complete cold ECDLP cheaply with B31 |
| S3 discriminant / y-only / injective-y | 17/31 y fibers have 3 Fp x-lifts; B31 support no natural-win | Do not exchange sign ambiguity for cubic-degree growth without exact target-query win |
| Native S3 root-index/LA (#4188) | Actual complete single prime31 Q; 46.20s vs plain Rust BSGS 0.0236s | **Stop attack optimization**, keep generic-equivalent correctness baseline |
| Balanced m8 S4/S3 (#4188) | Public known-witness input construction and bounded F4; no completed target-dependent arbitrary root extraction | **Prioritize** clean Rust input/F4 completion test |
| Certified quotient/subresultant (#4228/#4349) | Real Rust root worker, fixed-small-system certification; capped F4 incomplete, wide prime unsupported | Most promising engineering transfer, **requires m8 integration and cost audit** |
| Label-free selectors (#4338) | Proposed interpolation/implicit anchors/redundant shifts; no execution or no-hidden-label recovery | Mathematical hypothesis only; demand a specific uniform-target success test |
| Chained auxiliary transfers (#3877), cryptanalysis #1536 | No validated cheaper full cold prime31 target scalar recovery | Long-term math leads, not a substitute for an m8 run |
| Published PKM 2016 and APS 2018 | Classical prime-field Semaev approach, APS single-factor-base-relation variant | Mandatory **non-generic** comparisons before any originality or superiority claim |

Primary literature: [Petit–Kosters–Messeng 2016](https://doi.org/10.1007/978-3-662-49387-8_1), [Amadori–Pintore–Sala 2018](https://doi.org/10.1016/j.ffa.2018.01.009). Current research-KB not accessible in this tool runtime; GitHub primary files and publisher pages inspected as bounded fallback.

## Falsifiable Rust-native acceptance protocol

1. **Input/roots:** pin exact 16 natural public masks and 16 planted-positive controls, including repeated indices, cancellation and vertical/intermediate identity cases. A planted row does **not** establish natural yield. Require algebraic discovery without EC additions and no known-log lookup; verify each output after extraction using full target Y and independent EC arithmetic.
2. **Degree and completeness:** compare dense-F4, tower-F4 and local quotient/subresultant reductions with the **same admitted polynomial system**. Report actual F4 reductions/muladds, solving degree only for completed runs, matrix columns/nnz, peak RSS, original-ideal certificates, capped/undetermined cases and externally checked equal root sets. A timeout is not a negative relation.
3. **Relations and independent rank:** on mixed public points `R=A G+B Q` with held-out Q, solve `Σc_i log(P_i)−B log(Q)=A mod n`; require rank `B+1`, no known scalar inside solver, and all B factor-base logs + Q recovered and independently verified. Avoid generic root-index for discovery.
4. **Full cold costs:** count factor base, preprocessing, every negative target query, root extraction, signed replay, modular LA, failures/retries, all target generation, and 12 independent held-out prime31 Qs. Compare same host Rust against plain BSGS **and strongest optimized negation-map rho** (plus APS/PKM baselines if available). No amortized cost substituted for one-target cold cost.
5. **Decision:** admit a novel non-generic lead only if the algebraic relation mechanism cannot be reinterpreted as enumerating generic group-sum support and measured cold cost is lower at B31; otherwise freeze negative evidence and change mathematical model. Independent PCC is required for promotion beyond producer observations.

There is currently **no** demonstrated cheaper novel non-generic prime-field ECDLP on this fixture. The complete S3-root-index solver is explicitly generic-equivalent and lost the cold comparison.
