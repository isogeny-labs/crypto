# CT58 — Uniform cyclotomic obstruction to the full-torsion-field route

**Status:** [pcc:unverified]. Classical Weil-pairing theorem applied to the CT55 full-division-field construction, with independently implemented finite controls. This is not a new ECDLP attack, a general impossibility theorem for cross-characteristic transfers, or a measured speedup.

## Uniform theorem: complete torsion basis

Let \(\ell\) be an odd prime, let \(A/K_0\) be a *principally polarized* abelian variety of positive dimension over **any** number field, and let \(L/K_0\) be an extension over which all of \(A[\ell]\) is rational. The Weil pairing and the principal polarization give a nondegenerate Galois-equivariant alternating pairing

\[
e_\ell:A[\ell]\times A[\ell]\longrightarrow \mu_\ell.
\]

Choose two \(L\)-rational torsion points whose pairing is a primitive \(\ell\)-th root of unity. Since the pairing is defined over \(L\), that root belongs to \(L\). Consequently,

\[
\boxed{\mathbb Q(\zeta_\ell)\subseteq L,\quad [L:\mathbb Q]\geq \ell-1.}\tag{CT58-A}
\]

The conclusion holds **even if the number field, global elliptic curve, or Jacobian varies with the ECDLP input**, and it needs neither full Galois image nor Mazur's theorem. It applies to Jacobians with canonical principal polarizations and to any polarization with degree prime to \(\ell\).

In particular, CT55's explicit choice \(L=\mathbb Q(\widetilde E[\ell])\) cannot have field degree below \(\ell-1\) *regardless of how it engineers \(\widetilde E\)*. This improves the applicability of CT52/53's fixed-variety full-image bounds, but is weaker numerically under full Galois image.

### Binary residue-degree constraint

If \(A\) has good reduction at a place \(w\mid2\) of \(L\), with residue field \(\mathbb F_{2^f}\), prime-to-characteristic torsion reduction and the reduced Weil pairing give

\[
\boxed{\ell\mid 2^f-1,\quad \operatorname{ord}_\ell(2)\mid f.}\tag{CT58-B}
\]

This is equally seen from the symplectic multiplier of Frobenius: an operator fixing a complete torsion basis must have multiplier 1, while binary Frobenius's multiplier is 2. It applies in every genus under the pairing hypotheses, not just the CT55 supersingular target. CT55's specific formula \(f_{\rm full}=2\operatorname{ord}_\ell(-2)\) is compatible with the general divisibility constraint.

| Prime \(\ell\) | Minimum absolute field degree for full basis | Minimum binary residue degree |
|---:|---:|---:|
| 5 | 4 | 4 |
| 11 | 10 | 10 |
| 43 | 42 | 14 |
| 163 | 162 | 162 |
| 257 | 256 | 16 |

These are **necessary** constraints; the table does not claim existence of a variety attaining any lower bound. No asymptotic claim about primes for which 2 is a primitive root is made.

### Concrete control: globally rational point vs entire basis

The earlier rational 5-torsion example on

\[
E/\mathbb Q:y^2+y=x^3-x^2-10x-20
\]

