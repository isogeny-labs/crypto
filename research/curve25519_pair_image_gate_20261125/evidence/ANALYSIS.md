# Pair-image degree analysis

The exact native certificate passes every frozen gate at `d=7,9,19`.

| d | pair factors | linear | quadratic | degree | gcd(H,H') degree | trace blocks |
|--:|--:|--:|--:|--:|--:|--:|
| 7 | 28 | 7 | 21 | 49 | 0 | 4 |
| 9 | 45 | 9 | 36 | 81 | 0 | 5 |
| 19 | 190 | 19 | 171 | 361 | 0 | 10 |

There are no constant or zero factors. Every factor divides the product; the
independently regrouped trace-block product equals the direct product. Each
`c=2` diagonal block has degree `d`, and every nontrivial trace block has
degree `2d`. All three products are squarefree, so their full `d^2` degree is
also `d^2` distinct algebraic roots over the algebraic closure—not merely a
degree count with multiplicity.

Monic coefficient SHA-256 values, encoded as constant-first `u64` little
endian words, are:

- `d=7`: `56cb8990b21cf6de43c465135be4f74dc421aea124a84b1f9d881caad52ad0f1`
- `d=9`: `13d0091fcea3c7f611a9976249bc20f288f048baad4a97ed0bbfcdcc5a503bb4`
- `d=19`: `4df936b8c833a1bd39d0b21e9cef2305ce0f2cc91a5c590f6311bc500ca01e94`

## Decision

Close explicit pair-image construction, factor storage, and lookup as an
exponent-moving lead. The root-of-unity trace quotient removes the pair swap
label but does not compress the pair image: it partitions `d^2` distinct
support into `Theta(d)` blocks of degree `Theta(d)`. Materializing the product,
its roots, or every factor remains generic pair-sum/MITM-equivalent prior art.

The only surviving version is an implicit target-conditioned common-root
algorithm with a proved `o(d^2)` arithmetic bound and recoverable witnesses.
Straight block-by-block resultants or modular products do not qualify because
they still process `Theta(d)` degree-`Theta(d)` blocks. This finite result is
not an asymptotic lower bound, and no ECDLP speed or subexponential claim is
made.
