# ECCp-131 auxiliary-tower transfer audit

23 October 2026. **Question:** Can the certified eight-step rational 2-isogeny tower on `E': y²=x³+x+301` transport the published ECCp-131 discrete-log subgroup or its relations? Scope: the exact field `p=0x48e1d43f293469e33194c43186b3abc0b`, the published group `E(Fp)` and this one auxiliary curve `E'(Fp)`. This is a mathematical assessment of the retained construction, not a new target trial or a claim about all relation oracles.

## Exact order screen

The independent [published-curve PARI/GP identity check](runs/identity_check.stdout) verified `#E(Fp)=n=1550031797834347859219047037805205710577`, a prime. The [auxiliary search output](runs/search.stdout) records `#E'(Fp)=m=1550031797834347859229712325794928046336`, with `256|m`. The [replay certificate](runs/replay.json) binds both to the same field. [The deterministic order check](check_transfer_obstruction.py) reads that certificate and writes [its raw derived result](runs/transfer-order-screen.json): `gcd(n,m)=1`. A separate [direct GP order script](transfer_order.gp) recomputed both curve counts in one process and returned `TRANSFER_ORDER_OK` in [attempt 4](runs/transfer-order-attempt4.stdout). The 30 s wall timeout was enforced; this sandbox prevented a memory watchdog, as [the execution note](TRANSFER_EXECUTION.md) records with all failed attempts. The separate 256-order witness check in [replay_and_screen.py](replay_and_screen.py) supports the auxiliary 2-primary component independently of full point-count recomputation.

For any homomorphism `f:E(Fp)→E'(Fp)`, the image order divides both `n` and `m`, hence divides `gcd(n,m)=1`; so `f` is trivial. The same holds in the reverse direction. An isogeny defined over `Fp` induces a homomorphism on rational points; moreover isogenous curves over `Fp` have the same point count. Their unequal orders therefore rule out an `Fp`-isogeny between these exact curves. Since every curve in the published `Fp`-isogeny class has odd order `n`, none has a rational point of order 2 or a rational degree-2 isogeny with a rational order-2 kernel. This is a structural obstruction to **transporting the published subgroup through this rational 2-tower**. It says nothing about isogenies after a field extension or about unrelated higher-degree maps; their costs and subgroup recovery would require separate certificates.

## Typed path and obligations

The [typed diagram](transfer_path.svg), generated from [editable source](draw_transfer.py), shows the actual construction. The eight verified degree-2 arrows are among auxiliary curves. The later operation takes an integer abscissa `x`, tests whether `x` lifts to a point on `E`, and collects 115 such lifts. That operation is a partial *coordinate selection*, not a point map `E'→E` and not a homomorphism. Consequently the auxiliary tower can provide a structured factor-base candidate, but its 2-primary group relations cannot be carried into the published prime-order group. The independent `screen.py` point equations on `E` remain valid correctness controls, and the existing zero-natural-row result remains exactly scoped.

| Obligation | State | Evidence or limit |
| --- | --- | --- |
| Auxiliary degree-2 tower existence and map correctness | supported | `runs/search.stdout`, `runs/replay.json`, eight checked kernels/codomains |
| `Fp` homomorphism transporting published subgroup to this `E'` | refuted | Exact `gcd(n,m)=1`, independently recomputed group orders, and image-order theorem |
| Same-field rational 2-isogeny within published isogeny class | refuted | Every member has odd order `n`, hence no rational 2-torsion |
| Abscissa lift as group homomorphism | refuted | The operation is only a partial coordinate selection; it has no group-map formula |
| Independent published-curve relation from these 115 columns | unknown | Fixed 256+256 screen found zero; three-point support bound applies |
| Complete IC cost and target recovery | unknown | Relation, rank and descent gates remain open |

The transfer cost table has a measured 83.927 s auxiliary search and 8.276 s replay/screen on the recorded host, plus the separate nine-cell stage benchmark. Map-to-published-subgroup, relation collection to rank, linear algebra, target descent and recovery are unavailable, so neither a cold transfer cost nor a benefit can be computed. Direct source-group rho and coordinate-isomorphism controls would be required for a future end-to-end comparison; same-field random points and independent point addition remain the existing screen controls. No speed or ECDLP bound is claimed.

## Consequence and next check

The next relation-oracle candidate should specify **its mechanism on the published group** and calculate attainable natural-row support before execution. If it uses an auxiliary curve, it must give an explicit subgroup-preserving map or treat the curve only as a source of factor-base coordinates; the latter still needs a measurable exact-curve relation rate. For this auxiliary `b=301` curve, the group-order obstruction closes the transport interpretation. It does not close the broader index-calculus problem. The [problem record](../../problems/eccp131-full-parameter-index-calculus.md) tracks the remaining full-parameter relation, rank and target-descent gates. No canonical cost-frontier graph changes because this assessment has no end-to-end cost measurement.
