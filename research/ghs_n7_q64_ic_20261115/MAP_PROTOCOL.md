# Frozen q64 point and subgroup-map gate

15 November 2026 UTC. Freeze before selecting P or submitting Magma. The immutable first qualifying row of `screen.json` is `K=F2[z]/(z^42+z^7+1)`, `a` mask 34359738368, `b` mask 4169, `#E=4398047737114=2*2199023868557`, with prime `r=2199023868557`; `k=F64` and `#E0(k)=74`. The source lies in Hess/Teske's published n7 genus-eight selector.

Run `select_point.py`: enumerate x masks 0 through 65535, sort `E.lift_x(x, all=True)` by y mask, and choose the first R for which `P=2R` is nonzero and `rP=0`. Set `Q=1234567P`. Save all source, point and target masks, selected row, Sage version, script and screen hashes in immutable `point.json`. Stop if no point is found.

After point selection, commit an exact Magma source with these frozen masks and assertions `#E=2r`, r prime, P nonzero, `rP=0`, Q nonzero, `Q=1234567P`, `WeilDescentGenus(E,k,1)=8`, `C,h := WeilDescent(E,k,1)`, genus eight, `G=h(P) ne 0`, `rG=0`, `h(Q)=1234567G`, and homomorphism checks at 2 and 3. Record the target equation, embedded subfield generator, image classes, source hash, response, and timing in an immutable receipt. The map is a subgroup-preservation gate, not a cost or new-family claim. Preserve any assertion failure, timeout at 180 seconds, response truncation, or transport error.
