# Literature: deciding Serre's twist obstruction for genus-3 ppav over finite fields

Date 2026-11-21. Standing `[pcc:unverified]`. WebSearch only, generic mathematical terms
(four queries). Full texts could **not** be read: `arxiv.org` and `old.mccme.ru` do not
resolve from this session (fetching stopped after the second failure, per session
rules). Every attribution below is "reported by a search-engine summary", not a
quotation. research-kb (the lab's knowledge-base MCP) was not available to this agent.

| # | Source | What the summary attributed to it | Use here |
|---|---|---|---|
| L1 | G. Lachaud, C. Ritzenthaler, A. Zykin, "Jacobians among abelian threefolds: a formula of Klein and a question of Serre", Math. Res. Lett. 17 (2010); arXiv 0802.4017 | A principally polarized abelian threefold that is geometrically a (non-hyperelliptic) Jacobian is a Jacobian over the ground field iff the value of the Siegel modular form `chi_18` at it is a square (answering Serre); new proof of Klein's formula `chi_18 ~ Disc(quartic)^2`-type relation; fails for `g > 3`. Summary states it for subfields of C; Serre's assertion over number fields proved earlier. | The decision criterion. |
| L2 | S. Meagher, "Twists of genus three Jacobians" (thesis-based paper; reply to a 2003 letter of Serre to Top); Meagher-Top, twists of the Klein quartic | Criterion distinguishing a genus-3 Jacobian from its `[-1]` twist in terms of the product of the 36 even theta-nulls. | Same criterion, independent source. |
| L3 | C. Ritzenthaler, "Explicit computations of Serre's obstruction for genus 3 curves and application to optimal curves", LMS J. Comput. Math. 13 (2010) 192-207; arXiv 0901.2920 | Not known how to compute the value directly over a finite field; method: lift `(A, a)` to a number field, evaluate `chi_18` through theta-nulls analytically, reduce to the finite field. Applied to Jacobians isogenous to `E^3`; optimal curves exist for q = 47, 61, 137, 277, not 311. | Our route is an exact algebraic version of this lift: Thomae plus the duplication formula give `chi_18(J/L)` as a polynomial in fourth roots of branch-point differences, evaluated directly over `F_{p^28}`. |
| L4 | arXiv 0711.0415 (twists of genus-3 curves; summary only) | Analytic Siegel modular forms do not change under twisting while Katz (algebraic) modular forms do; matters when reading the square condition over non-closed fields. | Why the basis `E` must be `k`-rational and why the twist flips the class (`c^27`). |
| L5 | T. Ichikawa, Proc. AMS 146 (2018); arXiv 2009.06718 (search summary) | The constant in Klein's formula is reported as `(2 pi)^54 / 2^28` (attributed to LRZ) or `2^26 pi^54` under another convention; a Ciani-quartic version carries `-2^28`. | The rational normalisation of `chi_18` is convention dependent. We do not rely on it: the universal constant's square class `kappa_p` is calibrated by the positive control at each prime. |
| S1 | B. Smith, J. Cryptology 22 (2009) (via the lab's `src/cryptanalysis/smith_prop24.rs`, `smith_trigonal.rs`) | Pair-type kernels: Recillas' curve `X` is defined over `F_p` with `Jac X = J/S` iff `lc(s)` (equivalently the fibre norm `prod_{g(x)=t0} F(x)`) is a square (Prop. 4, Lemma 4); the lab checked this against point counts of `X` (`#X = #H` vs `2p+2-#H`). | Ground truth of the positive control. |

Not found: any statement about the twist (Serre sign) of affine-type `(2,2,2)` quotients of
hyperelliptic genus-3 Jacobians, or an explicit construction of that isogeny. The
lead-#3 literature note (`../g3_transfer_gates_20261120/LITERATURE.md`) already records
that no explicit map is known for non-hyperelliptic quotients (Milio, Math. Comp. 2020)
and that Smith's construction covers pair type only.

Sources (search result pages used):
- https://arxiv.org/pdf/0802.4017 (via search summary; host unreachable)
- https://www.cambridge.org/core/journals/lms-journal-of-computation-and-mathematics/article/explicit-computations-of-serres-obstruction-for-genus3-curves-and-application-to-optimal-curves/BAD4ED6221A2BA633A8B6A1A8BB5F94B
- https://arxiv.org/abs/0711.0415v4
- https://www.ams.org/journals/proc/2018-146-12/S0002-9939-2018-14244-6/S0002-9939-2018-14244-6.pdf
- https://arxiv.org/pdf/2009.06718
- https://arxiv.org/pdf/2004.03263 (homage to G. Lachaud; Meagher-Top mention)
