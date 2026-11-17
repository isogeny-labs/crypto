# EcGFp5 m = 5 quotient, block-Wiedemann finish (2026-11-02): documentary copy

Browsing copies of the report for
[`research/ecgfp5_quotient_bw_finish_20261102/`](../../research/ecgfp5_quotient_bw_finish_20261102/):
[`report.pdf`](report.pdf), [`bw_finish.svg`](bw_finish.svg) and a PNG render.

The study replaces msolve's change of order and parametrisation in the merged
[1 November quotient solve](../20261101-ecgfp5-quotient-solve/) with a block-Wiedemann
finish on the BNS22 chain matrix. It runs under a protocol frozen before the cells. It
is a per-system stage cost, not an end-to-end ECDLP solve, so it is **not** an admitted
benchmark row: it has no `VALIDATION.json`, no replay certificate and no matched rho
reference.

- **Correctness.** At p = 4111, 262231 and 33554371 the eliminant is identical to the
  baseline's, D = 2^16, and the planted point is recovered.
- **Whole-solve CPU-s.** 16,912, 18,823 and 22,516 against 31,004, 37,036 and 38,778:
  1.83×, 1.97× and 1.72× lower, under the frozen 1.5× bar at every prime. The finish
  alone is 3.1–4.1× cheaper.
- **G1.** c_cyc is 2^45.43, 2^45.58 and 2^45.84. The frozen [c/4, 4c] band straddles the
  CLOSE line (2^46), so the verdict is unchanged. The distance to PROMOTE (2^38) is
  7.4–7.8 bits.

This is a per-system stage improvement. It is not an index-calculus pipeline row, and the
IC leaderboard is unchanged.
