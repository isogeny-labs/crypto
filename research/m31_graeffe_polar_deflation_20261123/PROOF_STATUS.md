# Formal proof status and open obligations

Date: 2026-11-24 UTC. This supplementary status map accompanies
[MATHEMATICS.md](MATHEMATICS.md) and [REPORT.md](REPORT.md).
Every result below currently has a written argument only: **no Lean
declaration exists for this study, and none is formally checked**.
Native M31 development controls and separate-agent mathematical review do
not change that status. [pcc:unverified]

## Statement map

The RR5 convention is \(R=(r,s)\),
\(\ell=(X-r)(cX+d)+(s+y)(X+b)\), with divisor
\(\sum_i(P_i)+(-R)-5(O)\). Thus the signed points \(P_i\), whose
abscissae are the roots of \(U\), sum to \(R\).

| Written statement | Exact hypotheses and required bridges | Lean declaration / status |
| --- | --- | --- |
| RR5 tangent identification; MATHEMATICS Sections 1–3 | Characteristic \(p>3\); nonsingular short Weierstrass curve; fixed finite nonidentity target; the normalized RR5 function above; four distinct original roots \(x_i\ne r\); \(V(x_i)\ne0\), \(y_i\ne0\), and \(\phi'(x_i)\ne0\). The norm/divisor argument gives the fixed signed sum, injectivity of the three-parameter root differential, and \(\operatorname{im}(dz)=n^\perp\), where \(n_i=(2y_i\phi'(x_i))^{-1}\ne0\). | No Lean declaration; not formalized. The RR5 norm/divisor and invariant-differential bridges remain proof obligations. |
| Centered \(e_3\) polar lemma; Sections 2 and 4 | A polynomial or formal local map \(z\) from three parameters to four coordinates, \(z(\theta_0)=0\), characteristic \(p>3\), and differential of rank three with image \(n^\perp\), all \(n_i\ne0\). The residual must be \(F_3=e_3(z)\), using total parameter derivatives. For the terminal quartic its exact expression is \(-g_1-2\tau g_2-3\tau^2g_3-4\tau^3\). The six Hessian entries vanish and their Jacobian has rank three. | No Lean declaration; not formalized. This is the central finite polynomial/linear-algebra lemma selected for a future formalization attempt. |
| Regular-chart rank and root preservation; Sections 4–5 | Retain all four original centered residual equations and append all six total second derivatives. Use the admitted RR5 chart with \(\Delta=\operatorname{disc}(U)\ne0\), a fixed squarefree split selector, target abscissa outside that selector, and the nonzero-ordinate and tangent conditions above. The local order-three argument proves vanishing of the added equations at every original regular root. No distinctness condition is imposed on intermediate Dickson images. | No Lean declaration; not formalized. The derivative-only system is not asserted to have the intended global root set. |
| Saturated ideal equality; Section 5 | The preceding admission and exact RR5 chart, a finite set of signed selector divisors with unique normalized parameters, and the written reducedness argument for the full remainder ideal \(I_{\rm rem}\). If \(J\) contains the four collapsed equations and six Hessian equations, the stated conclusion is \(J:\Delta^\infty=I_{\rm rem}\). It concerns the regular open chart; the unsaturated repeated-root components are excluded from the claim. | No Lean declaration; not formalized. Finiteness, local reducedness, saturation, and field-extension contraction all require formal bridges. |
| Optional \(e_2\)-gradient rank criterion; Section 6 | The same three-dimensional tangent image \(n^\perp\) in four dimensions, with characteristic \(p>3\). Keep \(F_1\) and the three first total derivatives of \(F_2\). Their Jacobian has rank two exactly when \({\bf1}\cdot n=n\cdot n=0\), and rank three otherwise. | No Lean declaration; not formalized. The fixed synthetic M31 control is a separate finite check. |
| Conditional \(m\)-leaf extension; Section 8 | \(m\ge3\), characteristic \(p>3\), and \(p\nmid(m-2)\); a local map with \(z(\theta_0)=0\), differential of rank \(m-1\), and image \(n^\perp\) with every \(n_i\ne0\). All \(\binom m2\) second partials of \(e_3(z)\) then have joint Jacobian rank \(m-1\). An applicable elliptic chart and its rank must be established separately. | No Lean declaration; not formalized. No longer-sum implementation, native qualification, or extension of four-leaf automatic rationality is supplied. |

The full field/base admission in the mathematics note supplies the local
hypotheses for the intended M31 instance: squarefree split selector,
odd full rational curve order, target exclusion, and distinct original
abscissae. These assumptions must remain visible in any eventual formal
theorem or application theorem.

## Actual tooling inspection

The read-only inspection found:

- PATH lookup returned no executable for Lean, Lake, or Elan.
- The usual user Elan Lean/Lake paths and the inspected system Lean paths
  were absent.
- No crypto-lean checkout was found in the bounded shared-workspace
  layouts inspected. This is a local inventory observation, not a claim
  that the repository or a remote execution environment is unavailable.
- The repository-wide expected toolchain in
  [tools/lean/lean-toolchain](../../tools/lean/lean-toolchain) is
  leanprover/lean4:v4.35.0.
- This study has no pinned crypto-lean revision, Lake dependency closure,
  theorem modules, or proof-build receipt. The expected toolchain version
  alone does not supply those dependencies.

**No Lean build or proof attempt was performed, and no Lean build or
theorem proof failed.** The observed local tooling/dependency gap blocks
an immediate checked build in the inspected environment. It is not a
mathematical obstruction or a demonstrated inability to bootstrap a
working environment. No formalization attempt should be inferred from
the successful Rust controls.

## Feasibility and next formal step

The central polar lemma is a finite polynomial and linear-algebra
argument and appears mathematically amenable to formalization. That is a
feasibility assessment, not a completed Lean attempt. The full application
also needs the geometric and local-ring bridges listed above; no claim is
made that a particular missing Mathlib or crypto-lean declaration has
already been identified by a complete library audit.

The formalization obligation under
[AGENTS.md](../../AGENTS.md), “Mathematical results should include Lean
proofs,” remains open. Before any future Lean source edit, apply the
required lean-instance-gate: inspect existing structures and results,
record the inheritance table, and pin the crypto-lean revision and its
toolchain/dependencies. Then build the actual theorem modules, retain
the commands and outputs, inspect the transitive axioms of every claimed
declaration, and use the repository proof gate and fresh kernel checker
when available.

The smallest mathematical target is the conditional polar lemma with all
its rank, characteristic, and nonzero-normal hypotheses explicit.
Formalizing that lemma alone would be partial progress; it would not
establish the RR5 tangent bridge, saturated ideal equality, or the full
application theorem. Preserve that distinction in the statement map.
