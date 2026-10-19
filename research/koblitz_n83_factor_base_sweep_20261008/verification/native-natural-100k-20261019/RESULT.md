# Native N83 100000-conflict continuation

Both K2048 and K4096 reached100000conflicts with zero models, relations and
rank. Native query/custody replay and director receipt/output-digest checks
passed for each. Neither hit the900-second wall cap or OOM. No useful natural
relation, full-cold winner, unknown-scalar recovery or matched-rho ratio follows.

- K2048: native solver383937.390858ms; supervised outer393756.253810ms.
- K4096: native solver395987.838978ms; supervised outer407619.165871ms.

Each is n=1 under the separately frozen protocol; clocks are descriptive,
without an A/A timing interval or performance ranking. Both pre-solver setup
failures are preserved; no timed solver was retried.

[Full report](../../reports/run-20261019T1835Z/REPORT.md),
[inspected two-page PDF](../../reports/run-20261019T1835Z/report.pdf),
[native summary](SUMMARY.json), [cards and assessments](cards/),
[compact records](records/), [complete raw S3 publication](PUBLICATION.md).

The existing complete4398-pass/118-ignored library and native harness checks
are inherited evidence on unchanged measured code/ELFs, not new test reruns.
Source freeze660f0f4b10, measured worker3a31bd2b…. Unknowns remain bounded;
there is no UNSAT or absent-decomposition proof. Prior1000/10000-conflict
observations stay immutable and are linked through the upstream S3 publication.
