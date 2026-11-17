Independent algebra before producer source inspection:

In an abelian group, assume rG=rQ=0 and R=aG+bQ. Set c in {0,1}. Then R'=2R+cG=(2a+c)G+(2b)Q. If a'=(2a+c) mod r and b'=2b mod r, subtraction of integer multiples of r does not change the represented point. Canonical inputs 0<=a,b<r<=2^63 give 2a+c<=2^64-1 and 2b<=2^64-2; these unsigned 64-bit expressions cannot overflow. Their reduced outputs remain canonical. The bound is a sufficient explicit hypothesis, not a generic assertion about arbitrary words.

For paired Kummer state (kappa(R),kappa(R+G)), let D0=kappa(2R), M=kappa(2R+G), D1=kappa(2R+2G). Choice c=0 selects (D0,M)=(kappa(R'),kappa(R'+G)); choice c=1 selects (M,D1) with the same identity. The unused double can be skipped if arithmetic is deterministic on inputs and mutable counters do not affect branch decisions or overflow. Projective equality alone does not select signed R'; signed recovery must use the propagated pair and a signed G with the required recovery hypotheses. Final independently constructed R' may check that signed result, but must not supply its neighbour or branch choice.

These identities preserve labelled state. They do not by themselves verify relation-graph insertions, rank accumulation, geometric repair totality, or a completed collector. Source inspection will distinguish those trajectories.
