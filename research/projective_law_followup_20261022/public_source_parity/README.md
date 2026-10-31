# Public author-source parity on the exhaustive small-field fixture

The [PDF](REPORT.pdf), [graph](GRAPH.svg), [four-row CSV](ROWS.csv), and
[machine summary](SUMMARY.json) record a finite comparison of the pinned
author Magma `ADDDIVISOR` source with the compiled Rust public weighted law
and independent Cantor addition. The author source was **translated and
evaluated**, not executed by Magma; no Magma executable was available on the
host. The strict [translator](author_eval.py) admits only assignments,
returns, function definitions, conditionals, and field expressions in the
19 selected source functions. It verifies the entire author file's SHA-256
before translation and aborts on unsupported syntax. There were no parser
exclusions in the selected projective public-call dependency graph.

The fixture is `y²=x⁵+x+2` over `F7` and `F11`. The two fields have 66 and
114 valid reduced divisors, respectively. All 4,356 and 12,996 ordered
pairs were checked with unit weights and with `(z₁,w₁,z₂,w₂)=(2,3,3,5)`.
A 512-row-per-field (1,024-row total) bounded gate ran first and passed.
The full run then checked **34,704 weighted rows**, with 34,704 author/Rust
matches and 34,704 Rust/Cantor matches. It recorded zero mismatches, zero
undefined field divisions, zero invalid output weights, and zero other
invalid intermediates. The [failure file](failures.csv) has only its header.
The [branch matrix](branch-matrix.csv) contains 108 joint author-call-path /
Rust-label cells, representing 28 observed author paths and 16 Rust labels.

Every standardized pair row is retained in four compressed shards in the
repository artifact store. The tracked [.artifacts.json](.artifacts.json)
records their SHA-256 digests and sizes; hydrate the shards before opening
them or running the byte-for-byte replay:
[F7 weight 0](F7-weight0.csv.xz), [F7 weight 1](F7-weight1.csv.xz),
[F11 weight 0](F11-weight0.csv.xz), and [F11 weight 1](F11-weight1.csv.xz).
Their compressed digests are in [RAW_SHA256SUMS](RAW_SHA256SUMS); the
uncompressed combined-row, compiled-Rust-row, and input-catalog digests are
in `SUMMARY.json`. For example, `xz -dc F11-weight1.csv.xz | head` opens a
shard without special software.

The exact source bindings are the pinned
[author Magma file](https://github.com/ozbayelif/jac-on-jac/blob/ec98e115ed359c8b7facf559eb0d9f240703f6f5/jac-on-jac.m)
(SHA-256 `7416a7fffaafff371738e7498a6795057372e1c477e0079297eebf2347c2e72a`)
and `../src/bin/projective_law_gate.rs`
(SHA-256 `5976e7f30b3e9c5c49a5fc0f4095404b7322832e4b02f467bc3cb7e0d9dcc8c0`).
The [compiled fixture](probe_tests.rs) enumerates the same valid inputs,
records every public Rust output and branch label, and independently checks
Cantor on each row. The [comparator](compare.py) independently enumerates
the valid divisor catalog, checks it against Rust's catalog, and records
the exact translated author call path and output. The translated Python
program's SHA-256 is also in `SUMMARY.json`.

From the repository root, retrieve the pinned author source and replay both
the bounded and full runs without overwriting evidence:

```sh
python3 tools/artifacts/artifacts.py pull \
  research/projective_law_followup_20261022/public_source_parity
python3 tools/artifacts/artifacts.py verify --remote \
  research/projective_law_followup_20261022/public_source_parity
curl --fail --location --silent --show-error \
  https://raw.githubusercontent.com/ozbayelif/jac-on-jac/ec98e115ed359c8b7facf559eb0d9f240703f6f5/jac-on-jac.m \
  --output /tmp/jac-on-jac-ec98.m
sha256sum /tmp/jac-on-jac-ec98.m
python3 research/projective_law_followup_20261022/public_source_parity/run.py \
  --author-source /tmp/jac-on-jac-ec98.m --check
python3 research/projective_law_followup_20261022/public_source_parity/render.py --check
(cd research/projective_law_followup_20261022/public_source_parity && sha256sum -c RAW_SHA256SUMS)
```

The Rust harness is a temporary source copy compiled through the packet's
Cargo manifest; `run.py` removes it even on failure. Its log, raw rows, and
the source-evaluated author comparison are separate evidence. A translated
program may differ from Magma runtime behavior, so this packet does not
establish a live Magma comparison. These two small-field models do not prove
the law for every smooth odd-characteristic genus-2 curve, construct the
smooth-projective `Pic⁰` bridge, or change the frozen 1,260-target cost ratios.
