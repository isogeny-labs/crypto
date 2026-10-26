# Sources and use

Tate's integral homomorphism theorem for finite fields identifies
Hom(A,B) tensor Z_l with Frobenius-equivariant maps between their
l-adic Tate modules, for l different from the characteristic. The theorem
and its approximation argument are stated and proved in
[Sam Lichtenstein, Tate's isogeny theorem, Theorem1.1 and section3](https://math.stanford.edu/~conrad/mordellsem/Notes/L03.pdf).
This primary lecture proof was read on23 October2026 UTC. An attempted
fetch of the original1966 paper from the CNRS mirror timed out; no
unread excerpt from that attempt is used.

CM.md gives our deduction using the actual squarefree mod11 factors and
a rank-one lattice over each Hensel factor. This deduction is a written
argument, not a theorem quoted from the source and not an evaluated
correspondence. The modular/newform construction and its smooth compact
interpretation remain those of frozen Round15 CM.md and Round16 CM.md.

For the odd-cover construction, the input is frozen Round16
THETA.md and g16-diagonal-cover-section-bases.json. The elliptic group law,
Miller divisor recurrence and normalized Poincare correction are given
explicitly in THETA.md here. Native tests use the maintained binary-curve
arithmetic; the universal correction identity has a new source-bound Lean
proof and IDC record. Complete field/Miller/modular semantics are not
silently assumed to be formalized.

Saturation uses only the actual exported source DAGs and the Round16
universal quartic residual. Its source-bound IDC1h84c81bd7c932e9cf remains
frozen, with the actual Round17 input/node bindings generated separately.
