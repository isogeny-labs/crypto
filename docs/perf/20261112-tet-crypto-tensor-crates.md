# Tet offensive: crypto-tensor-crates (wall-only)

Scope: `crates/hashed-ecdl-tensor`, `crates/ecdl-tensor-ic`, `crates/hashed-ecdl-cpu`.
Base commit: `aaf15e6bdc`. Branch `perf/crypto-tensor-crates` (not pushed).
Rule: wall-only. No value, counted unit (`Meter.used`, `curve_additions`,
`square_roots`, `Counter` statistics, `opcount` tallies), RNG draw or output
may change. Evidence: deterministic Callgrind instruction counts (Ir);
wall times are secondary and are "local, unisolated, [pcc:unverified]"
(shared 4-vCPU VM, load average about 1.0).

## Commits

| # | commit | change | kernel | Ir before -> after |
|---|---|---|---|---|
| 1 | `9e08c8c49e` | ecdl-tensor-ic: multiply-rotate hasher for `SparseContractor` tables (SipHash gone), dense array for the public `fixed` map lookups | `ecdldemo -bits 10 -seed 1 -q` | 3,471,703,974 -> 2,038,079,285 |
| 2 | `25625d6e1a` | ecdl-tensor-ic: u64 fast path in `mul_mod` (no `__umodti3` for word-size products), no key clone on row hits | same | 2,038,079,285 -> 1,881,768,399 |
| 3 | `75285495da` | ecdl-tensor-ic: bit-packed (u64 words) row keys in `Table` | same | 1,881,768,399 -> 1,752,669,795 |
| 4 | `b41ebd1c27` | ecdl-tensor-ic: branch-free `encode_point`/`decode_point` | same | 1,752,669,795 -> 1,679,504,127 |
| 5 | `3e78c74f8b` | hashed-ecdl-tensor: Fourier column/row ops use a table of `(omega^stride)^k` and one final reduction instead of a `modpow` per term | seeded solve+replay, tiny curve (4 runs) | 63,154,038,992 -> 26,710,398,826 |
| 6 | `23b34dd160` | hashed-ecdl-tensor: cache the Fourier column twist powers | same | 26,710,398,826 -> 19,963,503,739 |
| 7 | `e139a09155` | hashed-ecdl-tensor: validate a `PredicateProgram` once per arity, drop the redundant operation re-validation, share identical power tables process-wide (16 tables / 2^18 elements cap) | same | 19,963,503,739 -> 15,502,157,419 |
| 8 | (this branch tip before the note) | hashed-ecdl-tensor: reuse port/input scratch vectors in `Network::evaluate` / `Program::evaluate` | same | 15,502,157,419 -> 14,018,268,371 |

Cumulative: `ecdldemo` 3.47 G -> 1.68 G Ir (-51.6%); hashed-ecdl-tensor
identity run 63.15 G -> 14.02 G Ir (-77.8%).

Wall (local, unisolated, [pcc:unverified], load ~1.0, alternating runs):
`ecdldemo -bits 12 -seed 1 -q` 4.36/4.23/4.51 s -> 3.37/3.48/3.37 s (measured
after commit 3); hashed identity run 5.28 s -> 2.19-2.30 s; lib test suite of
hashed-ecdl-tensor 43.2 s -> 23.9 s (99 tests, same count).

## Identity checks

- ecdl-tensor-ic: `ecdldemo` with `-bits 10/11/12`, `-m 3`, `-mitm -threads 1`,
  `-window 2`, `-gadgets` (bits 6 and 7), seeds 1-3, all output lines except the
  wall time (74 lines): sha256 of the normalised text identical for the base
  binary (`2a32361f65b6bb70`) and the final binary (`1631e542ea98fb2d`) after
  every commit; base A/A reproduced itself.
