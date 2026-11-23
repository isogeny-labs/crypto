# Signed four-list prime-field relation search

`src/cryptanalysis/prime_signed_mitm.rs` adapts the signed pair-sum method to
the existing `prime_fast::FastCurve` backend. It searches for a signed zero-sum
relation containing one point from each of four supplied subsets. Each record
retains `R = uG + vQ` metadata for scalar recovery. This is the **generic
decomposition** track under AGENTS.md, not the algebraic solver track.

## Interface and method

Construct records with `curve.point(x, y)` (Montgomery coordinates), then call
`search(&curve, [&f1, &f2, &f3, &f4], entry_limit, callback)`. Every callback
receives four indices and four signs. Returning `ControlFlow::Break(())` stops
the scan; returning `Continue(())` requests every relation. The first sign is
fixed to +1, removing simultaneous sign-reversal duplicates.

The left table stores every representation of `P1 + P2` and `P1 - P2`, keyed
by the full point, including the infinity value. The right scan computes
`P3 + P4` and `P3 - P4` and looks up both signs. Equal keys retain all their
originating pairs, so a metadata-dependent recovery can continue past an
uninformative collision. Formal inverse cancellations are rejected, including
when the input lists overlap. Every emitted relation is replayed by the group
law; `Relation::check` can replay it again without the table.

`Relation::recover` checks the selected metadata, subgroup membership and the
relation, computes `U = sum(sign_i*u_i)` and `V = sum(sign_i*v_i)` modulo the
subgroup order, and tries `k = -U/V`. It accepts only prime subgroup orders in
the backend's native range, rejects `V=0`, and checks `kG=Q`. Field primality
and nonsingularity remain the caller's `FastCurve` preconditions. This module
does not generate a factor base or introduce a polynomial solver.

## Scope and costs

For B points per subset, the left table has exactly 2B² representations;
the full right scan computes 2B² pair sums and performs 4B² lookups. Every
matched nontrivial representation incurs group-law replay and callback work.
Thus expected hash-table work is O(B²+C), with C matching representations,
and storage is O(B²). `entry_limit` bounds representation count rather than
bytes. An early exit reports only work actually performed. The counters name
logical requests; they are not calibrated group-addition equivalents.

Generic-group square-root bounds for prime-order discrete logarithms assume
an encoding-independent group oracle and account for all queries, setup,
storage and target-dependent work. Pair tables do not supply a non-generic
operation. The four-list witness search and the complete logarithm problem
have different success and accounting requirements. No new lower bound,
novelty, benchmark, or cross-method ranking is proposed here.

## Development checks

```sh
cargo test --release --lib cryptanalysis::prime_signed_mitm
```

The tests compare all emitted indexed sign patterns with exhaustive enumeration
in the scalar group for small fixtures, retaining colliding representations;
exercise overlapping lists, formal cancellation, empty lists, entry limits,
invalid points and malformed witnesses; and check a public known-answer
recovery fixture from the standalone prototype. The original Python prototype
and historical timing files are not imported as native performance evidence.
Development test output is a producer observation, not a signed PCC verdict.

## Sources checked

- Amadori, Pintore and Sala, *On the discrete logarithm problem for prime-field
  elliptic curves*, [ePrint 2017/609](https://eprint.iacr.org/2017/609), §3 and
  the split `S3(X1,X2,Y), S3(X3,X4,Y)` system in §3.3. The common intermediate
  x-coordinate motivates the signed pair join for rational factor-base points.
- `RESEARCH_SEMAEV_ROOTFIND_VS_GROEBNER.md` and
  `RESEARCH_PDP_SPEEDUP_AND_ISOGENY.md`: prior group-law decomposition work.
- `examples/m4_mitm_scaling.rs`: existing fixed-base target decomposition;
  this interface instead searches four subsets with recovery metadata.
- `src/cryptanalysis/native_signed_mitm.rs`: existing binary-field fixed-base
  signed lookup. This adapter uses the prime backend and preserves every
  colliding pair representation.
- Sources checked (KB): **KB unavailable**. No research-kb MCP is exposed and
  `bash scripts/research-kb status` reports no executable. The paths above are
  the bounded fallback review; this is not an exhaustive prior-art survey.

No priced result or registry record is added. The canonical leaderboard,
scoreboard, browser and ecdb imports therefore have no new result to render.
