# AWS replay and next gate

Use source commit `d3a09099cb1f16c39981dce95786b2a857be38e4` from
PR #2557 to reproduce the constructed witness. The factor base is already
published in S3 at the frozen prefix below. The large XCNF payload is not
published; regenerate it using the source-bound command. Use a clean checkout
and a new output path, because the worker refuses to overwrite evidence.

```sh
prefix=factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/d18-full-direct-subgroup-v1/a0
object=c295da8898cfad29a053621c768ca059eea4263f89f5ca78b4f98af43c383f3f.jsonl.gz
base_dir="$PWD/n83-d18-input"
mkdir -p "$base_dir/objects"
cp research/koblitz_n83_factor_base_sweep_20261008/verification/d18-full-orbit-20261025/manifest.json "$base_dir/manifest.json"
aws s3api get-object --bucket crypto-autoresearcher --key "$prefix/manifest.json" \
  --checksum-mode ENABLED "$base_dir/s3-manifest.json" > "$base_dir/manifest-get-metadata.json"
aws s3api get-object --bucket crypto-autoresearcher --key "$prefix/objects/$object" \
  --checksum-mode ENABLED "$base_dir/objects/$object" > "$base_dir/object-get-metadata.json"
cargo build --release --example koblitz_n83_factor_base_export
fixture="$PWD/research/koblitz_n83_factor_base_sweep_20261008/pilot-01"
mkdir -p "$PWD/n83-witness-output"
timeout 310s target/release/examples/koblitz_n83_factor_base_export \
  primary-full-d18-m4-l0-xcnf-witness "$base_dir" "$fixture" \
  "$PWD/n83-witness-output/result.json"
```

Require the output `status=PASS_NATIVE_XCNF_WITNESS`, the exact XCNF SHA-256
`0d7bb7ef45ae19937220945129f61baccdd72b85c58bb7f16dcb56ca3ec24d7c`,
329,726 variables, 2,807,752 CNF clauses, 3,901 XOR rows, and a source
attestation of `clean_checkout`. A different binary hash across OS or compiler
is expected and must be recorded. The first run's binary SHA-256 was
`7510e945f44bf7861adf8277d4ade50cac6a2df39f23bd682fda212c541fa082`.

The host-independent [`replay.py`](replay.py) checks the first run's saved
receipts against its exact model and XCNF, plus a fresh S3 base with provider
checksums. It was run successfully on the original local payloads; the
portable [receipt](portable-replay.json) is committed. Once the exact payloads
are available to the AWS host, invoke it with `--base`, `--receipts`, `--xcnf`
and `--model`. For a newly regenerated AWS run, invoke the same original-field
formula and group checks from [`audit.py`](audit.py) and retain an AWS-specific
source, binary, host and provider receipt rather than equating it with the
original Mac run.

The constructed witness is a correctness gate. The next selection gate is a
fresh, unpinned natural-target search with useful relation rank and factor-base
logs, followed by unseen-target descent, complete cold one-target costs and
matched same-target rho. The earlier 8-GiB Linux access gate is blocked by
`AccessDenied` on this S3 base for its GitHub AWS principal. Do not report a
natural-search or total-runtime selection from this constructed control.
