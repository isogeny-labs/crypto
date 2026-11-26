# ECC2K-130 low-solving-degree forms: support-aware edge lifts

Date: 2026-11-26. Target: the public ECC2K-130 challenge curve
`y^2 + xy = x^3 + 1` over `GF(2^131)`. This round contributes one new,
exactly matched presentation family, a deterministic support census, and a
frozen next experiment. It does **not** report a solving-degree reduction or
an ECC2K-130 attack improvement. All claims are `[pcc:unverified]`.

## Result

The three products in a cubic binary `S_3` link can be lifted separately.
Prior repository work used only `w = x_a x_b`. The two mixed-edge arms
`w = x_a x_c` and `w = x_b x_c` have the same Boolean-variable count, field
equation count, and generator degree (two), but a different descended monomial
graph because factor-base leaves and full-field intermediates occupy different
spaces.

An exact native census on 72 matched planted cells (`n in {11,13,17,19}`,
`a in {0,1}`, `b=1`, `m in {3,4,5}`, `ell in {3,4,5}`) found:

| arm | distinct monomials / `ab` (min / upper median / max) | term occurrences / `ab` | cells with fewer distinct monomials |
|:--|:--|:--|--:|
| lift `ac` | 0.479 / 0.677 / 0.814 | 0.701 / 0.965 / 1.164 | 72 / 72 |
| lift `bc` | 0.479 / 0.847 / 0.938 | 0.884 / 0.997 / 1.164 | 72 / 72 |

All 216 systems have degree two, equal matched variable counts, and satisfy
their planted relation. `ac` has fewer total term occurrences than `ab` in 47
of 72 cells; `bc` does so in 38 of 72. The raw rows are
[`results/edgecounts.jsonl`](results/edgecounts.jsonl), SHA-256
`9af7c84b4767330fbfc4c2c89c9567862b3e49f29fd0207bc12e27245f113cd7`;
the deterministic aggregate is [`results/summary.json`](results/summary.json).

This is a support diagnostic, not a degree-of-regularity measurement. Fewer
input monomials can reduce products and fill-in, do nothing, or make closure
worse. `PROTOCOL.md` makes closure degree, maximum built columns, and nonzero
entries the decision variables.

The quantitative range/median graph is
[`figures/support-ratios.svg`](figures/support-ratios.svg); the formulation map
is [`figures/edge-lift-map.svg`](figures/edge-lift-map.svg). Existing canonical
scoreboard, timeline, curve, and performance graphs were checked and left
unchanged because this diagnostic moves no validated cost or solving-degree row.

## Requirement-to-evidence ledger

| requested component | status | evidence / remaining gap |
|:--|:--|:--|
| Work in the Isogeny Labs workspace first | complete | workspace routing, sparse `crypto` checkout, branch and draft PR #5231 |
| Generate new summation-polynomial forms | complete at proposal/implementation scope | three matched edge lifts implemented in the existing native derivation harness |
| Aim at lower regularity and total solving degree | partial | every new generator is quadratic and `ac` reduces distinct support on all 72 toy cells; closure/solving degree is not measured |
| Avoid rediscovering known forms | complete for the inspected corpus | compared with chained `S_3`, direct/symmetrised `S_m`, Riemann--Roch, pair-invariant, half-trace, and line-norm records below |
| Select a next discriminating test | complete | frozen matched closure protocol in `PROTOCOL.md` |

## 1. The edge-lift family

For an ordinary binary curve, the three-variable summation polynomial is

```text
S3(x_a,x_b,x_c) = (x_a x_b + x_a x_c + x_b x_c)^2
                  + x_a x_b x_c + b.
```

For any edge `ij` with remaining vertex `k`, introduce a field variable
`w_ij` and replace the cubic link by

```text
Q_ij = w_ij + x_i x_j = 0
F_ij = w_ij^2 + (x_i x_k)^2 + (x_j x_k)^2
       + w_ij x_k + b = 0.
```

After Weil descent, squaring is `F_2`-linear and every displayed product is
bilinear, so both field equations have Boolean degree at most two. Substituting
`Q_ij` into `F_ij` recovers the original link in characteristic two. The new
arms were added beside the existing `ab` arm in
`../representation_leads_20261109/derive/src/systems.rs`; eleven native tests,
including planted relations and degree checks, pass under Rust 1.99.1.

The substituted integer-polynomial identities are sealed as
[`IDC1h02b24a926e268f4d`](identities/edge-ac.json) (`ac`) and
[`IDC1h943d3d2cc5f015d2`](identities/edge-bc.json) (`bc`), 32 derived points
each. Their coefficients are in `{0,1}`, so the records check the substitution
identity over the integers up to the stated v1 caveat as well as over the
certificate field. They do not certify Boolean degree, ideal equivalence before
the lift constraint, or algorithmic usefulness; those are separate checks.

The three presentations are algebraically symmetric over unrestricted field
variables. They are not support-symmetric after descent in an index-calculus
chain:

- `x_b` is a new `ell`-bit factor-base leaf;
- `x_a` is a leaf only at the first merge and a full-field running sum later;
- `x_c` is a full-field intermediate except at the target-anchored final merge.

That domain asymmetry is why `ac` and `bc` can have different monomial unions
even with identical counts of variables and equations.

## 2. Candidate forms

### C1: `SAEL-ac`, mixed-edge left chain — advance to closure test

Lift `x_a x_c` at every cubic link and keep the final target-anchored link
unlifted. It is the only new arm with a measured support advantage in every
toy cell and the best median reduction (32.3%). The hypothesis is narrowly
stated: this smaller monomial union reduces closure fill or refutation degree
relative to the matched `ab` lift. No end-to-end consequence is assumed.

