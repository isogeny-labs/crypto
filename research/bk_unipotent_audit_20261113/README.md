# All-extension certificate for a BK excluded-boundary curve

For the actual nonsingular curve E/F7: y²=x³+3x+2, Lean proves
#E(F7)=9 and #E[3](F7bar)=9. All geometric 3-torsion lies in a chosen
field of size 7^k exactly when 3 divides k, for every positive k and
every compatible embedding. In contrast, 3 divides 7^k−1 for every k
and the cyclotomic embedding degree is 1. The whole gate passes
32 declarations, zero failures and zero trusted native exceptions.
Exact local replay of **PCC-20261113-da72ba** passes at 2026-11-13T19:04:16Z; signed standing remains unverified. See [verification receipt](verification.json). [pcc:unverified]

This is an all-extension counterexample to deleting BK's exclusion
r not dividing q−1, even with r² dividing the actual group order.
The original Theorem 1 retains that exclusion, and its Remarks already
warn that r² dividing #E does not repair deletion. This packet confirms
that mechanism in the existing F7 witness. The paper's printed example
uses a different curve over F19. No qualified source theorem is refuted.
[pcc:unverified]

The coordinate proof uses the actual group/S3 equivalence and factors
the literal 3-division quartic as 3(x−5)(x³+5x²+3x+2).
The rational branch has y=±3. On the cubic branch, y=±3x,
x^7=6x²+6x+3 and (6x²+6x+3)²=x+1. Two explicit small polynomial
factor identities prove these equalities by ordinary checked ring
arithmetic in characteristic 7. Actual point Frobenius cubed fixes every
3-torsion point; an existing actual nonfixed point forces period 3.
Finite-field point descent connects this to actual point-map images.
The geometric cardinality proof uses the existing arbitrary short-curve
theorem after checking the literal curve and discriminant. [pcc:unverified]

The closed `BKUnipotentAudit.all_extension_boundary_certificate` also
constructs embeddings of `GaloisField 7 k` into the actual algebraic
closure for every positive k, proves their field cardinality and checks
the criterion for every such embedding. For all n, fields of size
7^(3n+1) give actual-image failures despite the cyclotomic divisibility.
No geometric basis, determinant, Weil pairing or rationality conclusion
is assumed. The directory name does not claim a separately computed
Frobenius matrix. [pcc:unverified]

Reproduce in `lean/` with `lake build`, or run from repository root:
[pcc:unverified]

```sh
tools/lean/proof-gate.sh research/bk_unipotent_audit_20261113/lean
```

Lean 4.35.0 and Mathlib 56a005855ea17e0b0958f73ec801ae140b0a1679 are pinned.
No holes, custom axioms, native evaluation, raised limits, global elaboration
flags or waivers are introduced. Failed sources, logs and exit codes are
retained losslessly; failed builds are not proof evidence. No native
experiment, paid compute or agent storage upload is included. [pcc:unverified]

Sources: original BK1998 Theorem 1 and Remarks, pp142–143, reread in the
retained primary PDF/text; existing actual F7 witness, point/finite-field
Frobenius descent and arbitrary geometric 3-torsion cardinality; pinned
Mathlib field, polynomial and `GaloisField` APIs. KB is unavailable;
bounded primary/repository/library fallback, no novelty claim. The
[fourteen-item map](../theorem_census_20261109/README.md#original-fourteen-requirements)
retains generic all-prime rank/determinant/Weil and the complete
all-extension census as open. No corpus or leaderboard status is promoted.
[pcc:unverified]
