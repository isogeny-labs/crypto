# PKM auxiliary-isogeny map sweep: first bounded rung

2026-11-23. Local component benchmark on Apple M4 Pro, arm64, Python 3.12.8. Protocol and raw rows: [PROTOCOL.md](PROTOCOL.md), [results.jsonl](results.jsonl). Code: [sweep.py](sweep.py). All empirical statements here are `[pcc:unverified]`.

## Maps and workload

The two source maps are genuine PKM auxiliary-curve Vélu chains over `F_1000003`, not the earlier Dickson `p+1` arm. The CM map has low degrees `(2,2,2,2,2,2,2,11)` and high degree 1408, with `c=219899`; the Method 1 map has `(2,2,2,2,3,3,5)` and high degree 720, with `c=234701`. Each candidate precomposes its high map with `x -> x-delta` for one of `delta=0,1,2,4,8,16,32,64`. A translation changes fiber placement and adds a degree-one step; it does **not** constitute a new subgroup or improve the underlying isogeny chain.

The target curve is `E: y²=x³+2x+3`. Each candidate uses all its fiber abscissae that lift to `E`, both point signs. Its interval control uses exactly the same number of lifting abscissae. The 144 fixed target points are independent of the bases; 48 are for screening and 96 for confirmation. The query enumerates verified two-point decompositions by subtraction and set membership. It is a relation-yield and query-component screen, not the PKM F4 algebraic relation solver or linear algebra.

## Observed results

All 16 candidate fibers passed the full recorded-fiber evaluation and degree/uniqueness checks. The run produced 16 candidate rows and 15 size-matched control rows (one size was reused). None met the prespecified confirmation gate. The most favorable CM screening count was `shift=16`: 36/48 target successes against 29/48 for the 688-abscissa interval, followed by 62/96 against 59/96 on confirmation (paired one-sided `p=0.3804`). The Method 1 `shift=2` row had 29/96 confirmation successes against 20/96 for its 366-abscissa control, but its screen was 12/48 against 17/48 and its paired confirmation `p=0.0939`. Both are far above the 0.000625 multiplicity gate.

The CM `shift=0` query took 0.303 s CPU for all 144 targets against 0.255 s for its size-matched interval; the base/map check took 0.014 s CPU. The Method 1 `shift=2` query took 0.144 s versus 0.136 s for its matched interval; its base/map check took 0.008 s. These are single-host Python timings, useful for the local screen only. Original source-curve search and isogeny construction were unavailable from the stored artifacts and **not charged**. The interval and isogeny bases also have different algebraic presentations, so query CPU here is not an F4 solver comparison.

The sweeper writes and fsyncs each row, then skips completed candidate IDs on resume. A second invocation left the result SHA-256 unchanged: `4699ac0117ec09541b9044cb7e652d8641bc7af16731bd1fc0f61ad9d1d9bd7e`. An independent exhaustive ordered-pair enumeration for CM `shift=0`, target index 2 `(852315,17757)`, returned two decompositions, matching the sweeper. A zero-relation target at index 0 also matched. Every reported hit is re-added to the target by the sweeper.

## Interpretation and next discriminating rung

This rung found no confirmed placement gain. The next experiment should vary **genuine low-degree map chains or auxiliary curves**, not merely translate the same two fibers, with the same target/size-matched controls and a frozen cost metric. `docs/ic/SWEEPER.md` already provides a `prime-algebraic` F4 space for ordinary auxiliary 2-isogeny towers; a second rung can feed its generated maps through this component gate, then use that F4 space for any candidate that passes. P-192 and the 59-bit fields need their own frozen workload and cannot inherit this field's measured yield. A claimed faster map needs a cold setup-charged, same-target comparison to proceed beyond component status.

## Second rung: actual auxiliary-curve cosets

After freezing [PROTOCOL_COSETS.md](PROTOCOL_COSETS.md), [coset_sweep.py](coset_sweep.py) generated 16 distinct `R'+<Q>` cosets for each source auxiliary curve and recomputed the terminal `c=Φ_x(x(R'))`. These are different fibers of the actual degree-1408 or degree-720 Vélu composition. The low-degree steps stay fixed. [coset_results.jsonl](coset_results.jsonl) contains 32 candidates and 26 size-matched interval controls; every fiber passed distinctness and full rational-map evaluation. No candidate passed the `0.01/32 = 0.0003125` confirmation gate [pcc:unverified].

