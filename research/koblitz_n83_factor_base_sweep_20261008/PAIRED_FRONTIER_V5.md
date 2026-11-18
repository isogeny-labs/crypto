# N83 symmetry-subset frontier v5

**Status:** frozen, addressable design extension. The new cases have not run a
relation producer, linear algebra, or a complete cold index-calculus pipeline.
`selected_best_total_runtime` remains null.

## Parent identity and coverage

The native generator `examples/n83_factor_base_sweep_v5.rs` verifies the exact
SHA-256 bytes and dimensions of `size-frontier-v2.json`,
`paired-frontier-v3.json`, and `paired-frontier-v4.json` before generating or
decoding the v5 design. Their hashes are recorded in
`paired-frontier-v5.json`. All **26,939,278,080** v4 ordinals retain their
original meaning. The native `--case` command resolves their base or pair and
solver choices; the frozen parent design supplies the original disposition and
support fields.

The frozen v3/v4 solver grid has six mutually exclusive symmetry labels:
`none`, `ordered_summands`, `signed_frobenius`, `two_torsion`,
`four_torsion`, and `ordered_first_pair`. V2 has the first five. V5 treats the
five nonempty policies as a five-bit subset. It appends all 26 subsets of size
two through five for each base shape, plus the `ordered_first_pair` singleton
missing from the v2 single-base shape. Each added mask crosses every one of
the **8,640** frozen non-symmetry solver settings. The original split bits,
backend (including WDSat and FES variants), binary/Gray enumeration,
zero/one/two large primes, thread count, linear algebra, and summand count
remain crossed. The base recipes and nested/cross-stream pair identities are
the exact frozen parent recipes.

| Suffix region | Source specs | Residual choices | New masks | Other solver settings | Added ordinals |
|---|---:|---:|---:|---:|---:|
| Single base | 1,320 | 1 | 27 | 8,640 | 307,929,600 |
| Same-stream nested pair | 2,970 | 3 | 26 | 8,640 | 2,001,542,400 |
| Ordered distinct-stream pair | 169,884 | 3 | 26 | 8,640 | 114,488,225,280 |
| **Added** | | | | | **116,797,697,280** |
| **V5 total, including v4** | | | | | **143,736,975,360** |

The saved design holds axes, masks, hashes, dimensions, and counts, not billions
of duplicated rows. Ordinal decoding uses constant space. The suffix order is
source specification, then residual shape when present, then ascending mask,
then mixed-radix non-symmetry solver setting. A v5 case records its parent
source reference, exact selected settings, and whether the composition adapter
is unavailable.

## Execution and selection boundary

An addressable combination is a **recipe**, not an executable solver. Simultaneous
policies need a group-action or ordering composition with verified orbit/domain
semantics and a native adapter. The single-base `ordered_first_pair` case needs
its own domain check. The new suffix has an unavailable-adapter disposition
except for the one exact retained K600 route below. Every suffix case still has
`executed_complete_pipeline=false` and no runtime. Some masks may
later prove redundant or incompatible for a particular factor-base closure;
that decision requires a proof or a checked counterexample and a versioned
disposition, rather than deleting the ordinal.

There is narrower prior art: the frozen full-D18 worker in
[`primary_adapter.rs`](primary_adapter.rs) can install a Frobenius orbit-phase
domain and an ordered first-pair constraint in the same encoding. That path is
bound to its retained full-D18 input and source gates; it does not establish a
general adapter for the v2–v4 base recipes or make a v5 suffix case executed.

A later [K2048 five-summand gate](verification/k2048-sorted-unpinned-20261114/RESULT.md)
also composed a signed-Frobenius root domain with all-summand x ordering on
one retained primary base. Its constructed positive passed original-XCNF,
archive, group, and relation-row replay; the unpinned search ended
`INDETERMINATE` at 1,000,001 conflicts, and the natural arm was withheld by
its frozen protocol. The external CryptoMiniSat build, exact formula, and
search envelope were recorded for that experiment, not bound to a V5 ordinal
or generalized over the V5 base and solver axes. It is evidence for one
composed circuit and a censored solver gate, not a completed V5 runtime cell.
The historical crosswalk added no graphable runtime result. Its original
dispositions are preserved as the prior catalog state.

### Exact retained K600 composed route

Ordinal `26960859236` is the single-base `a=0`, K600 `public_x_hash`, seed
`2026100801`, signed-Frobenius object with V5 mask `18` (signed Frobenius and
ordered first pair), six summands, zero split bits, SAT backend, binary
enumeration, zero large primes, one thread, and modular BigUint linear algebra.
The native exporter now has `primary-v5-retained-binding-check` and
`primary-v5-retained-chain-cold` commands for that **one** cell. The binding
requires the exact existing v3 store object SHA-256
`3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf`
at `s3://setec-astronomy-artifacts/store`, checks pinned parent and saved V5
design bytes, then routes to the existing ordered-orbit chained SAT path. Its
[frozen admission protocol](verification/v5-composed-adapter-20261116/PROTOCOL.md)
separates the binding check from a later bounded relation run. This is an
implemented exact adapter route, with no new relation, rank, target, or cold
runtime measurement. The cold route requires `RAYON_NUM_THREADS=1` before
launch; the [versioned thread-gate amendment](verification/v5-composed-adapter-20261116/THREAD_GATE_AMENDMENT.md)
records why. Other masks, bases, and solver settings remain without a
verified composed adapter; the global catalog status stays unchanged.

The finite grid is complete only over its **declared** base recipes, pair shapes,
five symmetry switches, and solver axes. It does not enumerate arbitrary new
field bases, higher-order base mixtures, alternative implementations, or
unbounded numeric settings. Those are open requirements under the user's
broader sweep request. A factorial count of recipes is not an empirical
comparison. Select a minimum-total-runtime factor base only after matched
primary-curve, same-target complete cold runs with construction, S3 I/O,
relation collection, rank, linear algebra, individual logarithm, verification,
memory, failures, and independent replay charged under a frozen protocol.

New factor-base publication is specified for the repository's write-once
`s3://setec-astronomy-artifacts/store` remote. Historical D18 input keys remain
bound to the original bucket. This design creates no factor-base object or S3
write. The exact writer identity and any historical mirror remain separate
storage gates.

## Reproduction

```sh
cargo run --release --example n83_factor_base_sweep_v5 -- --check
cargo run --release --example n83_factor_base_sweep_v5 -- --case 26939278080
cargo run --release --example n83_factor_base_sweep_v5 -- --case 26960859236
cargo test --release --example n83_factor_base_sweep_v5
```

The example's tests check the missing-mask partition, exact integer counts,
the v2/v3/v4/v5 region boundaries, and rejection of the first ordinal beyond
the design. The generator rejects changed parent bytes and refuses to overwrite
a nonmatching saved design. Follow-on work should audit the exact retained
adapter above, then run a paired controlled comparison against its singleton
controls. Failed and censored arms
stay in the run record. The primary-curve relation and total-runtime
obstruction remains tracked in
[`problems/n83-mixed-orbit-factor-base-selection.md`](../../problems/n83-mixed-orbit-factor-base-selection.md).
