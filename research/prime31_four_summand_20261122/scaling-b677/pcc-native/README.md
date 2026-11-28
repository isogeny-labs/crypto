# Native unsigned PCC metadata preparation

This administrative tool prepares one `pcc.claim/v1` record with
`kind: statement`, `claim_type: data`, and unsigned `attestation` subject pins.
It has no ledger-append command. The prepared claim and three-field pending
registration body remain separate from the canonical ledger until the parent
publication task has reconciled main.

## Contract and reuse

`tools/pcc/pcc.py` is the canonical specification. Its exact SHA256 is pinned
in `input.json`. The new tool supports a strict unsigned statement/data subset;
it does not replace the PCC trust or signature machinery. A statement with
sound pins remains `not_covered` under PCC and cannot establish execution or
mathematical correctness.

No existing Rust PCC writer was found in the current repository, the prior
RR5 publish/qualify checkouts, or the referenced Dickson study paths. The
canonical JSON routine is reused verbatim from
`src/cryptanalysis/ecbench/claim.rs`: `build.rs` pins the full source and selects
the exact `canonical_utf8` function. It sorts keys, uses compact UTF-8 JSON,
and rejects floating-point numbers. Claim digests exclude only the top-level
`standing` field, matching PCC. The preparation checks the digest of an existing
RR5 claim against its previously registered canonical digest.

## What the native check does

It checks the supported claim schema, repository path containment, exact file
hashes, declared exit statuses and unit-test summary lines. It follows each
recorded source inventory. For the first failed build/run only, the two source
paths corrected later resolve to their explicitly preserved original copies;
all other source paths must still match their recorded hashes. It parses the
existing scalar-diagnostic JSON without rerunning its arithmetic. It does not
execute a solver, unit suite, scientific verifier, admission gate, or signature
operation. Recorded binary hashes are receipt observations; this metadata check
does not establish the availability or provenance of external executables.

The claim pins important raw artifacts directly, including the source
inventories. PCC itself checks those pins as file bytes; it does not recursively
follow an inventory. The separate native preparation receipt describes that
additional administrative check. None of these producer observations is a
signed PCC verdict.

The later main refresh changed two source files covered by the unsuccessful
full-root gate, `ec_index_calculus.rs` and `research_bench.rs`. For that gate
alone, `input.json` explicitly resolves those two inventory entries to their
exact blobs at the recorded pre-refresh commit. The native checker reads their
raw Git bytes and requires the recorded SHA256 values. It labels those
historical resolutions separately from the direct first-build source copies;
it does not assert that the refreshed root tree was the tree compiled by the
old gate. No original receipt is edited.

`producer.commit` records the published core source/evidence commit supplied by
the parent publication task. Its declared tree must match the local core tree;
the native tool does not fetch or independently authenticate the remote commit.
The local HEAD is retained separately. The tested working-tree sources are
identified by their immutable inventories. The first failed suite, scalar diagnostic,
source snapshots, successful rerun, public-example checks, driver build and
failed admission attempt are all retained.

## Commands

From the repository root, after other bounded build/run processes have ended:

```sh
cargo build --offline --locked --release --manifest-path research/prime31_four_summand_20261122/scaling-b677/pcc-native/Cargo.toml
```

The built binary accepts:

```text
rr5-b677-pcc-metadata prepare REPO_ROOT REPOSITORY_RELATIVE_OUTPUT_DIRECTORY
rr5-b677-pcc-metadata check REPO_ROOT REPOSITORY_RELATIVE_CLAIM_PATH [EXPECTED_CANONICAL_SHA256]
```

The output directory must be new and below this `pcc-native` directory. Every
generated file uses exclusive creation. `prepare` writes the metadata receipt,
claim, pending registration body and separate native check receipt. It checks
that the canonical ledger's bytes are unchanged across preparation. The pending
body has only `claim_id`, `claim_sha256`, and the eventual canonical claim path;
`seq`, `prev`, `at`, and the ledger-envelope hash must be assigned by the
maintained append contract after main is reconciled.