has \(P=(5,5)\in E(\mathbb Q)[5]\). Direct counting gives \(\#E(\mathbb F_2)=5\). Hence a nonzero 5-torsion point is already rational over \(\mathbb F_2\). However \(t=2+1-5=-2\), and modulo 5 the Frobenius characteristic polynomial factors

\[
X^2+2X+2=(X-1)(X-2).
\]

Thus exactly one 5-dimensional-over-\(\mathbb F_5\) **line** is fixed by Frobenius over \(\mathbb F_2,\mathbb F_4,\mathbb F_8\), giving five fixed points at each level, while all 25 torsion points are fixed over \(\mathbb F_{16}\). A separate point-count recurrence gives \(\#E(\mathbb F_{2^f})=5,5,5,25\) for \(f=1,2,3,4\).

This example makes the key limitation of CT58-A explicit: **one globally rational generator does not force the field of definition to contain \(\mu_\ell\)**; the **complete two-dimensional** torsion basis does. Two unknown-log inputs \(P,Q=[x]P\) are dependent, so their common single-point field is not ruled out by the cyclotomic theorem.

### Cost scope

For a *dense explicit degree-d number-field power basis*, storing a generic algebraic number requires d coefficient slots. CT58-A therefore forces \(\Omega(\ell)\) slots on this representation of a complete division field; it compares unfavorably with a generic \(\widetilde O(\sqrt\ell)\) rho group-operation count. **This is not a universal computational lower bound:** a compressed or tower field representation may evade dense-storage cost, and a group-operation count is not the same cost unit as a field coefficient. Any efficiency claim must explicitly price arithmetic, root selection, compatible prime ideals, orientation and destination solving.

## Reproducible independent arithmetic audit

Python uses an explicit Frobenius matrix modulo 5 and enumerates its fixed vectors; the separately implemented JavaScript uses its distinct eigenvalues and the Frobenius trace recurrence. Both compute exact \(\operatorname{ord}_\ell(2)\) for 15 primes, check the full-basis residue constraint, and exhaustively test finite GL2 symplectic/determinant identities at \(\ell=3,5,7\). The false assertion that the full 5-torsion basis is defined over \(\mathbb F_2\) is rejected. These controls do **not** prove the global Weil-pairing theorem.

```bash
base=research/cross_characteristic_global_torsion_20261112/ct58
python3 -B "$base/ct58_cyclotomic_gate.py" > /tmp/ct58-py.txt
python3 -O -B "$base/ct58_cyclotomic_gate.py" > /tmp/ct58-py-opt.txt
node "$base/ct58_independent.js" > /tmp/ct58-js.txt
node --no-opt "$base/ct58_independent.js" > /tmp/ct58-js-noopt.txt
cmp /tmp/ct58-py.txt /tmp/ct58-py-opt.txt
cmp /tmp/ct58-js.txt /tmp/ct58-js-noopt.txt
diff -u <(head -n -1 /tmp/ct58-py.txt) <(head -n -1 /tmp/ct58-js.txt)
```

Local four-mode test and cross-language numeric-output comparison passed. Python/JS final PASS annotation differs, intentionally excluded from the comparison. [pcc:unverified]

## Sources and governance

Primary mathematics: Anthony Snowden, [Lecture 2: Tate modules and Weil pairing](https://websites.umich.edu/asnowden/teaching/2013/679/L02.html) (determinant equals cyclotomic character); Martin Orr, [Weil pairings for polarized abelian varieties](https://www.martinorr.name/blog/2011/09/06/weil-pairings-the-skew-symmetric-pairing/). The mathematical implication is classical; no novelty claim.

Internal: root PR #3877 (native Jacobian work untouched), CT51 positive control, CT55 universal existential division-field route, CT56 Serre bound, CT57 rational-isogeny gate, `problems/lift-reduce-second-prime-transfer.md`, AGENTS.md and 2026-11-13 session log. Shared research KB MCP and repository wrapper unavailable in this runtime; GitHub searches across crypto, cryptanalysis and crypto-autoresearcher are not a novelty certificate. No EKS spending, full Rust/Lean suite, or signed PCC verification.

## Next discriminating proof/implementation obligation

Replace *full-torsion-field materialization* with a **single oriented cyclic subgroup lift** for arbitrary encoded \(P,Q=[x]P\), over a number field allowed to vary. Require a uniform input-to-root selection algorithm that does not know \(x\), compatible prime ideals above \(p\) and 2, algebraic point/field representation, coefficient heights, and end-to-end cold cost against matched rho. CT57 rules out a globally stable large-prime line on elliptic curves over \(\mathbb Q\); it does not rule out a single point over a varying number field, and CT58 does not either. [pcc:unverified]
