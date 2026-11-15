# Frozen degree-six cover construction screen

15 November 2026. Let `K=F3[a]/(a^5+2a+1)` and keep the exact genus-five
`Y: y²=f(x)` in `TRACE26_CLASS_THEOREM.md`. Search a target
`E_b: v²=u³+2u²+b` for each b-mask in
`3,27,30,103,139,167,182,194,230,233`. Run masks separately and retain
source-bound complete JSON/stderr. The first execution will be b-mask 3.
The frozen `degree6_screen.sage` SHA-256 is
`ae65a83015e82b79fb3890677992ae55a61b2af442855fac3db896910ed55b4d`;
its deterministic Sage-preparsed Python SHA-256 is
`35232733acf17d7222b48aad33d1799bb16a07c3231cc53529f6a0a60643b74a`.

Normalize the rational point at infinity to the elliptic origin. A separable
degree-six map has `u=A/B` with coprime polynomials, `deg A=6`, and
`deg B=6-e_inf`, where the odd local index `e_inf` is 1, 3 or 5. The source
branch polynomial is `(x+1)Q1...Q5` over K, with five irreducible quadratics.
At finite branch poles, B has odd multiplicity; at nonbranch poles, even
multiplicity. Since `deg B` is odd, `x+1` divides B to an odd power. The
complete monic denominator shapes are:

| B shape | Denominators | Branch assignments |
|---|---:|---:|
| `L` | 1 | 243 |
| `L³` | 1 | 243 |
| `LQi` | 5 | 405 |
| `L(x-c)²`, c != -1 | 242 | 58,806 |
| `L⁵` | 1 | 243 |
| `L³Qi` | 5 | 405 |
| `L³(x-c)²`, c != -1 | 242 | 58,806 |
| `LQiQj`, i<j | 10 | 270 |
| `LQi(x-c)²`, c != -1 | 1,210 | 98,010 |
| `LH²`, monic quadratic H coprime f | 58,801 | 14,288,643 |
| **Total** | **60,518** | **14,506,074** |

Here `L=x+1`; the table records branch assignments before the one free
numerator coefficient in the `LQiQj` arm. Every unpoled quadratic branch pair
has exactly three Galois-compatible elliptic 2-torsion x-images. For denominator
B, let M be the product of the unpoled Qi and h the corresponding CRT residue.
If `deg M>=8`, any degree-six A is uniquely `(B*h) mod M`; reject if its degree
is not six. For `B=LQiQj`, `deg M=6` and every possible A is
`(B*h mod M)+lambda*M`, with nonzero `lambda in K`; enumerate all 242 values.

For `B=LH²`, write `H=x²+ux+v`. Compute the three high coefficients x^7,x^8,x^9
of `(B*h) mod Q1...Q5` as polynomials in u,v. Their common K-solutions are
found by a nonzero pairwise resultant in v, followed by a univariate gcd at
each candidate u. If every pairwise resultant is zero, enumerate all u,v in K.
If the gcd after specialization is the zero polynomial, enumerate all v.
This fallback is necessary to cover rank-deficient branch assignments.
Reject H sharing a factor with f. For every surviving A,B, check
`gcd(A,B)=1` and the exact identity
`f*C²=B*(A³+2A²B+bB³)` for `C in K[x]`. A hit must include A,B,C and
exactly verify the displayed identity. Any hit requires independent Magma
replay, morphism degree verification, and subgroup trace evaluation before a
transfer claim. If there are no hits, independently replay the complete screen
before using it as a degree-six exclusion.

An inseparable degree-six morphism would factor a degree-three Frobenius map
and a degree-two map from a genus-five Frobenius twist to an elliptic curve;
the earlier geometric degree-two exclusion rules it out. Thus the separable
search suffices. This protocol does not search arbitrary correspondences or
other genus-five carriers.
