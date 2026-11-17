# Primary-source review and odd-field route

Semaev's own2004 Oberwolfach contribution (report54/2004,pp2959-2960) defines the relation over the algebraic closure, supplies the odd cubic and constructs higher polynomials by resultants. It frames fast bounded-coordinate solving as the auxiliary problem. Summation polynomials and scalar resultant elimination are prior art; finite-field coordinates do not guarantee rational lifts.
https://publications.mfo.de/bitstream/handle/mfo/2873/OWR_2004_54.pdf?isAllowed=y&sequence=1

Trimoska, Ionica and Dequen2020 study binary point decomposition using WDSat and symmetry breaking, treating source permutations as equivalent for relation finding. Our ordered finite-chain contract explicitly reconstructs permutations and auxiliaries. Their comparison is relevant specialized prior art to evaluate; it supplies no measurement on this program's inputs or all-roots contract. Novelty review remains open.
https://pmc.ncbi.nlm.nih.gov/articles/PMC7334981/

For odd characteristic on Y^2=X^3+aX+b, Semaev's cubic in Z has ascending coefficients [(uv-a)^2-4b(u+v),-2((u+v)(uv+a)+2b),(u-v)^2]. The guarded join is field-independent: for Q=AX^2+LX+C and R=DX^2+MX+E, DQ-AR=(DL-AM)X+(DC-AE). If DL-AM!=0, test X=(AE-DC)/(DL-AM) in both polynomials. If that coefficient vanishes but DC-AE!=0, no common root exists. Otherwise solve one nonzero polynomial and filter the other, explicitly handling identically-zero polynomials if allowed. Only the remaining quadratic solver requires characteristic-specific arithmetic. A future odd implementation must use exact finite-field square-root/factorization and test characteristic3, odd extensions and all five requested primes. No odd implementation or timing is claimed here.
