# Bounded retained-state reader

This packet extends the independent binary inspector and live-prefix capture checker toward mathematical replay. It preserves the exact encoded normalized equations, expansion object and continuation tree (including each search node's local equations, native rows, proof multipliers, matrix batches and operations). It does not evaluate those proofs itself.

## API

```
from semantic_reader import inspect_selected, verify_and_extract
raw = inspect_selected(iterable_of_binary_streams)
objects = raw['retained']  # expansion, normalized, continuation
bound = verify_and_extract(sealed_live_directory, capture_binding_sha, four_source_hashes)
objects = bound['snapshots'][0]['retained']
```

Null/empty distinctions, list order, strings, u64 values, booleans and raw bytes are preserved. Objects use ordinary Python dictionaries/lists and are not mathematically trusted just because they were decoded. `retained['continuation']['partitions'][p]['search']['nodes'][n]['equations']` supplies the generators for that node's native row proof. `...['native']['basis']`, `...['native']['current']` and `...['native']['batches']` expose retained row objects. Rows have `polynomial` and `proof`; each proof entry is `[generator_index, multiplier_monomial]`. Verifiers must validate canonical polynomial syntax, indices, exact Boolean GF(2) identities and node-equation lineage independently.

`inspect_selected` structurally inspects raw wire bytes and makes no capture/source claim. `verify_and_extract` first checks the complete raw-line event chain and all payloads, then checks each snapshot against the externally supplied source identities, original owner-binding derivation, map bytes/ack, and chunk/publication order. Selected objects are returned only if all snapshot checks succeed. This means equality with caller-supplied hashes, not authentication of the producer or proof that the supplied hashes came from correct original algebra. Source inputs and map-to-equation descent remain independently unverified. Use an immutable copied/sealed input directory; concurrent mutation is outside this API's guarantee.

CLI commands emit small inspection summaries rather than full proof objects:

```
python3 semantic_reader.py wire chunk-000000.bin chunk-000001.bin
python3 semantic_reader.py capture /sealed/cell/live --binding HEX --sources trusted-sources.json
```

## Bounds

The inherited inspector bounds total bytes, values, map fields, strings and depth. A separate retention budget admits at most 100000 retained nodes and 16 MiB of conservative allocation-accounting units by default. It charges before materialization: 128 per value, container slots (64/list or 256/map), string storage (64+4*UTF8 byte count), raw bytes (64+2*length) and fixed capture-summary metadata. These are modeled admission units, not measured RSS. Hard API maxima are one million nodes and 256 MiB accounting units. One shared budget covers every returned snapshot. Raw map/certificate byte strings outside the selected subtrees are streamed/discarded; selected raw byte strings are bounded before allocation. Encoded input is still streamed in 64 KiB reads and files are opened one at a time.

The selected continuation is materialized as a bounded subtree, not streamed row-by-row. A larger snapshot can be rejected even when its hash chain is otherwise valid. This is an explicit resource refusal, not evidence against its mathematics. The full capture check still supports only prefixes from genesis, not supervisor delta exports.

## Evidence

39 tests pass: 11 new selected-reader controls and 28 inherited wire/capture controls. New controls cover exact nested content, null/empty distinctions, large selected raw-byte chunks, independent retention limits, declared-size refusal, arbitrary byte chunking, verified capture integration, invalid caps and bounded CLI summary. Tests remain software fixtures.

`RUST_FIXTURE_READBACK.json` records independent decoding of the existing actual Rust serializer fixture both as one 92745-byte file and as 361 ordered chunks, with identical selected objects and SHA256 `4603c87286dc7b60d7323cc9138b7429b76949ba17114096da40eec02509b718`. The fixture has 10147 retained nodes / 1960856 accounting units, a null expansion, two normalized equations and one continuation partition. Its native proof objects are deliberately synthetic and not valid polynomial identities; this readback must not be promoted to a successful semantic replay. No variable count is inferred from a null expansion.

Replay with `python3 -m unittest -v` and `python3 replay_fixture.py PATH_TO/owned-search-capture-encoding-20261107/fixture`. The fixture is referenced by hash and not duplicated. `SOURCE_REFERENCES.json` binds original donor Python and exact Rust wire encoder files; patch files show the two donor extensions. No full library build, native CLI rerun, scientific dataset generation, cloud action or Git mutation occurred. Original algebra, semantic replay, FFD/LFD, solve completion and S3 custody remain outside this reader's verdict.
