# Round21 preregistration

UTC 24 October 2026; parent 63bc0895df3d2cbd38e22a3efe828a52a0fad731.
Conductor T-299. Frozen R01–20 evidence remains unchanged. The requested
integral level77 map, theta descent, proper quotient geometry and full
saturation remain the scope; this round is a bounded continuation, not a
replacement completion criterion. All arithmetic/search/verification and
record assembly use Rust. No IC efficiency assessment is made.

## Q35 complete coefficient-Frobenius tangent obstruction

Test the proposed obstruction to complete arcs through Q34's four scalar
models: an ordinary infinity arc with node123 leading coefficient one
would have five trace-zero leading parameters whose nineteenth powers
are one. Check the actual source gates, S polynomial degree8/leading1,
ordinary elliptic local chart, and order-five augmentation descent rather
than suppressing any coefficient-Frobenius block. Construct F(2^18), all
19 roots and their binary relation space, independently enumerate all
19^4 prefixes with the fifth parameter forced by the trace condition,
and retain counts, witnesses and failures. Positive controls: two equal
roots sum to zero and all19 roots sum to zero. Verify order of2 modulo19
and explicit trace identities. At most2^20 finite checks; one native
command,20-minute limit,1 thread,4GiB. A null tangent search is not a
complete source-arc exclusion without the written valuative/descent
bridge. Formalize the universal roots/trace statement and source-specific
finite certificates wherever feasible. Do not promote a heuristic leading
pole calculation to a saturated ideal or polynomial separator.

## G31 descent of the integral recovery map

Use the actual G28 recovery and G25 row cycles. Average the five conjugate
recovery maps integrally, test A*Phi=5I and A*Pi=R*A, and record all actual
coefficients. Test whether denominator5 is removable and whether an
integral equivariant retraction is possible, using mod5 norm/fixed-vector
certificates. Do not conflate a split closed immersion with a split
retraction over F2. Prove the actual identities and any integral obstruction
with explicit CM/group-action hypotheses. Matrix dimensions at most20;
20-minute native limit,1 thread,4GiB.

## Other requested tracks and validation

Keep the final105/19 quotient/243 sections, full theta action and rational
trivialization, full h-saturation, level77 normalization/Riemann–Roch and
integral factor evaluator open unless actual new evidence resolves them.
No new external job, generator or waiting-freeze followup is authorized by
this protocol. Preserve migration/job reconciliation failures. Add exact
replay, source pins, native tests, full pinned Lean build/transitive audit,
required repository tests, source-bound identity certificates, reports,
editable SVG/150dpi PNG/PDF and canonical tracker updates. Archive raw and
validation trees deterministically and verify fresh extraction, native
member hashes, byte-identical replay and repacking. Update existing ready
PR2215 with a guarded descendant commit. CI and reviews must concern its
published head; missing checks are not passes.

## Amendment A — Q36 scalar boundary consequence

After Q35's130321 prefixes return zero completions, test all eight ordinary
Miller y*S gates (two functions on each of four factors), not just node123.
For every graph independently parse S from the actual DAG and verify its
exact degree8/leading1 and the ordinary curve/descent bindings. Only under
the written complete-coefficient/DVR/formal-group bridge may their scalar
boundary values be forced zero. Retain every Q13–Q17, Q23–Q34 condition,
add these eight scalar zeros and exhaust the same uncapped search with
the existing100000 visits/query and1000000 total guards. Preserve any new
models and caps. This is a scalar-boundary consequence, not an explicit
polynomial separator or an equality of saturated ideals. Record this
qualification in every new output, proof map and tracker. A finite Lean
model checker proves only the stated finite repaired system.

## Amendment B — Q37 all ordinary coupled norms and Q38

Q36 retains four models on155,157,184,186 after51 visits each; do not call
this full saturation. The old coupled relation covered only the first
ordinary factor. Generalize the exact Q33 monic-algebra norm computation
to all eight ordinary Miller functions per graph (32 relations), each
with its own actual P,S,D,U,V gates. Independently substitute in the
ordinary elliptic function field, verify denominators are constant1 and
retain the full norm matrices and terms. Add only their reduced boundary
linear consequences U=V, with exact top shape checks. Re-run the complete
same scalar model search as Q38, retaining every prior equation, all Q35
qualified scalar zeros, caps, failures and new survivors. Bound each new
native command by20 minutes,1 thread,4GiB; do not reduce the equations or
increase search guards to manufacture a result. This polynomial source
family is independent of the Q35 tangent bridge; distinguish these two
provenances in the report and output.

## Amendment C — Q39 all elliptic pure-source families and Q40

