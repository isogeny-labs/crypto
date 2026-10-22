#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)

= NIST base points and the B-163 seed: generator searches

2026-10-17 (records), 2026-10-22 (this report). Directory
`research/nist_basepoint_provenance_20261017` in `aburan28/crypto`.

*Status.* Measured negative result. No tested generator, library or
derivation reproduces the base points of P-224, P-256, P-384, P-521, the five
B-curves or the five K-curves, or the B-163 seed. This concerns the
provenance of published constants. It has no bearing on ECDLP hardness: any
point of order $n$ is an equally good generator.

*Question.* The six May-1997 base points fit FreeLIP `zrandom`. The other
four prime-curve base points, every binary-curve base point and the B-163
seed remained unexplained (`nist_seed_provenance_20261016`). The standards
never say how $G$ was chosen. FIPS 186-4 D.1.1.5 says "any point of order $n$
can serve". X9.62-1998 A.3.1 draws $R$ at random and sets $G = h R$.

*Binary-curve correction.* For $h = 2$ or $4$, the random draw is $x(R)$, not
$G_x$. Every binary target therefore includes all $R$ with $h R = G$, in both
the polynomial and the normal basis. The basis conversion was checked first:
the normal-basis $b$ equals the SHA-1 seed output for all five B-curves, and
each halving doubles back to $G$. There are 137 binary targets in all.

#figure(image("coverage.svg", width: 100%),
  caption: [Search coverage. Blue: searched, no target hit, positive control
  found. Light hatched: some layouts blind. Orange cross: the detector cannot
  see the target. Drawn from `out/coverage.json`, transcribed from the
  `out/*.out` records.])

== Searches (all controls found; 0 target hits)

#table(columns: (auto, 1fr, auto), inset: 4pt, stroke: 0.4pt,
  [*id*], [*search and scope*], [*targets*],
  [T1], [FreeLIP `zrandomb`, seeds $1 dots 2^24$, NBITS 26/30/62, offsets 0–255], [P-224…P-521 $G_x$],
  [T2], [FreeLIP, seed = Unix time 1990–1998, offsets 0–63], [same],
  [T3], [FreeLIP lattice, any seed (blind on P-224/P-256 and on P-384 at NBITS 62)], [$G_x$, $G_y$],
  [T4], [drand48 continuing the published seed run], [P-224…P-521],
  [T5], [drand48 chunks, un-reduced $x = X mod p$], [P-224, P-256],
  [T6], [GMP 2.0.2 `mpz_random` on glibc/4.3BSD `random()`, seeds $0 dots 2^24$ and 1997–98 times], [P-curves],
  [T7], [MIRACL `bigrand`, two parameter sets, default state and `irand` seeds], [P-curves],
  [T8], [PARI `random()`/genrand (5 packings); Maple V `rand` (control missed)], [P-curves],
  [T9], [SHA-1 expansion of the curve seed, counters $0 dots 2^16$], [P-curves],
  [T10], [$G = k P_0$, $x_0 <= 12$, $k <= 2^20$], [P-curves],
  [T11], [FreeLIP and drand48-family lattice scan], [137 binary],
  [T12], [PARI / Maple patterns], [137 binary],
  [T13], [glibc/4.3BSD `random()`, SysV/BSD/MSVC `rand()`, seeds 1997–98 $t$, $t\/2$], [B-163 seed],
  [T14], [drand48 family, every short window: covers any one-byte edit], [B-163 seed],
)

*Side test.* A preregistered CM-discriminant screen of the skip filter found
no split. Of the published candidates, 8 of 10 have $f = 1$ (the other two
have $f = 5$ and $f = 3$); of the skipped, 10 of 11 do. It is superseded by H7
in `nist_seed_skip_filter_20261017`.

*Caveats.* Wall times were not recorded. The invocations in `run.sh` are
reconstructed from each record's "done:" line; T9 and T12 were re-run and
match byte for byte. Magma, SIMATH, KANT, ZEN/BigNum and libI were not
tested. There has been no independent review.

*What this leaves.* None of these base points or the B-163 seed is
explained. Either the generator is one not covered here, or the choice is not
a function of a recoverable seed. Neither can be tested from public data.
