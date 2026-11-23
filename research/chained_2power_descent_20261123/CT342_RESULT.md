# CT342 result — F103 primary-group structure, odd-order inverse and 2-primary obstruction

**2026-11-23 UTC; exact public finite-field computation, one coordinator, unsigned [pcc:unverified].** Protocol [CT342_PROTOCOL.md](CT342_PROTOCOL.md) was committed on the former #5045 branch at `b3c9bc7ace69aac9856162eab13df6db03df8c4c` before this implementation ran. The parent #5045 was merged by an external GitHub transition at **2026-11-23T18:50:31Z**, deleting its branch while CT342 was in progress. This caused one source-create attempt to return HTTP 404; no source was written to the defunct branch. The work was transferred to new separately tracked **draft PR #5060** from main, without modifying the merged parent or its frozen source/results. The CT342 protocol is now on main.

## New group-structure result (public F103 positive control)

For the original CT336–341 bielliptic curve
[
C: y^2=prod_{j=1}^6(x-j),quad
E_+: y^2=(x-	frac14)(x-	frac94)(x-	frac{25}4),quad
E_-: y^2=(1-	frac{x}4)(1-	frac{9x}4)(1-	frac{25x}4)
]
over (mathbb F_{103}), a **new standalone Node.js group-law implementation** (no CT339–341 imports) enumerated every elliptic rational point, its order, and both Sylow structures.

| Quantity | `E_+` | `E_-` |
| --- | ---: | ---: |
| `#E(F103)` | 92 | 100 |
| Point-order exponent | 46 | 50 |
| Full invariant factors | **Z/2 × Z/46** | **Z/2 × Z/50** |
| Nonzero rational 2-torsion | 3 | 3 |
| Odd-primary subgroup | Z/23 | **Z/25** |
| Order-25 points | 0 | **20** |
| Order-50 points | 0 | **60** |

Thus
[
B=(E_+	imes E_-)(mathbb F_{103})
   cong (mathbb Z/2)^4 	imesmathbb Z/575.
]
In particular, `E_-` is **not** (mathbb Z/10	imesmathbb Z/10); the order-25 and order-50 witnesses distinguish its cyclic 25-part. `E_-` has an explicit order-25 witness `(x,y)=(0,1)`, and `E_+` has an order-23 witness `(41,2)`. The 2-torsion abscissae are `[26,28,32]` for `E_+` and `[4,29,92]` for `E_-`.

The CT341 independently computed zeta order `#J(C)(F103)=9200=16*575`. All six Weierstrass roots are rational, so (J(C)[2](mathbb F_{103})cong(mathbb Z/2)^4); this consumes the entire 2-primary order 16. The dual composition (PsiPhi=[2]) implies (J(C)_{m odd}cong B_{m odd}). Consequently
[
oxed{J(C)(mathbb F_{103})cong (mathbb Z/2)^4	imesmathbb Z/575}
]
**conditional on the frozen CT340/CT341 homomorphism and zeta premises**. This is a finite mathematical consequence with a new elliptic group-law crosscheck, not a signed independent Jacobian census.

## Transfer theorem and exact obstruction

Let (A,B) be finite abelian groups and homomorphisms (Phi:A	o B), (Psi:B	o A) satisfy (PsiPhi=[2]_A) and (PhiPsi=[2]_B).

* For each odd prime (ell), multiplication by 2 is invertible on the (ell)-primary subgroup. Hence (Phi:A_ell	o B_ell) is an isomorphism with explicit inverse ( [2^{-1}]Psi), using an inverse of 2 modulo the relevant subgroup exponent. This holds regardless of nonsurjectivity on all rational points.
* Since ([2]Bsubseteqoperatorname{im}Phi), the cokernel (operatorname{coker}Phi) is annihilated by 2; likewise for (Psi). CT341's kernel/image counts were 4/2300 for each direction. On F103 both 2-primary groups are 4-dimensional vector spaces over F2; therefore the maps restrict to **rank-two F2-linear maps**, with 2-dimensional kernels and cokernels:
  [
  operatorname{coker}Phicongoperatorname{coker}Psicong(mathbb Z/2)^2,
  qquad |operatorname{im}Phi|=|operatorname{im}Psi|=2300.
  ]
