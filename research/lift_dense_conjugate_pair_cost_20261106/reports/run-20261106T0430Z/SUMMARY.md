# Summary (coded rules, PROTOCOL.md)

- **Y1** (Y(conj)/Y(std) within [0.6, 1.6] × (k_conj/k_std)^4; Y(rand)/Y(std) in [0.5, 2]): **FAIL**
- **Y2** (ρ(conj)/ρ(std) in [4.8, 12.8]): **FAIL**
- **Y3** (every kept curve solved by conj, s verified): **PASS**
- **Y4** (median T(std)/T(conj) in [4, 16]): **PASS**

## Ratios

| p | Y(conj)/Y(std) | (k_conj/k_std)^4 | ratio / prediction | ρ(conj)/ρ(std) | Y(rand)/Y(std) | median T(std)/T(conj) | median T(rand)/T(conj) | mean D(conj)/D(std) |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 13 | 2.77 | 9.99 | 0.28 | 1.75 | 0.98 | 12.00 | 22.98 | 1.59 |
| 101 | 2.50 | 15.84 | 0.16 | 1.27 | 1.09 | 11.99 | 13.58 | 1.97 |

## Per arm

| p | arm | curves | mean k | mean D | relations / trial Y | ρ = Y/D | solved | median trials to rank | mean relations used | brute-force adds / trial |
|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 13 | conj | 16 | 13.0 | 10.8 | 2.7251 | 0.25204 | 16/16 | 11 | 29 | 359 |
| 13 | std | 16 | 7.3 | 6.8 | 0.9832 | 0.14432 | 7/16 | 126 | 11115 | 100 |
| 13 | rand | 16 | 6.5 | 6.0 | 0.9679 | 0.16132 | 6/16 | 202 | 12224 | 86 |
| 101 | conj | 16 | 101.0 | 97.2 | 4.0679 | 0.04183 | 16/16 | 112 | 476 | 20551 |
| 101 | std | 16 | 50.6 | 49.4 | 1.6247 | 0.03290 | 13/16 | 1642 | 8610 | 5180 |
| 101 | rand | 16 | 52.4 | 51.3 | 1.7758 | 0.03461 | 13/16 | 1673 | 9088 | 5376 |

## Curves (Y5)

- p = 13: 18 drawn, 16 kept; skipped for no all-lift conjugate-pair line: 1; skipped for order/v: 1; skipped for cofactor > 16: 0
- p = 101: 16 drawn, 16 kept; skipped for no all-lift conjugate-pair line: 0; skipped for order/v: 0; skipped for cofactor > 16: 0

