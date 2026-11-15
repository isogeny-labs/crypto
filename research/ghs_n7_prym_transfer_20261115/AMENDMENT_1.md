# Amendment 1: classical genus-seven collision

2026-11-15 07:15 UTC. The frozen `PROTOCOL.md` proposed a potentially new
genus-seven Prym transfer for a symmetric slice of the n7 W1 family.
After the q2 structural run, a primary-source check found a decisive
collision in Menezes--Teske (2004), §2 Remark 2. For every symmetric
coefficient, `b` lies in one of the irreducible cubic Frobenius kernels.
The same source curve therefore has the classical GHS presentation
`gamma1=1, gamma2=sqrt(b)` with magic number four and genus seven.
The q2 D4 target is explicitly the classical target after `s↦s+1`.

**Disposition:** stop the planned q16 symmetric-Prym novelty and cost
arms under protocol falsifier 4. Keep the q2 computation as a positive
control for subgroup-map software. Reclassify the candidate as a
rediscovery of a known genus-seven attack target. The exact
`2(q^3−1)` coefficient criterion and symmetric-ansatz exclusion are
recorded in `THEOREM.md`.

This amendment preserves the original protocol and its checkpoint
history. It does not retroactively redefine the run or its expected
values. Source: <https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf>,
Remark 2, PDF p. 5.
