# CT21 — rational Richelot frontier after the F101 degree-seven gluing

Date: 2026-11-11. **[pcc:unverified]** Standalone arithmetic development checks, not a signed claim, executable Jacobian 7-map, PCC verdict, or solver speedup.

## Sources and nonduplication

Read crypto #3877, CT20/CT20b #3920/#3921, the CT20 gluing oracle and global-factor certificate, CT20_PLAN.md, EMERGENCY_RESUME_20261111.md, the research queue, and cryptanalysis #1483. This extends the *new* CT20 monic sextics, not the previously completed CT16 six-cycle target audit. Primary math: Magma's [Richelot isogeny documentation](https://magma.maths.usyd.edu.au/magma/handbook/text/1615), including the general cubic-algebra representation of Galois-stable quadratic kernels; Howe–Leprévost–Poonen, [Large torsion subgroups of split Jacobians](https://math.mit.edu/~poonen/papers/large.pdf), §3.2, for the underlying split-gluing. This does **not** claim a new Richelot construction. KB MCP absent in the connected tool inventory and research-kb CLI/repository checkout unavailable here; the lead atlas remains encrypted, so global novelty search is incomplete.

## Fixed inputs and explicit next targets

Use the two CT20 supported monic charts (not their nonsquare-leading variants):

- **B+A**: \(C_1:y^2=t^6+92t^4+44t^2+33\).
- **B+B**: \(C_1:y^2=t^6+26t^4+42t^2+68\).

The script independently factors each into two cubic factors \(g(t),-g(-t)\) and constructs the six roots as \(r,r^{101},r^{101^2},-r,-r^{101},-r^{101^2}\) in \(\mathbb F_{101^3}\). Their Frobenius permutation is \((0\,1\,2)(3\,4\,5)\).

There are exactly three Frobenius-fixed perfect matchings of the six roots: the opposite pairing \(((0,3),(1,4),(2,5))\) and the two cross pairings \(((0,4),(1,5),(2,3))\) and \(((0,5),(1,3),(2,4))\). For monic quadratic factors \(Q_i\), compute
\[
\Delta=\det(\operatorname{coeff}_{1,t,t^2}(Q_i))_{i=1}^3,\quad
H_i=Q'_{i+1}Q_{i+2}-Q_{i+1}Q'_{i+2},\quad
D:y^2=\frac{H_1H_2H_3}{\Delta}.
\]
The sign and quadratic twist are *tested* against \(\#C(\mathbb F_{101})\) and \(\#C(\mathbb F_{101^2})\), not inferred from a leading-coefficient heuristic.

| Source | determinant values, in pairing order | representative Richelot codomain \(y^2=\sum_{i=0}^6 c_it^i\), low to high |
|---|---|---|
| Original \(t^6+6t^4+13t^2+56\) (existing CT16 control) | \(0,38,63\) | \([64,83,42,84,13,7,77]\) |
| **CT20 B+A** | **\(0,28,73\)** | **\([7,15,20,20,95,24,82]\)** |
| **CT20 B+B** | **\(0,33,68\)** | **\([1,63,81,46,18,69,84]\)** |

For each row, the *other* nondegenerate pairing gives the polynomial with each odd-degree coefficient negated. Thus \(t\mapsto-t\) is a base-field curve isomorphism between the two codomain models. Consequently **each CT20 monic chart has one codomain curve-isomorphism class among its two nondegenerate rational Richelot pairings**. This does not mean that their kernels coincide.

The bounded script checks that all six candidate output sextics have irreducible degree six over \(\mathbb F_{101}\) using Rabin's irreducibility criterion, not merely a rational-root check. It independently counts \(120\) and \(10436\) curve points over \(\mathbb F_{101}\) and \(\mathbb F_{101^2}\). This yields \(L(T)=1+18T+279T^2+1818T^3+10201T^4\) and \( \#J(\mathbb F_{101})=12317=109\cdot113\). A deliberately incorrect nonsquare (2) quadratic twist is rejected by the same point-count gate; the source reducible sextics are rejected by the irreducibility gate.

A Frobenius *six-cycle* on the output branch points fixes exactly **one** perfect matching (the three opposite positions on that cycle). The corresponding rational Richelot kernel must be the dual of the map just used, assuming the accepted explicit Richelot correspondence is separable with the stated codomain. Thus the proposed output models are **forced-backtrack endpoints for a further single rational Richelot step**. This mirrors the earlier CT16 obstruction, now applied to the two new degree-seven target gluing models. It is *not* a universal no-go for other degrees, composite steps, or extension-field intermediates. [pcc:unverified]

## Reproduce and independent scope

Run (on branch head):

```sh
python3 -S research/chained_jacobian_transfers_20261110/ct21_richelot_neighbors.py
python3 -m py_compile research/chained_jacobian_transfers_20261110/ct21_richelot_neighbors.py
```

The Python fixture implements its own F101 cubic extension ring, derivative/determinant brackets, polynomial Euclidean arithmetic, Rabin test, and F101² point-count census. Expected coefficients are frozen above. The producing environment executed both commands and additionally cross-checked the six target polynomials' irreducible factorization with SymPy; **none of these producer checks constitutes independent signed PCC verification**. The exact GitHub committed file and scoped Actions regression still need CI execution and an independent verifier. This script does not import the native Cantor arithmetic and does not represent an evaluated Jacobian homomorphism.

## Consequence for CT20 and remaining obligations

The **target-model candidate space narrows**: one output curve-isomorphism class per monic CT20 chart, even before resolving which of the two Richelot *kernels* transports under the proposed degree-seven \(\Phi\). But the actual \(\Phi|J[2]\) action is still unknown. CT20 must compute \(P_1\alpha N_0\), establish *actual* geometric 2-gluing, factor \([2]\), validate both 12,317-class dual identities and all 109 scalar transfers, and use 64 elliptic 4-torsion lifts to identify the transported kernel and concrete divisor map into the selected output model. Follow the corresponding kernel even if its codomain is isomorphic to the alternate listed model. Do not substitute a matching zeta function or a curve equation for an executable transfer.

No cost benchmark or paid cloud experiment was launched. S3 checkpoint admission, live IsoLab spend visibility, and root PCC/ledger/archive repair remain required before further gated experiments/merges. The cross-characteristic mapping problem is unchanged.
