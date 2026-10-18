# Round 316: the amortized per-target parity is size-bound

Date: 2026-10-17

**The Round 315 per-target parity at T=16 does not survive scaling.**  At
the three larger frozen sizes the amortized per-target ratio returns to
parity-or-worse with 95% intervals including 1.00, and the four-size fit
gives the amortized candidate an exponent of 0.476 against rho's 0.423 —
the same steeper-than-rho growth Round 314 measured one-target, with only
the constant `T` divided away.  This is a reproducible negative scaling
result.

## Boundary table

Amortized per-target unit (candidate whole-execution GAE / 16) against the
matched strong rho one-target reference.  Intervals are the `ecbench
compare` cluster intervals divided by T, matching the analyzer's
deterministic output.

| cell | log2 r | amortized / rho | 95% interval | verdict |
|:--|--:|--:|:--|:--|
| `icv1-f2m37-tm534059-32aad96b` (Round 315) | 27.78 | **0.686** | [0.630, 0.740] | parity, holds |
| `icv1-f2m41-tm2308219-7f48b14a` | 32.11 | 1.020 | [0.827, 1.273] | fails: interval includes 1.00 |
| `icv1-f2m43-tm998717-e2e742b0` | 32.11 | 0.908 | [0.765, 1.097] | fails: interval includes 1.00 |
| `icv1-f2m47-t22705043-f4e44623` | 36.64 | 1.066 | [0.867, 1.332] | fails: interval includes 1.00 |

## Scaling fits

Over the four frozen T=16 cells (`log2 GAE` against `log2 r`; finite-size
fits, not asymptotic proofs):

| metric | fitted exponent | R² |
|:--|--:|--:|
| amortized candidate per-target GAE | 0.476365 | 0.998 |
| strong rho one-target GAE | 0.422804 | 0.990 |

The amortized candidate's exponent is unchanged from Round 314's one-target
0.473794 within fit noise, as it must be: amortization divides a constant
`T` from the total.  Because rho grows more slowly, the amortized ratio
rises with size, and the n37 parity is a size-bound constant-factor win.

## Correctness and replay

- 432/432 new records verify; 360/360 measured records replay
  bit-for-bit (`ecbench verify --replay-all`, 0 problems per session).
- Counted A/A exact at every cell.
- All runs L0 on the development host; operation counts are the metric.

## P-256 amortization economics

Round 312's projection prices the shared relation harvest at
`2^165.155613` FME and the per-relation descent at `2^148.080980` FME.
Per-target amortized cost is `2^148.081 + 2^165.156 / T`; rho parity near
`2^128` needs `T >= 2^37` targets on one harvest, and the
free-relation-oracle floor of `394.425280 x` rho bounds the best case at
any `T`.  Multi-target amortization cannot rescue the P-256 Dickson line.

## Decision

Round 316 closes as a **negative scaling result**: the amortization lever
is real but bounded — it buys a constant factor that rho's shallower
exponent erodes with size, and it does not reach P-256 at any realistic
target count.  A further amortization round would need either a candidate
whose work exponent is at most rho's or a P-256 relation harvest below
`2^128 + 37` total.

## Reproduction

```bash
for d in n41 n43 n47; do
  target/release/ecbench verify \
    --dir research/ecbench_transport_amortize_round316_20261017/sessions/holdout-$d-t16 \
    --replay-all --exit-code
done
target/release/examples/transported_log_amortize_round315_analyze \
  research/ecbench_transport_amortize_round315_20261017/sessions/holdout-n37-t16 \
  research/ecbench_transport_amortize_round316_20261017/sessions/holdout-n41-t16 \
  research/ecbench_transport_amortize_round316_20261017/sessions/holdout-n43-t16 \
  research/ecbench_transport_amortize_round316_20261017/sessions/holdout-n47-t16 \
  /tmp/round316-ANALYSIS.json
cmp research/ecbench_transport_amortize_round316_20261017/ANALYSIS.json /tmp/round316-ANALYSIS.json
```
