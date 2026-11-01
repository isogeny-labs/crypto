# Static audit: version 2 linear-algebra gate predicate

Status: independent static justification for a proposed pre-panel version 3
amendment. The proposal is explicitly gate-result-informed: it was written
after seeing the version 2 prerequisite failure. This document is not a run
result and authorizes no rerun by itself.

## Immutable observation

Frozen version 2 run `41353962971`, head
`bb8a0b2e1c2e6d613a5057441f92e193081e8b51`, stopped before every requested-
prime panel. The counted p=101 prerequisite completed its computation and then
panicked at `src/cryptanalysis/jv_isogeny_walk.rs:2969:13` with
`LinearAlgebra has no field work`. The exact retrieved run log is
`v2-run-41353962971.log` (257716 bytes, 2027 lines, SHA-256
`2b1465614126bc18b91fcf42a2143ad715e4ab676097b470cfdb3782850e860d`).
GitHub reported zero artifacts; the p=1009, p=1511, and p=1823 panel job was
skipped. Thus 99 target attempts were planned, zero attempted, zero completed,
and 99 never started. No requested-prime datum exists.

## Independent source check

The failing predicate asks the wrong counter family for the linear-algebra
phase:

1. `src/cryptanalysis/jv_bitops.rs:45-46` defines `modr_*` as operations
   modulo the subgroup order `ℓ`, explicitly including Wiedemann and
   Berlekamp–Massey. Lines 98-99 define `Phase::LinearAlgebra` as the core
   filter and Wiedemann modulo `ℓ`.
2. `src/prime_hyperelliptic/ops.rs:199-202` defines `field_ops()` as only the
   first six `F_p` counter kinds. It deliberately excludes `modr_add`,
   `modr_mul`, and `modr_inv`.
3. `src/cryptanalysis/jv_sieve.rs:1849` enters
   `Phase::LinearAlgebra`; line 1889 calls `wiedemann_u64(..., l, ...)`, using
   subgroup modulus `l`.
4. `src/cryptanalysis/gaudry_cubic.rs:3392-3437` implements that solve with
   `am`, `mm`, and `inv_mod` at the supplied modulus. The counter classifier
   in `src/cryptanalysis/jv_bitops.rs:390-428` maps those operations to the
   `Modr*` kinds when the modulus equals `ℓ`.
5. The version 2 test's generic phase loop required `field_ops()>0` for
   `Phase::LinearAlgebra`, yet its later line separately required
   `ModrMul>0`. Those requirements contradict the documented counter split.

Source identities used for this check:

- frozen v2 `jv_isogeny_walk.rs` SHA-256:
  `907535915df37875edfec304f71aab4d5136e530a59f108704690950ffc459e9`;
- proposed `jv_isogeny_walk.rs` SHA-256:
  `272835772484f2e81ec4e9fd5728c9be9c109f5173034723478e101573b4fe3c`;
- `jv_bitops.rs` SHA-256:
  `5b5b793c5cac639db9684e68bdb3037cc3d364cca0c3dce5f9e55b48c342d29c`;
- `jv_sieve.rs` SHA-256:
  `a39ad0d60acbe5e30339c7404317b7990edcdd746527b98ff3d382763828f72e`;
- `gaudry_cubic.rs` SHA-256:
  `869be182322aa277b8dbff7854f4cf0482e1e91203f0e70ffa48a0934b6dbe05`;
- `prime_hyperelliptic/ops.rs` SHA-256:
  `668c2ea7804578fe09b8b3dc2dd905b1564be5ebbf0f5e6571d785fa7ad842f4`.

## Proposed correction and boundary

Keep `field_ops()>0` for the actual field phases and add the omitted
`Transport` phase to that loop. Remove only `Phase::LinearAlgebra` from the
field-only predicate and require `ModrAdd>0`, `ModrMul>0`, `ModrInv>0`, and
`WordOp>0` for it. This is stricter than version 2's separate `ModrMul` check
and matches the counter specification for the core filter, Wiedemann solve,
and Berlekamp–Massey work. Preserve route correctness, two-run tally
determinism, relation `WordOp`/`HashProbe`, descent `FieldInv`, and total IC
work assertions. Also require zero foreign-thread work, zero work at an
unclassified attack modulus, and zero unattributed attack work, which are
frozen invalidation conditions already exposed by the counter API.

The proposed patch changes only an ignored prerequisite test predicate. It
does not change the producer, target primes, instances, seeds, algorithms,
counters, prices, analysis, thresholds, stopping rules, or validation logic.
Static consistency does not establish that the corrected gate passes: a fresh
version 3 run must execute all three prerequisites from scratch at the exact
version 3 frozen commit, and no panel may start unless all three pass. Neither
version 2 green prerequisite is reused. Version 1 and version 2 failures remain
visible and must be reported alongside any later run.
