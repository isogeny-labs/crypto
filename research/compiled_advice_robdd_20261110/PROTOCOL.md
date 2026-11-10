# F5: is the decomposable set compressible? ROBDD size of X₂ against random sets (protocol, frozen 2026-11-10)

Moonshot E1 (`research/prime_moonshot_20261110/ideas-generator.md`). The preprocessing bound
`S·T² ≳ n` charges advice by its *size*; it does not forbid advice that is **more compressible
than a random set of the same density** with a fast membership query. If the set of decomposable
abscissae had a small ROBDD, index calculus could carry a large factor base at small `S`.
Class: boundary diagnostic.

## 1. Sets and rows
`p` = largest prime below `2^n`, `n ∈ {16, 18, 20, 22, 24}`, one prime-order curve per `n`
(seed `F5/curve/<n>`). Small-x base `F = {P : 1 ≤ x(P) ≤ B}`, `B = ⌈p^{1/3}⌉`.
- `X₂ = {x(P₁ ± P₂) : P_i ∈ F}` (signed two-sums; `|X₂| ≈ B²` after collisions, counted exactly).
- `X_E = {x : χ(x³ + ax + b) = 1}` (abscissae of all points; density `≈ 1/2`): the Legendre set.
- `RAND(X)`: 10 uniformly random subsets of `[0, p)` with `|RAND| = |X|` (seed `F5/rand/<n>/<set>/<j>`).
- `INT`: the interval `[0, |X₂|)` (positive control: ROBDD size `O(n)`).

## 2. Instrument (native Rust, std only, `tool/`)
ROBDD of the indicator of each set as a function of the `n` bits of `x`, built bottom-up from the
sorted element list with a unique table; node count for two fixed variable orders (MSB-first,
LSB-first) and after one pass of Rudell sifting (budgeted). Each ROBDD is verified by
re-enumerating its satisfying set against the input (fatal on mismatch). Counts: nodes, unique-table
operations, sifting swaps.

## 3. Predictions (frozen)
P1. `nodes(X₂) / mean nodes(RAND)` ∈ `[0.9, 1.1]` at every `n` and order; same for `X_E`.
P2. `INT` gives `≤ 2n` nodes (detector sees structure).
Prior that P1 fails downward with a decreasing trend in `n`: 0.003.

## 4. Decision rules
1. **Instrument failure:** P2 fails or any verification fails.
2. **Lead alive:** ratio `≤ 0.9` at every `n` for some order, and decreasing in `n`. Then extend to
   `X₃` and `n = 28` before any claim.
3. **Closed:** otherwise: the decomposable and Legendre sets are as incompressible as random sets
   under ROBDDs at these sizes. Says nothing about other compressed representations.

## 5. Route and outputs
Local host; exact counts. `results/run.json`, README, SVG (ratio vs `n`), REPORT.pdf, session log.
