# Synthetic volcano / ECDLP experiment

**Invented data: no elliptic curves, point coordinates, isogeny maps, relation
matrices or recovered logarithms were constructed.** This is a reproducible
planning model requested as a fully synthetic experiment. No number here is a
measurement of a cryptographic implementation or an empirical security result.

Start with the [interactive explorer](run-final/reports/explorer.html),
[eight-page report](run-final/reports/experiment_report.pdf), or the
[volcano diagram](run-final/reports/volcano.svg). Download the explorer to run
its embedded JavaScript; GitHub's source viewer does not execute it. The
explorer needs no network requests. Its data-download links resolve after
extracting the archive below.

## Requested coverage

| Requirement | Synthetic implementation and records |
| --- | --- |
| Field-size labels 30, 40, 50 | All nine size/family combinations, with a matched nominal subgroup prime just below `2^(bits-2)` |
| Prime, ordinary binary, Koblitz | Three family labels; only the hypothetical Koblitz-origin surface receives an assumed Frobenius discount |
| Deep volcano | Five strata with `1 / 4 / 12 / 36 / 108` vertices; 161 abstract vertices per component |
| Horizontal connections at every stratum | Degree labels 11, 23, 29 with illustrative cyclic shifts; 483 overlay multiedges and 160 degree-3 vertical edges |
| Ten points and min/max/average | Ten symbolic holdout target labels at every vertex; min, mean, max, median and SD retained |
| Index-calculus comparison | Rho, two-summand IC with a scan, and three-summand IC with a pair table |
| Original curve or floor | Surface, prespecified floor path, profile every vertex, and profile one vertex per stratum |
| Size-dependent patterns | Null, injected floor advantage, injected floor penalty; cold and ten-target shared setup; native-cost weights 0, 0.1 and 1 |

The original run has **130,410 synthetic observations**, 78,246 per-vertex
summary rows and 1,944 charged strategy-summary rows. Three distinct pilot
targets choose profiled destinations without reading the ten holdout labels.
The complete graph means this specified abstract component, not every elliptic
curve or every possible isogeny degree over a field.

## Reproduce

From this directory, a C++17 compiler and standard library are sufficient for
the simulator and independent CSV audit:

```sh
make check
```

That builds both native programs, checks depth 1/4/5 graph contracts, generates
the full requested matrix, audits every raw-to-summary statistic, checks a
second fixed-seed replay byte-for-byte, checks refusal to overwrite a run, and
audits the original archived records. It writes temporary runs outside the
working tree and removes them afterward. No solver is executed.

To produce a new persistent run and its reports:

```sh
python3 -m pip install matplotlib numpy reportlab
bash run.sh run-new
```

Python only lays out recorded values and documents. The native C++ programs
own simulation and statistics. Plotting imports the repository's canonical
[`volcano_diagram.py`](../../tools/volcano-diagram/volcano_diagram.py);
there is no study-local renderer fork. The v3 presentation flag uses
"abstract vertices" and "simulated range"; v1/v2 inputs remain supported.

The model accepts `--seed` and `--depth 1..5`. The delivered report prose is
specifically for seed 20261018 and depth 4; update captions before distributing
reports for a different design. Replaying with the same compiler/standard
library is byte-identical. C++ distribution algorithms can differ across
toolchains, so cross-toolchain bit identity is not asserted.

## Original data and provenance

The original October 18 raw tree is retained as one deterministic
[`runs.tar.xz`](runs.tar.xz), preserving its original `run-final/` paths.
The curated reports remain loose beside the archive for browsing. Extract and
audit without modifying the original observations:

```sh
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
make all
build/audit run-final
# Optional: restore the explorer's compressed-table download links.
gzip -nk run-final/trials.csv run-final/curve_summary.csv run-final/strategy_trials.csv
```

The archive retains raw observations and per-vertex summaries,
`config.json`, `dashboard_data.json`, level/strategy/pattern/pilot tables,
all vertices and edges, phase diagnostics, original environment and validation
records. The three large CSVs are stored once, as their original uncompressed
bytes inside the xz archive; compressed download copies can be regenerated
with the optional command above. `tools/check.sh` extracts into a temporary directory.
Extracted raw files and generated runs are ignored by Git.

The archive is packed with sorted names, fixed October 18 timestamps,
numeric owner/group zero and normalized modes. To reproduce its bytes after
extracting it into an otherwise empty staging directory:

```sh
tar --sort=name --mtime='2026-10-18 00:00:00Z' --owner=0 --group=0 \
  --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - run-final \
  | xz -6 -T1 > runs-repacked.tar.xz
```

Do not include the separately tracked reports or generated gzip copies when
repacking: use the archive's extracted raw tree before generating optional
download copies. The [source provenance](reference/sources.json) pins
the inspected repository blobs. `reference/ORIGINAL_README.txt`,
`reference/STANDALONE_SHA256SUMS`, the original integration note, and the
frozen PDF describe the standalone artifact as delivered on October 18;
this README describes its subsequent repository integration.

## Interpret the model

Work is an invented group count plus `weight * invented_native_count`. It is
uncalibrated and never converted into seconds. The cost model explicitly
includes preprocessing, relation collection, linear algebra, target descent,
transport, costed verification, and unsuccessful pilot candidates. Full rank
and recoverability are assumptions, not verified answers. The memory column
is modeled capacity; the simulator does not allocate the large pair tables.

The null has no injected ordinary-curve level multiplier. The positive and
negative controls multiply oracle scan cost by `0.55^level` and `1.7^level`.
Those trends are inputs. Any apparent size law is a consequence of the
chosen equations; three size labels establish no empirical exponent.

Per-curve statistics summarize ten targets. Per-level statistics first average
vertices for each target and then summarize the ten target-level means. There
are 108 floor candidates and only one surface candidate, so selecting a
minimum can favor the floor under an exchangeable null. Both full and balanced
pilot selection are retained. Report cold and batch costs separately.

A fixed-degree ordinary volcano has horizontal edges only at its surface.
Separate degree labels supply the requested deeper horizontal connections.
The shifts are not calculated ideal actions. No depth-4 Koblitz curve at
exact extension degree 30, 40 or 50, or certified curve containing the nominal
subgroup prime, is claimed to exist in this artifact. Descendants lose the
assumed degree-2 Frobenius discount and use ordinary binary costs.

## Validation and integration scope

The original run passed 986 structural/accounting assertions, the independent
native audit, and byte-identical replay on g++ 14.2/libstdc++. Its eight-page
PDF and graphs were inspected, and the self-contained explorer was checked in
Chromium on desktop/mobile with all nine family/size cells, vertex selection,
overlay toggling and no JavaScript errors. These checks establish simulator
and document consistency, not cryptographic correctness.

The integration adds a path-scoped CI workflow for native contracts, frozen
record audit and report rendering. Validation for the PR is recorded in
`INTEGRATION_VALIDATION.txt`.

The measured boundary ledger, index-calculus leaderboards, curve registry,
progress timeline and `benchmarks/` are unchanged: this artifact supplies no
measured performance result, speedup or realized curve identity to register.
The standard renderer adds explicit synthetic terminology without changing
ordinary labels. Its original example is checked alongside the synthetic
reports. No existing frozen evidence is rewritten.
