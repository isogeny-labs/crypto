# Verified empirical separator: strict splitting counts

The externally reported cutoff-691 criterion reproduces exactly. Read the
[report](REPORT.md), [PDF](REPORT.pdf), [all 21 rows](results/at691.tsv),
[verification](results/verification.json) and [canonical export](export.json).
This is an independent native replay of a previously reported empirical
pattern, not a new discovery or a historical explanation.

For each curve let `t=p+1−n`, `Delta=t²−4p`, and count odd primes ell≤691
with `(Delta/ell)=+1`. Published/omitted ranges are 57–60/61–62 for P-192,
50/53–59 for P-224, 42–57/61–71 for P-239 and 57/58 for P-256. Every
paired label separates in that lower-count direction. P-384 and P-521 remain
in the computation with counts 54 and 52, but have no skipped same-field
controls. This is a per-field criterion; one global count threshold fails.

The complete fixed-cutoff search has four separating positions:
691, 701, 709 and 769. Of 4,480 stream-count-preserving labelings, 23 admit
some separator in the declared 303-position strict-count family. This matches
the reported family calibration, not a correction for all investigated
hypotheses or an independent historical prediction.

Zero characters are excluded from the strict statistic. Including ramified
primes makes the cutoff-691 separation fail for P-192. The earlier SEA
snapshot counts and coverage proxy therefore are not this same measurement.
The G–b and complete-factor results remain unchanged in their frozen studies.

See [reproduction](REPRODUCE.md) and [run report](reports/attempt-001/REPORT.md).
Raw outputs, all three corrupted controls, executed binaries, source snapshots
and planning observations are in `evidence/runs.tar.xz` through tools/artifacts.
Hydrate the ignored runs/ members before canonical file verification.
The historical selector and both weakness hypotheses remain unresolved.
