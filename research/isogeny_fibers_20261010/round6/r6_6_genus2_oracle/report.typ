#set page(width: 210mm, height: 297mm, margin: 18mm)
#set text(size: 10pt)
= R6.6 — Genus 2 under a Nagao-style oracle, with a Richelot image arm
Toy instances, $y^2 = x^5 + b$ over $F_p$, $p in {1051, 2131, 4111}$; 40 random bases of size 80 per cell; bootstrap 95%. No sub-rho claim.

*Verdict: confirmed — the falsifier did not fire.*
- A-closed base: unknowns gain 10.00 [10.00, 10.00] on all six curves (falsifier window [8, 12]); class factor at $m = 3$ is 0.95–1.04.
- Richelot image: class factor 3.53–4.20 (R arm) and 3.60–4.07 (literal $phi(F)$ on $J'$); all intervals inside [3, 5].
- Translation closure: $|(C - infinity) inter (C - infinity - t)| <= 2$ for every tested $t$, so no saturated degree-1 base exists.
- Richelot validated: $\#J' = \#J$, 0/300 homomorphism failures, kernel exactly the (2,2) group, index-4 density 0.246–0.251.
- Nagao $m = 2$ oracle: 49–58 counted $F_p$ multiplications per call, 0 disagreements with exact enumeration.

#figure(image("measured_vs_predicted.png", width: 100%), caption: [Measured / predicted per arm and $p$ (1 = law).])
#figure(image("oracle_cost.png", width: 100%), caption: [Nagao $m = 2$ oracle: counted multiplications per call and calls per relation.])

Details, tables, commands and limits: `RESULTS.md`; raw data: `raw/*.json`.
