# 2026-10-28 spike40 candidate data

## What was done

Reconciled prior fixed point datasets, regularity/FFD/LFD certificates and current
EKS inventory. Consulted existing subspace sweep and primary literature. Prepared
an exhaustive native candidate point generator and explicit N83FB02 layout.

## Decisions made

Candidate basis positions0..25,40 replaces baseline0..26, with exactly2^27
candidate x-values per curve and one base across m2..9. Fully enumerate before
PDP comparison. Preserve actual point counts rather than assuming equality.
No novelty, elite-base, regularity or speed result is claimed by enumeration.

## Open threads

Complete and verify all384 candidate shards; publish source/data/schema/report.
Then freeze matched PDP inputs with common natural targets, separately planted
controls and fixed preprocessing. The entire elite-base goal remains active.

## Errors

Inherited batch-inverse test initially multiplied by selector indices instead
of mapped coordinates; corrected its high-domain reference and retained the
failed log. Final native tests cover mapped inversion, discontinuity, prime
wide-coordinate arithmetic and altered/missing record rejection. EKS storage
failure remains recorded; existing host fallback does not alter IAM or capacity.
