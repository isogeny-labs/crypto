# Three-instance 14-bit PKM degree-32 panel — result

## Outcome

The frozen panel stopped at its size-only selection gate. It did not run a timed arm.

All 400 CLI seeds from 20261200 through 20261599 resolved to the same committed generic 14-bit roster instance, `icv1-fp14-tm43-158b6914`, with subgroup order 16231. On every seed and every one of the 16 preflight rows:

- `prime-abscissa:below=32` contained 15 lifting abscissae;
- `prime-isogeny:t=5,seed=5,aux=ordinary` contained 16 lifting abscissae; and
- each deliberately truncated one-trial `subtract` preflight exited 1 with valid JSON and the expected constant factor-base size.

Consequently there were zero equal-size matches, fewer than the three required by `PROTOCOL.md`. The append-only store contains 800 preflight records, one header and the terminal selection verdict. Its SHA-256 is `b793cc4de3064cb78dff3a08687ed215289ad5195935c477d251278ca7223765`.

## Workload-identity finding

For `--bits 14 --family generic`, `ic bench` calls `roster_prime_instance(14)` before its seed-dependent curve search. The CLI seed therefore changes the planted scalar stream but does not change the 14-bit curve. The intended three independent generic instances cannot be obtained by varying `--seed` on this CLI route. The uniform 15-versus-16 size mismatch is a property of this one roster curve under the fixed two bases; it is not evidence about degree-32 performance at 14 bits.

## Required amendment before timing

A genuine three-instance 14-bit panel needs three explicit, independently generated prime-order curves. The existing CLI can ingest explicit EFD instances, but the curves, generators, orders, representation, exact size-matching rule and custody record must be frozen before preflight. Alternatively, the CLI could expose an opt-in route to `find_prime_order_curve(14, seed)` that bypasses the roster. Either route changes workload construction and requires a new protocol and store. The completed selection store must remain immutable and must not be pooled with that future panel.
