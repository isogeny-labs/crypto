# Iteration 8 — frozen natural-target and negative-query correctness specification

Date: 2026-11-22 Pacific. **Specification only** [pcc:unverified]. No Rust source has been implemented or executed by this iteration; no workload has been admitted or launched. The target generator should be implemented using the **existing original-Rust point arithmetic**, not substituted with preconstructed positive tuples.

## Frozen public parameters and domain

Use the exact original Prime31 fixture inherited from the existing lane:

    p=2147483647
    a=847514846
    b=2087444724
    n=2147446831
    G=(625037715,2100627251)

The source notes describe n as prime; this iteration does not re-count the curve or prove n prime. The generator requires explicit point-order checks in a native source-bound correctness gate. Use a fixed eligible full Dickson fibre chosen and verified by the existing setup construction; report actual q, c, W, B, point lifts, and its seed/setup cost, never substitute a planted tuple to meet a desired target.

## Exact byte-to-scalar rule (Rust implementation obligation)

The public fixed seed is the 32 bytes 00 01 02 ... 1f in ascending hexadecimal order. The domain tag is ASCII IL-PRIME31-NATURAL-TARGET-V1. For each independent scalar, concatenate the following bytes exactly:

    ASCII domain tag
    seed32
    mode: ASCII REL or REC (3 bytes)
    index: u64 big-endian
    scalar lane: ASCII A or B (1 byte)
    attempt: u32 big-endian, starting at zero

Compute SHA-256 of that byte string (no delimiter, no length prefix; each field has specified length), interpret the 32-byte digest as an unsigned **big-endian** integer z, and set

    U=2^256; M=U-(U mod n).
    If z>=M, increment attempt and hash again.
    Otherwise u=z mod n.

For REL set b=0 and sample a from lane A. For REC sample a from A and b from B; reject b=0, increment the B attempt and retry. Construct the public target

    R=aG+bQ,

where Q is a separately fixed nonidentity public target point, known to be in the same order-n subgroup; REL has no Q input, and is R=aG. If R=O, advance the **whole target index** to the next index and record the discarded target and its cost; never condition on success of relation extraction. Because a is uniform modulo n before excluding R=O, conditioned on R!=O the ideal random-oracle model yields a uniform nonidentity R, even with REC b!=0. Deterministic SHA-256 produces reproducible pseudorandom samples, **not** a mathematical independence or uniformity theorem for its fixed seed; exact finite-population yield identities assume ideal uniform targets and must be labelled as model statements.

Record (mode,index,attempts_a,attempts_b,a,b,R,group-operation-count,hash-count,accepted/rejected-reason). Generate all targets **before** invoking either candidate algorithm, with labels and order frozen. Do not choose relation-friendly scalars after seeing outcomes. Reserve distinct ranges for development correctness versus held-out cost comparisons; do not reuse a planted-positive control as a blind random target.

## Original-Rust correctness controls to implement

1. Fully enumerate **all** nonidentity targets on the existing small-field F101 prime-order group, and compare the candidate's signed multiset outputs and certified misses to the exact group-semiring contraction from EXACT_CONTRACTION_AND_YIELD.md. Include duplicate points, negative signs, pair subtotals O, and tuple P,P,P,-P.
2. Reconstruct the freely permuted left-associated affine S3 chain for every returned multiset with total R!=O. Check all internal prefix sums finite, Q1-Q3, full-Dickson membership and exact signed group replay. Do not require a fixed balanced-tree solution for P,P,P,-P.
3. Test at least one rational-projected S4 ghost from the frozen iteration6 controls and demonstrate that a closure root **without** rational auxiliary orientation cannot pass the signed verifier. Never count algebraic multiplicity as an extra relation.
4. For Prime31 B31, generate a frozen finite panel of 4096 REL targets and a separate 4096 REC targets using the rule above. These are **correctness and natural-target** controls only, not statistical or performance admission. Retain every miss (expected to dominate under an *ideal* uniform model), every witness, failed verification and hash identity. The accepted target count, rather than a selected positive-only subset, is the denominator.
5. Add independently replayed signed rows, coefficient-vector hashes, sparse rank increments and explicit target-recovery equations before claiming a complete solver. A raw signed tuple count is not a rank measurement. Apply the full repository checkpoint / EKS experiment and validation gates before *any* performance study.

## Charged phase accounting

Charge fibre choice and failed samples, rational sqrt/lifts, point storage, H table build and collisions, all group additions and inversions, SHA hashing/rejection, per-target group scalar multiplication and addition, empty-target lookup, output emission, full group verification, duplicate signed-row filtering, matrix assembly/solve, and final public Q recovery. Retain wall time, peak RAM and strongest maintained index calculus plus negation-map rho as separate matched comparisons **only after** preregistration and admission. Report every timeout/failed root isolation as incomplete, never as an empty certificate.

## Status

Freeze is a written proposed protocol, **not a compiled native source freeze** and not a signed proof. The active repo source still has unrelated root-library manifest/parser failures in retained original-lock receipts; their owners must reconcile source changes before attributing any new native CI result. The required cost advantage and low-query-exponent target remain open.

[ pcc:unverified ]