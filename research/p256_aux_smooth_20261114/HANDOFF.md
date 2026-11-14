# Resume: aligned P-256 auxiliary factor base

## Goal

Build the owner's new smooth auxiliary-curve factor base over the actual
P-256 field; baseline m=2..7 relation solving and pursue a matched 2x gain.
Keep novelty, regularity and performance as separate open requirements.

## Lead / hypothesis

PKM rational 2-isogeny coset membership with auxiliary w^2=x^3-3x.
Coefficient alignment gives uv=B, u=y+w, v=y-w. The oriented variant
propagates w to the fixed terminal point to avoid auxiliary sign copies.
Extra variables/equations can outweigh that benefit. Reference is dense
membership over exactly the same target base, not a different point set.

## Spec

`research/p256_aux_smooth_20261114/PROTOCOL.md`; starting main
0c2cc1b408e33a43e55ee31cb75307e90df9d619. Registered target slug is in
the protocol. Deterministic development fixture d=4 is FB1h2704fe49495e;
intended timing depth is8, scaling16/18 separately. Native artifacts and
hash manifests live in evidence/. Do not call d4 the d8 benchmark.

## State

Native C++/GMP/OpenSSL constructor and four complete relation-system
exports exist. Unit observations are claim PCC-20261114-4a3ab6,
not_covered/unverified, not independent verification [pcc:unverified].
The constructor source is pinned by that immutable claim; preserve it.
The separate magma_adapter.cpp includes the core and generates true
256-bit Magma inputs plus exact native solution replay. It does not call
a word-sized solver. No CAS run, benchmark, measured regularity or 2x
claim exists. Root cargo test cannot start: cargo is absent in this fresh
environment. No signed independent verdict has been issued.

## Next steps

1. Run the documented bounded native checks and adapter synthetic replay;
   keep fresh output directories and failed logs.
2. Admit a real checkpoint upload using the maintained gate, then qualify
   an exact-source EKS image with a suitable big-prime CAS. GitHub's
   current ecbench-only build bundle does not package this adapter/Magma.
   Do not dispatch its default integration fixture as the owner's run.
3. Freeze the d8 FB1 ID and exact script/binary hashes, live capacity/L2,
   A/A controls and full m2..7 matrix before timing under the protocol.
4. Validate CAS syntax/execution, full Fp relation sets and exact replay;
   record affine GB output degree separately from structured regularity.

## Runs in flight

None. Only local development builds/unit tests and static exports have run.
No EKS job or performance experiment has been launched.

## Stop and success conditions

Per prospective cell:60 seconds/4 GiB, one native process, no retries;
freeze changes before increasing. Zero incorrect relations and no lost
Fp-rational solutions on complete checked instances. >=2x median complete
stage improvement on identical instances with paid setup/conversion/replay
and qualified measurement is the performance goal. New mathematical
claims need independent proof/standing; input degree3 is not regularity3 [pcc:unverified].
