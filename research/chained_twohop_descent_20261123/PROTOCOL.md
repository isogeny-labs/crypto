# Frozen finite-kernel qualification, before implementation

This is an exhaustive structural development unit, not a benchmark, curve
search, original full Jacobian workload, or replacement for an EKS requirement.
All code and the actual caller closure will be published and read back before
the first compiler/proof/native call. No gain criterion or empirical pricing.
The first result, including any compiler or invariant failure, is retained
before a source revision. [pcc:unverified]

## Fixed native population and independently represented premises

1. A Rust std-only executable represents V=(F2)^4 in a fixed symplectic basis,
   independently compared with even subsets of six labels modulo complement.
   Basis pairs are {0,1},{1,2},{3,4},{4,5}. Compare every one of16 elements,
   every ordered addition/pairing pair, and every one of720 root permutations.
   Require720 distinct symplectic actions. Fixed-point-free actions must have
   cycle types(3,3),(6),(5,1), with40/120/144 actions respectively.
2. Enumerate every15 Lagrangian L in V. Inside G=(Z/4)^4, form
   P_L={x:2x in L}, then the16-element quotient P_L/L. Enumerate its15
   Lagrangians with the induced pairing and take their full16-element preimage
   kernels. Require225 routes, grouped8 good/6 mixed/1 dual per L, with
   |2K|=4/2/1 and |K[2]|=4/8/16. Require151 distinct composite kernels:
   120 good,30 mixed,1 dual. Check closure, orders, isotropy and full element
   sets; no target or determinant-based deletion is allowed.
3. Fixed representative permutations: (0 1 2)(3 4 5), (0 1 2 3 4 5),
   (0 1 2 3 4)(5), and positive-control (0 1 2)(3)(4)(5).
   For each matrix modulo2, enumerate all65536 entrywise lifts M+2H modulo4.
   Keep exactly the symplectic similitudes with multiplier1 or3, testing the
   entire matrix relation. Require1024 lifts of each multiplier per
   representative. Do not impose an order-three hypothesis on the lift.
4. For every retained lift and every225 route, compute full-kernel stability.
   For the three fixed-point-free representatives, require that any stable K
   with nonstable L is the full two-torsion kernel. Check directly that every
   good K has L=2K, and every stable mixed K fixes its unique nonzero doubled
   element. This is a fixed representative/lift validation; global conjugacy
   and geometric realization are not inferred from sampling representatives.
5. Retain counterexamples to overclaims: require a stable good K with stable
   L in the fixed-point-free set, and a stable mixed K with nonstable L in the
   positive-control action that has rational two-torsion. Print full first
   witness matrices/kernels deterministically. A copied mixed-kernel record
   with a corrupted doubled-image invariant must be rejected and its restored
   record accepted. The validator must keep every key it reads explicit.
6. Emit every225 full kernel bitset with its L, group type, doubled image and
   stability counts for all four representatives. Also emit complete action,
   route, lift, control and acceptance summaries. No timings or ratios. The
   complete stdout is designed below64KiB; never truncate it. If it exceeds
   policy, preserve it whole and keep raw publication pending rather than
   change the population or split/rename it to fit.

The theoretical cardinality checks above are fixed from the standard finite
symplectic/kernel classification before implementation, not fitted to output.
Source review must check that the quotient pairing is well-defined and that
the two-torsion and doubled-image sets are not conflated. [pcc:unverified]

## Lean component

A separate explicit binary-coordinate model of C4 x C2 x C2 proves its doubled
image is {0,p}, p is nonzero, and every additive injective self-map fixes p.
Prove the corresponding no-fixed-point-free-automorphism consequence and
doubling-image covariance in C4 x C4. A finite Bool model checks the fixed
vectors of the cubic action. Use kernel reduction/ordinary tactics, never
native_decide, sorry, new axioms, custom trust or resource-limit overrides.
Print axioms for every named theorem. The abelian-variety kernel classification,
full arbitrary Galois action and Picard realization remain outside this module.
All formal statement bindings must use full declaration text. [pcc:unverified]

## Frozen execution and custody envelope

Rust1.99.1; Lean4.35.0; no package/dependency download or Python scientific
arithmetic/orchestration. Thin shell caller, Python file/hash/JSON metadata only.
One worker,120s per phase,2GiB virtual memory,256MiB free RAM and64MiB scratch
floors. Strict Rust compile, Clippy -Dwarnings, rustfmt check, Lean warnings as
errors, then one immutable native qualifier call. The native code has no
threads, subprocesses, external files or network input. [pcc:unverified]

Copy and hash the complete actual source/caller/config closure, verify public
Git source before calls, and retain distinct build/pre/post manifests with tool
and qualifier hashes/modes. Preserve full stdout/stderr and exits for every
phase, including the expected corrupt-record rejection. Publish all small raw
streams promptly in GitHub so a tmpfs restart cannot erase their only copy.
Any full compiler/ELF packet held only in tmpfs remains explicitly temporary;
no durable store claim until approved storage or ordinary CI artifacts actually
retain it. No inline compressed data or policy changes. [pcc:unverified]

The original EKS-only workloads, paidSTOPe42949, unread locked autoresearcher
vault, signed replay, complete solver costs and existing map14/15/16 records
are unchanged. All outcomes remain producer-authored and unsigned. [pcc:unverified]
