# Round C (native): CM-derived orbit census cross-check and certified grinds

Executed by `examples/safecurves_orbit_census` (Rust, num-bigint; the
SageMath round-A/B scripts are retained as frozen evidence).  Repository
revision for this round: the commit landing this directory.  Toolchain:
rustc 1.93.1.

## Method

- The six sextic-twist orders of a `j = 0` curve over `p = 1 mod 6` are
  derived from the CM representation `4p = L^2 + 27 M^2` (Cornacchia with the
  sqrt(-27) seed lifted to `mod 4p`, odd seeds only) as `p + 1 -/+ t` for
  `t` in `{|L|, |L + 9M|/2, |9M - L|/2}`.  No SEA point counting anywhere.
- The `j = 1728` quartic orbit over `p = 1 mod 4` is `p + 1 -/+ {2a, 2b}`
  for `p = a^2 + b^2`.
- Factoring: `trial_factor` to `10^6` plus the repository's Brent-variant
  `pollard_rho_factor` with a per-piece iteration budget; a class is
  certified only when its order is fully factored within budget.
  Acceptance mirrors rounds A/B: published class and its quadratic-twist
  pair both `>= 0.79 * bitlen(p)` largest-prime bits, weak sibling
  `<= 0.40 * bitlen(p)`.

## Census cross-check (`census_crosscheck.json`)

All three SECG `j = 0` curves: the 18 CM-derived orders equal the 18 SEA
orders of round A's `census.json` (bit-for-bit), and each published order
appears in its orbit.  The `j = 1728` quartic formula is pinned against
the SageMath-verified fixture `p = 281474976710677` (traces
`{13320148, 30797298}`); both are unit tests in the example.

## Certified grinds (`grind_96.json`, `grind_128.json`)

| size | scanned primes = 1 mod 6 | published lf | twist lf | weakest sibling lf (rho) |
|---:|---:|---:|---:|---:|
| 96 | 3 | 79 bits | 95 bits | 31 bits (2^15.5) |
| 128 | 40 | 103 bits | 107 bits | 51 bits (2^25.5) |

The 96-bit hit is the same prime round B found with SageMath
(`p = 79228162514264337593543950963`), an independent reproduction of the
round-B result by the native path.  At 128 bits the certified pair was
found after 40 scanned primes (`2^5.3`), inside the registered
prediction bound of `2^12`.

## 256-bit bounded probe (`probe256.json`)

Strong-side certification at 256 bits requires peeling every prime below
the `2^202`-bit largest factor; with a rho budget of `2^21` iterations per
composite piece only factors up to about `2^42` are findable, so many
candidates are uncertifiable without ECM.  Executed result: **12 scanned
primes, 7 with uncertifiable orbits, no certified triple** among the 5
certifiable ones (`probe256.json`, stderr log beside it).  So at 256 bits
the binding constraint in this budget is *certification*, not existence:
the registered prediction (a certified pair within `2^12` scanned primes)
remains open until a native-ECM round (round D) lands.  The 128-bit run,
where certification is complete, is the current best native evidence for
the prediction (`2^5.3` primes).

## Claim hygiene

Synthetic instances only; no deployed curve is implicated beyond round A's
census rows; no key recovery.  Certification is conservative: classes
that cannot be fully factored within budget are skipped, never assumed.