Q38 keeps four models on180,182 after71 visits, no caps. Extend Q31's
source-bound exact lower/higher polynomial resultant family from input0
to each elliptic x-input0,2,4,6,8, with higher Horner nodes95,155,215,275,335.
Check their actual degree9/nonzero leading coefficient; group identical
lower polynomials per input and retain full coefficients, gate bindings,
normalized relations, exact substitution-zero checks and boundary tops.
Do not infer one input's equations for another. At most256 distinct
polynomials/input,20 graph/input families;20-minute command limit,4GiB,
1 thread. Q40 retains every earlier condition, adds only the certified
lower-node scalar zeros, and uses unchanged search guards. Preserve new
survivors and caps. Stop this registered round after Q40 and report the
remaining support/ideal and integral-map/theta requirements explicitly.

## Amendment D — verification cache locality after an actual timeout

The complete9079-job build passes. The first full axiom audit's20-minute
watchdog expires during NFS import,with no audit result. Preserve its
log/status and reconcile its owned processes before another checker.
Copy only the required compiled library objects (.olean/private/server,
.ir and .so) from the exact existing pinned Lean search roots to a new
local tmpfs directory. A Rust helper must verify source/destination SHA256
and sizes for every file and retain the root mapping/member manifest.
Limit cache contents to8GiB,copy process to4GiB with8 workers and1200s;
limit the unchanged auditor to8GiB/1200s. Run the same compiled local
modules and trusted-native policy with only LEAN_PATH redirected to the
verified local copies. No proof source,dependency revision,theorem,
axiom policy or native experiment cap changes. No external job or new
Diem/freeze generator is launched. A cache failure remains infrastructure
evidence,not a proof failure or a passing audit. The cache helper's source
and all receipts join this round's source pins/raw record.

### Amendment D1 — enforce Lean's documented memory budget

The first local invocation exits134 with a thread-creation resource error
under the8GiB virtual-address restriction;it reports no audit result.
Retain that failure. Retry the same auditor with documented -M8192
(maximum Lean memory in MiB) and -j1,using the same verified cache and
1200-second bound. The allocated-memory budget stays8GiB;file-backed
address mappings are not constrained by an unrelated virtual-address
ceiling. Disable core dumps for this verification invocation. No proof,
trust,dependency or scientific cap is changed. Report the actual retry
status rather than assuming this resolves the resource error.

### Amendment D2 — retain Lean IR signature sidecars

The single-thread local auditor reaches compilation but exits1 before
main:an imported private CollectAxioms function has no executable code.
Inspection of pinned Lean's importer identifies .ir.sig sidecars,which
the initial cache selector omitted. Supplement the existing verified
cache with the original required .ir.sig files;retain a separate member
receipt,verify each source/destination byte/hash match,and require the
combined object payload to stay within8GiB. Recheck every original cached
member locally before the supplement. No original cache/proof object is
rewritten and no proof/checker/trust policy changes. Preserve the failed
compiler log/status;retry the exact auditor only after the supplement
passes. Keep -j1,-M8192 and1200s.

### Amendment D3 — observed verifier memory requirement

The complete local cache reaches the unchanged auditor and begins its
axiom checks,but -M8192 exits1 on excessive interpreter memory before a
final result. Preserve the partial output/status. Raise only the temporary
verifier allowance to -M16384 (16GiB maximum Lean memory),with -j1 and
1200s unchanged,on the existing host. The compiled-object cache stays
within8GiB payload;all native scientific caps,external job budgets and
original Diem deadline remain unchanged. The theorem/module list,checker
source,trust policy and completion criterion are unchanged. No partial
'ok' output may substitute for the final zero-failure audit and exit0.


## Release reconciliation,2026-10-25

Science stops at Q40 as registered. Raw144 files and the validation246 files
are sealed;fresh extraction,member hashes and deterministic repacks pass.
The validation archive is14861500 bytes,above the10MiB Git limit. The
configured artifact-store status and push return AccessDenied/403 using the
migrated worker. No upload or index promotion succeeded. Preserve the
archive unchanged outside Git,its hash/member manifest and failed upload
receipt;RELEASE.md names the missing access. CI replays the committed raw
archive and retains the complete original Lean proof gate. It does not
pretend to retrieve the unavailable validation artifact.

ledger-seal01.json preserves the first222 source pins used by the sealed
archive validation receipt. Reissue ledger.json after this protocol/CI
release-only correction with the same144 raw files;require byte-identical
runs.sha256 and a fresh current/committed source-pin check. No scientific
computation,proof or budget is changed by this release amendment.
