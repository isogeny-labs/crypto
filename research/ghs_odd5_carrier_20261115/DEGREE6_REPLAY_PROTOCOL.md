# Independent degree-six replay protocol

16 November 2026 UTC. Replay the ten masks in `DEGREE6_PROTOCOL.md` with
`degree6_direct_replay.sage`. This is a separate Sage implementation, not an
import of the producer. For each denominator shape, independently reconstruct
the interpolation numerator. In the dominant `L H²` shape, scan every
`u∈K`, form the three high-coefficient quadratics in `v`, solve the first
nonzero one, and filter against all three. If all three vanish, scan every
`v∈K`. This avoids the producer's resultant projection and exceptional-case
logic. For every candidate, use descending coefficient reconstruction of a
square root and verify the complete polynomial square, avoiding the
producer's `Polynomial.is_square` test. Record each target's shape counts,
numerator counts, and full-equation hits. Compare them against the producer
receipts. A disagreement suspends the degree-six exclusion.

This remains a second computation over the same exact field and curve, not a
formal Lean proof of geometric exhaustiveness. Do not amend the lower bound
until all ten masks agree and any disagreements are resolved.

The frozen Sage source SHA-256 is
`370842a50c062658f6743779168b453a4435bb851aa9785870dd9ebdbd984bb2`;
the deterministic preparsed Python SHA-256 is
`0f954d8782669a218d292228e2531a5494a971f844be449d8ba004b404984e10`.

## Serialization amendment

The first mask-3 pilot reached final output after 59.149 seconds but exited 1:
Python JSON cannot encode Sage `Integer` count values by default. Its stdout
was empty and stderr is retained as `degree6_replay_mask3_first.stderr`.
The sole source change adds `default=int` to `json.dumps`; all search loops
and square testing are unchanged. The amended source SHA-256 is
`e1fd209fe5c426f6b638f545d04bddbd3775a26b030e43b1f241642e8ab27569`;
the preparsed Python SHA-256 is
`b27c7350cf8f30e82e0f65bad3d078a98ebb23f3fe2577a5ba56dd34edfc02e0`.