- hashed-ecdl-tensor: an ad hoc example (not committed) ran `reduction::solve`
  with `Seeded(seed)` on the tiny curve (p=5, r=7) for four (seed, scalar,
  backend) cases, replayed with `verify_with_rng(Seeded(17))`, and printed the
  SHA-256 of the serialised certificate, `Meter.used`, `curve_additions`,
  `square_roots` and the full `Counter` statistics. Base and every candidate
  commit print byte-identical lines (base A/A identical). An earlier attempt
  using the OS-random `verify` showed `used` varying by about 20 between runs
  of the unmodified base; that is the replay's own entropy, not a change, and
  the example was switched to a seeded replay.
- hashed-ecdl-cpu: `examples/scale` case 0-4 (seed 202) JSON archives and all
  printed counters are byte-identical base vs the tried candidates (the
  candidates were then reverted, see below).

## Tests

- hashed-ecdl-tensor `cargo test --release`: lib 99, bins 5+11, integration
  suites (certified_hash_prime 6, factoring_cli 1, finite_extensions 8,
  panel_driver 3, strict_json 4, structured 4, structured_cli 5,
  width_extensions 3, ...) all pass.
- ecdl-tensor-ic `cargo test` (profile.test opt-level 3): all suites pass
  (lib 3, window 12, gadget 9, 7+7, regression/mitm/parallel/sparse 3 each).
- hashed-ecdl-cpu: library tests do not build in this sparse worktree (they
  `include_str!` files under `research/`, absent here); no source of that crate
  is changed on the branch.

## Reverted ideas (honest list)

- hashed-ecdl-cpu: Barrett reduction (exact for 2^32 <= p < 2^62) in place of
  the `u128 % u64` path of `Scratch::subtract`. Output identical, but Ir
  10.41 G -> 11.39 G and wall 1.68 -> 1.83 s on `scale x 4 101`: compiler-builtins
  already lowers the 128/64 remainder to one hardware `div`. Reverted.
- hashed-ecdl-cpu: touching all home slots of a batch before probing (to
  overlap cache misses): wall unchanged within noise. Reverted.
- hashed-ecdl-cpu: branch-free `sub`: Ir unchanged (10.40 G), wall inside noise.
  Reverted.
- hashed-ecdl-tensor: building word values with a stack byte buffer and
  `BigUint::from_bytes_le` instead of `set_bit`: wall +10% (2.31 -> 2.55 s).
  Reverted; only the scratch-vector reuse of commit 8 was kept.
- hashed-ecdl-tensor: first version of the twist cache computed the twist
  powers per evaluation by running products with `%`; slower than the
  original `modpow` per index (small exponents). Replaced by the cached table
  of commit 6.
- ecdl-tensor-ic: pre-sizing tables for the function/predicate steps was not
  pursued (rows are mostly unique; rehash is 4% of the profile).

## Deferred method changes (would change a counter, RNG draw or method; not done)

- `find_witness` (ecdl-tensor-ic) re-contracts the whole network once per edge
  bit; a single DFS/backtracking witness extraction would change the number of
  `count` calls and the `contraction_rows_uncharged` tally.
- hashed-ecdl-tensor `Counter::witness_by_counts` / `functional_count` do the
  same self-reduction over full counts; a witness-only DFS changes
  `Statistics` (`contractions`, `witness_searches`) and `Meter.used`.
- Replacing the per-row `FnPredicate`/`PredicateProgram` evaluation with a
  native fixed-width kernel would skip `Meter.charge` calls made inside
  `ValueOperation::evaluate` (the Fourier terms are charged one by one).
- `src/cryptanalysis/hashed_ecdl.rs` (shared `paper` module, out of scope)
  holds the remaining generic kernels of hashed-ecdl-cpu.

## Host notes

Sparse worktree (`git sparse-checkout`, three crates plus `src/cryptanalysis`
and `docs/perf`) because the disk had about 2.7 GB free. Build/target dirs
under `/home/user/wt/tc-*` were deleted at the end. Binary sha256 (first 16 hex):
ecdldemo base `2a32361f65b6bb70`, final `1631e542ea98fb2d`; hashed identity
example base `c9c3f4fca07e64f8`, final `0e45221234ea9da4`.
