# Frozen bounded Richelot divisor-map unit14

Status: prospective, before new implementation, compilation or arithmetic.
This continues result13 in regular crypto PR4514. Preserve this protocol and
every first outcome. Do not alter its population, signs or limits after a
failure. This is a new correctness unit, not a substitute for the original
full12,317-class or fullLP EKS tasks. [pcc:unverified]

## Question and sources checked

Does the unchanged operational Richelot constructor/correspondence/dual work
on the first recorded joint monic CT49 pair, and agree with the explicitly
transported old-model correspondence? Does its existing closed-fibre route
agree, including the two closed quadratic witnesses? Does the actual map
preserve the declared109/113 prime-order scalar relations?

Read before selection: the literal unit13 CSV and PROOF_13; unchanged
ConsumerE079 `nonsplit.rs`, `nonsplit_map.rs`, `closed_fibres.rs`,
`CLOSED_FIBRES.md`, `NONSPLIT_MAP.md`; unit12 balanced core and shifted-chart
witness/transport bodies; Kunzweiler DCC92(2024) §3.2 Propositions7–8 and
the following degree5/rational-Weierstrass restrictions, from retained
publisher text `/workspace/scratch/richelot-kunzweiler.txt`, lines652–745.
Publisher PDF SHA256 is1b1a5b9a351038649a5b534b1734a8d67e02c58af72f6191ed53521d4777ca73,
as pinned in crypto `research/chained_jacobian_transfers_20261110/SOURCES_20261111.md`.
The earlier corrected Galbraith8.3.8/8.3.10/10.4.2 argument supplies the
Picard functor and complete split-infinity chart, not an affine-only map.
The paper's subsequent degree5 simplifications are not imported into the
balanced sextic implementation. KB MCP tools are absent, earlier wrapper127;
fallback is these primary texts and exact retained code. No novelty claim.
[pcc:unverified]

## Literal instance and fixed orientation

Use p101, cubic `[3,26,3,1]`, pairing `[(0,4),(1,5),(2,3)]`,
Frobenius permutation `[1,2,0,4,5,3]`, and literal13 encodings
`[1008151,720554,352389,357036,975881,738000]`.
Monic source `[97,29,96,18,87,99,1]`; monic target `[16,53,24,80,73,57,1]`.
The existing canonical constructor must choose target scale13. Do not
replace it with bc88 or choose a dual sign from an observed equality.

For conjugacy use old raw source `[52,0,25,0,77,0,84]`, lambda84,
source normalization square root36, common old pole a3, b22,c4.
Normalize the old source by84 and recover old roots as `3+1/new_root`.
Use source chart b_normal=96, fixed algebraically as22/36.
Build the old map by the same explicit-pairing constructor. Let T be its
canonical target scale; let s be the smallest positive root of T²/84.
Use the old normalized target chart c_normal=4/s. Determine
`kappa = 96*c_normal*T/13` algebraically before any divisor comparison;
it must be1 or100. This is the formula-derived sign, not a fitted sign.
Both transported chart equations must equal the two literal new models.

## Fixed population and success criteria

1. Build exactly17 source witnesses with the unchanged unit12 generator,
   using `Chart::new(new_source,0,20)` and aux=false. This covers identity,
   delta±, both pole ordinates/infinities, ordinary points, repeated support,
   distinct pairs and two opposite closed quadratic orbits. Preserve the
   deterministic original enumeration and labels.
2. For each witness D: validate D and phi(D); compare forward_closed(D)
   with phi(D); compare dual(phi(D)) with[2]D and the closed dual; compare
   phi(dual(phi(D))) with[2]phi(D); check inverse6159 recovery on D.
   Both labels `quadratic_orbit` and `opposite_quadratic_orbit` must take
   `ClosedRoute::Quadratic`, not silently pass through a fallback.
3. On all17 D compare phi(D) with the formula-signed transport of the old
   map. Also execute the reverse old/new chain and compare its output with
   the new dual and[2]D. Record both old/new images, not only booleans.
4. Check exactly34 ordered pairs: `(D_i,D_i)` and
   `(D_i,D_(i+1 mod17))`. Validate the sums and compare the map of each sum
   with the sum of images; compare the closed route on the same sum.
