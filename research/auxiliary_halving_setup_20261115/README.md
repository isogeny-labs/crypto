# Generated auxiliary setup by rational point halving

The fixed prime-field control now constructs its auxiliary curve, order-eight
subgroup and trace-good coset before point-only cold recovery. It recovers 11
for P=(4,3), Q=(18,1) on E/F19:y²=x³+2, after 70 setup trials and eight
solver calls, with four relations and one kernel. OpenSSL replays the setup
decisions, queries, rows, kernel and final answer. Result statement
PCC-20261115-3ff0b8 remains unverified. [pcc:unverified]

| Frozen qualification001 control | Outcome |
|---|---|
| All nonsingular short models and every finite P; p=5,7,11,13,17,19 | 14,754 complete doubling fibres match OpenSSL |
| Generated order-eight/trace-good setup; p=19,23,31,43,59,83,127,251 | All eight complete transcripts match |
| Cleared doubling identity | IDC1h6e44d7c75baf9f8d; 32 author + 128 extra evaluations |
| Legacy F19 cold | Summary and all three traces byte-identical to cold006; OpenSSL passes |
| Generated F19 cold | Scalar 11; 70 setup trials; 8 attempts/calls; 4 misses; 4 relations; native rank 2; 1 kernel |
| Per-arm exit records | All 25 are zero |

The candidate draws a,x0, sets b=-x0³-a*x0 and starts at a nonzero two-torsion
point. Two complete rational-halving choices produce exact order eight;
every lift is checked by doubling. It samples auxiliary points, tests
[2]phi(G)!=O and constructs the actual main base from the fibre. The cold
filter requires exactly three canonical main abscissas before target queries.
No auxiliary point count, main field/point census, supplied base logs, target
scalar or norm assignment enters that candidate path. The census is confined
to the separate outer correctness control and reference. [pcc:unverified]

For finite P=(xp,yp), the halving abscissa equation is

    u^4 - 4*xp*u^3 - 2*a*u^2 - (8*b+4*a*xp)*u + a^2 - 4*b*xp = 0.

It comes from the standard doubling law. Quadratic lifts are checked as
actual halves; poles and abscissa roots alone are insufficient. The unchanged
native certificate checker covers the polynomial identity over F_(2^61-1).
Its proof pointer is null. It does not supply an integer identity proof,
native source binding or full program correctness. [pcc:unverified]

The unfiltered F19 setup returns A:y²=x³+12x+10 after nine trials, with a
different fibre. The cold filter reaches A:y²=x³+2x+2, T=(16,8), G=(11,14),
terminal 15 and the same eight-root fibre as the previous preset. These
parameters were generated from the fixed seed, retaining every earlier draw,
path and rejection. This cold case does not exercise a new fibre family.
Collection uses seed 5678; legacy uses 1234. The eight-versus-twelve call
counts are different fixed controls, without a cost comparison.
[pcc:unverified]

The projected GraphOracle, original-equation lifting, collector and signed
row/kernel logic are reused. Tower/reference constructors accept generated
public parameters; defaults and original CLI retain their regression.
The OpenSSL reference includes no candidate implementation. It enumerates
doubling fibres and checks trace exclusion using 2G not in <T>, independently
of the candidate quartic/Vélu path. Shared law 3fc498f and the old exact F19
instance retain their earlier proof scope; no formal proof of the generator
or native program is claimed. [pcc:unverified]

The [conditional accounting](SETUP-BOUND.md) counts pointed short-model
orbits. It derives at least 1/d² success per abstract subgroup trial and
expected O(d² t poly(log p)) work when p>3, d=2^t>=8, p>=256d^4. Statement
PCC-20261115-4b68b3 remains unverified. It charges failed halving paths and
uses no exact auxiliary point count. The native finite caps, exact-three
base filter and tiny fields do not validate that bound. The relation
solver's degree/yield and general prime execution remain open.
[pcc:unverified]

Build with build.sh PREFIX FRESH_BUILD, using the parent's pinned native
msolve/FLINT prefix and installed Rust. The frozen qualifier takes
BUILD PREFIX LEGACY_ROOT FRESH_OUTPUT. Extract parent cold006 into a fresh
directory; its runs/recovery is LEGACY_ROOT. Copy qualify.sh into a separate
execution file and run that immutable copy. Every build/output path must be
fresh. Source 2ca55a9c9f and the pre-call source/binary/script hashes bind this
round. Backend limits stay 20s, 2 GiB, one thread and quotient 128, with all
original-equation/model/curve/fibre/group checks. No input or cap was retuned.

The whole [qualification archive](evidence/qualification001/runs.tar.xz) is
99,856 bytes, with a SHA256 file and exact normalized unpack/repack identity.
It retains all 25 arms, the full census, setup histories, certificate,
legacy comparison and cold trace. Both build failures remain under checks.
Current source/binary/script bytes match their distinct pre-call manifests.
This whole archive and the parent's 97,024-byte final Rust gate archive use
the documented artifact-autofix upload/full-byte replay. Run43717663512
uploads both objects and replays both complete remote bytes; manifest commit
ba62e3ae93 retains the two SHA256/length pointers and ignore rules. Local
hydrated verification passes for both objects with zero problems. Use
tools/artifacts/artifacts.py pull on either evidence directory to hydrate it.
No archive is split. [pcc:unverified]

The parent's full Rust suite at source 2fd16de7ca finishes nonpassing:
6,406 passed, 48 failed, 139 ignored. Output reports omitted sparse data/device
sources in untouched tests; a focused device failure confirms the missing
source. Full final evidence is in parent checks/root-rust-final002; the earlier
partial snapshot stays unchanged. No root-suite pass is claimed.
[pcc:unverified]

Sources checked (KB unavailable): [prior setup review](../projective_aux_norm_20261115/SETUP-REVIEW.md),
the auxiliary extension and primary modular-curve review linked above,
Conrad Theorem 4.2.1/Remark 4.2.2 re-opened with matching PDF hash, official
MIT 18.783 group-law/root-finding notes, FLINT API and actual native interfaces.
No novelty or exhaustive-search claim. The [report](report.pdf),
[setup-count graph](setup-counts.svg) and [pipeline](pipeline.svg) describe this
bounded component. Canonical leaderboard, progress and performance-gains
values are unaffected.
