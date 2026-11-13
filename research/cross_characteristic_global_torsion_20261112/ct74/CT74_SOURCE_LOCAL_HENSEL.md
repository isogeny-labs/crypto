# CT74 — Logarithm-free *source-local* torsion lifting; not yet a two-prime transfer

**Scientific standing: [pcc:unverified].** An explicit source-place inverse-specialization component and a negative control, not a global source-to-binary map or an ECDLP speedup. The local lifting theorem is classical (formal groups and prime-to-residue-characteristic étale torsion).

## Precise theorem and algorithm

Let \(p\) be an odd prime, \(E/\mathbb Z_p\) an elliptic curve with good reduction, \(N\geq1\) and \(\ell\geq2\) coprime to \(p\). For every encoded \(P\in E(\mathbb F_p)[\ell]\), let \(R\in E(\mathbb Z/p^N\mathbb Z)\) be **any smooth lift** of \(P\). Choose \(u\) satisfying

\[
u p^{N-1}\equiv1\pmod\ell,\qquad m=u p^{N-1}.
\]

Then \(T=[m]R\) is the **unique** \(\ell\)-torsion point of \(E(\mathbb Z/p^N\mathbb Z)\) reducing to \(P\). This procedure needs the public point coordinates, the curve, \(p,\ell,N\), and arithmetic over the finite local ring, but **does not need** \(\log_PQ\). On \(\ell\)-torsion it defines a group-homomorphic section of reduction; applying the same procedure to \(P,Q\) retains \(Q=[x]P\). [pcc:unverified]

**Proof.** Smoothness gives a lift \(R\). The kernel \(E_1\) of reduction has formal-group filtration \(E_i\) with \([p]E_i\subset E_{i+1}\); hence \([p^{N-1}]E_1=0\) in \(E(\mathbb Z/p^N\mathbb Z)\). Since \([\ell]R\in E_1\), \([\ell]T=[m][\ell]R=0\). Moreover \(T\bmod p=[m]P=P\) because \(m\equiv1\pmod\ell\). If two \(\ell\)-torsion lifts reduce to \(P\), their difference lies in the \(p\)-group \(E_1\) and is killed by \(\ell\), so it is zero. This proves uniqueness, independence of the arbitrary smooth lift, and additivity. With ordinary efficient group arithmetic, the scalar \(m\) has \(O(N\log p+\log\ell)\) bits. **No implementation or runtime measurement of the general [m] finite-ring method is claimed here**; the bounded implementation below uses an \(\ell=3\) division-polynomial shortcut. [pcc:unverified]

This removes the *source-local* existence and orientation-selection obstacle from CT73 under good reduction. It **does not** specify a global algebraic number field representative, an embedding at characteristic two, or an efficient compatible two-prime specialization. Selecting a target image from only the local \(p\)-adic residue expansion remains an additional task.

## Fixed public witness and two independent executable procedures

Reuse CT69–73's original integral model \(E:y^2+y=x^3-x^2-10x-20\), discriminant \(-11^5\), prime \(p=29\), and public order-three point \(P=(18,5)\). Its three-division polynomial is

\[
\psi_3(X)=3X^4-4X^3-60X^2-237X-21,
\quad \psi_3(18)\equiv0\pmod{29},\quad \psi_3'(18)\equiv15\pmod{29}.
\]

**Python** Hensel-lifts the simple root of \(\psi_3\) by Newton inversion modulo \(29^N\), independently lifts both ordinates, verifies the curve, negation, and \([2]T=-T\). **JavaScript** does not evaluate the division polynomial: it scans all 29 possible next \(x\)-digits, uses the curve equation to choose the unique ordinate digit, and tests \([2]T=-T\) via the tangent formula. Both derive the lift from encoded \((18,5)\), not a hidden logarithm or precomputed target table. [pcc:unverified]

The two computed lifts agree at six precisions, with the highest precision result

\[
N=6,\quad 29^6=594823321,\quad
T=(25104680,425642517),\quad
-T=(25104680,169180803).
\]

The full checks include reduction to \((18,5)\) and \((18,23)\), coordinate and tangent identities, all six \(N=1,\ldots,6\), and uniqueness across 29 initially perturbed \(x\)-digits at \(N=2\). **Negative control:** retaining \(x=18\) and Hensel-lifting only the curve equation produces a legitimate integral point that is **not order three** for every \(N\geq2\). Thus naive integer coordinate lifting, despite being on the curve, is not a torsion-preserving construction. [pcc:unverified]

| Precision | Lifted x | Lifted y | Naive lift has order 3? |
|---:|---:|---:|---|
| 1 | 18 | 5 | yes |
| 2 | 830 | 643 | no |
| 3 | 8399 | 5689 | no |
| 4 | 349845 | 566636 | no |
| 5 | 4593531 | 15419537 | no |
| 6 | 25104680 | 425642517 | no |

Local four-mode comparison, with normal Python, optimized Python, normal Node and Node --no-opt, was byte-identical. The common six-line JSONL SHA-256 is `b459b8be46bf3b7e9cc8af7ecc85d732e4e5fc5d51dfcb27773fae0a22d799e4`. This is producer-local execution, **not** independent signed PCC verification. [pcc:unverified]

```bash
D=research/cross_characteristic_global_torsion_20261112/ct74
python3 -B "$D/ct74_local_torsion.py" > ct74-py.jsonl
python3 -O -B "$D/ct74_local_torsion.py" > ct74-pyopt.jsonl
node "$D/ct74_independent.js" > ct74-js.jsonl
node --no-opt "$D/ct74_independent.js" > ct74-jsopt.jsonl
cmp ct74-py.jsonl ct74-pyopt.jsonl
cmp ct74-py.jsonl ct74-js.jsonl
cmp ct74-py.jsonl ct74-jsopt.jsonl
echo 'b459b8be46bf3b7e9cc8af7ecc85d732e4e5fc5d51dfcb27773fae0a22d799e4  ct74-py.jsonl' | sha256sum -c
```

## Literature, proof obligations, costs and exclusions

**Sources checked:** CT69–73 research reports and PR #4015, root #3877, concurrent cryptanalysis #1523, their repo instructions, and A. Snowden, [*Elliptic curves over DVRs*, Lecture 8](https://websites.umich.edu/asnowden/teaching/2013/679/L08.html) (finite-étale prime-to-p torsion specialization), plus [*The elliptic sieve and Brauer groups*, §2](https://londmathsoc.onlinelibrary.wiley.com/doi/full/10.1112/plms.12520) (formal-group filtration). The research-KB MCP server and its repository CLI wrapper and Conductor command were unavailable in this connector-only runtime; no novelty claim or exhaustive prior-art claim is made. [pcc:unverified]

**Unclosed work:** independent kernel/signed mathematical certification; an actual general finite-ring projective/formal-group implementation of \([m]\); global algebraic recognition of the input-dependent \(p\)-adic torsion lift (including field height and degree); one compatible characteristic-two residue embedding for **both** encoded points; independent end-to-end DLP and optimized rho/index-calculus comparison. Producing the finite-local-ring lift is not producing a characteristic-two point. The toy order-three case is too small to infer a scalable cost advantage. No paid EKS, GPU, or protected merge. [pcc:unverified]
