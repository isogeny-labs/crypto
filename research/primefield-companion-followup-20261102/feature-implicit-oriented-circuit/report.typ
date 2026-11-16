#set page(paper:"a4",margin:16mm)
#set text(size:10pt)
= Implicit oriented-mode constraints
T-1932 · Prime fields · 2026-11-08

The builder retains exact three-mode image support before dense phase expansion. It adds absolute scalar consistency to the earlier relative constraints. It represents three Fourier modes at each supplied radius, not every individual sign.

#image("flow.svg",width:100%)

*Derived chart.* Write (X−b)^h as the sum of X^e A(b,e)(Y), with X^m=Y. The observed modes are m A₀, mk A₁ and mk^(m−1) A(m−1). A nonzero plus mode gives k=fplus/(m A₁); otherwise a nonzero minus mode gives k=mY A(m−1)/fminus. Set k=N/D, impose N^m=YD^m, phase Y^L=epsilon, and all cross-multiplied mode equations. These imply a nonzero denominator and exact transcript equivalence. All-zero oriented modes use a separate unanchored case.

*Recorded exact checks.* 1,936 systems and200,048 image-membership equalities pass. Seven checked states have stricter support than the relative oracle. There are1,998 conditional phase checks;1,876 plus-anchor and60 unanchored systems. A synthetic empty-support case covers the minus-only branch; positive minus-only support is not demonstrated here. PCC-20261108-ac7576 is unsigned producer evidence.

*Compact representation.* Six controls over prime998244353 with m7 and L71,303,168 use at most7,983 stored nodes. Their Y values are supplied fixtures. A quadratic-modulus specialization agrees at both roots. No dense phase polynomial is built by the representation; no common-root extractor is supplied.

*Public binding.* Across300 public targets,277 produce transcripts and23 exit through verified identity/endpoints. All277 phase checks use fixture Y and verify the returned scalar against the point. API source calls total5,885, excluding setup/calibration;298 single-call guards pass. No unknown-image recovery count is claimed.

*Custody and failures.* Validation01 missed an inherited include. Validation02 failed a presumed-positive strictness assertion. Both are preserved. Validation03 reports actual counts after adding larger orbit-width controls; seven copied sources and13 compiler dependencies are hash-bound. Fresh overflow-checked build/check limits90/60 seconds. No timing comparison or cloud submission.

*Next step and costs.* Extract common image roots effectively from the implicit system. Charge public source setup/calls, point shifts, interning, allocation, circuit construction and specialization, scalar inversion and verification. Node and coefficient-slot caps are not peak-RAM bounds.

*Sources.* Predecessor signed-mode, oriented and packed modules; FFTW unnormalized DFT conventions. Finite-field orthogonality and the chart argument are derived explicitly in DERIVATION.md. KB unavailable. Independent signed review pending.
