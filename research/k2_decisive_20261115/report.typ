#set page(margin: 2cm)
#set text(size: 10pt)
= K2 decisive cells: degree-4 closure stalls at k = 7, 8 (m = 3) [pcc:unverified]
Registered grid (seed 1, blocked GF(2) closure, local 4 threads): L1 (m=4,n=13,k=3) and L2 (m=3,n=19,k=6) solve at D = 4; D1 (m=3,n=21,k=7, 124314 columns) and D2 (m=3,n=24,k=8, 213053 columns) stall at D = 4 with deficits 6979 and 24922; D3 (m=4,n=16,k=4, 396607 columns) not run (basis would need 19.7 GB, cap 9 GB). Verdict by the registered rule: D1 breaks, D2 breaks, D3 not reached; overall breaks. Controls pass. Post-hoc: the stall tracks k >= 7, not n (k = 7 stalls at n = 18..21, k = 6 solves at n up to 21); dense reference engine reproduces the n = 18, k = 7 deficit (10873).
#figure(image("figures/k2_deficit.svg", width: 85%))
#figure(image("figures/k2_decisive.svg", width: 85%))
Limits: one seed per cell; D = 5 not scanned; direct presentation and cost against rho not addressed. See README.md.