### C2: support-aware tree and edge schedule — design after C1

At arity `m >= 4`, enumerate binary merge trees and one of `{ab,ac,bc}` at
each cubic internal node. Score the **union** of descended monomials, not the
sum of per-link term counts, because reuse between links is the observed
effect. Keep the same number and dimensions of auxiliaries across matched
arms. The first screen should compare left and balanced trees at `m=4`; a
tree win only advances if it also lowers measured closure work.

This combines two existing ingredients (balanced chained `S_3` and quadratic
lifts) but the support-aware edge schedule is not present in the inspected
records. Novelty remains a literature-search result, not a proof.

### C3: sum-exposed mixed lift — secondary coordinate arm

For a leaf pair set `u=x_1`, `s=x_1+x_2`, and lift `z=u e`, where `e` is the
intermediate abscissa. Since the factor base is linear, `u,s in V` represents
the same ordered pairs as `x_1,x_2 in V`. The link can be written

```text
z + u e = 0
u^4 + u^2 s^2 + s^2 e^2 + u^2 e + s z + b = 0.
```

It is quadratic after descent and exposes the `s` coordinates that previously
made invariant guessing better than a raw-coordinate guess at `m=3, ell=8`.
However, `(u,s)` is an invertible linear change from `(x_1,x_2)`, so this arm
must not be credited for lower regularity without a matched closure result.
It is a presentation/ordering hypothesis, not a new algebraic relation.

### C4: half-trace reciprocal quotient — retain as a high-risk boundary test

With `s=x+y`, `p=xy`, and a nonzero intermediate `e`, put `h=e^-1` and
`t=p/e`. The link becomes the Artin--Schreier equation

```text
t^2 + t = s^2 + b h^2.
```

Because `n=131` is odd, half trace represents `t` with one kernel bit once the
trace condition holds. Half trace itself is already known in the repository
(`THM-RP-S3-LINEAR-HALF-TRACE`); it is not a new idea here. The new question
would be whether unordered leaf pairs can be replaced by `(s,p)` without
paying more to enforce exact membership in `Sym^2(V)`.

The first desk gate is adverse. If `L_V(T)` is the linearised subspace
polynomial, exact membership is `T^2+sT+p | L_V(T)`. Writing
`T^(2^j) = A_j T + B_j mod (T^2+sT+p)` gives

```text
A_(j+1) = s A_j^2,
B_(j+1) = p A_j^2 + B_j^2.
```

Without auxiliaries, ordinary degree grows with `j`; with a squaring chain,
the presentation adds the large auxiliary ladder already audited as dominated.
The nonzero reciprocal chart also needs an exact `e h=1` witness or an
explicit chart cover. C4 therefore does not advance until someone supplies a
pair-image encoding whose measured Macaulay width is below C1.

## 3. What was screened out

- **Adding `p=x_1x_2` without replacing leaves:** already measured at
  `(22,6)` with no lower degree and 5.1--5.5 times the degree-four columns;
  `(21,7)` remains censored. The new census does not revive it.
- **Half trace alone:** known two-point linearisation; no novelty and no
  removal of the pair-membership or inverse-chart cost.
- **Direct or symmetrised higher `S_m`:** higher generator degree and existing
  audits; not a lower-degree candidate by itself.
- **Line-norm (`e_2`) quadratisation:** already degree two with the same
  variables and monomial support as the old generic `ab` lift, but more term
  occurrences. It remains a required matched control.
- **Riemann--Roch/Nagao encodings:** they beat Semaev controls on small/full
  panels but the exact search remained the same order as pair enumeration and
  the structural oracle expired far below the relation-existence dimension.
- **Frobenius-slot decorrelation:** useful as a later orthogonal factor-base
  arm, but already represented by the unrun G6/R4 work; it is not a new
  summation form.

## 4. Decision and next action

Run `PROTOCOL.md` only after its system export is pinned and checkpoint upload
works. Advance C1 to an `m=4, ell=7` scale gate only if, on the matched toy
panel, it has strictly smaller closure refutation degree on at least 12 of 16
unsatisfiable draws in every `m>=4` cell and no reverse cell. Equal degree with
lower support is a support result only; it may motivate a timed F4 comparison
but is not a solving-degree win.

No ECC2K-130 row moves until a candidate preserves planted soundness, natural
relation yield, cold total cost, and a rho comparison in compatible units.

## Sources inspected

- Repository synthesis: `research/notes/ecc2k130/STATUS_20261117.md`,
  `RESEARCH_ECC2K130_DESCENT_DEGREE.md`,
  `research/representation_leads_20261109/`,
  `research/binary_semaev_irreducibility_audit_20261113/`, and
  `research/notes/ecc2k130/RESEARCH_ECC2K130_RR_SOLVER_PANEL.md`.
- Semaev, *Summation polynomials and the discrete logarithm problem on elliptic
  curves* (2004): <https://eprint.iacr.org/2004/031>.
- Petit--Quisquater, *On polynomial systems arising from a Weil descent*
  (2012): <https://eprint.iacr.org/2012/146>.
- Huang--Kosters--Yeo, *Last fall degree, HFE, and Weil descent attacks on ECDLP*
  (2015): <https://eprint.iacr.org/2015/573>.
- Karabina, *Point Decomposition Problem in Binary Elliptic Curves*:
  <https://arxiv.org/abs/1504.02347>.

The research-KB executable/service was unavailable in this environment. The
fallback was direct repository and primary-source review; novelty statements
remain `[pcc:unverified]`.
