# Independent cyclic walks: coupling and a quadratic mixing bound

Lean proves a finite-time mixing bound for the actual independent fair walk
on every `ZMod n`, `n ≥ 2`. Each step stays put or adds one, with probability
one half each. For every starting point, positive time `t` and event `A`,
the following bound holds. [pcc:unverified]

\[
\left|\Pr[X_t\in A]-\frac{|A|}{n}\right|\le\frac{n^2}{4t}.
\]

The probability comes from an actual Mathlib `PMF`: a normalized fair Boolean
law, mapped through concrete transitions and bound afresh at every step.
The theorem assumes no coupling, stationarity or mixing bound. This is an
unbounded kernel proof, with no enumeration cutoff. [pcc:unverified]

For every prime modulus and any two distinct increments `a,b` of equal
weight, the same bound holds with `n=p`. The proof establishes the exact
time-dependent affine coordinates `x+t*a+(b-a)*z`. Multiplication by `b-a`
is invertible in the prime field. This transfers event probabilities and
cardinality from the stay/add-one walk. It does not assume a fixed
conjugacy between the transition kernels. [pcc:unverified]

The concrete coupling uses one fair coin. Once its two positions agree,
both stay or move together. Before agreement, exactly one position moves.
Both marginal expectations equal those of the actual walk. The modular
difference therefore moves symmetrically until it reaches zero. Lean
checks the neighbor values, the potential `val(d)*(n-val(d))`, its exact
one-step expected drop and its bound `n²/4`. Coalescence makes disagreement
nonincreasing. Telescoping yields the finite-time disagreement bound;
pointwise event inequalities and actual uniform stationarity give the
stated mixing result. [pcc:unverified]

Two direct controls preserve the hypotheses:

- At time zero, the two-cycle singleton has event distance `1/2`, while
  the displayed expression with division by zero evaluates to zero in
  Lean. Extending the bound to that time is explicitly disproved.
- The actual deterministic add-one walk on the two-cycle has singleton
  event distance `1/2` at every time. Lean directly negates eventual
  distance at most `1/4`. This delimits deleting positive weights; it
  does not refute Hildebrand's strict-positivity assumption. [pcc:unverified]

Primary review: Hildebrand, [*Random walks supported on random points of
Z/nZ*](https://link.springer.com/content/pdf/10.1007/BF01199265.pdf),
PTRF100(1994),191–203, selected introduction/Theorems1–2 and Theorem2
proof/context on191–193/199–200. Theorem1 averages over ordered distinct
random supports with fixed positive weights. Theorem2 gives a uniform
quadratic bound. Both concern independent increments. The current result
proves a quantitative `k=2`, equalweight case; it does not refute the
qualified source. Full general weights, larger supports and the exact
random-support asymptotic statement remain open. No full-paper read or
novelty claim is made. [pcc:unverified]

The coupling method is credited to Levin and Peres, with contributions
by Wilmer, [*Markov Chains and Mixing Times*, second edition](https://pages.uoregon.edu/dlevin/MARKOV/markovmixing.pdf),
selected Proposition4.7, Theorem5.4/Corollary5.5, cycle section5.3.2 and
Example10.27 context. Printed50/62/63/64 were visually inspected.
The book's cycle coupling and quadratic potential inform this proof.
Only the selected source scope is marked read. [pcc:unverified]

The bound uses the paper normalization: the maximum absolute event
probability difference, with range `[0,1]`. It is stated for every event,
so no supremum convention is hidden. Mathlib's general `tvDist` uses a
different factor; no equality to that API is claimed. Static rho walks
reuse node labels, and their first-collision results remain distinct.
The random-table collision penalty and limiting collision constants
are not proved here. [pcc:unverified]

Main declaration: `CycleCouplingAudit.cycle_coupling_certificate`, in
[CycleCouplingCertificate.lean](lean/CycleCouplingCertificate.lean).
See [proof map](proof-map.json), [source plan](sources.json),
[source review](evidence/source-review.json) and [visual report](reports/report.pdf).
Exact local **PCC-20261114-13f615** replay PASS at 2026-11-14T09:48:10Z; signed standing remains unverified. See [verification receipt](verification.json).
[pcc:unverified]

```sh
tools/lean/proof-gate.sh research/cycle_coupling_audit_20261114/lean
python3 research/cycle_coupling_audit_20261114/restore_attempts.py /tmp/cycle-coupling-builds
```

Wholegate51 declarations/zero failures/zero trusted native exceptions.
Lean4.35.0/Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679 pinned.
All92 build/processing/provenance files retained losslessly, including
failed states. Standard foundations/default limits; no holes, custom
axioms, native evaluator, raised limits, global elaboration flags or
linter waivers. KB/Conductor unavailable; bounded primary/prior/official
fallback before code recorded. No corpus promotion, full14 closure,
native experiment, paid compute or agent upload. [pcc:unverified]
