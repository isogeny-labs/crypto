# D1 — Frobenius-transport relation graph: frontier sharing + algebraic A\* + matroid selection

**Status:** design / preregistration draft. No measurement. No claim. Design-only.
**Curve:** K₀: y²+xy=x³+1 / F_{2^131}; τ²+τ+2=0, End=Z[τ]=O_K, K=Q(√−7), h=1;
E(F_{2^131})≅Z/4×Z/r, r≈2^129 prime, μ=τ mod r of order 131. Toy rungs n=31, n=41.
**Counted unit:** ecbench GAE (I=10M, S=M, 1 GAE=I+2M+S); unpriced native ops
(canonicalisation, hash probes, heuristic eval) reported at the §5a band 0 / 0.1 / 1 GAE.

---

## 0. One-paragraph honest verdict (read this first)

Of the three ingredients, **only the A\* ordering is a candidate for a durable (still
constant-factor) gain.** The "cross-target frontier sharing" that names this direction,
when the overlap is computed instead of asserted, is **chance-level for independent
targets and vanishes exponentially in n** — I preregister it as a predicted *null* and
the test exists to confirm the birthday estimate, not to find a win. The matroid
selection stage is very likely dominated by the CADO merge already in `koblitz_sparse_la`.
So the real question the experiment answers is narrow: *does an algebraic A\* ordering cut
forward node expansions ≥2× over uniform-cost MITM at matched verified-relation yield, and
does the resulting arm beat the equal-envelope control at all three price points?* If both
main effects (sharing, heuristic) are null, the hybrid collapses exactly onto batched
MITM + compact-orbit, which already exists.

---

## 1. Mechanism and formulation

### 1.1 The graph G=(V,E)

Let F be the factor base: a set of signed-Frobenius orbit representatives (|F| columns).
Canonicalisation map `canon(P)` = lexicographically minimal element of the orbit
{±τ^k P : k=0..130} (262 points → 1 node; the pipeline already stores one unknown per
such orbit, so canon is reused, not new). Work in the prime-order subgroup of order r.

- **V** = canonicalised residual points. A residual ρ = R − Σ(partial chosen factor-base
  sum), stored as `canon(ρ)`.
- **Source nodes:** `canon(R_t)` for each target R_t in the batch.
- **Edges out of ρ:**
  - (a) *peel*: ρ → `canon(ρ − F_j)`, F_j∈F. Cost: 1 EC add (priced) + 1 canon (unpriced).
    Edge label carries (j, orbit exponent k, sign ε) so the exact sparse combination is
    recoverable.
  - (b) *small-norm Z[τ] multiplier*: ρ → `canon(α·ρ)`, α∈Z[τ], N(α)≤N — the measured
    `compact-orbit-scan:norm=N` move (9.2× one-target total at n=41, m=3). Cost: ≈N(α) EC
    ops. These edges enrich a single source into a correlated cluster of √N·-ish residuals.
  - (c) *meet (MITM)*: precompute the target-independent backward table
    T_d = { `canon(Σ ≤ d factor-base reps)` → combination }. Edge ρ → ACCEPT iff
    `canon(ρ) ∈ T_d` (hash probe, unpriced).
- **Accepting state:** a residual that closes to a complete relation
  R_t = Σ ε_i τ^{k_i} F_{j_i}. Every accepted path is re-added and verified in the group,
  fail-closed (repo rule). The planted/target scalar is never solver input.

A relation is a path source→ACCEPT. **MITM as it exists today is the depth-1 instance:**
one forward peel layer feeding T_d by a single hash join. D1 generalises to multi-layer
forward expansion with (i) an ordering (A\*) and (ii) multiplier edges (b) interleaved.

### 1.2 The A\* heuristic

Type-II ONB on F_{2^131}: squaring is a cyclic coordinate shift; product of two basis
elements has 2 nonzero coords. Let W = the F₂-subspace spanned by the x-coordinates of F
(the factor base is a subspace/Frobenius-closed base in the landed recipes). Define

    w(ρ) = ONB-Hamming weight of the projection of x(ρ) onto the complement of W.

`SOLVER_SELECTION.md` establishes this quantity (and the weight of x(ρ)−x(τρ)) is
computable **before** any solve — they are the learned selector's input features. Heuristic

    h(ρ) = ⌈ w(ρ) / w_max ⌉,  w_max = max single-peel weight reduction (measured, not assumed).

