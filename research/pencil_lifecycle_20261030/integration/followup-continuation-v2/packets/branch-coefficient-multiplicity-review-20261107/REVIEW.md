# Independent coefficient-data multiplicity source review

[pcc:unverified] T-1734. Source fidelity: faithful_scoped. Machine status: not_verified. Overall review: inconclusive pending compilation and transitive axiom readback.

The new declaration `BranchCoefficientMultiplicity.source_support_base_multiplicity_sum` connects the actual common-2+2 coefficient expressions to the existing three-support base multiplicity theorem. It removes the caller-supplied norm identity from this specialized interface. The conclusion retains all three support nonzero facts and the sum of their root multiplicities at the branch point equal to one.

The exact candidate SHA256 is `4e8cfd2ff984359b4fbd92e8fd0432db04b82e08fd19223eb5dac9d718995b76`. The preread plan SHA256 is `86374165d7d26279dfc6cf1d00582df6764c42cac6976454c2ea6cc65191ec11`. Seven primary source identities were checked and retained in `source-bindings.json`. No sibling review verdict was read. The producer's input manifest references an earlier review, but that review was not opened.

## Scope and proof joints

The type quantifies over an arbitrary field K, the normalized quintic coefficients, a branch root a and two pairs of degree-two support/degree-one ordinate coefficients. It retains hroot, global separability, a nonzero coefficientData determinant, both divisor congruences and vanishing of the three-support product at a. It does not assume the conclusion, support nonzero facts, finite valuation, or a generic norm identity.

1. `derivativeAt_ne_zero_of_separable` takes exactly the same quintic coefficients, a, separability and branch-root hypotheses. It supplies the simple-root condition required by the generic multiplicity theorem.
2. The numerator, firstSupport, secondSupport and residual arguments match the definitions used by `source_norm_identity`; the selected scalar is precisely coefficientData's first component. The identity orientation is `n^2-u1*u2*U=C(b^2)*f`, as required by the generic theorem.
3. The generic theorem returns three nonzero supports followed by the same left-associated three-term sum equal to one. The wrapper transfers the whole conjunction unchanged, including the residual nonzero fact.

This is source inspection of a direct application, not successful Lean elaboration. The import chain must actually expose `ActualNonbranchNorm` and `WeightedProjQuinticHead`; the inspected BranchRootMultiplicity -> BranchNormTransport -> Nonbranch source chain is consistent with that use. Full import closure, toolchain qualification and elaborated types were not checked here.

## Independent hand derivation and adverse cases

The preread derivation predicted specialization by applying the generic multiplicity theorem to an independently supplied actual norm identity. The candidate has three supports rather than the schematic two-factor shorthand used in that plan; the source review explicitly checks all three and their association. No blind derivation of the inherited valuation theory is claimed.

A concrete paper nonvacuity witness is K=Q, f=X^5-X, a=0; (q1,r1,s1,t1)=(0,1,0,0), (q2,r2,s2,t2)=(0,-1,0,0). Then u1=X^2+1 and u2=X^2-1 both divide f with ordinate zero. The five roots of f are distinct over the algebraic closure, so f is separable. Direct substitution in coefficientData gives (B,A,C,Q,Rbar)=(-4,0,0,-16,0), hence numerator=0 and residual=-16X. Thus the product vanishes at zero and the three multiplicities are (0,0,1). The norm identity is 0-(X^2+1)(X^2-1)(-16X)=16(X^5-X). This is an explicit mathematical witness, not an executed or kernel-checked fixture.

The zero residual case is not admitted by replacing its multiplicity with the convention rootMultiplicity(0)=0: the inherited theorem first proves each support nonzero from a finite stalk order sum. Equal support inputs make the determinant zero and fail hb. The inseparable example f=X^5 at a=0 over Q fails hs; it cannot supply the simple-root bridge. There is no added characteristic-not-two premise in this wrapper; its field generality is inherited from the supplied branch theorem, not newly proved here. Singular branches, hb=0 and points outside the support product are outside this declaration's domain.

## Evidence boundary and next check

No compiler, theorem evaluator, run or controls were launched; the campaign deadline had expired. `#print axioms` in the text is an audit request, not audit output. The result is a supported source composition with kernel verification outstanding. Compile this named declaration together with the exact dependency closure and obtain separate transitive axiom readback and a formalized nonvacuity control under a renewed authorized execution envelope. No measurements, cohort outcomes, signed-recovery property or scientific status transition follow from this packet.

At review time branch HEAD was `8c7fd2020c991c158f2e70bf557783fb1a4dd70c` and fetched main was `3a5e683e3749eb108a1573cc251352d7001ad4cb`. Root's read-only merge-tree assessment found only the session-log conflict; Copilot resolution was pending. Source review proceeded against exact pinned primary inputs without editing the producer snapshot or ledger. Candidate publication receipt is recorded separately after root publication; until then the review has no immutable GitHub candidate binding.

Publication follow-up: candidate commit `6a803d2d9d74956e91881575bc3dbc023dd2573a` was independently read back from origin at 00:29:54 UTC on November 7. Its committed source blob hashes to the reviewed candidate hash above.
