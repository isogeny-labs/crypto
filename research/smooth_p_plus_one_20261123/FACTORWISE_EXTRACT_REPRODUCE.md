# Reproduce the factorwise complete-output controls

[Frozen protocol](FACTORWISE_EXTRACT.md) · [results](FACTORWISE_EXTRACT_RESULT.md).
These are complete-answer controls on fixed public fixtures, not recovery
or an admitted timing comparison.

Use source `d54d9d37bd41d29ad19add7055b13d4926f82470` in a separate checkout.
The [before-call identity](evidence/factorwise_extract_v1/precall_identity.json),
published at `8dd11c8e447013eda2d725a0e381048f7b8d2c9d`, pins the protocol,
command, lockfiles, native sources, executable and msolve. Driver SHA-256:
`6ed2bac2fe2038bce74ac7b4514899c7fd08d9b28d02bf9d1db425493cc3bd33`.
The msolve executable SHA-256 is
`74088d6775c82845faedcbdc24a82d3709e39ee7ebf3e3e4aabde080f9ab3c28`.

```sh
CARGO_BUILD_JOBS=2 cargo +1.99.1 test --release --locked \
  --manifest-path research/smooth_p_plus_one_20261123/native/Cargo.toml \
  --bin dickson-four-summand --features production-controls
CARGO_BUILD_JOBS=2 cargo +1.99.1 test --release --locked \
  --manifest-path research/smooth_p_plus_one_20261123/native/Cargo.toml \
  --bin dickson-four-summand
CARGO_BUILD_JOBS=2 cargo +1.99.1 build --release --locked \
  --manifest-path research/smooth_p_plus_one_20261123/native/Cargo.toml \
  --bin dickson-four-summand --features production-controls
```

Record the rebuilt executable's actual digest and copy it to an immutable
new path. Use a new evidence directory; never overwrite the retained panels.
Preserve the [exact wrapper](evidence/factorwise_extract_v1/command.sh):
one worker, 2 GiB virtual memory, 60 seconds plus five seconds termination
grace per panel, all three panels attempted, separate stdout/stderr/exit
capture. Its live-capture hook copies JSONL without changing stdout or exit
status. The three native interfaces are:

```sh
/absolute/driver 127 4 /absolute/msolve factorwise-cutoff cofactor-msolve
/absolute/driver 8191 4 /absolute/msolve factorwise-cutoff cofactor-msolve
/absolute/driver 8191 5 /absolute/msolve factorwise-cutoff cofactor-msolve
```

The binary fixes the target schedule, method order, three-second shared
query deadline, sixteen-form policy, construction/capture caps, conservative
RUR guard and whole-answer comparison. A partial chart union is never an
accepted answer. Base/reference setup and final JSON emission are outside
the query deadline. Failed-child reasons are retained, but separate child
stderr and partial timeout files are not archived by the shared wrapper.

Replay the recorded administrative summary from the repository root:

```sh
sha256sum -c research/smooth_p_plus_one_20261123/evidence/FACTORWISE_EXTRACT_SHA256SUMS
jq -s -f research/smooth_p_plus_one_20261123/evidence/factorwise_extract_v1/summarize.jq \
  research/smooth_p_plus_one_20261123/evidence/factorwise_extract_v1/f127_d4.jsonl \
  research/smooth_p_plus_one_20261123/evidence/factorwise_extract_v1/f8191_d4.jsonl \
  research/smooth_p_plus_one_20261123/evidence/factorwise_extract_v1/f8191_d5.jsonl
```

Compare the output to `summary.json`. This extracts existing records; the
actual Rust caller performed the signed whole-set equality checks. The old
pushforward arm in the graph is the retained prior panel and was not rerun.
Render the recorded-value graph and report from the study directory:

```sh
python3 plot-factorwise-extract.py
dot -Tsvg factorwise-extract-flow.dot -o factorwise-extract-flow.svg
dot -Tpdf factorwise-extract-flow.dot -o factorwise-extract-flow.pdf
pandoc FACTORWISE_EXTRACT_RESULT.md --from markdown+tex_math_single_backslash \
  --pdf-engine=xelatex -V geometry:margin=0.6in -V fontsize=10pt \
  -o FACTORWISE_EXTRACT_RESULT.pdf
```

These results are producer-local and unsigned. The three new raw files await
required artifact upload and byte replay along with five older files. Use
the supported artifact workflow and preserve the raw records if it fails;
Git publication is not evidence that object-store migration succeeded.
