# Preregistered Prime31 transformed-slot S3 chain, 2026-11-19

## Hypothesis and exact test (frozen before implementation)
Existing #4812 uses the eight ordinary leaf x-coordinates, a Horner doubling chain, and a 14-variable degree-10 eliminated system. Replace its leaf x-values with the **public** slot-transformed abscissas
```
T_i = { x([2^i]P_j): j in one eight-point window },  i=0,...,7.
H_i(X) = product_(t in T_i) (X-t).
```
Write `z_i=x(sum_(j=i)^7 [2^j]S_j)`, set `z_7=t_7`, and substitute the public target `z_0=x(Q)`. Enforce `H_i(t_i)=0` for eight slots and
```
S3(t_i, z_(i+1), z_i) = 0  for i=0,...,6.
```
This is an exact **finite x-chart** for signed eight-slot relation membership: 8 transformed leaf variables, 6 intermediate variables, 15 equations, max total degree 8 (degree-8 H_i and total-degree-4 S3); **no doubling equations**. It is not an F4 benchmark or extractor. At exceptional partial sums equal to infinity, a separate projective chart is required; count and report omissions.

The variable-eliminated *alternative* sets
```
K_i(U,V) = Res_X(H_i(X), S3(X,U,V)) = product_(t in T_i) S3(t,U,V)
```
for i=0,...,6, with `H_7(z_7)=0`, `K_i(z_(i+1),z_i)=0` and `z_0=x(Q)`. This is 7 unknown suffix abscissas, 8 equations, max total degree 32. `K_i=0` is logically equivalent to existence of an allowed leaf x in that slot. It MUST NOT be advertised as faster merely because it uses fewer variables; degree, monomial count and F4 cost might explode.

## Registered source / instance / custody
- Original unchanged curve: `p=2147483647, a=847514846, b=2087444724, n=2147446831`, cofactor one, generator `(625037715,2100627251)`; native `Arithmetic`, `PRIME31`, `check_curve` loaded from pin `1df714645aa683cf975bc76fcacb9842cb4780fd`.
- Original first 31 ascending ordinary x with the four B8 windows `0..7, 8..15, 16..23, 23..30`.
- Fixed seed `0x54524e5333463301`, SplitMix64 sequence, 128 planted signed words/window (512 total), 8 selections and 8 sign bits per word; repeat leaf selection permitted.
- Positive gate: all finite planted chains satisfy all `H_i=0`, all seven `S3=0`, all seven `K_i=0`, and independently recovered signed original leaves add to **public** Q. Post-solve sign lift must not consult planted signs.
- Negative gates: one wrong-slot root must violate `H_i`; a mutated target x must violate `K_0` on retained suffix values (except count exact accidental zero events, do not silently discard), and ordinary group identities must reject altered target points.
- Independent checker in Python with separately implemented affine group operations, S3, slot-root evaluation and sign recovery; original experiment code in Rust only, loaded from pinned crate. Keep original group operation use confined to fixture production and post-solve verification, not algebraic extraction.
- Run original crate 40 tests, focused example tests and a release 512-case experiment in GitHub Actions; serialize full summary and deterministic first-case certificates; compute source and output SHA256.

## Falsifiers and exclusions
- If transformed supports overlap or have nonordinary x-values, record a failed assumption, do not force an eight-distinct-root claim.
- If any finite planted chain violates a polynomial, or sign recovery fails, reject the formulation.
- State separate finite-chart exceptional count. Do not turn a planted relation into a claim of public-target-only unknown-index extraction.
- Explicitly record `public_only_unknown_index_extraction=false`, `complete_ecdlp_solved=false`, and `pcc_signed=false`.
- No benchmarking claim until an F4/remainder solver recovers unknown slot indices on sealed 46-positive/18-negative controls without generic sum tables, then supplies the existing rank31/1024-log Rust pipeline and same-runner fully charged index calculus, shared BSGS and optimized rho comparisons.

## Prior art / ownership
- #4812: original 14-variable degree-10 rational Horner chart, degree-4 21-variable chart; #4692: existing public-only F4/remainder extractor; #4822: sparse Wiedemann; #4816: fixed *unweighted* quotient (not interchangeable); #4728: complete Rust W8 generic grader; merged #4532: fastest same-runner shared BSGS, 170.113 ms/1,024 logs. The present transformation is a sparse *encoding* hypothesis and can be an algebraically equivalent reparameterization with no speed advantage.
- Semaev S3 and point decomposition are classical, see Kousidis–Wiemers (2019), Kosters–Yeo (2015), and PKM 2016. Shared KB MCP is not available in this runtime; accessible repository and web sources checked, no priority claim.
