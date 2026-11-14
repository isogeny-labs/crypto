# General fixed weights: uniform quadratic mixing and a prime-modulus control

Lean proves the qualified uniform-support quadratic theorem from
Hildebrand1994, Theorem2: for any fixed finite support size at least two,
fixed strictly positive normalized weights and tolerance ε>0, a positive
constant γ works for every prime modulus, every distinct support and every
starting point. At every integer time t≥γp², every event differs from its
uniform probability by less than ε. The constant is chosen before the
modulus and support. [pcc:unverified]

The law is constructed from the actual normalized nonnegative weight vector
as a Mathlib `PMF`. Each step binds afresh and adds the chosen increment.
This is an independent-increment law. Lean proves its real finite expectation
formula, actual coupling marginals and uniform stationarity. It assumes none
of those probability or mixing conclusions. [pcc:unverified]

The quantitative result is stronger in its hypotheses: an arbitrary actual
finite label PMF only needs two positive weights at distinct increments.
For every positive t and every event A,

\[
\left|\Pr[X_t\in A]-\frac{|A|}{p}\right|
\le \frac{p^2}{8w_aw_bt}.
\]

One can take γ=1/(4w_aw_bε), giving distance at most ε/2 and hence strictly
less than ε. Other weights may be zero in this stronger version; the source
wrapper retains all strict positivity and support injectivity hypotheses.
[pcc:unverified]

The concrete coupling chooses two independent labels. Before the positions
meet, it swaps the second increment only on the two selected ordered label
pairs. Their product masses are equal, so both marginals remain the original
walk. After meeting, increments are shared. The difference divided by the
selected increment difference moves by +1 or −1 with probability w_aw_b
each, and stays otherwise. Lean derives the exact potential drop 2w_aw_b
until agreement, using the actual modular potential k(p−k). Telescoping,
coalescence, event inequalities and finite translation stationarity give
the displayed bound. [pcc:unverified]

The product coupling gives p²/(2t) for two half-weight increments. The prior
[fair antithetic coupling](../cycle_coupling_audit_20261114/README.md)
retains its sharper p²/(4t) bound. The current component extends the weight
and support scope; it makes no sharper-constant claim. [pcc:unverified]

Lean directly disproves deleting the prime-modulus hypothesis. The actual
independent half-weight walk on `ZMod 4` with distinct increments 0 and 2
stays in the actual event `{0,2}`, which has cardinality two. Its event
probability is one, versus uniform probability one half, at every time.
Lean directly negates existence of any positive quadratic time constant
that makes every event distance less than 1/4. Both weights are positive
and the support map is injective. The paper itself warns about composite
moduli and subgroup confinement; this is a formal hypothesis control,
with no novelty claim. [pcc:unverified]

Primary source: Martin Hildebrand, [*Random walks supported on random points
of Z/nZ*](https://link.springer.com/content/pdf/10.1007/BF01199265.pdf),
PTRF100(1994),191–203. All13 published pages, their proofs, discussion and
references were read; all13 rendered pages were visually inspected.
Theorem2 is on193, its proof on199–200. Definitions and the explicit
composite-modulus warning on192 were checked. The random-support average
and exponent in Theorem1 remain a separate proof obligation. No source
refutation or corpus-wide label promotion is made. [pcc:unverified]

The established potential/coupling method is credited to Levin and Peres,
with contributions by Wilmer, [*Markov Chains and Mixing Times*, second
edition](https://pages.uoregon.edu/dlevin/MARKOV/markovmixing.pdf), selected
Proposition4.7, Theorem5.4/Corollary5.5, cycle section5.3.2 and Example10.27
context. The book is not marked fully read. The event-distance normalization
is the paper's `[0,1]` convention; no equality to Mathlib's differently
normalized general `tvDist` API is asserted. [pcc:unverified]

Main declaration: `WeightedCycleCouplingAudit.weighted_cycle_certificate`,
in [WeightedCycleCertificate.lean](lean/WeightedCycleCertificate.lean).
See [proof map](proof-map.json), [source plan](sources.json),
[source review](evidence/source-review.json) and [visual report](reports/report.pdf).
Exact local **PCC-20261114-c938e8** replay PASS at 2026-11-14T10:13:48Z; signed standing remains unverified. See [verification receipt](verification.json).
[pcc:unverified]

```sh
tools/lean/proof-gate.sh research/weighted_cycle_coupling_audit_20261114/lean
python3 research/weighted_cycle_coupling_audit_20261114/restore_attempts.py /tmp/weighted-cycle-builds
```

Wholegate59 declarations/zero failures/zero trusted native exceptions.
All102 build/processing/provenance files retained losslessly, including
failed source/config/log/exit states and the compiler-panic diagnostic.
Lean4.35.0/Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679 pinned;
standard foundations/default limits. No holes, custom axioms, native evaluator,
raised limits, new global elaboration flags or linter waivers. KB/Conductor
unavailable, bounded primary/prior/official fallback before code recorded.
Static rho, the stronger random-support exponent, collision constants and
other fourteen-item gaps remain open. No full14 closure, native experiment,
paid compute or agent upload. [pcc:unverified]
