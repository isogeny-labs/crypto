#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt)
= Direct oriented phase supports
T-1877 · 2026-11-07 · Prime-field compressed recovery

The new adapter folds F0 directly modulo each sparse phase equation Vⁿ = u. It keeps the resulting oriented support in V and applies later constraints in scalar quotient rings. It does not build the reciprocal H / Dₙ transform or lift back from W.

#image("flow.svg", width: 100%)

*Construction.* Palindromic symmetry supplies the lower half from the same n+1 upper coefficient deliveries. Each phase gets a residue of degree below n. Intersect with Vⁿ − u using modular exponentiation and gcd; expand the phase polynomial only when the folded equation is identically zero. Normalize back from the fixed anchor V = t0 Z at materialization.

#table(columns: (3fr, 1fr), inset: 4pt,
 [Native control], [Observed],
 [Configurations / folded-residue equalities], [23 / 92],
 [Oriented supports equal to dense and reciprocal references], [13,980],
 [Zero / nonzero-constant equation controls], [11 / 376],
 [Initial degree-cap rejections], [23],
 [Candidate states matching over four radii], [2,516],
 [Source calls: 281 targets at cap 64 / cap 16], [2,881 / 3,152])

Both public panels retain their recovery routes, with zero unresolved targets. The cap-16 panel preserves 260 later updates and 265 deferrals. Earlier representations remain as controls.

*Costs and limits.* Folding removes the balanced reciprocal matrix transform, but still stores four n-entry residues and performs phase gcds/modular powers. A direct V support can be larger than an unresolved W support. It removes quadratic-pair updates and reciprocal lifts from this path; no universal memory or timing advantage follows. All coefficient, source, initial-observation, order-check, allocation and final-verification work remains charged.

*Evidence.* validation01 passes with 88 source pins; inherited controls pass. No new compile/native/render failure or cloud/timing run. Producer assertion, independent signed verdict pending. Original 4,563-case cohort unchanged. Derivation and replay: DERIVATION.md, RESULTS.md, README.md.

*Sources and next.* Exact reciprocal/canonical/fused predecessor and official FLINT quotient conventions, flintlib.org/doc/nmod_poly/index.html. KB unavailable. Next investigate whether multiple observations yield a low-degree constraint before a degree-n initial support is constructed.
