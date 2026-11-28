# Native compact spectrum compiler

Rust source: `tools/torus-spectrum-compiler`. Protocol and source identity:
`PROTOCOL.md`, `SOURCES.sha256`, `SOURCE_QUALIFICATION.json`. Native statement:
**PCC-20261123-989b50**, producer unsigned/unverified. The original assessment,
seven-input compiler and every old evidence/claim file retain their bytes.

The compiler takes only public Spec, R, Q=[8]R and a node cap. It builds the
normalized monic sextic, guarded localized Chebyshev operator, squarefree and
torsion resultants, its characteristic polynomial and six unknown bounded
coset labels. Repeated coset labels are permitted. The separate extension API
takes supplied coefficients/labels; it is used only for known-answer controls.

| Fixed control | Variables | Equations | Semantic inputs | Original replay assignment |
| --- | ---: | ---: | ---: | ---: |
| Original q213156431, k31 | 14001 | 14009 | 68 | 819 values |
| Original q213156431, k20633 | 15231 | 15239 | 188 | 1059 values |

These are representation counts, not normalized work, memory bytes, solving
degree or solver performance. All equations have degree at most two. At k20633,
the 188 inputs are five section coefficients, three inverse guards and 180
Boolean label/slack bits. The rank-six candidate quotient does not bound the
dimension of an unknown-coefficient solution ideal. [pcc:PCC-20261123-989b50]

All 12 fixed sections extended, including repeated coset labels. Native values
matched the unchanged numeric sextic and operator remainder; characteristic
polynomials matched independently enumerated determinants at 0,1,7. Original
support, sign/raw-index extraction and aggregate group/circuit replay passed.
Each actual k20633 control rejected 211 single-wire corruptions. Every admitted
control additionally rejected recomputed zero inverse guards, non-Boolean and
overflow inputs, plus malformed/size/point and truncated-assignment controls.
These finite checks are not a compiler soundness/completeness theorem.
[pcc:PCC-20261123-989b50]

The pinned native cvc5 GPL/CoCoA release 1.4.2 passed two scalar-root/model replay
checks and one 15-bit contradiction. This admits the syntax/backend interface;
it did not solve either section system. The SMT exports contain every gate and
constraint, plus redundant exact bit-sum annotations. No unknown coefficient
root or curve-absence certificate was produced. [pcc:PCC-20261123-989b50]

The larger expanded coefficient compiler remains a historical comparator. The
same-target original point replay already uses1059 values, so compacting that
expanded construction is insufficient to select this representation as faster.
`NEXT.md` states the concrete unknown-root and matched-comparison gate.
`REPRODUCE.md` gives exact source/export hydration and native commands.
`FROZEN.md` separates source/evidence from future versions. Full attack-family,
useful-yield/rank/indexing/recovery and cold-cost contracts remain in the
unchanged consumer criteria and coset-spectrum-v1 review. [pcc:unverified]
