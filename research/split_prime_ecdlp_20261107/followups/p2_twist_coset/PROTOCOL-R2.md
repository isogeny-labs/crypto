# P2 round 2: F4 cost on unplanted targets (protocol, frozen 2026-11-08)

Follows round 1 (`PROTOCOL.md`, `RESULTS.md`, red team `../red-team-p1-p2.md`, which
noted that round 1 measured F4 cost on planted targets only, while real decomposition
attempts almost never have a solution). Same primes, curves (cseed 0), cosets, seeds
and arms (idx:0 random, idx:1 A/A, max selected) as round 1's M3; same msolve limits
(1.5 GiB, 1,200 s). Stage diagnostic.

## Cells

Per prime (4 primes), curve 0: m = 2 at t = 4 and 6, m = 3 at t = 4; arms idx:0, idx:1,
max; **4 unplanted targets** per arm (R = [k]G, k from `--tseed 101..104`), emitted with
`--target unplanted` (Addendum A). 144 msolve cells, as in round 1.

## Measurements

F4 work proxy (Σ rows × columns), D, widest matrix, quotient dimension, and whether the
target decomposes over the coset (`decomposes` in the emit metadata). Descriptive: the
ratio of unplanted to planted F4 work per (prime, m, t, arm), against round 1's M3.

## Decision rules (frozen)

1. **Coset-independence on unplanted targets holds** if, at every (prime, m, t), the mean
   F4 work proxy, D and widest of the selected arm differ from the random arm's by at most
   10 % (round 1's kill line); otherwise **fails**, and round 1's rule 4 gain is withdrawn
   for the relation search as a whole.
2. A/A (idx:1 vs idx:0) is reported as the noise floor.
3. Targets that happen to decompose are counted and kept; the decision uses all four
   targets per arm.

## Addendum A: instrument (pinned before any claim cell)

Pending the builder's report (`--target unplanted`, `parse` on inconsistent systems).
