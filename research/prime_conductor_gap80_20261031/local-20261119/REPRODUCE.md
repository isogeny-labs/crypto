# Local measurement evidence

The user explicitly selected local execution. All timed jobs are finished. See
[RESULTS.md](RESULTS.md) and the [four-page report](reports/report.pdf) for the
measured scope, counts, controls and limitations. Native result values are in
[summary.json](summary.json), [both-pilots-analysis.json](both-pilots-analysis.json)
and [the scalar manifest](results/scalar-l0/manifest.json). These are descriptive
local L0 observations; the endpoint pilots each use one target and two rounds.
The separate scalar experiment uses32 points per model and six measured rounds
plus warmup. [pcc:unverified]

The frozen pre-call manifests and run scripts identify original argv, inputs,
qualified library/binary hashes, caps and output locations. `build-scalar.py`
records the exact manual CLI/test build against retained qualified release
artifacts; this is not a claim that an arbitrary fresh dependency resolution
reproduces that binary. Three focused native tests and scalar clippy pass.
The historical full-library build has an existing duplicate-test failure; it
is not reported passing here. [pcc:unverified]

`analysis.rs` uses the actual ecbench record reader and native statistics.
`prepare-*-audit.py` only serializes native inputs for PARI; the `.gp` programs
perform independent curve arithmetic. Final scalar audit v4 checks all1344
outputs; previous rejected development versions and both negative controls
remain preserved. `render.py` only presents native results. [pcc:unverified]

Complete original `results/` bytes are retained locally as
`local-results.tar.xz`,136624bytes, SHA256
`9cc1e5008cb9c1def70c1022a29143ec48c8028044181e0d73d6cd48855d2795`.
`package-results.py` verifies every extracted file and a byte-identical
deterministic repack. [publication-pending.json](publication-pending.json)
inventories every file. The desired S3 URI is a destination only: no upload or
remote verification has succeeded. Fresh `publication/sts.stderr` records the
missing credential dependency. There is no remote `.artifacts.json` claiming
the archive exists. The four benchmark records remain open until authenticated
raw-data publication is completed. [pcc:unverified]

After obtaining this exact archive, verify its digest, extract it in this
directory, and use the recorded native audit commands. Then perform authenticated
upload and independent readback before recording publication or closing the
benchmark records. The raw files are not replaced by endpoint-join summaries.
No timed job should overlap a build, audit or other measurement.

The original80bit prime construction was actually attempted locally and reached
its60second cap without quotient/images; see [the attempt](CONSTRUCTION_ATTEMPT.md).
The retained next construction proposal is [LARGER_MAP_NEXT.md](LARGER_MAP_NEXT.md).
Its twist bridge has not been implemented or evaluated. [pcc:unverified]