A\* expands nodes in increasing g(ρ)+h(ρ) where g = accumulated priced cost.

**Admissibility is itself a hypothesis, not a premise.** If one peel can reduce w by more
than w_max in rare cases, h is inadmissible and A\* loses its optimality guarantee; it then
degrades to weighted-A\*/greedy/beam, still a valid search but without the optimal-path
guarantee. The preregistered claim is about **node-expansion count at fixed yield**, which
does not require admissibility. Admissibility is measured as a secondary outcome (fraction
of peels with Δw > w_max).

**Prior that lowers confidence (recorded, not hidden):** the same weight features gave the
learned selector *no routing headroom* for pairs-vs-SAT (`SOLVER_SELECTION.md`, 288
queries). That is weak evidence the weight signal may be too flat to order a search well.
The A\*-on vs A\*-off arm is the direct test of whether this prior carries.

### 1.3 The cross-target frontier-sharing claim — and its computed null

Claim as usually stated: expanding one forward frontier for a batch of b targets lets a
node expanded once serve every target whose cone contains it, so per-target cost is
sublinear in b **without adding columns**.

**Computed overlap.** Canonicalised node space size M ≈ r/(2n). For b independent targets
each expanding E forward nodes, expected nodes shared by ≥2 targets (E²≪M) is
≈ (b²/2)·E²/M, so the **fraction of forward expansions saved by sharing** is

    f_share ≈ (b/2)·E / M = (b·E·n) / r.

- n=41 (r≈2^39, M≈2^31≈2.1·10⁹), b=32, E~10⁴ ⇒ f_share ≈ 32·10⁴·2/2.1·10⁹ ≈ **7.6·10⁻⁴** (negligible).
- n=31 (r≈2^29, M≈8.7·10⁶), b=32, E~10⁴ ⇒ f_share ≈ **≈0.18** (measurable only because the rung is tiny).

So **sharing scales as b·E·n/r and dies exponentially in n.** The backward table T_d *is*
shared across targets — but that is already true of batched MITM and is **not novel**. For
genuinely independent targets the *forward* cones meet only at the birthday rate. (If the
"32 targets" are instead one signed-Frobenius orbit, canon collapses them to a single node
— there is then one target, not 32.)

## 2. Exact falsifiable claim + quantitative prediction

**H_share (preregistered as a NULL):** measured cross-target forward-sharing fraction at
n=41 is within [0.5×, 1.5×] of the birthday prediction f_share = b·E·n/r; i.e. there is no
structural overlap beyond chance. Prediction f_share(n=41,b=32)≈7.6·10⁻⁴; f_share(n=31)≈0.18.
Falsified (sharing is real) only if measured overlap > 1.5× prediction at n=41.

**H_A\* (the live hypothesis):** A\*-on reduces forward node expansions per verified
relation by ≥2× vs uniform-cost (Dijkstra/BFS) MITM at matched yield and matched T_d.
Secondary: admissibility rate (fraction of peels with Δw≤w_max) reported with a Wilson
interval.

**H_beat (the gate):** the best D1 arm's GAE per verified relation (and end-to-end
one-target GAE) beats the equal-envelope control at **all** of 0, 0.1, 1 GAE/unpriced-op.

## 3. Cheapest discriminating test

K₀ at n=31 and n=41, m∈{2,3} matching the landed F4/MITM cells. 2×2 factorial,
**interleaved** arm order, **masked** arm labels during analysis:

| arm | frontier sharing | A\* heuristic |
|----|----|----|
| S0H0 | off (per-target) | off (uniform cost) |
| S0H1 | off | on |
| S1H0 | on (shared frontier) | off |
| S1H1 | on | on |

Same F, same T_d depth d, same multiplier norm N, same counted instrument on every arm.
≥6 derived seeds per cell; ≥32 independent targets per seed (random [c]G, c secret).
Primary outcomes per arm: forward expansions/relation, GAE/relation, end-to-end one-target
GAE. Report main effects (sharing, heuristic) and interaction.

## 4. Baselines & references

1. **Primary baseline:** batched MITM pair-table (`mitm-frobenius`) + CADO filter
   (`ic logs --filter-strategy cado`) on the identical instance/base/unit = arm S0H0.
2. **Equal-envelope control (the decisive one):** same total priced field-op budget spent
   on a wider orbit-union base with plain per-target MITM. This is the control that reduced
   the double-large-prime graph from 23.8× to **2.1×**; D1 must beat *this*, not plain IC.
