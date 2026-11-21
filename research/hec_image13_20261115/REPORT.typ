#set page(paper:"a4",margin:17mm)
#set text(font:"Helvetica",size:10pt,fill:rgb("233746"))
#set par(leading:.5em)
#text(size:22pt,weight:"bold")[Quotient image equation candidates]

Both genus-3 coordinate images · Frozen interpolation gate · 15 November 2026

*Both images have no fitted quadric, eight fitted cubics, and a unique quartic whose gradients span that cubic space.* All unused-point checks pass. The nonzero equations remain global-identity candidates; the full-rank quadratic matrix already excludes any global quadric relation.

#image("IMAGE.svg",width:100%)

*Registered sample.* Original curve, kernels, normalization scalars and eight coordinate descriptors remain fixed. New public multiples n=1001 through 4000 are considered in order. The search reaches 192 joint regular points at n=1199, retaining seven exclusions. The first 160 points fit the equations; the final 32 are unused for fitting. All 384 coordinate vectors, attempts and exclusions are stored. No favorable kernel, field or replacement sample is selected.

*Exact linear algebra.* Native F127 elimination uses all 36 quadratic, 120 cubic and 330 quartic monomials. Quadratic and cubic matrices have 160 rows; stacking eight partials per point gives 1,280 quartic-gradient rows. For both kernels, ranks are 36, 112 and 329, hence nullities 0, 8 and 1. Every nullspace vector is checked against the original matrix. Exact coefficient tests put every quartic partial in the fitted cubic span and give gradient rank eight. Thus each quartic is unique up to scalar in the sampled gradient kernel.

*Holdouts and controls.* Across both images, 1,088 held-out polynomial and gradient evaluations vanish. Quartic values are checked as well as all partials, consistent with Euler’s identity since 4 is invertible mod127. Adding a single x₀⁴ coefficient is detected on every holdout: 64 failures. Before real points, rational-normal-curve controls have nullities 21 and 98 for quadrics and cubics, including unused control points. A squared quadric and its gradients vanish at 20 planted points; corruption is detected at all 20. Shared arithmetic is not independent external validation.

*Claim boundary.* Tian’s complex nonhyperelliptic Jacobian construction motivates the Coble-type pattern. These samples do not establish the nonzero equations globally, identify the complete coordinate image, prove a quotient curve or settle same-field/Serre descent. Raw output explicitly retains global-identity-certified=false. A global quadric would belong to the zero-dimensional sampled kernel, so its exclusion is already exact.

*Next certificate.* A post-run source check found the even-section bound 112 for degree-three monomials. If its finite-field principally polarized setting and linearization hypotheses are verified, at least eight global cubics must exist. Containment in the eight-dimensional sampled kernel would prove equality; the coefficient-gradient test would then certify the singular quartic globally. NEXT-CERTIFICATE.md records the argument and remaining source checks. No global claim is promoted on that basis yet; a C³ zero-bound proof is an alternative.

*Custody.* Protocol 5e22454497 preceded source b1fb249426b6 and checkpoint 20261115T145452Z-b1fb249426b6. Clean locked offline Rust build; terminal execution exits successfully with empty stderr and no cap or rerun. Full coefficients, monomial order and hashes are retained. No timing claim, cloud worker, historical-record or default change. Existing native CI does not run this standalone gate.

*Sources and onward work.* Tian, arXiv:2007.03172, Remark 3.4; Müller, Explicit Kummer varieties of hyperelliptic Jacobian threefolds, Section 3; prior pinned coordinate reports. Exact links and gaps are recorded. Next: global certificate, quotient geometry, curve/descent and divisor transport, then complete-cost comparisons. The full mathematical 10× genus 2/3/4 objective remains active.
