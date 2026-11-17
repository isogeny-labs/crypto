# Frozen Vieta speed panel: recorded paired costs

Canonical seed-table validation fails360 main/60 calibration cells; see REGISTRATION_ERRATA.md. These are reproducible descriptive observations, not fully protocol-qualified or L2-admitted results.

Primary plan504 calls; attempted 504, successful 504, failed 0, warmup calls 72. Calibration72 is separate. Exact per-target values, ranges and descriptive4096-bootstrap median intervals remain in `run/summary.json`; no populations or configurations are pooled. All observations are producer-owned [pcc:unverified].

Ratios are reference/half-GCD, so above1 favors half-GCD and below1 favors the reference. S=schoolbook; N=Euclid/Newton; L=lookup. Elapsed ratios are local belowL2 diagnostics, never admitted speed claims. These tables reproduce precomputed native medians, without refitting. Full construction is charged to every arm; query cost remains separate.

| Exact group | n | S/H query time | N/H query time | L/H query time | S/H cold time | N/H cold time | L/H cold time |
|---|---:|---:|---:|---:|---:|---:|---:|
| coset:p224:m2048:conditioned-sum:finite | 6 | 1.1797 | 0.5779 | 0.0003 | 1.1598 | 0.6310 | 0.1295 |
| coset:p224:m2048:seeded-random-abscissa:finite | 6 | 1.1455 | 0.5734 | 0.0003 | 1.1256 | 0.6284 | 0.1307 |
| coset:word62:m16384:conditioned-sum:finite | 6 | 1.5441 | 0.6165 | 0.0003 | 1.5366 | 0.6220 | 0.0157 |
| coset:word62:m16384:seeded-random-abscissa:finite | 6 | 1.5205 | 0.6086 | 0.0002 | 1.5123 | 0.6144 | 0.0153 |
| coset:word62:m8192:conditioned-sum:finite | 6 | 1.1430 | 0.4857 | 0.0003 | 1.1389 | 0.4971 | 0.0229 |
| coset:word62:m8192:seeded-random-abscissa:finite | 6 | 1.1384 | 0.4848 | 0.0003 | 1.1356 | 0.4959 | 0.0225 |
| rational:p224:m128:conditioned-sum:finite | 6 | 0.8843 | 0.6307 | 0.0018 | 1.0101 | 0.9751 | 0.9202 |
| rational:p224:m128:seeded-random-abscissa:finite | 6 | 0.8618 | 0.6236 | 0.0018 | 0.9812 | 0.9696 | 0.9375 |
| rational:p224:m512:conditioned-sum:finite | 6 | 1.2615 | 0.5860 | 0.0006 | 1.0354 | 0.9411 | 0.8590 |
| rational:p224:m512:seeded-random-abscissa:finite | 6 | 1.2185 | 0.5845 | 0.0006 | 1.0349 | 0.9481 | 0.8479 |
| rational:word62:m2048:conditioned-sum:finite | 6 | 1.3235 | 0.4805 | 0.0007 | 1.1157 | 0.8503 | 0.7095 |
| rational:word62:m2048:seeded-random-abscissa:finite | 6 | 1.2302 | 0.4826 | 0.0007 | 1.0521 | 0.8298 | 0.6799 |

| Exact group | S/H query muls | N/H query muls | L/H query muls | S/H cold muls | N/H cold muls | L/H cold muls |
|---|---:|---:|---:|---:|---:|---:|
| coset:p224:m2048:conditioned-sum:finite | 1.3404 | 0.7205 | 0.0004 | 1.2705 | 0.7779 | 0.2057 |
| coset:p224:m2048:seeded-random-abscissa:finite | 1.3405 | 0.7204 | 0.0004 | 1.2706 | 0.7778 | 0.2057 |
| coset:word62:m16384:conditioned-sum:finite | 3.0995 | 1.4437 | 0.0001 | 3.0741 | 1.4384 | 0.0122 |
| coset:word62:m16384:seeded-random-abscissa:finite | 3.0995 | 1.4437 | 0.0001 | 3.0741 | 1.4384 | 0.0122 |
| coset:word62:m8192:conditioned-sum:finite | 2.3398 | 1.1204 | 0.0002 | 2.3156 | 1.1182 | 0.0182 |
| coset:word62:m8192:seeded-random-abscissa:finite | 2.3398 | 1.1204 | 0.0002 | 2.3156 | 1.1182 | 0.0182 |
| rational:p224:m128:conditioned-sum:finite | 0.9478 | 0.6808 | 0.0024 | 0.9996 | 0.9975 | 0.9922 |
| rational:p224:m128:seeded-random-abscissa:finite | 0.9468 | 0.6745 | 0.0024 | 0.9996 | 0.9975 | 0.9923 |
| rational:p224:m512:conditioned-sum:finite | 1.3551 | 0.6392 | 0.0009 | 1.0059 | 0.9940 | 0.9834 |
| rational:p224:m512:seeded-random-abscissa:finite | 1.3559 | 0.6383 | 0.0009 | 1.0059 | 0.9940 | 0.9834 |
| rational:word62:m2048:conditioned-sum:finite | 2.3340 | 0.7636 | 0.0003 | 1.5446 | 0.9035 | 0.5919 |
| rational:word62:m2048:seeded-random-abscissa:finite | 2.3341 | 0.7636 | 0.0003 | 1.5446 | 0.9035 | 0.5919 |

| Exact group | Exact count controls | Local timing separation | L2 admission |
|---|---|---|---|
| coset:p224:m2048:conditioned-sum:finite | true | true | false |
| coset:p224:m2048:seeded-random-abscissa:finite | true | true | false |
| coset:word62:m16384:conditioned-sum:finite | true | true | false |
| coset:word62:m16384:seeded-random-abscissa:finite | true | true | false |
| coset:word62:m8192:conditioned-sum:finite | true | true | false |
| coset:word62:m8192:seeded-random-abscissa:finite | true | true | false |
| rational:p224:m128:conditioned-sum:finite | true | true | false |
| rational:p224:m128:seeded-random-abscissa:finite | true | true | false |
| rational:p224:m512:conditioned-sum:finite | true | true | false |
| rational:p224:m512:seeded-random-abscissa:finite | true | true | false |
| rational:word62:m2048:conditioned-sum:finite | true | true | false |
| rational:word62:m2048:seeded-random-abscissa:finite | true | true | false |

The Lean gate checks21 theorems/0trusted exceptions, including the conditional normal-form implication and finite projector/full-splitting/nilpotent certificates. The arbitrary-depth tower NF instance and independent signed standing remain unfinished; the full degree bound remains unverified. Shared crypto-lean PR26 CI did not start because GitHub reports an account billing lock. See `ASSUMPTION_AUDIT.md`, `PROOF_MAP.md`, `REQUIREMENTS.md`, `COST_MODEL.md`, raw runs, protocol/publication receipts and the report sources.
