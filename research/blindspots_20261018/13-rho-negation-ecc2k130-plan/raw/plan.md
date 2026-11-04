# Item 13 part B: costed plan for completing ECC2K-130

## Measured CPU rate of the current `ecc2k130/` client

5 repetitions of `ecc2k130-cpu --curve 131 --bench --threads 1`, one thread each, built from the committed `ecc2k130/src/main.cu` by the card, on this host (shared 4-vCPU container, isolation level L0, load recorded in raw/): 14.702, 15.189, 14.532, 15.206, 15.605 M iterations/s.

Median **15.189 M iterations/s per core**, min 14.532, max 15.605. Committed single-core figures for comparison (ecc2k130/README.md Status): 12.6 M/s per AVX-512 core, 15.7 M/s per M4 Pro core. This is the shipping sigma walk's CPU rate; it is not a GPU rate and not a prediction of one.

## Work

Expected iterations to solve: 2^60.9 = 2.1514e18 (ecc2k130/README.md; measured for the sigma walk 2^60.91 to 2^60.92 on completed trails, ecc2k130/WALK-CONSTANT.md). Iterations per distinguished point: 2^28.41 at weight 32 (the live bucket, ecc2k130/benchmarks/dp-interval/README.md, 132 workers), 2^25.27 at weight 34 (Bailey et al., same file). Record: 32 bytes per point (ecc2k130 `--dp-file`).

| distinguished-point weight | points to collect (expected) | storage at 32 bytes |
|:--|--:|--:|
| 32 | 6.032e9 (2^32.49) | 193.0 GB |
| 34 | 5.317e10 (2^35.63) | 1.70 TB |

An in-memory collision table needs at least the 8-byte key of every point: 48 GB at weight 32, 425 GB at weight 34. A sorted-run or disk-backed table with 32-byte records needs the storage column.

## CPU alone

3.935e7 core-hours = 4.488e3 core-years at the measured median rate.

## GPU hours and cost (expected work)

GPU-hours = work factor x 2^60.9 / (rate x 3600). Cost = GPU-hours x the price of the same GPU part. Every price is an input from a committed observation with its date; no live quote was available to this run. Rates are the committed GPU measurements, not measured here (no GPU exists on this host).

| GPU row | rate (B it/s) | work factor | GPU-hours | CPU core-hours per GPU-hour | cheapest listed price ($/h) | break-even CPU core-hour price |
|:--|--:|--:|--:|--:|--:|--:|
| RTX PRO 6000, sigma walk with the shared-memory sigma-table kernel (the campaign's shipping iteration function) | 15.884369 | 1.000000 | 37623 | 1046 | 1.2128 | $0.00116 |
| RTX PRO 4500, promoted x-only two-bridge map (a different iteration function; work factor 1.006696 from its frozen 2,000-trial collision study) | 12.499203 | 1.006696 | 48133 | 823 | 0.5705 | $0.00069 |

| GPU row | price row | $/h | total cost, expected work (USD) |
|:--|:--|--:|--:|
| RTX PRO 6000 | AWS g7e.2xlarge spot, us-west-2 (RTX PRO 6000) | 1.2128 | 45629 |
| RTX PRO 6000 | AWS g7e spot per GPU-hour (RTX PRO 6000) | 1.3100 | 49286 |
| RTX PRO 6000 | AWS g7e.48xlarge on-demand per GPU-hour (RTX PRO 6000) | 4.1400 | 155760 |
| RTX PRO 6000 | Modal list, RTX PRO 6000 | 3.0300 | 113998 |
| RTX PRO 4500 | AWS g7.2xlarge spot, us-west-2 (RTX PRO 4500) | 0.5705 | 27460 |

## Work to complete at a stated probability (modelled, not measured)

The expected work is a mean; the first-collision work of a random map follows approximately a Rayleigh law, whose p-quantile in units of the mean is sqrt(-2 ln(1-p)) / sqrt(pi/2). This is a model (the classical random-map law the corpus assumes), not a measurement of this walk's tail.

| probability of having solved | work, multiple of the mean | iterations | sigma-table GPU-hours (RTX PRO 6000) | cost at the lowest RTX PRO 6000 price (USD) |
|--:|--:|--:|--:|--:|
| 0.5 | 0.939 | 2.021e18 | 35345 | 42866 |
| 0.9 | 1.712 | 3.684e18 | 64419 | 78128 |
| 0.99 | 2.421 | 5.210e18 | 91103 | 110490 |

Sources of the rates and prices:
- RTX PRO 6000, sigma walk with the shared-memory sigma-table kernel (the campaign's shipping iteration function): ecc2k130/benchmarks/sigma-table/headline/result.json candidate_median_million_per_second 15884.369; ecc2k130/README.md
- RTX PRO 4500, promoted x-only two-bridge map (a different iteration function; work factor 1.006696 from its frozen 2,000-trial collision study): ecc2k130/README.md (12.499203 B/s paired median; 1.006696 mean-work ratio, benchmarks/g7-12b-xonly-doubling-bridge)
- AWS g7e.2xlarge spot, us-west-2 (RTX PRO 6000): ecc2k130/RTX-PRO4500.md, observed 2026-09-16
- AWS g7e spot per GPU-hour (RTX PRO 6000): ecc2k130/FPGA-CEILING.md section 5 (prices dated 2026-09-11)
- AWS g7e.48xlarge on-demand per GPU-hour (RTX PRO 6000): ecc2k130/FPGA-CEILING.md section 5
- Modal list, RTX PRO 6000: ecc2k130/B200.md
- AWS g7.2xlarge spot, us-west-2 (RTX PRO 4500): ecc2k130/RTX-PRO4500.md, observed 2026-09-16

Not used as a completion rate: the 20.078 B/s table-walk figure (ecc2k130/ONE-BLOCK-GEOMETRY.md) was measured under the superseded v2 cycle rule and does not transfer to the v3 table walk, which measures about 5.1 B/s (ecc2k130/README.md); the sigma walk is the faster one.

## Wall clock

| GPUs (sigma-table, 15.884 B/s each) | days at expected work |
|--:|--:|
| 100 | 15.7 |
| 1000 | 1.6 |
| 10000 | 0.2 |

## What this plan claims and does not

It claims the CPU rate measured above and arithmetic on the committed GPU rates, work constants, record size and price observations listed with their sources. It claims no GPU rate measured here, no live price, no campaign progress credit (distinguished points already collected are not subtracted), and nothing about the collision-finding cost beyond the storage lines. The weight-34 interval is the committed 2^25.27 and the weight-32 interval the measured 2^28.41. The Rayleigh quantiles are a model. Network, orchestration, preemption and data-transfer costs are not priced.
