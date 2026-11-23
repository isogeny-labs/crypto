#set page(paper: "a4", margin: 2cm)
#set text(size: 9.5pt)
#set heading(numbering: "1.")
#set par(justify: true)

= Frobenius as a symmetry of the decomposition search: audit and prepared m-sweep

Campaign directory `research/frobenius_msweep_20261027/`. 2026-10-26 to 27.
This is a search report: an audit of a closed lead and the preparation of
five measurement cells. *No experiment has been run and no data are
presented.*

== Question

LEAD-ICB-frobenius-search-symmetry, in the crypto-autoresearcher atlas
under `knowledge/leads/ecdlp/ic-binary/`, is closed. Its stated obstruction
has five parts, C1–C5. The owner's challenge was that the area was examined
only at m = 2 or 3, where m is the number of factor-base points per
decomposition, while chained encodings make m up to about 20 reachable. The
question is whether that closure actually covers m ≥ 3.

== Method

An audit workflow (`wf_3231c71e-f6b`, 13 agents, read-only) ran in three
stages:
- one auditor per sub-claim re-read the cited ledger evidence and code;
- an m-coverage census listed 25 prior experiments;
- a gap synthesis was followed by three idea generators, each with an
  adversarial red-team.

The full JSON is kept with this campaign as `audit_20261026.json`. The
coverage figure is drawn from the recorded per-claim values in
`coverage.json`. Status of each statement: _derived_ (audit argument),
_recorded_ (prior evidence), _prepared_ (design only).

#figure(image("coverage.svg", width: 100%),
  caption: [Proved, measured and prepared coverage of C1–C5 by m. Sources: the audit (`wf_3231c71e-f6b`) and the cell protocols. Prepared cells (gold) have not been run.])

== Findings of the audit

- *C1, C2, C4 are proved for all m* (_derived_).
  - Frobenius maps the ideal of target R to the ideal of σR, because
    S#sub[m+1] has F#sub[2] coefficients. This holds uniformly, including
    for chained encodings with intermediate points.
  - Independent per-slot shifts are never a symmetry of a fixed-target
    instance. That would need Σ(λ#super[k#sub[i]] − 1)P#sub[i] = 0.
  - Experiments behind C1 and C2 ran at m = 2 only (n = 17, 19). They are
    close to tautological checks of the lemma.
- *C3 is unsupported* (_recorded_). The ≥ 0.95× figure is a pre-registered
  threshold in H-BINSTD-c72284, and its Stage 2 solver census was never
  authorised or run.
- *C5 is measured only on the m = 2 barrel* (_recorded_), at 2.15× / 3.15×
  over gf2\_elim for n = 11 / 13. The production pipeline builds no
  σ-symmetric system, so "not applicable" is structural, not a measured
  regression at m ≥ 3.
- *No Frobenius-symmetry solver experiment exists at m ≥ 3* (_recorded_,
  census of 25 cells). The largest m with any Frobenius structure is the
  6-sum extraction (m = 4–6, n = 23–41) and the m = 10 rotated chain at
  n = 83, which has no committed solve.
- *The m = 2 orbit-union cost comparison was mispriced* (_derived_). It was
  priced against n#super[m−1] systems, but the exact count of distinct
  systems is D(n,m) = C(n+m−1, m)/n, about n#super[m−1]/m!. The gap is a
  factor of order m! and grows with m.
- *A genuine m-growing stabiliser exists* (_derived_). Translation by the
  2-torsion point, ι: x ↦ √b/x, is a same-instance stabiliser of order
  2#super[m−1], but only on ι-stable bases. Linear subspaces cannot be
  ι-stable at n = 131, so only non-subspace bases remain.
- *Red-team* (_derived_): 12 new ideas were generated and none survived as a
  lever. Most were relabellings, amortisation, or failed the decisive count.

== Prepared cells (not run)

Each cell has a frozen PROTOCOL.md with dated amendments, native Rust code,
unit tests, a RUN\_CARD.md and a README. Each went through two independent
review rounds and three repair passes. All unit tests pass (G1–G5: 148; G6: 32).

#table(columns: 4, stroke: 0.4pt,
  [cell], [question], [m], [tests],
  [G1], [chained-S3 SAT: plain vs exact Frobenius-orbit domain; C3 grade], [4–6], [25],
  [G2], [orbit-union yield by shift-pattern class, priced at D(n,m)], [2–5], [30],
  [G3], [the never-run C3 census (H-BINSTD-c72284 Stage 2)], [2–3], [38],
  [G4], [ι-stable non-subspace bases, (Z/2)#super[m−1] stabiliser], [2–6], [24],
  [G5], [Proposition T / Σ and sporadic-overlap census], [2–8], [31],
  [G6], [exact census at large m: normal-basis interleaved vs size-matched partitions; ι quotient], [9–20], [32],
)

*G6 derived finding (not a measurement).* No code in the repository solves
a binary chained decomposition at m ≥ 7. G6 is therefore an exact
meet-in-the-middle census.

Its review turned up a property of the normal-basis interleaved
("rotated") partition, which is the design used by the existing n = 83,
m = 10 relation gate:
- Its slots are exact Frobenius rotations of each other.
- A decomposition therefore behaves like a Z[τ] digit expansion, and many
  tuples collide on the same sum.
- A digit model predicts the fraction of distinct sums. It agrees with the
  group law exactly at n = 13 and with the real builder at n = 19, m = 12.
- The prediction is 0.055 for the interleaved design against 0.989 for a
  size-matched generic partition at n = 23, m = 12, an 18× loss.

If this persists at n = 83, that gate pays for Frobenius-correlated slots.
Status: _derived_ (exact arithmetic plus unit-scale agreement). The census
will confirm or refute it.

The ι quotient is an identity, not a lever:
H#sub[ι]\[R\] = 2#super[k−1] · (H\[R\] + H\[R+T\]).

== Companion notes in this PR

- `research/ic_liftcheck_audit_20261027/` (lead 1, static analysis). The
  incumbent Koblitz rows use a pair-table m = 3 collection, which has no
  root, y-recovery or rationality step. Where roots do appear, no step that
  the twist theorem makes redundant costs more than 1% per relation. Lead
  closed.
- `research/pE_density_closure_20261027/` (lead 2, recorded rows
  re-analysed natively). The yield is C(|S|+m−1, m)/\#E. The slope of log
  yield on log p#sub[E] is 2.96 ± 0.06 at m = 3 (identity: 3). The spread
  of 3× across curves is exactly p#sub[E]#super[m]. Choosing a curve in
  ECC2K-130's class buys about 5·10#super[−5] in yield at n = 131 under a
  stated heuristic. The atlas change is not applied and awaits review.

== Why nothing was run

The repository requires a resume checkpoint in S3 before any experiment.
The container's ambient AWS identity, IAM user `adam`, is refused
`PutObject` on the checkpoint bucket. That identity is also outside the Iso
Lab account. The GitHub route to Iso Lab needs Actions, which is blocked by a
billing failure. The runs therefore wait for one of:
- checkpoint write access, or a `CHECKPOINT_UPLOAD_TOKEN`;
- an identity that can reach Iso Lab EKS;
- restored Actions billing.

The repository's `main` also did not compile: five broken merge
resolutions. They are repaired in PR \#2675 and carried on this branch.

== Scope and limits

- This report presents no measured result and no change to cost or the
  leaderboard.
- G6 covers m = 9–20 with an exact census, not a solver. Solver behaviour
  at m ≥ 7 is still unmeasured, and no repository code solves there.
- The cells' authors expect nulls in most cells. A measured null at m ≥ 3
  that replaces an asserted closure is itself the deliverable.
