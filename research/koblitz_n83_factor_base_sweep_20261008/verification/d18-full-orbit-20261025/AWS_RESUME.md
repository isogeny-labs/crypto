# AWS resume: complete primary N83 D18 factor base

Resume from PR [#2557](https://github.com/aburan28/crypto/pull/2557), branch
`codex/n83-aws-handoff-20261025`, stacked on PR #2551. The published
construction and custody result is commit
`4e9faa76506cf6a19c48b9c0ce7a665121eb91d3`; the exporter and protocol
were frozen at `5bac5f1d9221b908f743520aa9039dd48d175f0e`. Preserve those
identities when auditing this archive. The curve is
`icv1-f2m83-tm6151469093347-debefd74`, Koblitz `a=0`, prime subgroup
order `2417851639230796216685689`. Do not combine it with the separate
`a=1` diagnostic subgroup or treat its performance as primary-curve evidence.

The complete direct-subgroup dimension-18 archive has policy
`direct_subgroup_x_d18_full`, **32,643 signed-Frobenius columns** and
**5,418,738 unique labeled point rows**. Its immutable S3 object is:

```text
s3://crypto-autoresearcher/factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/d18-full-direct-subgroup-v1/a0/objects/c295da8898cfad29a053621c768ca059eea4263f89f5ca78b4f98af43c383f3f.jsonl.gz
```

The hash-bound S3 manifest is the sibling `manifest.json`. The compressed
object has 160,747,529 bytes, SHA-256
`eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b`,
and BLAKE3 `c295da8898cfad29a053621c768ca059eea4263f89f5ca78b4f98af43c383f3f`.
The S3 manifest SHA-256 is
`1b93351febd47dac4de8c5dd39dbf58564d0057a044054baf448d978f6c0d616`.
The independently reconstructed point-set BLAKE3 is
`31aa8e1c1340f4db4898144af59f6f34c87fa63e0d88b175b854f58ce9008a7d`.
The [result](RESULT.md), [generic replay](replay.json),
[fresh-GET replay](fresh-replay.json), [provider receipts](s3-independent-audit.json),
and [`verify_s3.py`](verify_s3.py) give the complete existing audit. All raw
provider metadata and five rejection controls are retained here.

To independently fetch and replay on an AWS Linux host, first check out the
exact PR head and use a fresh output directory. The commands below assume
this directory is the current working directory and that AWS credentials
can read the user's bucket; choose any new path for `base_dir`:

```sh
base_dir="$(mktemp -d)"
mkdir -p "$base_dir/objects"
cp manifest.json "$base_dir/manifest.json"
prefix='factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/d18-full-direct-subgroup-v1/a0'
object='c295da8898cfad29a053621c768ca059eea4263f89f5ca78b4f98af43c383f3f.jsonl.gz'
aws s3api get-object --bucket crypto-autoresearcher --key "$prefix/manifest.json" \
  --checksum-mode ENABLED "$base_dir/s3-manifest.json" > "$base_dir/manifest-get-metadata.json"
aws s3api get-object --bucket crypto-autoresearcher --key "$prefix/objects/$object" \
  --checksum-mode ENABLED "$base_dir/objects/$object" > "$base_dir/object-get-metadata.json"
cmp s3-manifest.json "$base_dir/s3-manifest.json"
printf '%s  %s\n' \
  1b93351febd47dac4de8c5dd39dbf58564d0057a044054baf448d978f6c0d616 \
  "$base_dir/s3-manifest.json" | sha256sum -c -
printf '%s  %s\n' \
  eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b \
  "$base_dir/objects/$object" | sha256sum -c -
cargo run --release --manifest-path ../../../../Cargo.toml \
  --example n83_d18_full_orbit_export -- replay "$base_dir"
```

The generic replay requires the committed source CSV and census summary in
this checkout. It independently checks every point, subgroup label,
Frobenius phase, sign, orbit closure and global point-set hash. Check the
fresh GET response's `ChecksumSHA256`, `ServerSideEncryption`, `ContentLength`
and metadata against the saved receipts. Keep the GET metadata, fresh replay,
exit status and host/resource observations with the new run. If any digest or
identity differs, stop before measuring a search.

## Solver admission for this larger base

The opt-in `primary-full-d18-adapter-check` command imports this exact
archive into the generic signed-Frobenius base and independently reconstructs
its orbit maps and labels. It requires the committed producer manifest,
generic replay and S3 audit, exact compressed SHA-256/BLAKE3 and point-set
hash. Run it only after the fresh S3 fetch above:

```sh
cargo run --release --manifest-path ../../../../Cargo.toml \
  --example koblitz_n83_factor_base_export -- \
  primary-full-d18-adapter-check "$base_dir" "$base_dir/import.json"
```

Require a `PASS` receipt before the [frozen first access gate](NATURAL_PROTOCOL.md).
The [separate local positive-control diagnostic](l0-positive-20261025/RESULT.md)
re-imported the complete base and built the pinned circuit, then reached its
300-second shared-host L0 cap before a solver answer. Its audited censoring
does not replace the Linux control or justify a natural-yield estimate.
The [opt-in shared-root comparison](shared-root-20261025/RESULT.md) reduced
the full-base model by exactly 8,617,488 clauses, but its own planted
control also reached the same 300-second cap without a solver answer.
That gate adds an opt-in `primary-full-d18-m4 positive|natural` worker with
hard resource checks, durable phase/shape records, SAT-model verification,
group lift, row replay and rank-one control. Its natural result requires a
separate replay before interpretation. The standard panel policy whitelist
and the previous 16,627-class D18 worker remain separate. PR #2490 has a
shared-root-bit encoding for the half-size base; test a reviewed integration
against this full-base identity before using its capacity or timing claims.

For the existing per-representative signed-Frobenius circuit at `n=83`,
`m=4`, `r=32,643`, the exact **preflight bound** from
`chain_s3_max_variables` plus `frobenius_orbit_domain_size` is 329,726 SAT
variables and 11,238,656 *domain* clauses. The earlier 300,000-variable /
8,000,000-domain-clause cap rejects it before construction. A 400,000-variable
and 12,000,000-domain-clause cap admits the preflight calculation; it does
not establish that a model fits memory or searches usefully. Its full CNF
also contains field-arithmetic clauses. Measure actual build wall, RSS,
total clauses, XOR rows, propagation, and search under a hard memory and wall
cap before allocating a natural-query panel. An optimized shared-root-bit
encoding needs its own exact size and equivalence controls.

The [natural protocol](NATURAL_PROTOCOL.md) freezes the first positive and
natural arms before execution. Use the same public primary-curve fixture as
the earlier U/D panel and retain distinct base identities. Run known-relation
positive and changed-target negative controls through the exact importer and
model. Then run same-target natural
queries with independent group and rank replay. Record import, construction,
all failed/capped searches, useful-rank growth, sparse linear algebra,
unseen-target descent and matched same-target rho as separate cold charges.
A timeout or memory cap is censored. The earlier half-size D18 m4/m5 natural
panels and this PR's XOR panels ended at their stated bounds without a
checked natural relation; none supplies a yield estimate for this larger
base. `selected_best_total_runtime` remains `null` until the full comparison
is measured and independently replayed.
