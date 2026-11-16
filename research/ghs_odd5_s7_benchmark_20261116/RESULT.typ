#set page(paper: "a4", margin: (x: 18mm, y: 18mm))
#set text(font: "New Computer Modern", size: 10pt)
#set heading(numbering: "1.")
#show link: underline

= Degree-seven cover transfer: source-bound s=7 run

_Research report, 16 November 2026. PCC statements PCC-20261116-a3af33 and PCC-20261116-4818d8 are unverified._

== Exact instance and transfer

Let $K=bb(F)_3[a] / (a^5 + 2a + 1)$, $L=bb(F)_(3^35)$, and $k=bb(F)_(3^7)$, with embeddings and point coordinates pinned in `s7_transfer.json`. The source and carrier are

$E/L: v^2 = u^3 + 2u^2 + a+a^3$,

$Y/k: y^2 = x^11+x^9+2x^8+x^7+2x^5+x^3+x^2+x+1$.

The degree-seven cover is $u=A(x)/B(x)$ and $v=C(x)y/B(x)^2$. `s7_transfer.sage` checks the polynomial cover identity. The subgroup map pulls a point to $J(Y)(L)$, subtracts the fixed two-torsion pole divisor, sums its five $3^7$-Frobenius conjugates, then descends the fixed class to $J(Y)(k)$. `audit_s7_transfer.sage` reconstructs saved images. These are producer-side checks. [pcc:unverified]

The source group order is `50031545359078046 = 2*29*109*7393*1070455751`. The prime subgroup order is $r=1070455751$. Saved $P$ and $G$ have order $r$, $G != 0$, and $Q=123456789P$ maps to $H=123456789G$. The independent $37P$ transport also passes. Exact coordinates are in `s7_transfer.json`. [pcc:unverified]

== Complete saved run

The third attack attempt records factor-base size 1093, relation rank 1093, 2186 accepted relations in 269998 attempts, and a completed five-term target descent. It recovered scalar 123456789 in the carrier and source subgroup. The first attempt was interrupted after the generic sparse solve exceeded 16 minutes; the second failed in JSON serialization. Their logs are retained. [pcc:unverified]

#image("cost.svg", width: 100%)

The full GHS path records 422.170193 CPU seconds. Thirty corrected Frobenius-quotient rho trials have median one-solve CPU 19.4710205 seconds, including rho setup. The ratio is 21.6819757, with GHS slower. These are historical Sage/Python-source timings. The receipts omit wall time, host identity, and isolation level. A native matched rerun and independent signed PCC validation are required for a benchmark claim. [pcc:unverified]

== Prior work and claim boundary

The exact base-change instance is new to this search; the degree-seven cover orbit is already present in this repository. Iijima, Momose and Chao classify the relevant `(2,...,2)` odd-characteristic prime-degree setting through seven under their stated conditions. This run does not establish a new eligible family, attack method, or below-rho vulnerable instance. [pcc:unverified]

Primary source: #link("https://eprint.iacr.org/2015/805.pdf")[Iijima–Momose–Chao (2015)]. Local prior work: `../ghs_odd5_carrier_20261115/DEGREE7_FINDING.md`. Full raw sources and receipts are beside this report. The cyclic-11 p-rank exclusion is separately documented in `../ghs_cyclic11_p_rank_bound_20261116/THEOREM.md`.
