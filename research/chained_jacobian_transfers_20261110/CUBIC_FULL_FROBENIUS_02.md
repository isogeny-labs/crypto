# Full cubic Frobenius transport at223

`lean/CubicFullFrobenius22302.lean` proves the coefficient embedding and full q-power action under explicit ring hypotheses. Both unchanged original Lean4.35/4.34.1 gates pass24 audited declarations/0 trusted:12 old named,11 new named and one old generated definition equation. Whole source/commands/build/audit/caller outcomes and original PCC records are in `evidence/full-cubic-frobenius-final-08`, dataPCC-20261116-d5a397. Claims remain unsigned. [pcc:unverified]

## Canonical base coefficients

For a commutative ring R with characteristic223, define e(a)=(a.val:R) for a in Fin223. The module proves that e is injective and preserves addition, multiplication and powers. Its power proof uses induction with the shared semiring laws, so both original toolchains check the same statement. A kernel reduction over only223 base residues supplies a^223=a; transporting it proves e(a)^223=e(a). No223³ coefficient population is enumerated. [pcc:unverified]

## Full expression and third power step

Given z in R with z³=3, define E_z(a,b,c)=e(a)+e(b)z+e(c)z² and T(a,b,c)=(a,183b,39c). Characteristic-aware ring normalization proves (u+v)^223=u^223+v^223. Combining that identity, multiplication of powers, the embedded coefficient equality and the frozen01 generator identities gives the exact formula below. [pcc:unverified]

    E_z(a,b,c)^223 = e(a)+183e(b)z+39e(c)z² = E_z(T(a,b,c)).

Iterating this equality three times and using the original T³=id gives ((E_z(x)^223)^223)^223=E_z(x). These are quantified statements for every coefficient triple and every R,z satisfying the stated hypotheses. They construct no quotient field and assert no native implementation correspondence. [pcc:unverified]

## Fixed expression with the exact remaining hypothesis

The new fixed_expression_iff states, under the additional explicit hypothesis that E_z is injective, E_z(a,b,c)^223=E_z(a,b,c) iff b=c=0. The forward direction uses the full-expression formula and injectivity to recover T(a,b,c)=(a,b,c), then the frozen01 coefficient fixed-subspace theorem. The converse needs only that same coefficient equality. A later additive module will attempt to derive basis injectivity from the three Frobenius equations; it is not assumed to have done so here. [pcc:unverified]

## Proof map and declaration-extractor scope

| New declaration | Original PCC record |
| --- | --- |
| `CubicFullFrobenius22302.frobenius_add` | `PCC-20261116-40f44a`; unsigned. [pcc:unverified] |
| `CubicFullFrobenius22302.base_fermat` | `PCC-20261116-689c57`; unsigned. [pcc:unverified] |
| `CubicFullFrobenius22302.embed_injective` | `PCC-20261116-f7ec43`; legacy extractor truncates the type, incomplete binding; unsigned. [pcc:unverified] |
| `CubicFullFrobenius22302.embed_add` | `PCC-20261116-f1b811`; legacy extractor truncates the type, incomplete binding; unsigned. [pcc:unverified] |
| `CubicFullFrobenius22302.embed_mul` | `PCC-20261116-17ec3c`; legacy extractor truncates the type, incomplete binding; unsigned. [pcc:unverified] |
| `CubicFullFrobenius22302.embed_pow` | `PCC-20261116-44b51a`; legacy extractor truncates the type, incomplete binding; unsigned. [pcc:unverified] |
| `CubicFullFrobenius22302.embedded_coefficients_fixed` | `PCC-20261116-891920`; legacy extractor truncates the type, incomplete binding; unsigned. [pcc:unverified] |
| `CubicFullFrobenius22302.embedded_action_constants` | `PCC-20261116-e01b59`; legacy extractor truncates the type, incomplete binding; unsigned. [pcc:unverified] |
| `CubicFullFrobenius22302.full_expression` | `PCC-20261116-294012`; unsigned. [pcc:unverified] |
| `CubicFullFrobenius22302.three_power_steps` | `PCC-20261116-c03e0c`; unsigned. [pcc:unverified] |
| `CubicFullFrobenius22302.fixed_expression_iff` | `PCC-20261116-d5aecf`; unsigned. [pcc:unverified] |

The complete original kernel audits check all actual declarations. Six original declaration records truncate at a named argument inside the type and do not bind the full statement; five bind complete statements. CorrectionPCC-20261116-a1a45d preserves the original records and qualifies the earlier d5a397 wording. Later03 will add fully type-annotated corollaries without changing this frozen source or the original verifier. [pcc:unverified]

## Failure custody and sources

The first complete direct attempt reached120s; isolated default/restricted/done additivity modes reached20s. A direct interactive ring diagnostic emitted No goals and erroneous downstream sorryAx, and was rejected. Structured try-ring followed by done then compiled the whole source. The first dual attempt passed Root24/0 but failed Consumer because Fin.val_pow is absent in4.34; the complete failed source and both outcomes remain in first-dual-toolchain-failure-07. Induction replaced only that proof and the final unchanged dual gates pass. No failed or partial module is accepted. [pcc:unverified]

Final source SHA256`5198ffb8ef0945dd4754255ef2f37d420ca9ad7f4064d03f799c8ca593011edf`. The exact frozen01 byte prefix remains e81532c9df6d5c416f26693ed7849d70e512d5225558adbab5147266f1e59738. Each original gate uses one worker,120s and original static proof memory defaults. Only propext,Classical.choice,Quot.sound occur; no new axiom, hole or native decision is accepted. [pcc:unverified]

Sources checked: frozen01 source/derivation; whole original common.rs832fd288 constructor and diagonal Frobenius; official Lean power/characteristic/Fin interfaces and interactive ring/try/done semantics in the actual4.35/4.34 sources. Reviewb717b7 and subsequent tactic/toolchain failure records retain the receipts. KB MCP unavailable and exact/negative/theorem wrappers127; no novelty or absence inference. [pcc:unverified]

Field construction, UInt64/powmod/compiler/counter correspondence, actual Jacobian maps and admitted matched construction/transfer/solve/recovery costs remain separate. Dedicated exact-source CI retains the actual head, executing workflow, original tools and every partial/full outcome. No native population, timing result or paid launch was performed. [pcc:unverified]
