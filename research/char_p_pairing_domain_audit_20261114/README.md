# Characteristic-order pairings: actual domains and failed strengthenings

Lean proves why the field-characteristic exclusion matters for ordinary
elliptic-curve point groups. The targets and domains here are actual
field units, their power quotients, Mathlib curve-point kernels and curve
quotients. No assumed pairing table or supplied nondegeneracy premise is
used. The qualified primary Weil/Tate theorems remain intact.
[pcc:unverified]

For every prime p, every characteristic-p field and every natural k,
the only unit satisfying u^(p^k)=1 is the identity. Over every **finite**
such field, the unit p^k-power homomorphism is surjective and its actual
quotient has cardinality one. Frobenius injectivity and finite
injective-to-surjective conversion supply these proofs. Any left-bilinear
map from an additive group killed by p^k into units of a characteristic-p
field is identically one; a nonzero left element excludes left
nondegeneracy, even before alternation or right bilinearity is required.
The target field can be an arbitrary field of that characteristic,
including an algebraic closure. [pcc:unverified]

For every finite additive commutative group, Lean proves the quotient by
n-multiples has the same cardinality as the n-kernel, using the actual
endomorphism range and subgroup index. This specializes to every actual
finite curve group E(K). [pcc:unverified]

- `actualLeft W n` is the actual kernel of multiplication by n.
- `ActualRight W n` is E(K) modulo its actual n-multiple subgroup.
- `PowerTarget K n` is K* modulo the actual nth-power image.

These are the point-group domains used in the preceding source audit.
Canonical divisor/valuation constructions are not identified here.
[pcc:unverified]

For every finite characteristic-two field and nonsingular general
Weierstrass model, the actual left and right domains for n=2 each have
cardinality one if a1=0 and two otherwise; the target has cardinality one.
Every ordinary model therefore excludes any left-bilinear left
nondegenerate map on the proper Tate domains and any unit-target candidate
over any characteristic-two field. The credited actual model
`y²+xy=x³+1` has an explicit rational point (0,1) of exact order two over
every characteristic-two field, and over every finite one its domain
sizes are **2 / 2 / 1**. Lean directly negates the universal unit-pairing
existence statement with nonsingularity, prime order and divisibility of
the actual curve order after deleting coprimality. [pcc:unverified]

For every finite characteristic-three field, every nonsingular ordinary
cubic `y²=x³+a*x²+b*x+c` with a≠0, and every nonsquare twist parameter d,
the prior constructed nonzero three-point lies on the curve or its actual
partner. On that partner the actual domain sizes are **3 / 3 / 1**, and
both unit and proper Tate nondegeneracy are impossible. The result does
not assume three-torsion is rational on the original curve. The credited
F3 partner `y²=x³+x²+2` has actual point (1,1) of exact order three and
supplies a named instance. Prior actual geometric cardinality, twist
construction, models and group proofs are credited. [pcc:unverified]

An imperfect-field control checks the finite-field target step. Over
F2(X), the actual X-unit has a nonidentity class modulo squares; its target
quotient does not have cardinality one and the square homomorphism is not
surjective. The prior integer-degree proof that X is nonsquare is reused.
Nevertheless every unit square-root of one is still one. Lean directly
negates the universal trivial-target claim after dropping finiteness.
This separates injectivity, valid over all fields here, from surjectivity.
[pcc:unverified]

Source fidelity is explicit. [Galbraith author chapter26](https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch26.pdf),
Definition26.2.2 and Theorem26.2.3, impose coprimality for the Weil pairing.
Exercise26.3.1 and Theorem26.3.3 specify the Tate quotient domains,
coprimality and a root hypothesis. Printed574–577/PDF2–5 were read and
visually inspected. Lean additionally proves that a **prime-order
nonidentity root** in a finite field already forces coprimality with the
field cardinality. Thus the characteristic-order controls do not satisfy
that root hypothesis. The separate direct negation for Tate concerns an
**unqualified** prime-order/divisibility claim that lacks both the
coprimality and root hypotheses; it is not a counterexample obtained by
removing only coprimality while retaining the prime-root condition.
[pcc:unverified]

The earlier literal composite-order/nontrivial-root source counterexamples
remain separate. These point-group statements do not refute perfect
pairings of group schemes involving nonreduced roots of unity. General
canonical Weil/Tate/MOV/Frey–Rück constructions, qualified nondegeneracy,
End/Tate-module bridges and the other fourteen-item gaps remain open.
No full chapter/book reading, novelty claim or corpus promotion is made.
KB and Conductor were unavailable; explicit bounded primary/prior/official
fallback preceded selecting the approach and implementation.
[pcc:unverified]

The main declaration is
`CharPPairingDomainAudit.characteristic_pairing_domain_certificate` in
[CharPPairingCertificate.lean](lean/CharPPairingCertificate.lean).
The [proof map](proof-map.json), [source plan](sources.json),
[source review](evidence/source-review.json) and
[visual report](reports/report.pdf) retain exact scope.
Exact local **PCC-20261114-f7faf2** replay PASS at 2026-11-14T08:22:13Z; signed standing remains unverified. See [verification receipt](verification.json).
[pcc:unverified]

```sh
tools/lean/proof-gate.sh research/char_p_pairing_domain_audit_20261114/lean
python3 research/char_p_pairing_domain_audit_20261114/restore_attempts.py /tmp/char-p-pairing-builds
```

Lean4.35.0/Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679 are pinned.
Failed source/config/log/exit states are retained. Standard foundations and
default limits only; no holes, custom axioms, native evaluation, raised
limits, new global elaboration flags or linter waivers. No new native
experiment, paid compute or agent storage upload. [pcc:unverified]
