# CT343 — multiplication-free Prime31 character coefficient evaluation

2026-11-24 UTC. [pcc:unverified]. Additive arithmetic qualification on draft #5066; frozen parent 7027b80f723b56431d5d50b6f66676bc9145f548. [Prospective protocol](https://github.com/isogeny-labs/crypto/pull/5066#issuecomment-7118455531).

## Derivation

Reuse CT343_PRIME31_DIVISORS.md and CT343_CHARACTER_CERTIFICATE.md. On the original six-root model, in odd chart z, c=-120, selected roots a=1 and a=1/2, p=2147483647. Both -c=120 and 2 are squares mod p. For monic U of degree n, homogeneous evaluation j^n U(1/j), j=1 or 2, has the same nonzero squareclass as (-c)^n U(1/j). Thus degree one uses 1+j*u0; degree two uses 1+j*u1+j^2*u0. For j=2, use shifts rather than general multiplication. Canonical coefficients give nonnegative inputs bounded by 6p+1. Mersenne fold (x & p)+(x >> 31), then at most one subtraction of p, gives the canonical residue. For U(a)=0, the ramified factor sign is precomputed from c^3 F'(a): bits 1 and 0 respectively. For degree two the remaining factor U/(z-a) at a is u1+2a; multiply by j (square-neutral), yielding j*u1+2. Reject zero there (repeated branch support). Identity returns zero; degree-one ramified support returns its constant bit.

The online nonzero character evaluation uses Jacobi symbol (q/p), computed via stripping powers of two and quadratic reciprocity, not Euler exponentiation. This is an established integer-arithmetic method, not a new Jacobi algorithm. References checked: prior CT343 source reports, Stoll 2001 bibliographic record https://doi.org/10.4064/aa98-3-4 ; Brent and Zimmermann 2010 abstract https://doi.org/10.1007/978-3-642-14518-6_10 (fast Jacobi algorithm, distinct from the elementary implementation here). KB/Conductor unavailable; no novelty clearance.

```
Monic U -> homogeneous coefficients -> Mersenne fold
                                      | nonzero -> Jacobi bit
                                      | zero    -> ramified constant XOR quotient bit
```

No general modular multiplication or exponentiation online, conditional on fixed precomputed curve constants and canonical input. Integer remainder, shifts, branches, additions and input checks still cost time. The elementary Euclidean-style loop takes O(log p) word divisions/remainders, not constant-time execution; observed counts are not worst-case bounds. At Prime31 all values fit in uint64. No claimed speedup over optimized binary Jacobi or an addition-chain Euler implementation.

## Frozen arithmetic-only result

| Quantity | Observed |
|---|---:|
| Synthetic polynomial entries | 2059 |
| Character agreements vs original Euler formula | 4118 |
| Repeated ramified rejections, two implementations | 4 |
| Jacobi calls | 4114 |
| Integer remainder steps | 43460 |
| Maximum remainder steps per call, observed | 18 |
| Two-adic strip iterations (including zero shifts) | 46332 |

These are monic synthetic coefficient vectors, NOT 2059 valid Jacobian classes. They include degree 0, 1024 seeded each degree 1/2, explicit simple branch controls, and separate repeated-branch negatives. Both normal and optimized Python yield identical JSON. Two intentionally corrupted variants (--bad-scale, --bad-branch) each fail in both modes. No prior divisor experiment is rerun. Source [ct343_prime31_homogeneous.py](ct343_prime31_homogeneous.py), [raw](CT343_PRIME31_HOMOGENEOUS.json). Reproduce with python3 ct343_prime31_homogeneous.py > local.json; python3 -O ct343_prime31_homogeneous.py | cmp - local.json; cmp local.json CT343_PRIME31_HOMOGENEOUS.json. Run each mutant in both modes and require nonzero exit. Source SHA256 666660b68aba1c4532fba14ac8ecc85f0a757cf560cda7ba026d281aa2ab785d; raw SHA256 66db51f26d0a82c15534e17ca66db5c2a2ef0be98e5d64c464291b8494f06814. Remote Git blobs matched 37384b74c5d6f7088d11ca17926713759de2bd5d and 6f74dec6c924adb7946f05f7bef3e5f970869937.

## Scientific cutoff: odd-order source

If H is a known odd-order cyclic subgroup, every homomorphism H -> F2 is zero, because multiplication by two is bijective on H. A transfer whose kernel is 2-primary is already injective on H. Consequently character-lift optimization cannot reduce odd-order scalar ambiguity, since none exists. It can serve as a membership/coset check outside H, but cannot certify H membership by itself. Furthermore, if an elliptic source maps via algebraic homomorphisms through a split Jacobian into a product of elliptic factors and the composite is nonzero on a prime-order subgroup, at least one component is nonzero there and preserves the full prime order. The nonzero elliptic morphism is an isogeny; the endpoint comparison must include a direct mapped elliptic DLP, not assume that a split higher-dimensional representation creates a cheaper solver. This does not exclude an actually cheaper algorithm on an isogenous model, nor nonsplit Jacobian IC. It places the next research emphasis back on the norm-reuse and genuinely nonsplit/alternative algorithm lanes, rather than continued character optimization for odd-order DLP. [pcc:unverified]

## Handoff

What was done: read current AGENTS, ACTIVE, latest Nov24 shared log, root #3877 merged, draft #5066, persistent #3904, prior CT343 source/report/task-local log, #5007 merged, cryptanalysis #1523 merged. Search for open norm-point work and autoresearcher CT343; no absence claim from search. Batched independent reads; workflow run listing on parent returned empty (not a green gate). Nov23 shared journal returned empty content; preserve unchanged. Registered protocol before local arithmetic; one coordinator, no native agent slots, Rust or signed verifier; source and raw readback matched Git blobs. No parallel elapsed gain claimed.

Decisions: additive arithmetic only; no paid compute, original 695/701 norm-reuse, new curve points, timed comparison, CI rerun, merge or locked vault. No benchmark leaderboard changes, since no measured performance. Existing incumbent retains C169/C13a/C13b/N2_actual/D and matched cold rho/BSGS/IC. Do not keep optimizing character lifts as a route to odd-prime-order advantage.

Open: independently owned native evaluator and PCC, physically priced complete cold comparison only if even-order membership/coset use justifies it; prioritize existing norm reuse admission and actual distinct transfer mechanism. Shared journal/worktree, repository PDF packaging and required gates remain unresolved. Remain draft.

Errors: all intentional mutated formulas failed as expected, no unexpected arithmetic failure. No Rust toolchain/Conductor/KB exposed; no signed PCC, no attempt to bypass. [pcc:unverified]
