# Executed: twist7 kernels → rational isogenies → source relation rows

The fixed p827 experiment passes native production/replay and independent
PARI. [Report and graph](../reports/kernel-relations-20261027/RESULTS.md) ·
[PDF](../reports/kernel-relations-20261027/RESULTS.pdf) ·
[strict admission](ADMISSION.json) · [protocol](PROTOCOL.md) ·
[structured pre-data freeze](protocol.json) · [instrument pins](FROZEN.json).

All8 twist7-kernel lines define degree7 rational source maps; all16 generic
dual compositions and every rational point/recovery check pass. All576 signed
query sets and18 coefficient matrices preserve their source rows exactly.
Ordinary base:40 witness rows/rank9/recovered37. Twist progression:66 rows/rank8;
the [separate diagnostic](../execution-v9/README.md) confirms target ambiguity.
These are one public DLP and correlated relation queries, not18 independent
targets. No novelty, extra transported information, solving gain or NIST result.

Native source93a8281dbf29a51ba4be3ba57dce3d6222402bc8 and protocol were pinned
and committed before data; Setec gate receipt is in execution-v4/checks/
kernel-relations-pre-data-checkpoint-v1. Bounds300s/4GiB/CPU0/one thread,L0.
The documented EKS access/custom-image fallback applies; no paid resources.
[Host](HOST.json), exact actual argv/exits/hash receipts and every raw record
are retained. The full unified [export](experiment.export.json) binds to
the frozen protocol and every file with zero findings. Operational clocks are
unadmitted. No validation-gated benchmark/leaderboard row is added.

All nine exact generator-bound prime-field records have
[EC1 aliases/full UIDs](EC1_MODELS.json). [Ordered routes](ORDERED_ROUTES.json)
record source/destination UIDs, direction, degree, map IDs and artifact hashes.
Study TWK1 map IDs do not claim global ISO1 certification. Eight canonical
model records are emitted in registry-additions.json; historical catalog
records absent from the checkout are restored from unchanged old manifests,
then the native canonical registry checker checks the complete union.

Four structural Lean theorems are in ../lean-v5. Read their exact hypotheses:
additive groups/homomorphisms, dual∘map=[7],823-torsion. Fresh frozen replay and
native axiom audit pass on standard foundations. Concrete finite-field curve
groups, map definitions, descent and dual-identity formal bridges remain open.

## Hydrate and replay

Use the maintained artifact tool; the configured remote is the user-confirmed
`s3://setec-astronomy-artifacts/store`:

```sh
python3 tools/artifacts/artifacts.py pull research/twist_routes_1_3_20261021/execution-v8
cd research/twist_routes_1_3_20261021/execution-v8
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
```

From repository root, build the source at the FROZEN.json revision with
Rust1.90, Cargo.lock, no optional library feature:

```sh
cargo +1.90.0 build --release --locked --manifest-path research/twist_routes_1_3_20261021/runner/Cargo.toml --bin twist-kernel-relations --bin twist-pari-transport
research/twist_routes_1_3_20261021/runner/target/release/twist-kernel-relations produce /tmp/FRESH_KERNEL_REPLAY
```

Compare RESULTS.json and verify.gp byte-for-byte with the extracted tree.
Run native twist-pari-transport SCRIPT FRESH_RECEIPTS using the pinned GP/tool
layout, then `twist-kernel-relations audit RAW PARI FRESH_ADMISSION`. The audit
recomputes all native output and enforces actual process exit, hashes, exact
sentinel and stderr policy. Existing output directories are rejected. For Lean:
`bash research/twist_routes_1_3_20261021/lean-v5/replay.sh FRESH_ABSOLUTE_RECEIPTS`.
Dependencies/toolchain are pinned alongside the source; no supplied credentials
are in files or commands. Published replay paths need local tools/hydration.

Raw:one deterministic archive,22,024 bytes/10 files, SHA256
58618a66dfec1cbdf743d56e8d0eedde83279bb3b9906adea1710b89f50dd768.
[Member inventory](RAW_MEMBERS.json), [packing recipe](support/archive-evidence.mjs)
and [fresh remote/all-member/identical-repack verification](RAW_VERIFICATION.json)
are separate from immutable raw bytes. New local runs use fresh paths.

Next cost panel is **not run**: price kernel/map discovery, conversion,
same-base complete decomposition, signs/exception handling, useful relation
algebra, recovery and strongest group/solver controls. Card
XP-20261027-twist-kernel-isogeny-orbit is partial, with its cost phase blocked
and zero funding. Original full-campaign criteria remain unchanged.
