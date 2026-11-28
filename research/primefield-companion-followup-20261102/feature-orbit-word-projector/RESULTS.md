# Full orbit-word projector and all-mode phase

Validation02: 220 transcript systems and 1,140 exact image-membership checks
passed, including L=1. There are 164 phase checks, three supported symmetric
images correctly left without a unique phase, and six strict image-support
reductions relative to the three-mode equations. These are deterministic
fixture counts, not prevalence estimates. [pcc:unverified]

The public-point path checks E/F17: y^2=x^3+2x+2, P=(5,1), r19, m3 for every
k0..18; and E/F257: y^2=x^3+x+7, P=(1,3), r281, m5 for
k=[0,1,2,3,7,17,31,63,127,280]. Radii are [2,1,3,2]. All29 cases return a
point-verified scalar:18 from the extracted image and11 identity/endpoints.
There are332 nominal source queries,27 guard queries, and2,600,842 entered
monomial pairs for nominal recovery. Setup/calibration is additional. No image
or scalar is supplied to the recovery function. [pcc:unverified]

The algorithm multiplies zero-safe sign selectors in X^m=Y and normalizes its
zero section by m/t, with t the observed rotational stabilizer. This removes
generic zero-testing powers. Its explicit node/indicator representations remain.
The previous selected-terminal path recorded685,238 public pairs on these
fixtures; the new construction has a larger pair count and retains more input
information. No clock, total-work or speed ranking was measured.

At r998244353, m7 and L71,303,168, the circuit has11,379 nodes and evaluates to1
at the fixture image. Extraction refuses the64-term cap after14,594 monomial
pairs and5,696 retained coefficient slots. The known-image check and capped
unknown-image extraction are distinct outcomes. [pcc:unverified]

Synthetic period2 and period3 sign words have joint stabilizer1 while both
oriented Fourier modes vanish. This demonstrates why all-mode Bezout can add
an anchor; it does not establish a positive curve transcript with that pattern.
96 field Bezout identities, six ternary selector checks, four capacity controls,
and malformed-zero-word rejection pass. No native compile/assertion failures.
Validation01 (before L1 coverage) is retained with its source snapshot.
[pcc:unverified]

Ten copied files and14 compiler dependencies are hash-bound in both receipts.
Native arithmetic is Rust; Python creates custody/report artifacts only. The
original4563-case cohort and older adapters are unchanged. Full Cargo suite and
inherited cohort were not rerun for this standalone component. Unsigned producer
evidence; independent signed review pending. Read DERIVATION.md for domain,
proof, source review and accounting; validation02/check.stdout for exact counts.

Next: selected trace evaluation through the structured word product, retaining
costs of operands/dual states. The explicit indicator still has the singleton
support lower bound L. Source queries, shifts, DFT, stabilizer comparisons,
inversions, maps/copies/allocation and final group verification must all be counted.

Claim record: PCC-20261108-48ab67, unsigned producer evidence. [pcc:unverified]
