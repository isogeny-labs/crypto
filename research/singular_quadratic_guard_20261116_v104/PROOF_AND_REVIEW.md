# Exact local replacement with a fully covered singular branch

The oracle tests the finite premise left open in C103: all cubic false
positives in each actual GF32 fixed-trace coefficient cube equal precisely
H=(X+t)Q^2 for irreducible monic quadratics Q. It compares complete2^20-bit
masks and independently tests every quadratic for a root in the field.
If equality fails, compiler/search dependencies do not pass. No result from
counts or sampled examples is promoted to the finite certificate.

Assuming that certificate, the cubic predicates are exact off the singular
stratum h2=t*h3,h0=t*h1. On it, recover Q=X^2+sqrt(h3)X+sqrt(h1). If h3=0,
Q is split with a repeated root. Otherwise Q splits iff
Tr(sqrt(h1)/h3)=0. No inversion of zero is attempted. The exhaustive
quadratic root oracle is independent of this trace formula. Multiplicities
and zero coefficients are included, not excluded. Smaller residue fields
use their already exhaustive-certified cubic predicates with no extra guard.

Each relation basis is weight-reduced only by r_i<-r_i+r_j,j!=i with strict
weight decrease, hence unchanged span. Independent rank checks confirm both
spans. Squarefree feature monomials are built once, and parity chunks combine
them by exact XOR. The 10-bit bad-quadratic lookup is compiled as a reduced
decision diagram, with each mux equal to low XOR(select AND(low XOR high)).
The final reject wire is bad_Q AND all_zero(singular_residual). Every local
remainder bit is supplied by the SAME76global parameters and SAME3q bits.
Leading/infinity guards retain their exact C101 binding.

Compiler controls compare local circuits to literal factor division and32
actual parameter assignments to the original numeric nilpotency predicates.
All2^20 local states are certified mathematically by the whole-mask premise,
span invariance and exact Boolean gate identities; no claim that32 samples
alone prove equivalence is made. Search reconstructs the circuit cold, checks
its frozen graph hash, and charges reconstruction and native formulation
inside the same45-second budget. Worker lifetime includes field setup,
model checking, serialization and final reporting under90-second outer
supervision/4GiB/one core. After78seconds worker results are demoted.

Every native model is independently checked in the old numeric predicates
and full prefix support. Unsupported H can be blocked across q aliases;
supported H pauses for original source/U recovery. Native UNKNOWN and UNSAT
are never independent original completeness certificates. This exact local
replacement does not cover every divisor chart/ghost branch or constitute
a general all-roots solver. No novelty or qualified speed ratio is claimed
for finite tables, Boolean interpolation, XOR circuits or trace algebra.
