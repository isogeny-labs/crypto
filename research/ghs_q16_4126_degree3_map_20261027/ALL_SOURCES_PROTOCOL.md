# All-source order-4126 degree-three cover census

27 October 2026. This extension starts after the one-source exhaustive
certificate and keeps that frozen input intact. Use all 108 exact
normalized ordinary source equations at order 4126 from the independently
replayed GF(16) cubic closure certificate, crossed with all 140 listed
hyperelliptic models in catalog class `3.16.a_a_bd`.

The published degree-three Artin--Schreier branch-pattern classification
used by the earlier exhaustive screen remains the completeness argument.
This implementation replaces a 4,093-element hidden-point loop with an
algebraically equivalent residue equation. For each chosen single branch
point `s` and each orientation, let `r` be another branch point,
`H=h²`, and `c` be the known ratio of the two requisite residues after
eliminating the scalar. The necessary equation is

```text
c = (r²+H)/(s²+H),
(c+1)H = r²+c*s².
```

If `c=1`, no finite `H` works because `r!=s`; check the separate
infinity candidate explicitly. Otherwise this gives the sole possible
finite hidden point. Check the other two branch residues and the full
Artin--Schreier coboundary equation. For the four-support pattern,
apply the necessary simple-pole residue equality before the full
coboundary test. Run the original enumerator against a frozen exact
source and the two constructed controls to test that the filters retain
all its hits and produce the same zero result.

Record each pair and witness, exact source and model identities, full
candidate counts, control results, and hashes. A negative result covers
only these 140 listed hyperelliptic curves and these 108 normalized
elliptic equations, for separable degree-three curve maps over `K`.
The plane quartic and isogeny-only correspondences remain outside scope.
