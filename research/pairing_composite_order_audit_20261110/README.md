# Composite order exposes a missing Tate root hypothesis

Lean refutes the literal composite-order nondegeneracy statement in Galbraith,
*Mathematics of Public Key Cryptography*, Theorem 26.3.3, when its stated
nontrivial-root condition is read literally. Two nonsingular curves satisfy
coprimality, divisibility of the actual curve order and the existence of a
nonidentity nth root, yet no map on the stated domains can be both bilinear
in its left argument and left nondegenerate. [pcc:unverified]

| Field and n | Actual curve | Actual curve / left / right counts | Actual point P | Nontrivial nth root |
|:--|:--|:--|:--|:--|
| F3, n=4 | y²=x³+x+1 | 4 / 4 / 4 | (0,1), order 4, 2P=(1,0)≠O | −1, exact order 2 |
| F5, n=6 | y²=x³+1 | 6 / 6 / 6 | (0,1), order 3, 2P=−P≠O | −1, exact order 2 |

These counts and points are Mathlib's actual nonsingular elliptic-curve
objects. The left domain is the kernel of multiplication by n; the right
is the actual curve group modulo its n-multiple subgroup. The target is
actual field units modulo the image of the nth-power homomorphism.
No substitute group table or assumed canonical-pairing implementation is
used. Both target quotients are nontrivial and every target element squares
to one. [pcc:unverified]

## Why the statement fails

For a left-bilinear map e into an exponent-two multiplicative target,
e(P+P,Q)=e(P,Q)e(P,Q)=1 for every Q. Thus any nonzero double lies in its
left radical. Each actual n-torsion domain above has such a double.
The generic theorem proves this obstruction for every right domain;
the concrete modules specialize it both to E/nE and to the unquotiented
rational point group used in the source's quantifiers. [pcc:unverified]

The main declaration directly negates the universal implication from the
source conditions to the existence of such a map. Separate kernel proofs
show that primitive fourth roots are absent in F3 and primitive sixth roots
are absent in F5, while the exhibited roots are primitive second roots.
These are controls for the distinction between nontrivial and primitive.
A primitive-nth-root-qualified nondegeneracy theorem and a prime-order
pairing theorem are not refuted by these examples. [pcc:unverified]

## Proof map

| Namespace / declaration | Exact checked obligation |
|:--|:--|
| `CompositeTateObstructionAudit.nonzero_double_obstructs` | Any exponent-two target kills all doubles under left bilinearity |
| `CompositeTateObstructionAudit.TateLeft`, `TateRight`, `TateTarget` | Actual torsion kernel, curve quotient and field-unit quotient |
| `CompositeTateThreeAudit.curve_cardinality`, `P_order` | Actual F3 curve count 4 and point order 4 |
| `CompositeTateFiveAudit.curve_cardinality`, `P_order` | Actual F5 curve count 6 and point order 3 |
| Both concrete namespaces: `left_cardinality`, `right_cardinality` | Actual domain counts, using Lagrange and quotient equivalences |
| Both: `source_hypotheses`, `primitive_root_absent`, `primitive_two_control` | Literal source hypotheses and exact root-order controls |
| Both: `target_square`, `target_nontrivial` | Actual quotient exponent divides two and quotient is nontrivial |
| Both: `no_left_nondegenerate_bilinear_pairing`, `no_left_nondegenerate_on_curve` | Impossibility on quotient and unquotiented right domains |
| `CompositeTateCertificateAudit.literal_nontrivial_root_nondegeneracy_claim_false` | Direct negation of the universal literal-source implication |
| `CompositeTateCertificateAudit.composite_tate_audit_certificate` | Combined source conditions, actual counts/orders, root controls and both disproofs |

## Source fidelity and scope

The [official chapter](https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch26.pdf),
printed pages 576–577, supplies general natural n, gcd(n,q)=1, n dividing
#E(Fq), the actual quotient domains and the nontrivial-root condition.
Those two pages were visually inspected; section 26.3's definitions and
Theorem 26.3.3 were read. The later prime-order restriction is distinct.
The current official errata list was checked for chapter-26 entries; none
was present in that bounded list. This is no novelty claim. [pcc:unverified]

The original Frey–Rück paper has not been read: direct AMS PDF access failed.
No error in its theorem is asserted. The corpus node already names correct
domains and fields; this source-specific counterexample does not turn every
qualified use of that node into a false statement. Actual canonical Weil/Tate
constructions, correctly qualified nondegeneracy and MOV/Frey–Rück computations
remain open requirements. [pcc:unverified]

Lean 4.35.0 / Mathlib 56a005855ea17e0b0958f73ec801ae140b0a1679 are pinned.
All accepted proofs use only propext, Classical.choice and Quot.sound; the
generic double obstruction needs no axioms. No holes, custom axioms,
native_decide, linter waiver or added elaboration setting is admitted.
Failed quotient coercion and record syntax attempts remain preserved.
No native experiment, sampling, timing or paid compute is claimed.
[pcc:unverified]

This study proves group-theoretic impossibility, not a polynomial equality
over the identity-certificate schema's fixed 61-bit prime. That schema
cannot express the field-specific quotient domains or nondegeneracy
quantifiers. The faithful proof artifact is the Lean theorem binding above.
KB and Conductor were unavailable; primary source, repository and bounded
Mathlib declaration reads were used instead. [pcc:unverified]

```sh
cd research/pairing_composite_order_audit_20261110/lean
lake build
cd ../../..
tools/lean/proof-gate.sh research/pairing_composite_order_audit_20261110/lean
```

Source commit `0b81d51a` precedes `PCC-20261110-5e63e9`. Full local
verification passes at 11:31:14Z; signed standing is unverified. Whole gate:
50 public declarations, zero failures and zero trusted-native exceptions.
The source plan and actual verdict are in [sources.json](sources.json) and
[verification.json](verification.json). All 22 attempt files and five gate
files have byte-equal archive roundtrip checks. [pcc:unverified]

See the [two-page report](reports/proof-20261110/report.pdf),
[editable diagram](reports/proof-20261110/proof.dot) and
[rendered diagram](reports/proof-20261110/proof.svg). Both PDF pages and the
diagram were visually inspected. Canonical IC graphs keep their measured
performance scope: no new speed, rank, cost or isogeny edge is asserted.
[pcc:unverified]
