# Native N83 200000-conflict continuation

[Full report](../../reports/run-20261019T1922Z/REPORT.md),
[inspected PDF](../../reports/run-20261019T1922Z/report.pdf),
[native summary](SUMMARY.json), [cards and assessments](cards/),
[compact records](records/), [complete raw S3 publication](PUBLICATION.md).

New caps200000conflicts/1200seconds each, identical retained worker and query.
Full core14/30 is separately frozen; earlier observations stay immutable.
Both actual setup failures and their assessments/correction are preserved.
Unknowns stay bounded, with no UNSAT, best-base, fastest-tool or recovery claim.
Recorded new outcomes (native audit; n=1 each):
K2048: PASS_NATIVE_BOUNDED_ZERO_RELATION_REPLAY; terminal conflicts 200000, models 0, relations 0, rank 0; solver 759947.84721 ms, outer 765409.242176 ms.
K4096: PASS_NATIVE_BOUNDED_ZERO_RELATION_REPLAY; terminal conflicts 200000, models 0, relations 0, rank 0; solver 786700.6455450001 ms, outer 797133.398274 ms.
