# L2: representation and coordinate levers for EcGFp5 (desk audit, 2026-11-02)

The research agent's findings, saved by the main session (the agent could not write files).
Scripts are in `src/` (`fq.py`, `ringu.py`, `variants.py`) and systems in `runs/`.

## Closed by proof or literature

| lead | status | reason |
|---|---|---|
| any degree-2 coordinate change (translation, Möbius over F_p or F_{p^5}) | closed | D = 2^{(m-1)^2} for every N-stable subline; a translation changes the target; a Möbius map changes only the factor-base subline |
| larger symmetry groups (FGHR Edwards and Jacobi intersection, FHJRV full E[2], 4-torsion, Hessian, Huff) | closed | EcGFp5 has one rational 2-torsion point, no rational 4-torsion (b and a^2 - 4b are non-squares) and 4 ∤ #E even up to 2-isogeny; the only usable group is ⟨N⟩, which gives D = 2^16 at m = 5 |
| factor-base cosets by N, Weil-descent basis, Frobenius-orbit factor base, twist sharing, 2-isogenous curve, non-Galois coordinates | closed | no new relations; same ideal; j ∉ F_p; degree bound ≥ 3^4 per unknown |
| atlas claim "Kummer line Bézout-optimal" (IDEA-20260905-d0fee4) | conclusion holds, one step wrong | step (C2) fails for coordinates that factor through an isogeny; the u class ties |

## Open and measurable: Pornin's u = x/y ring form (new)

- u(P+N) = -u(P) and u(-P) = -u(P), so u is the quotient by P ↦ N - P. Checked independently on 200 points at p = 4111.
- H_R lies in K[s_1..s_{m-1}, π], with s_k = e_k(u_i^2), π = ∏u_i and s_m = π^2 (FHJRV Prop. 13; it does not need φ(-P) = φ(P)).
- At m = 5 the system has 6 unknowns with degrees (8,8,8,8,8,2), the same supports as the current quotient (1287 and 792 terms), and a two-term quadric s_5 - π^2 in place of the dense w^2 - Q(s).
- D measured equal to 2^{(m-1)^2}:

  | m | p | D |
  |---|---|---|
  | 3 | 4111 | 16 |
  | 3 | 262231 | 16 |
  | 4 | 4129 | 512 (re-measured with msolve by the main session) |
  | 4 | 262153 | 512 |

  The planted point was recovered in every case. m = 4 is even, where the current λ-construction does not exist.
- The ledger statement that the ring form is "unavailable for double-odd curves" (CORR-20260923-064d0d, KN-OPEN-9b4a2b and KN-OPEN-814b4c) is too broad. It holds only for coordinates with φ(-P) = φ(P).
- **Next step:** one m = 5 cell at p = 4111 against W0 (F4 CPU bar 6,900 s; whole-solve CPU 31,004 s), under a frozen protocol. Expect D = 65,536 and no degree fall; any gain is a constant factor.

## Other open items (lower prior)

- Subline choice within a family: fold into the u cell with two μ' values.
- Weighted 5-variable u ring form: needs Magma or FGb (not on this host).

## Sources read

- FGHR 2014: https://hal.science/hal-00700555/document
- FHJRV 2014: https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/Semaev2torsion.pdf
- Huang–Petit–Shinohara–Takagi 2013: https://christophe.petit.web.ulb.be/files/IWSEC2013.pdf
- Pornin 2022/274 (local frozen copy in crypto-autoresearcher `inputs/`)
- Abstract only: eprint 2014/806 and 2010/383.
