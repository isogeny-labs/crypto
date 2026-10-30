# Reproduce the broader fixed-criterion check

Source freeze:1fe0d49974b2a3083772d003b69f4e75fe48aa4b. The original fifteen
source/input bindings remain unchanged in SOURCES-SHA256SUMS. The preparation
README and BUILD.json retain their original preparation status; RESULTS.md
records the completed run additively.

Requirements:C++17,GMP/GMPXX,PARI2.17-compatible headers/library. The default
PARI_ROOT is /workspace/tools/sage_skeleton_env; set it explicitly elsewhere.
The native build uses warnings as errors. No point-count function is invoked
on a production curve. Reproduction performs exact existing-record analysis,
including primality/point-certificate verification; it is not a benchmark.

```sh
cd research/nist_skip_criterion_20261025/broader_census_reanalysis_20261030
sha256sum -c SOURCES-SHA256SUMS
sh build.sh /tmp/broader-build-fresh
sh run.sh /tmp/broader-build-fresh /tmp/broader-run-fresh
g++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic summarize.cpp \
 -o /tmp/broader-build-fresh/summarize
/tmp/broader-build-fresh/summarize /tmp/broader-run-fresh/results /tmp/broader-derived-fresh
```

Fresh directories are required. Launcher checks source/binary bindings and
records every argument, output, native status and expected status. All ten
phases must match; no failure becomes a rejected curve. Shared TSV I/O does
no arithmetic. The separate verifier checks all six files, including all69
position profiles. The three corruption copies must each be refused without
creating a verification receipt. Its toy self-test uses ellcard only on F17.

The local archive named by DELIVERY.json retains the complete run tree,
including all wrong copies, phase records, source snapshots, binaries, native
derived values, document logs/inspection and source package. Extract it into
a fresh directory and check every MEMBERS.json size/hash. This is local
custody, with no remote S3 receipt. The previously denied older paper archive
and all credentials are excluded by explicit input-path allowlist.

The four-page report uses the repository's shared papers/paper.mk. Its four
table files are native summary outputs copied byte for byte. Build with make
check and bundle with make eprint in papers/nist-broader-census-20261030.
The standalone ZIP includes paper.mk. Its fresh build has identical extracted
text and four page rasters; all fonts are embedded and all pages inspected.
