# LEAD-15 literature and evidence manifest

Checked 2026-11-01 before implementation or target data. “Full text” means the
retrieved PDF itself was read, not a search snippet. Temporary retrievals are not run
artifacts; their hashes make the reading auditable.

| source | access | SHA-256 of retrieved PDF | material finding |
|---|---|---|---|
| Diem–Thomé, *Index calculus in class groups of non-hyperelliptic curves of genus three*, J. Cryptology 21 (2008), 593–611 | [archived author copy](https://web.archive.org/web/20220101000000id_/https://hal.inria.fr/inria-00107290/document), full text | `0d1445c6aa82fcd31ffcc87e1e871d21d8b0e22b568553265c64ce627283b48b` | practical 2LP graph is heuristic (Assumption 12); published `q≈2^31` matrix has 87,803 rows and mean 352 nonzeros; relation-only and end-to-end rho comparisons must not be conflated |
| Diem, *On the discrete logarithm problem for plane curves*, JTNB 24(3) (2012) | [official PDF](https://jtnb.centre-mersenne.org/item/10.5802/jtnb.815.pdf), full text | `709bac20a012554e41c108a4e823dc58b1dff597b35d5be91b1c5e8812401dab` | Theorem 1 proves expected `O~(q)` for non-hyperelliptic genus 3; its proved construction is not the constant-factor practical DT arm |
| Diem, *Index calculus with double large prime variation for curves of small genus with cyclic class group* | [arXiv](https://arxiv.org/abs/math/0606607), full text | `caa2bb51bd0fbff5977289126a454d0efa42a24648dde3d6ffda249833b2949c` | general double-large-prime asymptotic context |
| Flon–Oyono–Ritzenthaler, *Fast addition on non-hyperelliptic genus 3 curves* | [Waterloo copy](https://uwaterloo.ca/combinatorics-and-optimization/sites/default/files/uploads/documents/2007_10.pdf), full text | `13ba34d28537966192b24d36a35d435fb2c1992d8db12e7a1b1ae438b00cf4f8` | explicit group-law constants are context, not needed by the relation-only harvest |
| Smith, *Isogenies and the discrete logarithm problem in Jacobians of genus 3 hyperelliptic curves* | [arXiv](https://arxiv.org/abs/0806.2995), full text | `44b809946008b6c5d1e9dcadb54b6103fa46ab56a13c0ebeadf9a191b2763140` | Smith supply is a separate route; it does not change Diem's plane-quartic exponent |
| Tian, *Cover attacks for elliptic curves with prime order* | [arXiv](https://arxiv.org/abs/2012.07173), full text | `b7748c551bea45474f3785cb21c86987449ab858438719300f9273c3f6b9cb5d` | prime-order isogeny-cover route is distinct from the exact-cofactor-4 population here |

## Repository evidence used

- Evaluator e5 promotes this as the only candidate in its cluster that could move an
  exponent, corrects the proved-algorithm constant, distinguishes relation-only from
  end-to-end rho, replaces the optimistic row weight 12 by the published warning 352,
  and specifies the 24-cell experiment and K1–K4
  (`research/pdp_strategy_evaluation_20261031/cards/e5.md:8–27`).
- The original lead explains the line-system mechanism and why it is non-generic,
  identifies the unmeasured GHS-family yield/nullity/weight links, fixes lookup and
  `M_ell` prices, and records the historical random-quartic desk check
  (`research/pdp_literature_leads_20261022/leads/LEAD-15.md:72–107`). Those desk values
  are not combined with the prospective data.
- The prior `m=3,r=5` construction uses branch set `{a,b,r0,r1}` and proves the cubic
  conjugation pattern (`research/ghs_cubic_hyperelliptic_genus3_20261013/README.md:41–74`).
- The exhaustive toy census finds both cover types on 24 `q=5` and 49 `q=7` classes,
  and observes cofactor-4 genus-3 supply
  (`research/ghs_cubic_cover_census_20261012/README.md:94–106`). These are supply
  precedents, not large-q outcome evidence.
- The matched round-4 incumbent has fitted slope 1.408 over sizes 16–22 and cost
  `9.967e10` at size 22
  (`research/ic_improve4_20261015/01-genus3-relations/results/run/analysis.md:22–30,52–68`).
- Evaluator e5's appendix gives the same-instance rho formula and explicitly labels the
  plane-quartic practical arm heuristic
  (`research/pdp_strategy_evaluation_20261031/cards/e5.md:81–90`).

## Soundness boundary

The cross-ratio/Möbius common-cover certificate and every finite-field identity are
exact computations. Smoothness, descent, order, primality, line membership, LP
elimination, and rank are independently checked by the committed implementation.
The claim that the practical graph reaches a giant useful component remains heuristic;
this experiment measures it. The projection `7.3M+0.53M*w`, the fitted hyperelliptic
reference at `2^15`, and rho expectations are projections, never observations. No
complete-cost ranking against rho is possible until rho emits the §5a native counters.
