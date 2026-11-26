# Conditional algebra, inherited laws and native boundary

PCC09b3cb binds four explicit declaration hashes, source/configuration and
INSTANCE_GATE.md. CryptoLean5f2ca251cb34806f69fd5455175434cac3567dfa,
Mathlibcfc55fc58714a59dc06e6e79edc6d1d95140d3c2 and Lean4.35.0 stay pinned.
Assembly.lean applies existing monic-division/evaluation laws, then checks
monicity of the actual coefficient expression using existing degree laws.
No new generic division/group law or custom field wrapper is introduced.

| Claim | Lean declaration, namespace ReceivedIdeal | Exact boundary |
| --- | --- | --- |
| Received quartic vanishes | received_quartic | CommRing; explicit raw quadratic, curve and w=-uv equations |
| Denominator-cleared expression vanishes | cleared_quartic | CommRing; J*beta=B and quartic zero hypotheses |
| Raw branch satisfies quadratic | raw_branch_quad | Field; tau!=0,2!=0,epsilon^2=1 |
| Cleared quartic factors by branches | branch_factorization | Polynomial identity over CommRing; no irreducibility assertion |

The focused project gate audits4named endpoints/0trusted. Local PCC verify
passes allfour when invoked in the pinned Lake environment, which supplies
Mathlib search paths for these source-file bindings. Bare CLI invocation
failed to build imports; its JSON is retained separately. No claim, verifier,
keyring or gate policy was changed to obtain the configured-environment pass.
All foundations are propext/Quot.sound, plus Classical.choice for the guarded
field calculation. No sorry/native_decide/custom axiom/limit override remains
in final source. No lean4checker executable was available.
[pcc:PCC-20261123-09b3cb]

Readable algebra: divide u^5+A*u^4+u^3 by u^2+A*u+beta. The quotient is
u^3+(1-beta)*(u-A), and remainder L=(1-beta)*((A^2-beta)*u+A*beta).
Use w^2=L. With C=(1-beta)*(A^2-beta), E=A*beta*(1-beta) and
c2=A*(1-beta)*(A^2-3*beta), the quartic equals
(w^2-L)*(w^2+L+c2)+C^2*(u^2+A*u+beta). Clearing via B=J*beta gives J^5
times the quartic. These are curve-coefficient instance obligations using
existing ring normalization, not a new elimination algorithm.

Fidelity: the algebra source does not instantiate native BigUint arithmetic
as a Mathlib field or establish actual primality, norm-one subgroup, compiler,
group, rank, divisor/root count, selection or cost. Those are missing bridges,
not consequences of the polynomial identities. Nativefce173 supplies only
fixed producer controls and full local raw inventories. The bounded55/54
quotient is a vector-space complement, not complete ideal membership.
[pcc:unverified]
