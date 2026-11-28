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

2026-11-16 continuation supersedes the execution status below: see
continuation_20261116/REPORT.md. Raw/reduced m2 depth1/2 CAS controls now
match complete native references. Exact affine reduction has72 native
cases. Eight public requests include three preserved failures; no runtime
or regularity result is inferred. Claim PCC-20261116-2335a8 is unsigned
[pcc:unverified]. Frozen core/protocol/proof pins are unchanged.

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

The eight public unit requests have finished. No remote unit is still
running; do not repeat them automatically. Original depth8 m2..7 EKS/L2
measurement and actual regularity remain open.

## Stop and success conditions

Per prospective cell:60 seconds/4 GiB, one native process, no retries;
freeze changes before increasing. Zero incorrect relations and no lost
Fp-rational solutions on complete checked instances. >=2x median complete
stage improvement on identical instances with paid setup/conversion/replay
and qualified measurement is the performance goal. New mathematical
claims need independent proof/standing; input degree3 is not regularity3 [pcc:unverified].

## Publication

[PR4147](https://github.com/isogeny-labs/crypto/pull/4147) is regular/open,
with auto-merge MERGE enabled under repository cadence, pending requirements.
First code head:30ff45786a34e8f9525e5bc58e4e40e3bb52c0eb. Initial complete
check-run API census returned23/23: native-checks succeeded, other jobs were
pending, with no submitted reviews or inline comments. This snapshot is
PCC-20261114-3db01e, unsigned; it does not establish later-head CI or merge
success [pcc:unverified]. No experiment/check-in automation was created.

2026-11-16 publication: code head3f57265a is pushed to this same PR.
Both native-checks and linear-replay succeed there. One parser failure
requires supported PINNED registration for the two exact-byte workflow
subjects; local draft-tier/workflow/merge-policy checks pass on that fix.
Two other checks fail on shared archived input HeadObject403s; see
problems/ci-archive-object-read-403.md. PCC-20261116-b9df82 is unsigned;
no full CI or merge success is claimed [pcc:unverified]. Keep the PR open
while requirements are red and do not bypass checks or retry unchanged
403s. Auto-merge remains enabled; the pre-existing needs-human label remains.
