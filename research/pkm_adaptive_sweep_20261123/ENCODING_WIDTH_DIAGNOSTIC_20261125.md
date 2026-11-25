# Encoding-width diagnostic: what the 32× actually measures — 2026-11-25

Controlled experiment on one instance (`icv1-fp13-tm13-9e2d11f6`, seed
20261185), one base (`prime-abscissa:below=32`, 18 abscissae), one binary
(`8e654529…`, commit `cc7d6141f2`), 16 repeats. The **only** change is the
oracle's encoding width: `bits=5,lo=0` versus `bits=6,lo=0`.

The registry states what that parameter is: `prime-abscissa:below=2^T` is
"pkm-semaev's bits=T interval (the naive algebraic arm)" — the interval
membership is expressed by T bit variables per base element. So this compares
the same relations under a 5-bit and a 6-bit encoding.

## Result

| Quantity | bits=5 | bits=6 | ratio |
|:--|---:|---:|---:|
| F4 calls | 1645 | 1645 | — |
| Relations found | 219 | 219 | — |
| Target trials | 1666 | 1666 | — |
| Multiply-adds per call | 127.38M | 1956.84M | **15.4×** |
| Elimination steps per call | 11.73 | 21.10 | 1.80× |
| Max matrix rows | 1493 | 6037 | 4.04× |
| Max matrix cols | 928 | 3629 | 3.91× |
| Non-zeros per row | 189.9 | 465.5 | 2.45× |
| Charged GAE (16 repeats) | 0.991B | 8.257B | 8.3× |

Relations and trials are **identical**, so nothing about the search changed:
the entire 8.3× GAE difference is the shape of the polynomial system. One extra
bit variable per base element — two variables in total, for a two-summand
decomposition — costs 15.4× in elimination work.

For reference, the degree-32 isogeny arm on the same instance is 3.97M
multiply-adds per call with 12.4 steps and an 821×583 matrix. So:

- isogeny vs interval at `bits=5` (the size-matched, fair comparison): **32×**
- isogeny vs interval at `bits=6` (deliberately over-encoded control): **494×**

## What this means

The 32× is the cost of the interval arm's bit-window encoding at T = 5. It is
not evidence that the isogeny's elimination algebra is intrinsically 32×
cheaper than an interval's at equal encoding; it is evidence that an isogeny
fiber needs **no** encoding, while an interval does, and that the encoding is
the dominant term. That is precisely the motivation the PKM construction gives
for itself (§3.4 builds a fibre map instead of a bit-decomposed interval), so
the comparison is fair as posed — but the mechanism sentence has to name the
encoding, not a generic "banded vs dense" difference.

It also produces a falsifiable scaling prediction: because the interval arm's
cost jumps ~15× per extra encoding bit while the isogeny arm's grows with tower
length only, the measured advantage should improve **in steps** as the
size-matched base crosses a power of two:

- base 15–18 points (T = 5): 32× per system, observed at 13, 14 and 15 bits;
- base 19–32 points (T = 6): predicted ~10²–10³× per system.

The 15-bit panel's bases were 19, 17 and 14, all still inside the T = 5
window, which is why they reproduced 32× rather than something larger. A pair
with ~26–32 abscissae would test the prediction directly. It is not free: the
`bits=6` interval arm at 18 points already took ~10 minutes per 16-repeat call
on this host, and a 30-point base would be worse, so the test should be run
with fewer repeats and a budget cap rather than as a full panel.

## Caveat on the contrived control

The `bits=6` run over-encodes an 18-point base that only needs 5 bits. It is
not a legitimate performance comparison and is not reported as one; it is a
sensitivity probe that isolates the encoding term. The only peer-reviewed
comparison in this lane remains the size-matched one.

Stores: `encoding_width_bits5_20261185.json`
(SHA-256 `0b9e4637652d6f4f18fccd632bd1c073c8e0b4f03ae4827d41683dddae05e414`) and
`encoding_width_bits6_20261185.json`
(SHA-256 `987de48fba0fbe76aebe16b2737666b9d950a2e0fc9110090ae80e7e8e37b6fe`).