5. For each q in[109,113], choose the first nonzero `[12317/q]D_i` in the
   fixed17 list. Fail if none. Directly check nonzero/[q]G=0 and nonzero/
   [q]phi(G)=0. Check the fixed scalars `[0,1,2,37,q-1]`, comparing actual
   source/target scalar relations, closed evaluation, dual[2] and
   inverse6159 recovery. These are known-answer controls, not DLP solves.
6. Reject three separate malformed assessments: a repeated pairing index,
   a duplicate root and source constant coefficient+1 mod101. Check each
   constructor returns an error. On both prime generators, the negative of
   the prescribed dual image must differ from[2]G; retain these two wrong-
   sign controls. A valid alternate square root is not called malformed.

All criteria must hold for a positive outcome. Any failed assertion,
constructor rejection, timeout or resource failure is retained verbatim and
closes that first attempt as incomplete/negative at its actual scope. No
population narrowing, higher timeout or sign change to turn it positive.
Both algorithmic routes reuse the same polynomial/group engine; agreement
is not an independently implemented full replay. [pcc:unverified]

## Implementation and resources

Scientific code is Rust only. Preserve exact reused method bodies:
`nonsplit.rs` prefix before its first test; `nonsplit_map.rs` prefix before
its first test, removing only the unused seven_glue module registration;
the whole closed_fibres; exact balanced core; whole original unit12 shifted
source as a prefix with new visibility adapters appended. Record original
and extracted byte intervals/hashes. New wrappers use distinct namespaces;
do not repeat the unit13 Result alias collision. Existing unused inherited
methods may have a narrowly scoped dead_code allowance; all other Clippy
warnings remain errors. No field or group-law implementation is replaced.

Use the already retained complete native12 archive as prebuilt dependency
custody, with exact matching Rust1.99.1 rlibs:
num_bigint3,138,230B; num_traits2,041,652B; num_integer374,786B.
Pin all full member bytes/SHA256 before calls, copy those full rlibs into
the new hold, and include them in the complete declared14 packet. No
dependency cold build is claimed. Use direct rustc --test with explicit
--extern and dependency search path, opt-level3, codegen-units1 and stripped
symbols. Use direct clippy-driver with the identical scientific sources;
run an isolated positive/negative eq_op lint sentinel to check that direct
Clippy invocation actually enforces the requested lint.

One local native arithmetic test, one worker,120s wall limit, exact new-test
filter. The12 old backend tests and unchanged unit12 test are compiled and
filtered, not re-executed or counted as new passes. No original full-class
or LP test is included in the extracted prefixes. No benchmark or paid job.
Keep the original13 limits unchanged; this is a separately frozen unit.

Before setup require at least8MiB available in /dev/shm and3MiB in the
separate runlock compiler scratch. Use task-specific directories. Record
compiler/tool versions and binary hashes, all actual argv/env, exit codes,
full stdout/stderr and every expected-failure control. Copy the executable
to a555 qualifier, pin before/after hashes, and execute that qualifier only.
Guard original and copied sources, every dependency binary and qualifier
before and after the unit. Do not overwrite raw create-new output paths;
the Rust test emits to separate captured stdout.

After every outcome retain one whole archive of the declared copied inputs,
full rlibs, original compiler/qualifier outputs, logs, assertions and receipts,
with every member byte/mode compared. Prospectively exclude only temporary
compiler intermediates, inventory any leftovers, and keep all failures.
Do not delete compiler intermediates needed to explain a failed attempt.
Produce an inspected PDF and exact-scope typed PCC records. Publish source
before arithmetic and final outcome in the existing regular PR4514; do not
tag the PR head or alter workflow/gate/trust policy. [pcc:unverified]

## Continuing obligations

On a positive result, proceed to cubic-only closed-fibre specialization and
the next actual chained map. On failure, preserve the unchanged first source
and diagnosis before a separately frozen successor. Original EKS workloads,
full provider12 ZIP custody, signed/independent validation, general Lean
formalization and complete solver/cost comparisons stay open. PaidSTOPe42949,
locked autoresearcher8edec422, original/frozen indices and all CI/merge rules
remain in force. [pcc:unverified]