* **Single-generator scalar-recovery corollary:** even if (Phi) is not injective on all of (A), its restriction to any cyclic subgroup (langle Gangle) has kernel killed by 2, hence **at most two elements**. If (operatorname{ord}(G)=n) and (operatorname{ord}(Phi G)=m), then (n/min{1,2}). A valid DLP solution in the destination determines (kmod m); at most two candidate lifts (k) or (k+m) need source-group verification. This qualification is stronger and more useful than claiming that the index-four rational cokernel destroys DLP recoverability.

The last corollary is a statement about **correctness and bounded ambiguity**, not computational superiority. It requires a computable (Phi), known subgroup order and a complete charged destination DLP solver. The two candidate verifications and all setup/map/linear-algebra/memory belong in cold single-target cost.

## Executed evidence

* Source: [ct342_primary_structure.cjs](ct342_primary_structure.cjs), SHA256 `a04753219055304b2bda82e779525986a48722c2e6d90f50962c87255be4ffa3`; Git blob `3bce4e0974d0eb5756f556525f2ee5e958049848`, reread from GitHub after a corrective newline-escape commit.
* Frozen [CT342_RESULT.json](CT342_RESULT.json), SHA256 `71eb14deb8f190556fc27c6e3ece1df7eaeaeef87678e985e5b536db1b46decc`; Git blob `4ca275ed1257043cececa9bfc5faf4b902801752`, reread from GitHub. 92/92 plus 100/100 point-order checks, exact inverse/annihilation for every point, and **8000 deterministically selected associativity triples per factor**.
* Executed `node ct342_primary_structure.cjs > ct342_normal.json` twice and `cmp` returned identical JSON. Three changed-source negative controls were actually rejected nonzero: `--wrong-coeff` alters the minus curve (count becomes 123), `--wrong-order` requires a false 93-point plus group, `--wrong-primary` tests the false `Z/10 × Z/10` minus structure. An initial local `wrong-primary` control used a tautologically false comparison; it was fixed **before source publication** and final output was rerun. A GitHub source transcription initially rendered the `\\n` string escape as a literal line break; the immediate corrective commit restored byte-identical Git blob SHA matching the local executed source.
* Run: `node research/chained_2power_descent_20261123/ct342_primary_structure.cjs > /tmp/ct342.json && cmp /tmp/ct342.json research/chained_2power_descent_20261123/CT342_RESULT.json`, followed by separate wrong-coeff/wrong-order/wrong-primary commands each required to exit nonzero.

## Ownership, source revisions, limitations and next gate

Actual roles: **one coordinator**, zero native agent workers. Mathematics (dual-map/cyclic-kernel lemma), implementation (standalone Node elliptic arithmetic) and validation (the same coordinator's separate canonical reruns and mutation controls) were not independent personnel. Research-KB MCP/Conductor and a Rust compiler were unavailable in this runtime. Prior source fallback: CT340/CT341 frozen results; [Diem 2006](https://arxiv.org/abs/math/0312413) and [Magma Richelot handbook](https://magma.maths.usyd.edu.au/magma/handbook/text/1615). There is no novelty claim, signed PCC, independently owned Rust/Cantor replay or elapsed-time benchmark. The source and raw blobs were not modified post-freeze apart from the explicitly recorded published-source newline repair.

Original EKS-only authenticated 695/701 norm-point fixture; physical `C169/C13a/C13b/N2_actual/D` pricing; strongest same-host cold optimized rho, BSGS and index-calculus references; other F3/E7 native owners and locked vault remain untouched. No new automation, paid job, CI rerun, or manual merge. The shared `docs/session-logs/2026-11-23.md` is ~2 MB and contained unresolved conflict delimiters on main; to avoid replacing concurrent entries, this complete source-pinned handoff is committed instead.

**Next:** independently reproduce the 2-primary map ranks and the at-most-two scalar-lift corollary against **actual Cantor divisor classes** in Rust on the frozen 9200-class fixture; then pursue signed PCC, and separately let the incumbent norm-point owner conduct the charged cold comparison. This experiment is an intentionally split F103 positive control, not an ordinary prime-field ECDLP speedup. [pcc:unverified]
