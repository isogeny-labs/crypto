# Big goal: a usable measurement instrument at 60-bit Fp

**Status: planning + first measurements.** This file fixes the target so the
next changes are judged against numbers, not vibes and not wall-clock noise.

## Claim boundary (unchanged and non-negotiable)

Public synthetic / known-answer instances only. No sub-rho claim, no key
recovery, no statement about deployed curves. The reduction is conditional on
the unpublished Boolean tensor contraction result; without it this code is
exponential. Everything here measures *encodings and exact counts*, not attacks.

## The goal

Make `hashed-ecdl-tensor` runnable, fast, and independently checkable on a
certified 61-bit prime-order curve, with a measurement harness that a reviewer
can reproduce on one machine.

Concretely, by the end of the study:

| milestone | today | target |
|---|---|---|
| certified 61-bit prime-order curve | 0.4–4 ms (CM, harness) | < 1 s, in-crate |
| Lemma 11 network, faithful encoding | ~118k edges, ~12 ms | **< 20k edges, < 1 ms** |
| Lemma 11 network, programme-vertex mode | ~61k edges, ~6 ms | **< 20k edges, < 1 ms** |
| exact count vs brute force | 3–5 bits | **24–28 bits** |
| independent replay of every count | partial | receipt per run |
| contraction backend | CPU only | GPU exact fallback |

The two large structural costs are now identified:

1. **Two curve-validity checks** (~30k edges each). `A₁, A₂` are free interface
   bits, so each needs `y² = x³ + b`; dropping either lets off-curve points into
   the language and silently inflates the count.
2. **The modular-multiply gadget** (~9.2k edges, degree 520), which sets the
   network degree and therefore the contraction cost.

Progress therefore means attacking *those two*, not shaving gates.

## Workstreams

### A. Cheaper validity (largest single win)
- Batch the two curve equations into one constraint system instead of two
  independent circuits.
- Or move membership to a form that certifies on-curve for free (e.g. a base
  built by point addition from a known generator, which is on-curve by
  construction, so validity is structural rather than checked).
- Success: network under 20k edges with unchanged exact counts.

### B. Cheaper modular multiply
- Exploit squaring (reuse one forward transform) for `x²`, `y²`, `slope²`.
- Constant-operand specialisations that preserve the asymptotic degree bound.
- Success: `point_add` dominated by four multiplies at ≤ 6k edges each.

### C. Make counts possible above 20 bits
- The exact counter is exponential in the interface. Target a 24-bit curve with
  a *pinned* target and a small factor base, and compare against brute force.
- Success: one exact count at ≥ 24 bits with a reproducible receipt.

### D. GPU exact fallback
- Port the elimination kernel (already measured at ~120 G entries/s on an
  RTX PRO 6000) behind the same counter API, with CPU/GPU agreement tests.

### E. Repro and evidence
- One command per row of the milestone table.
- Every number in `results.txt` regenerable; every claim-check receipt stored.

## Immediate next steps (ordered)

1. Land this harness and the vendored fast copy (this PR).
2. Propose the sound encoder fixes upstream: no duplicate validity check in
   `decomposition_network`, no `valid_point(result)`, `3x` as additions,
   `a=0/b=0` constant folding, and `x²` reused from validity.
3. Replace the AKS primality test with deterministic Miller–Rabin behind a
   feature (exact below 3.3e24), keeping AKS as the default for larger inputs.
4. Attack workstream A; re-measure against the table above.

## What "significantly" means

Not a few percent. The study is only interesting if the faithful 61-bit network
drops by an order of magnitude (118k → under 20k edges) and an exact count
becomes possible at 24 bits. Anything less is a micro-optimisation and should be
recorded as such.