3. **A/A arm:** two independent seeds of S0H0 for the noise floor; any effect below the A/A
   spread is not reported.
4. **Matched rho:** `rho.signed_frobenius`, same target, same process, same unit. Report
   **gap-to-rho** (best arm GAE / matched-rho GAE) as distance remaining, never pass/fail.
   (Context: IC at these rungs sits ~17× rho after the large-prime round.)

## 5. Confounders & variable ledger

Instrument (one counted-GAE path on all arms), host (one host, or host as a blocked
factor with A/A per host), memory layout / base ordering (frozen), arm order (interleaved,
not blocked), seeds (derived from one root seed, logged), analyst (labels masked until the
export is frozen), spot preemption (isolab-eks checkpoint; re-run cells that preempt).
Unpriced ops (canon, hash probe, heuristic eval, matroid greedy) counted as named native
counters and swept at 0 / 0.1 / 1 GAE. One `experiment.export/v1` with every row; protocol
frozen (`experiment.protocol/v1`) before data; resume checkpoint in S3 before any run.

## 6. Falsification thresholds

- **Reject H_A\*** (heuristic worthless) if S*H1 expansions/relation ≥ 0.5× S*H0 at matched
  yield (i.e. <2× cut), CI-separated.
- **Confirm the sharing null** if measured overlap ≤1.5× f_share at n=41.
- **Reject the direction as an improvement** if the best arm fails H_beat at any of the
  three price points, or if (sharing null) ∧ (H_A\* rejected) — then D1 ≡ batched MITM +
  compact-orbit, already on main, and nothing new is claimed.

## 7. Expected gain & honest ceiling

Best plausible outcome: A\* + multiplier edges cut forward expansions 2–4× over uniform MITM
at one toy rung, partially absorbed by the equal-envelope control, net maybe 1.3–2×
end-to-end at n=41. **Constant factor, one rung, no exponent.** Sharing contributes ~0 at
n=41 and nothing asymptotically (f_share→0 as r→2^129). This does not reach n=131 and does
not change the subexponential/exponential picture; it is an engineering constant on the
relation-search phase, reported against the rho gap.

## 8. Novelty vs named prior art

- **Large-prime relation graph** (`koblitz_large_prime`, `large_prime_filter`): a graph over
  the oracle's **output** — large primes are vertices, partial relations are edges, cycles
  are complete relations. It **adds columns** (the D set), and the per-target equal-envelope
  control on a base of size |F∪D| absorbed it to 2.1×. D1 is a graph over the oracle's
  **internal search state** (residuals = nodes, group/endomorphism moves = edges) and **adds
  no columns**, so the per-target equal-envelope control does not automatically absorb an
  A\*-ordering gain. This is the one structural reason D1 is worth a run.
- **MITM pair tables** (`mitm-frobenius`, batched): the depth-1 bidirectional instance of G
  (one forward layer + the shared backward table T_d). D1 adds multi-layer forward
  expansion, an ordering (A\*), and multiplier edges.
- **compact-orbit-scan:norm=N** (9.2× at n=41): the multiplier edges (b) from a single
  source with no ordering — a special case of D1. D1's increment over it is the A\* ordering
  and the matroid-coupled selection.
- **Joint matroid relation-selection:** min-weight basis of the linear relation matroid over
  Z/r (Rado–Edmonds greedy, exact for fixed costs) coupled to search-funding (online/
  stochastic, only approximable). Honest expectation: dominated by CADO merge, which already
  optimises LA fill; the only increment is pricing *search* cost into the keep/drop
  decision, likely small.

## 9. Single sharpest risk & how the test reveals it

**Risk:** D1 collapses to batched MITM + compact-orbit (already on main), because (1) forward
sharing is chance-level (preregistered null), and (2) the weight heuristic may be too flat to
order the search — the exact feature set that gave the learned selector *no routing headroom*.

**How the 2×2 reveals it immediately:** if both main effects are null — S1*≈S0* (sharing
dead, as predicted) and S*H1≈S*H0 (heuristic dead) — then all four arms equal the existing
batched-MITM baseline within the A/A floor, and D1 is reported as *no improvement, reduces to
known method*. No narrative can survive a null 2×2. The cheap n=31 cell runs first precisely
because it is where sharing would show if it exists anywhere (f_share≈0.18); its absence
there ends the sharing sub-claim before the n=41 spend.
