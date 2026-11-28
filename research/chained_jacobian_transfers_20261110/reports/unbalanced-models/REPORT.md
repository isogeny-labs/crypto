# Two explicit models for unbalanced factor transfers

The complete changed native suite passes 34 tests with no failures, ignores or
filters. Both new genus-two models retain their raw nonsquare leading and
constant coefficients. Exact covers, every rational curve point, extension
corners and transported gluing graphs pass; whole Jacobian map evaluation
remains phase B. Native full02 is PCC-20261112-925b02, selected01 is
PCC-20261112-feff57. All results here are unsigned. [pcc:unverified]

Over F101 the models are

```
C10: y² = 61x⁶ + 8x⁴ + 93x² + 89,
C20: y² = 18x⁶ + 75x⁴ + 45x² + 40.
```

| Recorded object | C10 | C20 |
| --- | --- | --- |
| Intrinsic elliptic factors | E1=(26,58), F0=(44,21) | E2=(59,27), F0=(44,21) |
| Finite F101 curve points | 120 | 120 |
| Rational infinities / rational x=0 points | 0 / 0 | 0 / 0 |
| Full curve counts N1 / N2 | 120 / 10436 | 120 / 10436 |
| Jacobian order | 12317 | 12317 |
| Transported two-graph class | Gamma21 | Gamma6 |

These are recorded native values, not timings or new catalog counts.
The graph identifies the actual geometric four-point kernel rather than
inferring a target from its zeta polynomial. [pcc:unverified]

## Covers and the global construction

For f=c6*x6+c4*x4+c2*x²+c0, the direct quotient uses (u,v)=(x²,y) with
cubic [c0,c2,c4,c6]. The reciprocal quotient uses (1/x²,y/x³) with cubic
[c6,c4,c2,c0]. A cubic v²=a*u³+b*u²+c*u+d maps to short coordinates by
X=a*u+b/3,Y=a*v, followed by the checked scale X→s²X,Y→s³Y.

| Model / cover | Destination | a | b/3 | s |
| --- | --- | --- | --- | --- |
| C10 direct | F0 | 61 | 70 | 43 |
| C10 reciprocal | E1 | 89 | 31 | 43 |
| C20 direct | F0 | 18 | 25 | 33 |
| C20 reciprocal | E2 | 40 | 15 | 33 |

Coefficient identities, all 120 rational point images per model, both
infinities and both x=0 points over the declared extension, and all six
Weierstrass roots are checked. The factor roles swap relative to the source;
the code checks the destinations. The origin and infinity charts are retained
even though their curve points are not F101-rational. [pcc:unverified]

Let P_i be the canonical product of the two cover pullbacks, with
H_i=P_i^dagger and H_i P_i=P_i H_i=[2]. The actual source two-graph is extracted
as [2]Q0 from the old T0 Cantor kernel and independently compared with the old
C0 root pairing. Its transports by (alpha,id) and (alpha2 alpha,id) are exactly
the kernels of P10/P20. The quotient property defines global maps
F10 P0=P10(alpha,id) and F20 P0=P20(alpha2 alpha,id), of separable degrees
7 and 49. Their adjoint norms descend diag(7,1) and diag(49,1), respectively.
These degrees are coprime to the 109/113 subgroup orders. The global quotient,
Picard, optimal-cover and cyclic-kernel arguments are written in
[UNBALANCED_MODEL_PROOF.md](../../UNBALANCED_MODEL_PROOF.md), claim
PCC-20261112-128eb1. Its bibliographic spelling “Mille” means Milne; the
already pinned text is preserved. [pcc:unverified]

The same actual-coordinate transport by (alpha,id) on the primitive four-graph
is Galois stable but has 96 nonidentity product-pair values, all -1, among
256 comparisons. This realizes the previously credited CT44 obstruction.
Transport by (alpha2 alpha,id) preserves anti-isometry and gives class A6
up to relative sign. Its Richelot model T20 is still phase C. [pcc:unverified]

## Proof map and native certificates

