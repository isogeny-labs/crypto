# Frozen order-4126 degree-three cover screen

27 October 2026. Conductor T-5502. Literature was checked before this
screen: Hess (2004), Theorems 11--14 and Section 6; Menezes--Teske
(2004), Sections 4 and 7; Tian--Li--Wang--Yu (2018) on odd-characteristic
degree-three covers; Tian (2020/2023) on cubic Weil-restriction isogenies;
Frey--Kani (2011/2012) on characteristic-two correspondences. Neither
an elliptic degree-three cover nor an isogeny of Weil restrictions is
a new attack method by itself.

## Frozen exact pair population

- Source: `K=GF(4096)` with Conway polynomial
  `w^12+w^7+w^6+w^5+w^3+w+1`, and
  `E: y^2+xy=x^3+K.from_integer(32)*x^2+K.from_integer(63)`.
  Its order is `4126=2*2063`; retain `G=(62,1499)` and
  `Q=(2633,1798)=[7]G` as the target pair.
- Targets: every one of the 140 catalog-listed genus-three
  hyperelliptic models in LMFDB isogeny class `3.16.a_a_bd`, using the
  frozen `catalog_models_raw.json` input. These are **not** the plane
  quartic witness from the preceding screen.
- The complete degree-three two-pattern Artin--Schreier enumerator and
  its positive controls are the cited preceding project scripts.

For each model, independently check genus three, that its quartic
`h(x)` has four distinct roots over `K`, and the nonzero branch residues.
Test all 24 four-support assignments and all
`4*2*(4096-4+1)=32,744` five-support assignments. Retain any
`R,t` witness, verifying the exact Artin--Schreier equation. Replay
constructed positive controls for both branch patterns and record
per-model counts and input hashes.

## Interpretation

A hit would be a degree-three map from a cataloged curve to this exact
elliptic source over `K`; it is only a candidate transfer route until
its conjugate maps descend to `k`, act nontrivially on the selected
subgroup, and a full target solve beats same-source rho. A miss rules
out these degree-three covers for the **140 listed hyperelliptic models**
and this one exact source equation. It does not exclude the separate
plane quartic, other Jacobian models, other isogenous sources, maps of
higher degree, or isogeny-only correspondences.
