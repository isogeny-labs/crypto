# Complete cutoff catalogs and their singular charts

2026-11-25 · finite qualification `full_cutoff_v1` · producer-local, unsigned.

The fine-image multiset catalog covers every distinct-abscissa tuple in the
three existing public factor bases. On F8191 at degree 32 it contains 119
systems, compared with 8 in the prior regular catalog, and covers 1001 unsigned
tuples. Of 16008 admissible signed RR5 bindings, 1104 have effective Jacobian
rank two; the other 14904 have rank three. This locates a singular subfamily
inside the complete point-set construction. No algebraic solver was invoked.
Claim PCC-20261125-941283 remains unverified. [pcc:unverified]

## Coverage and local algebra

| Fixture / selector degree | Base size | Regular / full charts | Unsigned tuples | Valid RR5 bindings | Rank 2 / rank 3 |
|:--|--:|--:|--:|--:|--:|
| F127 /16 |8|4 /36|70|1038|12 /1026|
| F8191 /16 |7|0 /18|35|560|16 /544|
| F8191 /32 |14|8 /119|1001|16008|1104 /14904|

These are complete finite counts for the stated bases, with no sampling
interval. Targets are generated as sums of four public signed points to
check equation coverage; these counts are witness bindings, not newly
collected relations for a fixed target. [pcc:unverified]

![Recorded catalog sizes and the largest fixture's differential ranks.](full-cutoff-counts.pdf){width=100%}

Across the three inputs, all 17696 sign assignments are accounted for:
17606 valid RR5 bindings, 16 identity-target exclusions, and 74 target-abscissa
overlaps. Every valid binding has a unique consistent RR5 interpolation,
nonzero leaf denominators, the expected norm quartic, and a satisfying
assignment to the actual cutoff equations. The full catalogs include every
prior regular chart. [pcc:unverified]

All rank-two bindings occur in charts with exactly two fine images. On the
largest fixture these are 24 charts covering 69 unsigned tuples. The remaining
60 three-image charts and 35 four-image charts have rank three at every
admissible binding checked here. This finite observation does not assert
rank three on every three-image chart over arbitrary fields. [pcc:unverified]

\newpage

## Mechanism and limits

![Catalog construction and its current qualification boundary.](full-cutoff-partition.pdf){width=100%}

Write \(\phi=T_{d-3}\), where \(T_j\) is the j-fold composition of \(X^2-2\). If \(n_b\) base
abscissae map to \(b\), enumerate multiplicities \(0\le m_b\le n_b\) with \(\sum_b m_b=4\) and retain
the monic polynomial \(P_m(Z)=\prod_b(Z-b)^{m_b}\). Each four-element subset of
the base determines exactly one such multiset, and every feasible multiset
has a realization. Equality of the pushed quartic with \(P_m\) selects its
chart, including repeated images. The [frozen argument](FULL_CUTOFF.md)
also gives the RR5 normalization and target-disjoint domain. It concerns
rational point sets, not equality or reducedness of unsaturated ideals.
Claim PCC-20261125-f9071f is a written argument awaiting independent proof.
[pcc:unverified]

When fine images coincide, columns of the root-to-coefficient differential
coincide. With \(r\) distinct images its rank is at most \(r\), so the effective
three-parameter Jacobian has rank at most \(\min(3,r)\). The two-image subfamily
therefore cannot inherit the earlier full-rank argument. Catalog coverage
alone does not give easy elimination: both singular branches and the larger
number of systems must be included in a complete solver comparison.
[pcc:unverified]

The classical quartic pushforward identity has evaluated certificate
**IDC1h2ed7d95dc884eb62**, issued and replayed by the unchanged native repository
checker at 32 statement-derived plus 16 additional finite-field points.
A false identity with one added to the right side is rejected. The certificate
has no formal proof pointer; its evaluated standing does not prove the abstract
partition or its cost. Claim PCC-20261125-f70c64 remains unverified.
[pcc:unverified]

## Evidence and next step

The driver and identity example build; 36 feature tests and 9 default tests
pass. All five frozen commands exit 0 with empty stderr, and their before-call
hashes remain unchanged. Mutations remove a chart, duplicate one or alter a
coefficient; all are rejected by the coverage checker. Source-test claim
PCC-20261125-3a7978 remains unsigned. The checker shares this lane's field,
curve and equation source. No independent runner or signed verification is
represented. [pcc:unverified]

Source **16f5902e2d**, before-call identity **5f69c1ea75**;
[raw records](evidence/full_cutoff_v1/), [summary](evidence/full_cutoff_v1/summary.json),
[reproduction](FULL_CUTOFF_REPRODUCE.md). Five commands were planned, attempted
and completed; zero retries or solver calls. The local flow receipt has its
own schema and does not claim canonical export validation. No timing, peak
memory measurement or admitted cost ratio is reported.

Next integrate the complete catalog into the bounded extractor, preserving
whole-answer comparison and incomplete status whenever any branch fails.
Inspect the two-image charts under the existing RUR completeness guard;
price every branch before comparing cold recovery. The prior output/recovery
records and their graphs are unchanged. No leaderboard graph changes because
this component records coverage and ranks, without a performance comparison.

Sources: [Petit–Kosters–Messeng, PKC 2016, sections 3.2/3.4](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf);
the lane's [earlier refinement](REFINEMENT.md); the lab's
[first-collision study](../prime31_ic_20261115/native/DICKSON_FIRST_COLLISION_S3_20261115.md).
KB was unavailable; this is a targeted review. The composed-map factor base
and Graeffe identity are prior art. Existing artifact migration for the prior
larger records remains [pending](DEVELOPMENT_CHECKS.md#artifact-migration-pending).
