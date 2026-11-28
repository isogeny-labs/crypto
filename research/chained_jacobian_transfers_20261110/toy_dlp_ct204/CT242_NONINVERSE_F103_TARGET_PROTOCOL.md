# CT242 — independent non-inverse target gate for F103 norm-image ECDLP (2026-11-19)

**Preregistration before producing CT242 source or new outcome. [pcc:unverified].**

## Why this control is essential
For E/F103 of source order r=109, the degree-three norm identity gives `3^{-1} mod 109 = 73`. The CT238–241 primary public target also has known log **73**. This numerical coincidence is *not* an error by itself, but it leaves a cheap false-positive mode: a buggy decoder returning the fixed inverse-degree constant could pass a Q73-only known-answer test.

## Exact registered control
- Keep CT240's immutable F103 E/C, complete 22-orbit norm roster, 6,540 relation-candidate cap, 60 original offsets, exact signed 2LP elimination, and no private source fixture. Use **only B=12 and B=22**, since these bracket the observed reduced/full rank strategy. Reconstruct factor-base/LP logs using actual CT240 raw norm equations, not ECDLP lookups.
- Two new public **point inputs**: `Q37=(21,6)`, `Q51=(73,17)`; known logs 37 and 51 appear **only in the final known-answer assertions**, never the relation builder or target norm-identity computation. Each is fixed before running.
- For each candidate target expression with coefficients c_i, verify independently on E that `sum(c_i*P_i) = [3]Q + [3a]G` using the **public Q point**, not its scalar. Recover `k = 3^{-1} sum_i c_i log_G(P_i) - a (mod 109)`, and check both `[k]G == Q` and `[k]phi*(G) == phi*(Q)`. Only then compare k to the known-answer scalar. Explicitly assert that neither target log equals 73.
- Cap at 6,540 target candidates and retain unsuccessful targets with counts. Verify each reconstructed base/LP log on E. Record source head, exact commands, independent target identity counts and all failures; shared Python arithmetic is not independent native execution.
- Do not claim cold timing, lower costs, scaling or PCC. The separate CT241 native Q73 paired run must be completed and cost-audited before any performance inference; cross-target tests strengthen correctness only.

## Prior work and boundaries
CT240 finite norm stream and CT237 independent E norm relation verifier are reused; the degree-three identity is standard algebraic Picard norm/pullback composition. The numeric Q73 coincidence is particular to r=109; it is neither a general DLP attack nor evidence of hidden weakness. No paid compute, protected merge, original 695/701 authentication bypass, or speedup claim. [pcc:unverified].
