# Sparse paged binary pair-index screen

Frozen on 2026-11-16 before implementing or testing the standalone index.
Source ancestor: `7a3c3f215313e5c1f3f6c46adcc52941b14d49b8`.
This screen concerns data-structure semantics and requested backing bytes, not
complete-call timing or a performance claim. [pcc:unverified]

## Mechanism and prior work

The torsion-harvest builder in `src/cryptanalysis/ic_boundary.rs` inserts the
first witness under each nonzero binary key `x + 1`, and reports a collision
when a later pair has the same key. Its opt-in flat direct layout allocates
`2^n + 1` eight-byte words after first building the hash table. The proposed
follow-up uses a four-byte page directory indexed by `key >> 4` and fixed
16-word pages. A directory zero means absent; a present directory entry is
the one-based page index. Each witness keeps the existing packed `i+1, j`
encoding. The producer would insert directly into pages, eliminating the
intermediate hash table. Page backing is preallocated to
`min(floor(2^n/16)+1, a^2+1)` pages for `a` factor-base abscissae; exceeding
that bound must refuse. This is a candidate layout, not a new relation
algorithm. [pcc:unverified]

The repository's rootfinding program paper
(`papers/ecdlp-index-calculus-program/sections/rootfinding.tex`, KB source
`source:e9db8cc2be204429e8fb1daa12a3324c`) documents prior binary pair
tables and dense-index engineering. The N83 dense-root protocol
(`research/n83_dense_root_20261013/PROTOCOL.md`) concerns a different
prefix-root table. The current direct conversion and K13/K19/K23 collision
census postdate KB revision `9141e386254340a9bdba3036f2bb706a`
(last refresh 2026-11-14T21:44:48Z); their local pinned files, rather than
the older index, determine current source behavior. [pcc:unverified]

## Fixed checks

- Read `base_abscissae`, `unique_keys`, `insert_attempts` and the exact curve
  IDs from the three immutable JSON records under
  `research/rootfind_pair_collision_census_20261116/reports/run-20261116T0711Z/`.
  Derive directory words, page capacity, fixed requested backing bytes, flat
  direct-array bytes, and their ratio with integer arithmetic. The directory
  includes the page of key `2^n`; key zero remains invalid.
- Implement a dependency-free Rust prototype in this study directory. Test
  first-witness preservation and collision order against a standard hash map
  over a deterministic stream with duplicate keys; test every K13 key,
  boundary keys, absent lookups, `i = 0`, `j = u32::MAX`, insufficient page
  capacity, and field-degree and key refusals. Its tests do not certify the
  elliptic-curve builder or target solve.
- The storage count covers directory words and preallocated page slots only.
  It excludes `Vec` headers, allocator overhead, factor-base storage, curve
  arithmetic, rank updates and process RSS. Report all of those as unknown.
  A fixed backing request is a theoretical memory comparison, not a wall
  result. [pcc:unverified]

## Decision rule and next gate

An incorrect first witness, collision stream, range check or storage formula
invalidates this candidate. Passing the screen permits a separate guarded
integration once Conductor T-6708 releases the two source paths. That
integration must compare actual K13/K19/K23 pair keys and same-target verified
answers with the hash and flat layouts. A later frozen timing panel must charge
cold setup, table insertion, scan, rank, verification, native writes and peak
memory on matched public targets, with dense A/A and independent replay. The
existing EKS L2 F4/F5/F6 panel and its admission gate remain unchanged.
[pcc:unverified]