The smallest paired confirmation `p` was 0.02048 for `cm:coset=5`: 72/96 successes against 59/96 for its 688-abscissa interval. Its screen was 25/48 against 29/48, and full-query CPU was 0.207 s against 0.197 s. The best Method 1 paired `p` was 0.06625 for `isogeny:coset=12`: 30/96 against 20/96, with screen 12/48 against 17/48. These selected extremes do not establish a repeatable relation-yield or speed gain. The result SHA-256 is `bf70d5095807415567c10d47622e02dd58cb6557e4c435f78f4759c71a9231b2`.

The next discriminating rung is now narrower: change the **isogeny chain or auxiliary curve**, not just `R'`. Search and construction, F4 solving, relation collection, rank and scalar recovery all remain to be charged before any full algorithm comparison.

## Third rung: eight distinct chains

Following [PROTOCOL_CHAINS.md](PROTOCOL_CHAINS.md), [chain_sweep.py](chain_sweep.py) invoked the repository `pkm-smooth-tower isogeny` generator for seeds 1–8 at `p=1000003`, smoothness bound 13 and minimum fiber 500. Its binary SHA-256 was `82ae63d60b04e3fbf3c8ad2ada00ab2cd09a03a1609e8ad511a83eab1495aed0`; the generator's two relevant source files were clean against local commit `5097d0d45c46`. The overall local checkout contained unrelated changes, so these are development measurements, not a clean-source release run. Generator CPU was 0.768–1.119 s per successful seed. All eight generated auxiliary curves were distinct, and their low-degree chains and high-degree maps are preserved in [seed artifacts](.) and [chain_results.jsonl](chain_results.jsonl) [pcc:unverified].

| Seed | Low degrees | High degree | Lifting abscissae | Screen hits, map/control | Confirm hits, map/control | Paired p |
| ---: | --- | ---: | ---: | ---: | ---: | ---: |
| 1 | 2,2,2,2,3,5,5 | 1200 | 590 | 26/24 | 52/43 | 0.1312 |
| 2 | 2,2,2,2,3,3,5 | 720 | 338 | 10/16 | 19/20 | 0.6321 |
| 3 | 2,2,3,3,3,3,5 | 1620 | 782 | 34/31 | 59/67 | 0.9179 |
| 4 | 2,2,2,3,3,3,11 | 2376 | 1189 | 46/45 | 88/92 | 0.9270 |
| 5 | 2,2,2,2,2,2,2,5 | 640 | 306 | 11/13 | 17/15 | 0.4253 |
| 6 | 2,2,3,3,3,11 | 1188 | 601 | 26/26 | 49/45 | 0.3220 |
| 7 | 2,2,2,3,3,3,3 | 648 | 315 | 12/16 | 14/16 | 0.7214 |
| 8 | 3,3,3,5,11 | 1485 | 729 | 28/30 | 71/63 | 0.1279 |

All eight full fibers were independently evaluated through their recorded rational chains, with degree equal to fiber cardinality; every hit was checked by point addition. Independent exhaustive ordered-pair enumeration for seed 1, target index 0, found two relations, matching the sweep. No row passed the `0.01/8=0.00125` paired confirmation gate, even before charging generation. The raw result SHA-256 is `fb4185f7a039c162cd11781ce5edd9194bee1f9e38a53e540b09a4ee54d7ed3c`.

Across the three frozen rungs, 56 candidate maps or fibers have been screened: 16 affine placements, 32 genuine cosets and eight distinct chains. None has a confirmed relation-yield and cost improvement over its size-matched interval. This does not settle the larger PKM question: the tests use one field, one target curve, `m=2`, enumerated relations and no F4 or cold single-target DLP pipeline. The next test should use the existing `prime-algebraic` F4 space on a fixed candidate/control pair and charge the complete relation pipeline; P-192 and 59-bit fields need separate runs.

Sources checked: Petit–Kosters–Messeng, [Algebraic approaches for the ECDLP over prime fields](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf), §§3.1–3.4; the repository `docs/ic/SWEEPER.md` and `research/isogeny_vs_mu_factor_base_20261102/`; KB `source:81a446d8a0f88023cfe934a2fcb608bb` (existing prime-algebraic sweep and its provisional baseline).
