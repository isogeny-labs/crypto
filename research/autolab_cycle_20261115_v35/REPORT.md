# Inversion-free all-k binary norm charts

For a valid target (z,y) on Y²+XY=X³+AX²+B with Tr(A+z)=0, choose c²+c=A+z and d=y+zc. The affine change r=z+ch+u, s+zr=z²+yh+ck+v is bijective for every h,k, including zero. The full ascending monic quintic norm quotient is

[zv²+dk², v²+kv, zu²+hv+dh²+ku+k², u²+hu+k, h²+h+z, 1].

This extends the k0 chart to every k, cancels mixed context terms and makes E4=v²+kv independent of target and h. Both AS choices are retained through (u,v)->(u+h,v+k). No squarefree assumption is used. The context builder requires odd degree; even fields need a supplied context. Trace-one targets are unsupported by this chart, never UNSAT, and require the unmodified norm route. Valid target/canonical coordinates are caller preconditions.

The first frozen oracle checked149760 complete tiny h/u/v states over GF4/8/16/32, both A choices and both AS choices. Its actual GF83 oracle checked all31126 zero/unit/pair evaluations for249bits. The proved Boolean degree2 bound certifies the complete k0 map. Worker8.297085012seconds; peakRSS58608KiB.

The fresh all-k successor checked25600 tiny states: complete h/k/u/v maps over GF4/GF8 and unique1024point samples per context over GF16/GF32, coordinate inverses and AS-choice equivariance. All55279 quadratic ANF evaluations of the actual332bit map matched independent polynomial convolution. The degree bound certifies the identity on all GF83^4. Worker14.743995166seconds; peakRSS63008KiB. These certify coefficient maps, not an original-root search.

A portable verifier using a different bitwise multiplication and literal convolution independently replayed all four counts149760,25600,31126,55279. It passed in32.513825593seconds at12416KiB. Run REPLAY_RECORDED.py after restoring the three raw.json datasets if artifact tooling relocates them.

A third frozen experiment compared4096 original-coordinate h/k/r/s inputs including zeros. The competitor is a factored, square-reusing specialized A0 direct norm formula. Both pay generation/JSON parsing, field construction, preflight checks, full literal verification and full546503byte output serialization. The new arm pays AS setup and inverse coordinates. Parent cold clocks include startup through stdout/termination. All six outputs matched.

| Arm | Median cold seconds | Median evaluation seconds | A/A spread | Child peak RSS KiB |
|---|---:|---:|---:|---:|
| Factored direct |2.376623803|0.774118030|3.095%|15972–15980|
| Normalized, conversion paid |2.172464668|0.628994213|6.659%|15972–15976|

Exploratory paid component ratio1.093975814; evaluation-only ratio1.230723612. Parent memory65620KiB is separate; worker13.680127384seconds. No qualified isolation, hard all-roots baseline or cross-field speed claim follows. The modest scalar result motivates block reuse rather than another scalar retry.

Raw SHA256: k0 86208cadba8b473551b31c7caae6171cddd1e85773aa125c0b573f211aa9c12f; allk e5d314dec3b2baf029004d7e6c9ec66f05a5108e9d52b49a04905cf51966ae82; cost 0632c4864d0157cbff47211ded83c9ff6ffd58ef262c59e82c8b342f7974d99a.

Three immutable preregistrations executed via lab.runner resume/run-ready, each90seconds/4GiB/onecore,78second progress/84second reporting guards. Prior attempts/source/raw are unchanged. Preparation is sole-model reviewed; independent algorithmic oracles/replay do not imply external source admission. Private management records remain separate.

Primary literature: Fong, Hankerson, López and Menezes, Field Inversion and Point Halving Revisited, IEEE Transactions on Computers53(8),2004, Sections3.1–3.2, Lemma3.5: https://www.ece.uvic.ca/~fayez/research/gf3m/fong_j04.pdf . Their lambda²+lambda=x+a equation, two solutions and odd-degree half-trace are established prior art; historical inversion costs are not this backend's timings. The normalized norm/compiler still needs broader novelty review.

Next test exact E4-block reuse across hcharts at fixed k, preserving zero/rank-drop branches and charging setup, conversion, Schur elimination and full-fiber reconstruction. Prime and odd-extension routes retain separate proofs and benchmarks. The requested8x goal and immutable genuine F4/F5/F6 gate remain open: no complete original339variable roots, full source/support/ghost/chart closure,20hard matching inputs or qualified all-field end-to-end gain.
