# FreeLIP source and full-cohort replay protocol, version1

Frozen before any full-cohort execution,27October2026. The original user goal
is a criterion explaining historical skips. No measured DLP, attack, speed,
security or historical-caller claim is included. Prior21 labels are visible;
this is a fixed mechanistic test, not a blinded or predictive holdout search.

## Literature and source rationale

The primary [curve-origin report](https://www.anthropic.com/research/ecdsa-curve-origins)
identifies six May1997 base points with the FreeLIP RNG, six small seeds and
seven discarded single-limb outputs. The
[independent enumeration](https://words.filippo.io/seeds-solved/) still leaves
11 skips unexplained. The previous full-census fitted rules,52,520 generation
models, exact embedding checks and fixed ECPP trial did not recover that rule.
FreeLIP is a new actual-code candidate, not a claim that the original caller
used its primality function.

Recover FreeLIP1.0 from the FUNET and GWDG mathematical-software mirrors. Both
201,037byte archives must match SHA256
6ef1e0bb398eec0c6cbd22f51b06cbbbbd4d995673ae8da942de6f9d3a7ecec5.
Original README dates1May1995. Recover1.1 from pinned source mirror commit
00898a4814d2cc1244bf348682be6ed10dda35f0 in lrobot/freelip; original README
dates21July1997. Bind all original bytes and licensing text; no source body
may be patched. Mirror agreement is reproduction evidence, not authentication
of a1997 copy or an original-generator linkage. The author reference returned
502; the initial guessed FUNET subdirectory returned404; neither is hidden.

Source inspection identifies zprobprime(n,t): trial division, deterministic
base2 strong probable-prime test, then t Montgomery strong probable-prime
trials. zprime(n,t,2) uses the ordinary modular-exponentiation route with base2
first and t−1 RNG bases. Fix t=7 for zprobprime and t=8 for zprime so both
have seven random-base trials. This value is fixed from the already published
seven-output base-point recipe, not searched against chosen/skipped labels.

## Mathematical expectation and its limits

For an odd prime n and any0<a<n, write n−1=2^s d, d odd. Fermat's theorem
and the uniqueness of roots of X²−1 over F_n imply a^d=1 or
(a^d)^(2^j)=−1 for some0<=j<s. Correct strong probable-prime arithmetic
therefore accepts every such n. Under NBITS26, all generated small bases lie
between3 and2^26−1; every recorded cohort order is much larger. Correct
trial division below n cannot reject these primes. This is an algorithmic
expectation under exact arithmetic, not proof of the old implementation,
compiler/ABI behavior or any historical use.

Both recovered versions have the same zprobprime body; actual bytes/ranges
will be recorded. The1.1 README names an extended-Euclid bug fixed after1.0.
That statement is not evidence it affected primality or any curve. Follow
called routines before attributing a result to it.

## Workload and controls

Full workload: all21 original decimal group orders and identities, copied
without arithmetic from the verified ECPP replay004 input packets and frozen
historical.tsv. Ten published and eleven skipped remain mandatory controls;
no incomplete or rejected observation may be dropped. Fresh process per
version/method/order: two versions times two methods times21 orders=84 calls.
Default FreeLIP RNG initialization7157891; no additional warmup/draws.

Build the original sources as a declared modern LP64 functional configuration:
C GNU89,O0,fcommon,fwrapv,SINGLE_MUL,NBITS26,SIZEOFLONG8, long8/double8/char1.
The driver is strict C99/O2/Wall/Wextra/Werror. Preinclude only standard
prototypes required by modern C; record every such build setting. No legacy
source body is changed. This is not an emulation or authentication of a
historical ILP32 executable. A configuration mismatch or failed control stops
cohort admission; do not change flags after seeing labels.

Local development builds/unit checks are allowed by the current isolab-eks
instructions and are not the cohort experiment. Fixed generic prime fixtures:
2,3,65537,2147483647,170141183460469231731687303715884105727. Fixed composite
fixtures:0,1,4,15,341,561,2047,3215031751. Run both functions through the same
strict driver. Compare requested/root-reported decimal integers and require
exact result inventory; a driver/parser error is indeterminate, not rejection.
No historical order is run during these development checks.

An optional separate six-known-answer reconstruction checks whether seven
actual zprobprime random trials, followed immediately by zrandomb point
sampling, reproduce the six published May1997 Gx values using their recovered
seeds. It requires its own frozen table and native verifier before execution;
it is NOT part of this version1 workload. Seven RNG calls need not equal
seven state advances because zrandom uses rejection sampling. Do not infer
that reconstruction or original caller from the number seven alone.

## Backend, budgets and stopping

Current repository default is EKS Spot. Existing AWS identity is in the correct
lab account, but actual CodeBuild BatchGetBuilds and EKS DescribeCluster are
AccessDenied. S3 result listing works. Neither an absent MCP nor a matching
account alone grants submit/verification rights. The current portable bridge
supports the ecbench benchmark contract, not this arbitrary C command; do not
substitute its deployed validation fixture. No submission occurred and no
paid resource was launched. Keep this execution gap distinct from mathematics.

Prepare the exact native executable, inputs, hashes, command and resource
limits. Do not execute the84-call cohort locally unless the user explicitly
selects that backend, or an authorized EKS custom-command path is available.
Development controls do not stand in for this workload. Planned cohort limit:
serial calls,60seconds per call,10minute total watchdog,768MiB virtual-address
cap, core dumps disabled, no automatic retries. Any backend amendment is
recorded prospectively, followed by a successful S3 checkpoint gate.

Retain each stdout/stderr/status. ACCEPTED means strict input/configuration
binding, completed zero-exit process and legacy result1. REJECTED_SOFTWARE
means those bindings hold and result0 for a separately proved-prime original
order. Every timeout/crash/missing/malformed/configuration/control failure is
INDETERMINATE. The ordinary and Montgomery routes are compared without
threshold fitting. No p-value or new statistical calibration is claimed.
All-accepted rules out only the tested recovered program/configurations as a
complete reject-all-skips explanation. A rejection needs exact-path replay
and independent arithmetic diagnosis before any historical interpretation.

## Evidence and delivery

Keep source audit, proposed native replay and actual executed results separate.
Record actual flow, including zero attempted if the backend remains blocked.
Archive exact original sources, mirror receipts, source excerpts/diffs,
development commands/statuses and prepared inputs; full upload/GET/per-member
verification precedes ignoring any raw tree. Source-linked report, diagram,
PDF and current ready PR2666 update are required. No old report or certificate
is rewritten. Original criterion remains open until evidence meets its exit
conditions in problems/nist-candidate-selection-weakness-hypotheses.md.

## Prospective development amendment2: actual IEEE word layout

No historical order has executed. The first strict driver builds exposed
private static declarations/nested comments in the original public header.
Keep that header and source unchanged; use a typed dependency shim built with
the library's GNU89 settings, and keep the separate I/O driver strict. All
failed development attempts remain retained. The source excerpt/hash helper
finds zprobprime,zmcomposite,zsmexp,zrandom,zrstarts,zrandomb,zmstartint bodies
byte-identical across recovered1.0/1.1; zcomposite differs in memory cleanup.

Both initial LP64 developer binaries built but the first generic fixture2
reached its30second watchdog with no stdout. Neither build/configuration is
admitted. Source audit identifies the SINGLE_MUL FetchHiLo union as double
plus two unsigned-long words; LP64 gives the wrong word layout. This is a
source-level ABI incompatibility, not proof of the precise observed hang path
and not a rejected historical curve. Do not repair the original arithmetic.

Use a prospective explicit ILP32 build instead: official Zig0.15.2 x86-linux-musl,
original SINGLE_MUL/NBITS26/default SIZEOFLONG4, long4/double8/char1, unchanged
GNU89/O0/fcommon/fwrapv source bodies, strict C99/O2/Wall/Wextra/Werror driver.
The official download manifest SHA256 for the53,733,924byte compiler archive is
02aa270f183da276e5b5920b1dac44a63f1a49e55050ebde3aecc9eb82f93239.
Run generic development fixtures through previously hash-verified QEMU i3867.2.0
with256MiB guest reservation and768MiB host address cap. This does not establish
historical compiler/libc/OS equivalence. The full84-call workload remains
unexecuted and backend-blocked; it must use the same admitted ILP32 binaries
and all inputs/controls, after any explicit backend selection and S3 gate.

The first ILP32 development link failed before execution: Zig's O0 library
objects contain default undefined-behavior instrumentation, while the O2
strict-driver link omitted that runtime. Retain the diagnostics and link with
explicit fsanitize=undefined, not by disabling the library instrumentation.
Set UBSAN_OPTIONS=halt_on_error=1 for every development or eventual cohort
call. An instrumentation failure is indeterminate and blocks admission; it
is never classified as software rejection or a weak curve. All prior failures
remain preserved. No historical order has run during these amendments.

The following ILP32 generic fixture2 still timed out before a root/result was
printed. Inspection of the original configuration utility liptimer.c:4221
shows that x86 must define DOUBLES_LOW_HIGH for SINGLE_MUL. The initially
empty generated lippar.h omitted that required endian setting. Add only that
source-defined platform flag and a strict runtime IEEE word-order assertion;
keep instrumentation, arithmetic bodies and expected fixture results. The
first ILP32 launcher also used a downloaded emulator cache file without its
executable bit (exit126); install a separate hash-identical executable copy.
All attempts are retained, and no historical order has run. These are failed
build/platform controls, not selection observations.

## Prospective amendment3: bounded local fallback, before cohort execution

Both recovered source versions now pass all13 generic development fixtures
through both methods in the explicit ILP32/little-double configuration. No
historical order has run. Final admitted developer binaries are v1.0-008 and
v1.1-004; bind their complete bytes, original sources, strict adapter/shim and
native summarizer before execution. The shim avoids original header-private
static declarations in the strict driver; no original arithmetic body changed.

The EKS account matches, but actual BatchGetBuilds and DescribeCluster calls
are denied for the configured caller; the portable bridge also admits only
the ecbench benchmark contract rather than this legacy command. No installed
EKS MCP custom-command connection is available. No Spot job or paid resource
has been launched. Do not broaden IAM, deploy a service, or run the unrelated
default integration fixture.

Use the already-authorized session's bounded local native-analysis workflow
as an explicit fallback for this functional replay. The repository instruction
is an EKS default, not an instruction to abandon an authorized workload when
its supported route cannot execute/verify it. The earlier version1 sentence
requiring a new backend selection was an unnecessarily restrictive protocol
choice; this amendment changes it before affected data, without inventing new
user approval. Inputs, all84 calls, two original versions, both algorithms,
seeds, mathematical expectations, control outcomes and unknown handling stay
fixed. No benchmark or historical-time comparison is made. The user-facing
update names the backend change before execution. Require a successful source
checkpoint gate, serial calls,60s/call and600s total,768MiB host address cap,
256MiB QEMU guest reserve, undefined-behavior halt and no automatic retries.
