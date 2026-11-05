# res1eq chained restarts (null)

Engineering step on the `residual-fused-eq` collector: restart each walk at the previous start plus a random jump (one addition) instead of two scalar multiplications (`residual-fused-eq-chain`, mode 5).

Result: **null**. Paired cost ratio chain/baseline at B=1 is 0.994 [0.953, 1.037] (w=0), 0.998 (w=0.1), 1.011 (w=1); at B=2 it is 1.010, 1.015, 1.028. C1 (upper bound below 1 at all prices) is not met. res1eqc is 1.591 x matched rho at w=0 (1.80 at w=0.1, 2.73 at w=1). Goal (IC near rho parity) not met.

Files: `PROTOCOL.md` (frozen before data), `specs-local/E-p36.json`, `scripts/analyse.sh`, `analysis/`, `REPORT.html`/`.pdf`, `ratios.svg`. Raw session (`sessions/E-p36`, 4608 verified records) is in the artifact store (`.artifacts.json`).
Reproduce: `ecbench run --spec specs-local/E-p36.json --out sessions/E-p36 --allow-busy --quiet` then `scripts/analyse.sh`.
