# Prior-work audit (24 October 2026)

This audit addresses the precise tested examples in `results/`. It is not an exhaustive literature proof of the absence of an identical published curve. A search failing to find a hexadecimal coefficient does not establish novelty.

| Exact source model and subgroup r | Executed edge; exact quotient if any | Literature classification | Exact-instance status |
|:--|:--|:--|:--|
| GF(2^10), modulus 0x409, a=0, b=32, r=43 | direct quadratic GHS | known characteristic-two GHS/isogeny family [1–5] | exact publication match unresolved |
| GF(2^10), modulus 0x409, a=0, b=32, r=43 | degree 3, kernel [861, 103], quotient (a,b)=(0x0,0x15a) | known characteristic-two GHS/isogeny family [1–5] | exact publication match unresolved |
| GF(2^10), modulus 0x409, a=0, b=32, r=43 | degree 2, kernel [0, 66], quotient (a,b)=(0x0,0x42) | known characteristic-two GHS/isogeny family [1–5] | exact publication match unresolved |
| GF(2^12), modulus 0x1009, a=1, b=512, r=199 | direct quadratic GHS | known characteristic-two GHS/isogeny family [1–5] | exact publication match unresolved |
| GF(2^12), modulus 0x1009, a=1, b=512, r=199 | degree 5, kernel [8, 576], quotient (a,b)=(0x1,0x249) | known characteristic-two GHS/isogeny family [1–5] | exact publication match unresolved |
| GF(2^12), modulus 0x1009, a=1, b=512, r=199 | degree 2, kernel [0, 520], quotient (a,b)=(0x1,0x208) | known characteristic-two GHS/isogeny family [1–5] | exact publication match unresolved |
| GF(2^6), modulus 0x43, a=0, b=32, r=19 | direct quadratic GHS | known characteristic-two GHS/isogeny family [1–5] | exact publication match unresolved |
| GF(2^6), modulus 0x43, a=0, b=32, r=19 | degree 2, kernel [0, 36], quotient (a,b)=(0x0,0x24) | known characteristic-two GHS/isogeny family [1–5] | exact publication match unresolved |
| GF(2^8), modulus 0x11b, a=0, b=32, r=67 | direct quadratic GHS | known characteristic-two GHS/isogeny family [1–5] | exact publication match unresolved |
| GF(2^8), modulus 0x11b, a=0, b=32, r=67 | degree 2, kernel [0, 197], quotient (a,b)=(0x0,0xc5) | known characteristic-two GHS/isogeny family [1–5] | exact publication match unresolved |

The degree-12 `c=0x1`, base-degree-four genus-one profile (and 14 other such profiles in `results/leads-n12-a1-b200-g2.json`) is a Hess-parameter structural screen, not an executed map or an attack. Hess 2004 covers the generalized family; no exact-instance publication claim is made.

The coefficients alone do not identify a curve: the field polynomial, subgroup generator and target coordinates in each receipt are part of the exact identity. The quotient receipts add the exact kernel and codomain coefficients.

Sources checked, before interpreting these runs:

1. Gaudry–Hess–Smart, *Constructive and destructive facets of Weil descent on elliptic curves*, Journal of Cryptology (2002), DOI 10.1007/s00145-001-0011-x. Classical characteristic-two descent and genus/span framework. https://doi.org/10.1007/s00145-001-0011-x
2. Galbraith–Hess–Smart, *Extending the GHS Weil Descent Attack*, EUROCRYPT 2002, pp. 29–44. Explicitly studies moving across isogenies before GHS. https://www.iacr.org/archive/eurocrypt2002/23320028/smart.pdf
3. Hess, *Generalising the GHS attack on the elliptic curve discrete logarithm problem*, LMS J. Comput. Math. 7 (2004), 167–192. General Artin–Schreier descent. https://www.cambridge.org/core/journals/lms-journal-of-computation-and-mathematics/article/generalising-the-ghs-attack-on-the-elliptic-curve-discrete-logarithm-problem/315278965D8D277A85812D9498236A88
4. Maurer–Menezes–Teske, *Analysis of the GHS Weil Descent Attack on the ECDLP over Characteristic Two Finite Fields of Composite Degree* (2002). Feasibility analysis is separate from low genus. https://www.cambridge.org/core/journals/lms-journal-of-computation-and-mathematics/article/analysis-of-the-ghs-weil-descent-attack-on-the-ecdlp-over-characteristic-two-finite-fields-of-composite-degree/D4B264D082FF67354EE9968D9803102A
5. Magma handbook, *Weil Descent in Characteristic Two*: nonzero descent parameter and explicit divisor map; genus alone is not an attack certificate. https://magma.maths.usyd.edu.au/magma/handbook/text/1561
6. A 2026 independent classification research page explicitly tabulates classical binary GHS Frobenius-span and genus for degrees 4, 6, and 8 while marking arbitrary covers and isogeny walks outside its scope. It reinforces that genus enumeration alone does not establish an attack. https://math.st/research/weil-descent-classification
7. Local earlier investigations `research/ghs_cubic_hyperelliptic_genus3_20261013`, `research/ghs_cubic_quartic_ic_20261012`, `research/ghs_cover_census_n5_20261013` were checked for overlap in method. These do not give newness to the quadratic inputs here.

Exact-instance search queries used: `"GHS" "0x43" "0x20"`, `"GHS" "0x11b" "0x20"`, `"GHS" "0x409" "0x20"`, `"GHS" "0x1009" "0x200"`, paired with `Weil descent`, `isogeny`, and genus two. Quotient-specific searches additionally paired `GHS`/`isogeny` with `(0x43,0x24)`, `(0x11b,0xc5)`, `(0x409,0x42)`, `(0x409,0x15a)`, `(0x1009,0x208)`, and `(0x1009,0x249)`; these literals identify the six executed codomains in the same respective fields. No source demonstrating this exact instance surfaced. The family is unambiguously prior art, and all exact-instance novelty fields therefore remain `unresolved`, not `new`.
