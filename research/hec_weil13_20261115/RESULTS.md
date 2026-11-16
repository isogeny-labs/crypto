# Root-free genus-3 Weil evaluations

Both stored 27-element kernels now have explicit curve principal functions
and unnormalized Jacobian Weil-function evaluators on the registered generic
Mumford chart. The implementation uses only F(127^13); it does not construct
the source branch roots or their degree156 compositum with the kernel field.
This removes that dependency for this evaluator, not for every later quotient
reconstruction algorithm. No runtime ratio was measured.

## Derivation
For a nonzero kernel class P=[E-3infinity], E=(u,v), choose h=a+b*y with
poles bounded by9infinity. The basis is 1,x,x²,x³,x⁴,y,xy. A Hensel lift
w of v modulo u³ converts triple vanishing on E to a+b*w=0 mod u³. A9x7
homogeneous solve has nullity1 on every registered nonzero input. Normalize
its first nonzero coefficient to1. Exact checks give

    a²-f*b² = c*u³,  c != 0,
    a+b*w = 0 mod u³,  deg a<=4, deg b=1.

Since u is squarefree and coprime to v, this certifies div(h)=3E-9infinity:
all three branches have multiplicity at least3, norm degree9 supplies exact
multiplicity and no extra zeros, and the xy term has pole order9.

For theta=2infinity, H0(E+2infinity) has basis 1,x,(y+v)/u. These are regular
away from E and infinity, have the required pole bounds, are independent,
and Riemann-Roch gives dimension3. H0(5infinity) has basis1,x,x².
For X=(U,V) on the generic degree3 squarefree chart with gcd(U,u)=1, put
r=(V+v)/u mod U. The determinant ratio is the coefficient r2 of x². Hence

    eta_P(X) = r2³ * Norm_(K[x]/U)(a+b*V).

This is the specialization of Tian Eq2.2, with his eta_P=f_(-P) convention.
The norm is a3x3 multiplication determinant; arbitrary roots of U are not
needed. The functions are unnormalized. Identity eta is1. Division failures
or unsupported charts are explicit exclusions, not supplied by this evaluator.
The source's theta normalization and singular/repeated evaluation handling
remain necessary for a full quotient algorithm.

## Frozen outcome
Protocol3eb90f69aa preceded source ef2521ac49bd and pre-run checkpoint
20261115T140904Z-ef2521ac49bd. The unchanged original54point certificate is
input; no kernel or curve selected again. Terminal RUN.txt reports:

| Check | Recorded count | Outcome |
|---|---:|---|
| Nonzero principal functions |52|All exact identities pass|
| One-coefficient corruptions |52|All rejected|
| Frobenius function propagation |54|All match canonically|
| Root-free/direct determinant evaluations |864|All equal|
| Registered zero values / poles |0 /0|None in these triples|
| Kernel-sum values in F127 |32|All fixed by Frobenius|
| Norm determinant/resultant comparisons |324|All equal|
| Public multiples outside registered chart |2 of8|Skipped and retained in terminal count|
| Public D1 treated as3torsion |1|Rejected|

The16triples are the first lexicographic distinct-x triples from the first16
ascending-x canonical-y base-field places. Six of the first8public multiples
have supported squarefree degree3charts; the other2are not evaluated. These
are finite deterministic correctness controls, without confidence intervals.
Norm controls include1, a constant and U itself. Direct checks share field
arithmetic, but use explicit pointwise determinants/products instead of the
quotient algebra; norm recurrence is a separate arithmetic algorithm.
Frobenius sums use the unnormalized functions and are NOT established
kernel-translation invariants. No quotient curve/map is inferred from them.

## Reproduction and custody

```sh
export PATH=/Users/adamburan/.rustup/toolchains/stable-aarch64-apple-darwin/bin:$PATH
export CARGO_TARGET_DIR=/Users/adamburan/pq-work/hec-weil13-target
cargo build --offline --locked --release --manifest-path research/hec_weil13_20261115/Cargo.toml
$CARGO_TARGET_DIR/release/weil13 \
 research/hec_torsion13_20261115/CERTIFICATE.txt \
 research/hec_kernel_admission_20261115/fixtures/select-g3-p127-f0.out
```

The original execution is terminal with empty stderr and no cap; do not repeat
it just for timing. RUN.txt retains all54principal-function coefficient pairs,
32trace values and the terminal counts. SOURCE.txt/hashes.sha256 pin source,
binary, shared field implementation, input coordinates and dependency lock.
The initial build's debug stripping warning (missing libLLVM for rust-objcopy)
is retained in BUILD-FIRST.stderr; debug=1 removed that tool invocation before
source freeze. Final build is clean. Host macOS ARM; no performance isolation
or time claim. Native Rust science only. No new performance benchmark,
closed-record/default/leaderboard change or cloud worker.

Sources: prior torsion13 protocol/results; Tian
https://arxiv.org/html/2007.03172 Section2.1 and5 (determinant construction,
normalization and evaluation exceptions). Shared KB was ready lexical at
revision9141e386254340a9bdba3036f2bb706a, refresh2026-11-14T21:44:48Z;
its adjacent internal hits did not validate this formula or current outcomes.
Established Riemann-Roch and finite-field algebra, no novelty claim.

Next: certify the translation cocycle at multiple regular points, normalize
both complete kernel Weil sets, and construct nonzero invariant theta
sections before quotient reconstruction/descent and divisor transport.
All construction/normalization/transport costs remain chargeable in a later
complete comparison; the mathematical10xgenus2/3/4goal remains active.
Existing native CI does not execute this standalone scientific gate.
