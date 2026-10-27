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
review rounds and three repair passes. All 148 unit tests pass.

#table(columns: 4, stroke: 0.4pt,
  [cell], [question], [m], [tests],
  [G1], [chained-S3 SAT: plain vs exact Frobenius-orbit domain; C3 grade], [4–6], [25],
  [G2], [orbit-union yield by shift-pattern class, priced at D(n,m)], [2–5], [30],
  [G3], [the never-run C3 census (H-BINSTD-c72284 Stage 2)], [2–3], [38],
  [G4], [ι-stable non-subspace bases, (Z/2)#super[m−1] stabiliser], [2–6], [24],
  [G5], [Proposition T / Σ and sporadic-overlap census], [2–8], [31],
)

== Why nothing was run

The repository requires a resume checkpoint in S3 before any experiment.
This container has no upload credential, so the runs wait for a session that
has `CHECKPOINT_UPLOAD_TOKEN`, or for the Iso Lab EKS route.

The repository's `main` also did not compile: five broken merge
resolutions. They are repaired in PR \#2675 and carried on this branch.

== Scope and limits

- This report presents no measured result and no change to cost or the
  leaderboard.
- The prepared cells stop at m = 8 (see the figure). Coverage for m = 9–20
  still needs a chained encoding at those sizes. The m = 10 rotated chain
  at n = 83 is the only existing code there.
- The cells' authors expect nulls in most cells. A measured null at m ≥ 3
  that replaces an asserted closure is itself the deliverable.
