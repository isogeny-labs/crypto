# Frozen s=11 base-change transfer check for degree-seven covers

16 November 2026 UTC. The five exact degree-seven covers in `degree7_quadratic_square_*.json` have verified nonzero order-109 traces to `J(Y)(F3)` under `DEGREE7_SUBGROUP_MAP_PROTOCOL.md`. This experiment tests whether their explicit rational-map pullback and Galois trace remain nonzero on the selected 69-bit prime subgroup after coprime base change `s=11`.

Fix `K=F3[a]/(a^5+2a+1)`, `L=F_(3^55)` using the Sage modulus and first embedding saved in `primitive_isogeny_s11.json`, and `k_s=F_(3^11)`. For each selected b-mask, base-change the exact A,B,C and Qi coefficients from K into L. The source group order is `N=174449211009094048378706606=2*109*2399*r`, `r=333566377062870325133` prime. With a frozen seed, obtain a nonzero `P=[N/r]R` from an L-rational point R and check `[r]P=O`.

For the degree-seven map, compute the corrected pullback `D(P)=(U,V)-(Qi,0)` exactly as in `DEGREE7_SUBGROUP_MAP_PROTOCOL.md`, now over L. Verify `[r]D(P)=0`. Compute `T_s(P)=sum_(i=0)^4 Frob_(3^11)^i D(P)`, verify `[r]T_s(P)=0`, nonzero image, and `Frob_(3^11)(T_s(P))=T_s(P)`. Verify scalar preservation on `37P` by a fresh divisor preimage and trace. If T_s vanishes, retain that failure and do not call the map subgroup preserving over k_s. This check is a transfer result, not a full GHS decomposition or attack-cost benchmark.
