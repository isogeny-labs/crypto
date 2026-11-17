# Splitting alone fails on the broader recorded census

30 October 2026. The fixed lower strict-splitting criterion does not separate
the ten published positions from all 59 unpublished positions at any integer
cutoff from 100 through 5000. The separate PARI verifier reconstructs all six
native output files, proves the 76 distinct field/order primes and checks the
existing point certificates. All ten execution phases finish as expected.

This is a scope test of the same criterion, using the already recorded 69
prime-order seed positions and all 82 certified branches. It generates no new
curve, point count, random sample, factorization or discrete logarithm.
The [four-page report](../../../papers/nist-broader-census-20261030/paper.pdf)
contains every one of the 69 selected endpoint scores and all counterexamples.

## What changes when the cohort expands

At decimal 691, 22 unpublished positions tie or fall below their field's
published maximum. The strict rule requires every unpublished count to be
higher, so ties are failures too.

| Field | Published / unpublished | Published strict range | Unpublished strict range | Margin | Positive cutoffs / 4901 | Violators at 691 |
| --- | --- | --- | --- | --- | --- | --- |
| P-192 | 3 / 23 | 57–60 | 44–71 | −16 | 0 | 14 |
| P-224 | 1 / 11 | 50 | 50–66 | 0 | 1801 | 1 |
| p239 | 3 / 14 | 42–57 | 54–71 | −3 | 0 | 5 |
| P-256 | 1 / 5 | 57 | 56–67 | −1 | 0 | 2 |
| P-384 | 1 / 6 | 54 | 56–60 | +2 | 635 | 0 |
| P-521 | 1 / 0 | 52 | NA | NA | NA | NA |

Joint strict success: zero of 4901 cutoffs. Joint inclusive sensitivity success:
zero. P-192, p239 and P-256 each fail individually throughout the range.
All field values come from the [native summary](results/field-coverage.tsv).

The 22 counterexamples comprise 14 of the 31 other-root-only positions and
eight of the 17 order-width-only positions. None of the eleven named omissions
violates the original 691 rule. For example, p192 index 2771 scores 44 while
its field's published maximum is 60; p224 index 2328 ties its publication at 50.
The [complete counterexample table](results/at691-violations.tsv) retains all 22.

The [original 21-choice control](../expanded_cutoff_reanalysis_20261030/RESULTS.md)
reproduces exactly: strict windows 691–718 and 769–772, with no inclusive
window. The broader result limits splitting alone. It does not refute a
conjunction that first applies the additional root/width screens and then
the splitting test: that conjunction still fits these recorded labels.
Its historical use and motive remain unknown. The source draft analysis
does not authenticate those extra screens as original acceptance policy.

## Branches and base points

Every original branch is retained in [branches.tsv](results/branches.tsv).
Branches of each position have equal discriminants, checked before selecting
canonical when present, otherwise the sole other branch. P-224's coefficient
signs are isomorphic. A quadratic twist has opposite trace and the same
discriminant, since (−t)²−4p=t²−4p. Splitting counts alone cannot distinguish
such twist branches. Counting them as independent historical samples would
overstate the census size; the comparison uses 69 positions, not 82 samples.

The earlier [G/b study](../generator_b_execution_20261029/REPORT.md) remains
unchanged: 100548 checked trait cells, zero separators among 19152 fixed
single-envelope tests and zero matches in 880 public-scalar checks. Assigned
smallest-x points obey b=y²−x³+3x: x=0 gives y²=b and x=1 gives y²=b−2.
The current score depends on p,n and supplies no additional G/b attack.
Historical generators for unpublished choices remain unknown.

## Verification and limits

Preparation commit: 1fe0d49974b2a3083772d003b69f4e75fe48aa4b, published before
actual cohort extraction. The producer uses GMP affine arithmetic and residue
enumeration. The verifier independently uses PARI ellmul, proved isprime,
kronecker and pairwise margin differences. Only the GMP integer family and
TSV I/O are shared. All six files agree byte for byte. The original certificate
file's SHA256 matches the earlier acceptance analysis. No inherited complete-
factor feature is asserted for the extra positions; no factoring feature is
used in this comparison.

The run checks 54776 odd-prime branch characters, 338169 seed/cutoff profiles
and all 4901 integer outcomes. Two controls, producer, verifier and three
corruption preparations exit zero; three altered character/profile/cutoff
copies are correctly refused with native exit one. Nothing was excluded or
retried. [Verification](results/verification.json), [completion](results/completion.json)
and [execution metadata](EXECUTION.json) retain the admission boundary.

The recorded census is a lower bound on numerical eligibility under the
listed draft tests, not an exhaustive near-prime search or original visit log.
The separately reported smooth-cofactor extension is not mixed into this
full-prime comparison. All labels were visible. No prospective holdout,
permutation p-value or probability of manipulation is claimed here.

There is no demonstrated ECDLP advantage or authenticated skip criterion.
The 60000 accepted-random-curve baseline and publication percentiles remain
unrun, with the execution/checkpoint/worker requirements unchanged. No finite
trait catalog exhausts all mathematical differences. Exact local analysis
uses the AGENTS9a existing-results development exception recorded before
execution; it does not waive any sampling gate. Raw custody is local, with
no S3 upload or receipt. Reproduction and archive details are in
[REPRODUCE.md](REPRODUCE.md) and [DELIVERY.json](DELIVERY.json).
