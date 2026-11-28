#set page(paper: "a4", margin: 14mm)
#set text(font: "DejaVu Sans", size: 9pt)
#set par(justify: true, leading: 0.55em)
#set heading(numbering: "1.")

#align(center)[
  #text(size: 18pt, weight: "bold")[Curve25519 split-auxiliary relation contracts]
  #linebreak()
  #text(size: 10pt)[Isogeny Labs Crypto research note · 2026-11-18]
]

#v(8pt)
#box(fill: rgb("eef6ff"), inset: 9pt, radius: 4pt)[
  *Result.* Two native Rust tests now pin the sign and identity-point
  boundaries of a split S3 relation on the exact full-width RFC 7748 model.
  A study-local Rust diagnostic importing the same model source checked the
  frozen witnesses. The integrated root test gate remains blocked by existing
  main-branch compile failures. [pcc:unverified]
]

#figure(
  image("split-chain.svg", width: 100%),
  caption: [Exact signed joins, information lost by an x-only split chain, and the projective/lift-aware next target.],
)

= Checked witness

On Curve25519 over $F_(2^255 - 19)$, set
$P=[2]G$, $Q=[3]G$, $S=[5]G$, $T=[7]G$, and $R=[12]G$.
The diagnostic observed $P+Q=S$, $S+T=R$, and zero for both S3 joins.
Negating P and Q preserves every abscissa but makes the chosen fixed-signed
four-point relation false. Separately, $[2]G+[-2]G=O$ has no affine
abscissa while $O+[7]G=[7]G$. These are checked witnesses, not a new general
identity. [pcc:unverified]

= Contract

+ An x-only S3 chain is existential over sign choices.
+ A signed relation row needs compatible lift/sign data plus group verification.
+ An affine auxiliary needs an explicit identity branch, or projective $(X:Z)$
  coordinates with $Z=0$.

= Baseline

The repository's T0 study measured about 2x blind-oracle coverage from the
$x -> 1/x$ symmetry, while full cofactor projection had about 3.8--4.0x the T0
coverage for the larger tested bases. The exponent study found no m=2 lever and left
m=3/4 censored or undetermined. The closure note explicitly excludes split
Semaev systems and F4 lower bounds from its restricted unsplit XL model.
This round adds a correctness boundary, not a timing or exponent result.

= Proposed optimization target

On the existing k=20 Curve25519 miniature at m=4, target at least 25% fewer F4
field multiply-adds per *group-checked* relation than the current pkm-semaev split
baseline, using matched targets and seeds, zero invalid rows, and charging all
auxiliary, sign, infinity, setup, and verification work. Advance only if the
paired 95% interval for the cost ratio lies below 0.80, then replicate at k=24.
This is a proposed decision rule, not a measured outcome.

= Evidence boundary

The study-local Rust diagnostic imports the exact model/S3 source, uses an
independent minimal BigUint affine point adapter, exited 0 under rustc 1.94.1,
and wrote `raw/contract-check.json`. The root release build is blocked
by 35 unrelated current-main errors already covered by the ACTIVE repair lane;
the integrated tests therefore remain implemented but not checked by the root gate.
No timing, solver, relation-collection, or end-to-end DLP claim is made.

= Sources

Petit--Kosters--Messeng, _Algebraic approaches for the ECDLP over prime
fields_, PKC 2016; Galbraith--Gaudry, _Recent progress on the ECDLP_, 2015;
the repository's T0 symmetry, exponent-study, algebraic-oracle, and
PRIME_FIELD_IC_CLOSURE_20261112 reports. Full paths and URLs are in README.md.
