# Source correspondence

Pinned native_solver.rs SHA256 061f4f9f45d261b61af3482509ca5f09e03964abc670765852922d20f946bbd1.

- 812–821: native add is (a+b)%p; imported word_add_decode derives correct modular addition from canonical operands and p<=2^63.
- 963–968: source active coefficients/padding are model inputs a,b,c,e,f. Source slice validity and degree selection are not proved by this model.
- 969–972: initialization (a,add(b,c),e), constant dd=add(c,c), slope f.
- 973–975: source range emits old ux/vx; observations model indices in List.range count. The actual Rust iterator is not formally refined.
- 977–979: all three updates use old values; step explicitly reads the previous triple. Final update is retained even after last observation.
- 980 onward: common-fold theorem compares identical branch functions on identical decoded observations. It does not independently verify the Rust exceptional branches, complement code or collector.

Nine new theorem declarations and eight controls passed named axiom readback. Arbitrary-iteration word refinement is now composed; full Rust semantics remain separate. Canonical natural representatives are not a separately named theorem in this packet.
