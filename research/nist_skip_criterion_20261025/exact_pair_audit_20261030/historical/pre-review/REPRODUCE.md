# Reproduce the exact allocation audit

Source freeze fc7bac88dc217d01c59ff750d973d2796ebba1b4; all 21 original
preparation bindings stay unchanged. The preparation README/BUILD status is
historical; RESULTS.md adds actual completion. Requirements are C++17 and
standard-library tools, with no PARI/GMP or external Monte Carlo dependency.

```sh
cd research/nist_skip_criterion_20261025/exact_pair_audit_20261030
sha256sum -c SOURCES-SHA256SUMS
sh build.sh /tmp/exact-pair-build-fresh
sh run.sh /tmp/exact-pair-build-fresh /tmp/exact-pair-run-fresh
g++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic summarize.cpp \
 -o /tmp/exact-pair-build-fresh/summarize
/tmp/exact-pair-build-fresh/summarize /tmp/exact-pair-run-fresh/results /tmp/exact-pair-derived-fresh
python3 flatten.py /tmp/external-score-fields-fresh.tsv
cmp inputs/external-score-rows.tsv /tmp/external-score-fields-fresh.tsv
```

Directories/files must be fresh. flatten.py only copies integer source fields;
it computes no trait, percentile or null. The launcher records arguments,
native/expected statuses, stdout/stderr and source/binary bindings. Every phase
must match, including three expected refusals with no verification receipt.
Any timeout or unexpected failure stays a failure, not a dropped observation.

The raw local archive in DELIVERY.json includes all source snapshots, both
methods, inputs, distributions, controls/corrupted copies, build/phase records,
native summary and paper/source/inspection files. Verify every MEMBERS.json
size/hash after fresh extraction. Its delivery receipt sits outside its own
archive to avoid a hash cycle. No S3 custody is asserted.

The source/publication audits retain scope: 60 exact seed-ID score matches,
eight publication-score observations checked as within-field multisets. Source
model percentiles and unauditable 48-bit aggregates are not validated by this
audit. External procedural deviations remain; no external gated run is repeated.

The report lives under papers/nist-exact-pair-audit-20261030 and uses shared
paper.mk. Both table files are native summary output copied without arithmetic.
make check and a freshly extracted standalone ZIP build pass; extracted text
and all page rasters match. No earlier paper, source binding or raw archive is
rewritten. The separate60000-real-curve experiment remains unrun.
