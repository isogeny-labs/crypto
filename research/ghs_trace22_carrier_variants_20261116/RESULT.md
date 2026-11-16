# Twelve-carrier trace +22 degree-seven cover census

16 November 2026 UTC. The frozen 24-cell screen in [PROTOCOL.md](PROTOCOL.md) completed with **zero exact degree-seven cover identities** in its `B=H²` or `B=Q_iH²`, monic-quadratic-`H` shapes. The [manifest](manifest.json) retains all 24 result cells, and the [coverage audit](audit_manifest.json) independently checked receipt hashes, census membership, full cell coverage, counters and timings. The audit does not reimplement the algebraic elimination. Finite claims remain [pcc:unverified] until PCC admission.

## Exact finite scope

The base field is `F3`, `K=F3[a]/(a^5+2a+1)`, and every carrier has form `Y_m: y²=f_m(x)` with `f_m=x^11+Σ_(i=0)^10 floor(m/3^i) mod 3 · x^i`. The twelve fixed masks are

```text
32280  35349  36380  51009  67186  106121
112490 113379 127268 168265 175537 175771.
```

They are precisely the twelve trace +22 monic odd-degree models in the earlier finite carrier census; each has Weil polynomial `X^10−22X^5+243` and branch factorization one linear plus an irreducible degree-ten factor over F3. The two source representatives are `E_68: v²=u³+u²+2+a+a²+2a³` and `E_84: v²=u³+u²+a+a⁴`, each with 222 K-points. The source class census has ten trace +22 equations, partitioned into the two five-element Frobenius orbits whose exact mask arithmetic is checked by [Trace22Orbit.lean](Trace22Orbit.lean).

The 24 run cells are the Cartesian product of the twelve carrier masks and source masks 68 and 84. Each cell completed 243 `H²` and 405 `Q_iH²` resultant systems, representing 37,946,232 logical assignments. Across all cells that is **15,552 systems and 910,709,568 logical assignments**. The retained [runner](run_variants.py) preparsed the frozen source once, used four local workers, wrote complete JSON and `/usr/bin/time -l` records atomically, and bound the final manifest to the source and runner hashes. Aggregate user CPU across the 24 runs was **4,020.27 seconds**; this is cumulative CPU, not wall time or an attack cost.

For any one of the ten source models in these two orbits, a degree-seven hit in the searched shapes would conjugate under `a↦a³` to a hit for one of the two representatives: each `f_m` is defined over F3, the `Q_i` factors are permuted, and monic quadratic `H` remains monic. Thus the zero-hit result extends from 24 representative cells to **120 exact carrier/source pairs** in these shapes. The analogous simultaneous quadratic twists inherit the same shape-restricted conclusion by untwisting the cover equation. This is a finite computational exclusion for the listed models and denominator shapes, not a lower bound on all cover degrees and not an exclusion of other correspondences or genus-five carriers. Its elimination code has not been independently implemented in another algebra system.

## Vulnerability and priority boundary

The trace +22 class is separate from the trace ±26 twist orbit. Its exact group-order recurrence has an **80-bit prime factor** `785807256797924665975331` at `s=11`; [the order receipt](../ghs_trace22_cover_screen_20261116/order_screen.json) checks its factorization. The bare leading exponent screen `r^5 > 11^5(3^11)^16` is checked by Lean, but the units differ and the constants are absent. Since this finite search found no evaluable map, there is no selected-subgroup transfer, complete GHS solve or same-target rho comparison for trace +22. It is a remaining construction lead, not a measured vulnerability.

The same branch's [trace +10 screen](../ghs_trace10_cover_screen_20261116/RESULT.md) found zero hits on one fixed carrier and two source orbits in the same shapes. The [trace −26 report](../ghs_twist_s7_attack_20261116/S7_RESULT.md) remains the only complete GHS-faster exact instance here, with 231.125131 CPU seconds versus 3268.034356 for the same-target order-seven-Frobenius rho run. Its cover is a quadratic twist of the positive construction orbit, so that result is a potentially vulnerable **exact curve record**, not an independent new eligible family or new attack method.

The shared KB and targeted exact-equation searches found no publication match for these selected carrier masks; that does not prove worldwide priority. The geometric carrier census itself was already a project record, and [Iijima–Momose–Chao 2015](https://eprint.iacr.org/2015/805.pdf), [Joux–Vitse 2012](https://www.iacr.org/archive/eurocrypt2012/72370010/72370010.pdf), and [Diem 2006](https://arxiv.org/abs/math/0606607) establish nearby direct cover, isogeny-assisted transfer and Jacobian index-calculus methods. A second vulnerable family has not yet been established.
