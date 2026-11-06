# Summary (coded rules, PROTOCOL.md)

- **Y1** (Y(conj)/Y(std) within [0.6, 1.6] × (k_conj/k_std)^4; Y(rand)/Y(std) in [0.5, 2]): **FAIL**
- **Y2** (ρ(conj)/ρ(std) in [4.8, 12.8]): **FAIL**
- **Y3** (every kept curve solved by conj, s verified): **PASS**
- **Y4** (median T(std)/T(conj) in [4, 16]): **FAIL**

## Ratios

| p | Y(conj)/Y(std) | (k_conj/k_std)^4 | ratio / prediction | ρ(conj)/ρ(std) | Y(rand)/Y(std) | median T(std)/T(conj) | median T(rand)/T(conj) | mean D(conj)/D(std) |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 13 | 36.59 | 13.26 | 2.76 | 19.45 | 0.47 | 11.46 | 9.00 | 1.88 |
| 101 | 35.75 | 18.52 | 1.93 | 17.23 | 0.93 | 17.20 | 16.54 | 2.07 |

## Per arm

| p | arm | curves | mean k | mean D | relations / trial Y | ρ = Y/D | solved | median trials to rank | mean relations used | brute-force adds / trial |
|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 13 | conj | 16 | 13.0 | 12.8 | 1.8114 | 0.14138 | 16/16 | 12 | 20 | 363 |
| 13 | std | 16 | 6.8 | 6.8 | 0.0495 | 0.00727 | 14/16 | 112 | 129 | 55 |
| 13 | rand | 16 | 6.6 | 6.5 | 0.0232 | 0.00358 | 13/16 | 71 | 91 | 40 |
| 101 | conj | 16 | 101.0 | 101.0 | 1.3387 | 0.01325 | 16/16 | 92 | 136 | 20578 |
| 101 | std | 16 | 48.7 | 48.7 | 0.0374 | 0.00077 | 16/16 | 1518 | 62 | 4538 |
| 101 | rand | 16 | 48.3 | 48.3 | 0.0348 | 0.00072 | 16/16 | 1448 | 57 | 4513 |

## Curves (Y5)

- p = 13: 52 drawn, 16 kept; skipped for no all-lift conjugate-pair line: 0; skipped for order/v: 1; skipped for cofactor > 16: 35
- p = 101: 132 drawn, 16 kept; skipped for no all-lift conjugate-pair line: 0; skipped for order/v: 1; skipped for cofactor > 16: 115