| Obligation | Evidence and exact scope |
| --- | --- |
| Direct cubic-to-short cleared identity | `UnbalancedCovers.direct_short_quotient`, PCC-20261112-1bde23; bound local-pass Lean over every commutative ring |
| Reciprocal cleared identity | `UnbalancedCovers.reciprocal_short_quotient`, PCC-20261112-d03d51; same scope |
| Actual scales, fields, divisors, kernels and global degree/norm | Complete native model checks plus written proof 128eb1; actual AV/Picard theory is not formalized |
| Evaluated algebra records | IDC1hdb5d8c25ad6d1262 and IDC1h15a63844aad1bcd0, 32 issuer + 64 additional checker points each over 2^61-1, exact final Lean source digest |
| Whole algebra replay | PCC-20261112-f404e3: 23 tests, 11 source guards, seven emitted certificates, five old outputs byte-identical |

The local proof gate passes two declarations with no trusted native axioms.
The project now has 171 written declarations in 34 modules; the new bulk
compiled count is an independent CI observation still pending. Source, Lean
toolchain, statement bindings, audit/build outputs and local verdicts are in
`evidence/unbalanced-{claims-01.json,local-verdicts-01.json,gate-01.txt}`.
Local acceptance is not a signed scientific verdict. [pcc:unverified]

## Reproduction and custody

From `successor-preflight`, use the existing locked/offline release test
command `cargo test --release --offline --locked -- --nocapture` under the
protocol's 15-minute bound, alongside its oldguards, formatting and strict
Clippy checks. RUSTUP_TOOLCHAIN=1.99.1 and the pinned Lean 4.35.0 are used.
All 622 source/fixture/engine hashes are unchanged across full02.
The whole 82,084-byte native output is standard XZ of 9,716 bytes, with raw
and archive SHA256 and exact roundtrip; source snapshots, before/after hashes,
start/finish and status accompany it in
[full02 custody](../../evidence/unbalanced-full-02/custody.json).
Selected01 is one test with 33 filtered; only full02 supplies complete-suite
acceptance. [pcc:unverified]

Use the `algebra-certificates` job commands in
`.github/workflows/chained-jacobian-transfers.yml` for all 23 algebra tests
and the seven-output `cargo run --release --offline --locked`; compare all
five old outputs byte-for-byte and both new source-bound outputs with the
committed records. The focused workflow registers both new certificate
subjects and UnbalancedCovers. No protected arithmetic engine was replaced.
Native and Lean checks are local development checks, not a benchmark.
[pcc:unverified]

## Sources and remaining work

Sources checked: Bruin–Doerksen [0902.3480](https://arxiv.org/abs/0902.3480),
Lemmas 2.4/2.6, Definition 2.7, Theorem 3.2; Galbraith Chapters 25/26;
Milne's degree, polarization and quotient results cited in
ISOGENY_CYCLE_PROOF_V2.md; exact CT44 source at cc5d5541, preserved with its
original blob. Characteristic 101 assumptions are checked; the paper's
characteristic-zero Proposition 2.8 is not used. Sources checked (KB): MCP
absent, wrapper has no executable, lead atlas encrypted at 8edec422;
repository notes, primary PDFs and native code are the fallback. No novelty
conclusion follows from that access gap. [pcc:unverified]

| Requirement | Current state / next concrete action |
| --- | --- |
| Exact two-glued models and covers | Complete local phase A: 34 native / 23 algebra / two bound Lean identities |
| Global degree 7/49 transfers and nonuniform norms | Written quotient proof; implement generic nonmonic sextic chart and norms from input Mumford divisors |
| All 12317 rational classes, dual/recovery, geometric kernels | Phase B open; use UNBALANCED_PHASE_B_NOTES.md and retain all exceptional cases |
| Primitive four-glue T20 and whole transfer | Phase C open; construct actual image kernel and its Richelot matching |
| Computational advantage | Unmeasured; no speed claim or solver comparison is supplied by these model checks |

The new models have canonical intrinsic-factor sum degree 4, whereas the old
T_i have degree 16; their E-factor j values distinguish the old C_i.
The written intrinsic-factor argument makes them distinct from the earlier
six curves. They are two additional explicit presentations within the fixed
72-Jacobian catalog. The existing catalog/closure figures retain their fixed
scope and counts; this new diagram records the newly explicit models.
[pcc:unverified]

Editable `transfers.dot`, `counts.json`, `render.py`, `report.typ`, SVGs and
the inspected PDF accompany this report. Frozen prior reports, negative
results, the original native test bodies and the old consumer pins remain
preserved. Consumer 1509 at f656cf33 has its own pending full CI; root 51d's
four jobs are successful in 2c4744. [pcc:unverified]
