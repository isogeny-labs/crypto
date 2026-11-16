# Four-input factorization and normalized character consumption

2026-11-05. Task T-1712. Derived program identity and native development checks;
claims remain unsigned. [pcc:unverified]

## Identity

Fix the source geometry and its partition (w,k,s,e), where
h=(r-1)/2=2wk+2s+e. Let the nonzero canonical inputs be c,u,v,z:
the normalization seed, normalized half-index value T_((r+1)/2), normalized
stride value T_(2w), and normalized tail value T_(s+e). When s=0 use z=1.
The source ratio satisfies

    R(c,u,v,z) = R(1,1,1,1) c^delta u^alpha v^beta z^gamma.

Here G(n)=n(n-1)(n+1)/3, and

    delta = 2w [kw(w+1) + wk(k-1)] + 2k G(w) + 2 G(s),
    alpha = 4wk + 2s + e,
    beta  = 2w G(k),
    gamma = 2s.

These are integer exponents. The ratio identity concerns represented values;
individual homogeneous coordinates can have different common factors.
All four inputs and geometry factors must be nonzero. [pcc:unverified]

## Derivation from the actual recurrence boundaries

Use exponent-vector coordinates (c,u,v,z). The initial states -u and u both
have vector (0,1,0,0). The baby recurrence adds (2,0,0,0) to its second
difference. Baby j therefore has vector (j(j+1),1,0,0).
After w steps the two boundary vectors are those at w-1 and w. Negating the
older numerator changes no exponents. The center recurrence adds (0,0,2,0),
so center i has vector (w(w+1)+2wi,1,i(i+1),0).

The accumulator raises the product of k centers to 2w and the product of w
babies to 2k. The partial tail contributes z^(2s), the first s babies squared,
and geometry factors. The singleton contributes -u. Summing these vectors
gives the displayed exponents. Resultants and signs depend only on geometry
and partition. This proves the program factorization on its nondegenerate
domain without assuming independent algebraic freedom of actual curve values.
Substituting u=psi_t*c^(t^2), v=psi_(2w)*c^((2w)^2), z=psi_(s+e)*c^((s+e)^2)
recovers the prior one-seed exponent E=delta+alpha*t^2+beta*(2w)^2+gamma*(s+e)^2.
[pcc:unverified]

## Native consumer and retained costs

The production residual contains no value-division initialization. It executes
the frozen scalar residual with all four inputs equal to one, then multiplies
its numerator and denominator by the corresponding powers of four homogeneous
input pairs. Exponents reduce modulo p-1 only for nonzero inputs and caller-
certified prime p. Divide a factor of G(n) by 3 as an integer before modular
products; 3 need not be invertible modulo p-1. With 3<p<=u64::MAX/2, reduced
factor products fit u128. r is odd and at least7; transport inherits the
prime-subgroup and derivative-phase requirements. [pcc:unverified]

The transport adaptation appends n=1 (the seed), taking its coordinate from the
already generated public shifts. It adds no multiplier lane. It retains the
full grids from the same joint geometry pass. All normalized columns, including
the seed, share the same direct phase and value context at initialization. At
most the first two non-anchor offsets initialize directly; subsequent rows use
recurrence or public anchors. The consumer does not divide by c^(n^2) to undo
normalization. Calibration stores a homogeneous ratio and compares cross-
products, without introducing a calibration inverse. Identity returns sign0.
The conditional seed law underlying transport is inherited, not reproved here.
[pcc:unverified]

Setup, two direct initializations where needed, seed-column updates, saved grids,
per-input powers, residual recurrences, resultants, and allocations are still
costs. The consumer returns an entire requested window; adaptive cursor/skip
integration with the quartic decoder is the next implementation step. No timing
claim follows from this change. Original4563-case cohort remains untouched.

## Evidence and boundaries

Native tests compare the exponent formula with a separately iterated four-
coordinate recurrence, arbitrary independently scaled inputs against an extracted
scalar reference, and public-point signs/ratios against direct frozen jets.
The separate checker imports the frozen pre-transport scalar source, not this
consumer. It reconstructs point sequences and checks raw ratios and character
signs. It is separate code authored by the same producing session, not an
independent reviewer. See receipts and raw logs for actual outcomes.

## Sources checked

KB unavailable: no relevant MCP tools loaded; scripts/research-kb status reported
no executable. Read feature-native-seed-factor/DERIVATION.md, the complete raw
residual and recurrence boundaries in feature-native-quartic-recovery/routed/
source.rs, and feature-native-normalized-transport/lib.rs. Re-read NIST DLMF
23.10.3 (https://dlmf.nist.gov/23.10.E3) for the sigma-addition antecedent of the
transport identity. It is not a proof of this program-specific factorization.
No novelty-priority claim, characteristic-changing runtime map, or formal Lean
proof is made. Local correctness tests are development work under AGENTS.md;
no comparative experiment or cloud job was launched.
