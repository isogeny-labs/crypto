# Recovered FreeLIP routines accept every published and skipped order

27 October 2026. All 21 original group orders pass both tested primality
routines in both recovered FreeLIP versions. The fixed replay completed
84 calls: 10 published and 11 skipped orders per configuration, with zero
software rejections and zero unknowns after source-faithful output validation.
These routines do not explain the skips when given the correct group orders.
The historical caller, its computed inputs and its point-counting code remain
unrecovered. The original criterion goal is still unresolved.

![Full fixed-cohort acceptance](reports/outcomes.png)

The graph plots the [native summary](results/summary.json). Four configurations
reuse the same 21 historical choices; they are not 84 independent historical
samples. Exact counts have no sampling interval. The [complete table](results/cohort.tsv)
and [native unified export](export.json) retain every call. Stream/index labels
and counts are decimal; exact curve parameters and ICV1 identities remain in
the frozen parent inputs and original assessment.

## Why this library was tested

The primary [curve-origin report](https://www.anthropic.com/research/ecdsa-curve-origins)
links six May 1997 published base points to the FreeLIP RNG and seven discarded
single-limb outputs. This gives FreeLIP a provenance connection to the observed
generation process. It does not identify the primality routine or original
caller. The [independent enumeration](https://words.filippo.io/seeds-solved/)
still describes the remaining skips as unexplained. The previously tested
ECPP artifact likewise had no recovered original-caller link.

FreeLIP 1.0 was recovered from the [FUNET archive](https://mirrors.nic.funet.fi/pub/crypt/math/freelip_1.0.tar.gz)
and [GWDG mirror](https://ftp.gwdg.de/pub/misc/crypt/math/freelip_1.0.tar.gz).
Both 201,037-byte archives have SHA256
6ef1e0bb398eec0c6cbd22f51b06cbbbbd4d995673ae8da942de6f9d3a7ecec5.
The original README dates 1 May 1995. Version 1.1 comes from pinned mirror
commit [00898a4814d2cc1244bf348682be6ed10dda35f0](https://github.com/lrobot/freelip/tree/00898a4814d2cc1244bf348682be6ed10dda35f0),
with original README dated 21 July 1997. Source bodies and original notices are
preserved. Mirror agreement binds reproducible bytes, not an authenticated
1997 copy or evidence that the curve generator used this version.

The [byte-bound source audit](source-audit.json) finds seven relevant bodies
identical across the recovered versions: zprobprime, zmcomposite, zsmexp,
zrandom, zrstarts, zrandomb and zmstartint. The zcomposite difference is memory
cleanup. The documented extended-Euclid fix in 1.1 is not by itself evidence
of a primality failure, a skipped curve or a vulnerability.

## A concrete seven-call mechanism, with a remaining gap

Source zprobprime(n,t) performs trial division, a deterministic base-2 strong
probable-prime test, then t random-base Montgomery tests. Fixing t=7 therefore
uses seven zrandom calls on an accepted multi-limb input. The ordinary route
zprime(n,8,2) uses base 2 first and seven random bases through ordinary modular
exponentiation. Those exact settings were fixed before cohort execution,
using the already published seven-output recipe rather than searching labels.

![Source path and proposed connection](reports/source-path.png)

This is a source-supported candidate reason for seven RNG calls before point
generation. It does not establish that these were the historical calls.
Each zrandom uses rejection sampling, so seven calls need not mean seven
state advances; its bounds also differ from a discarded full-limb draw. No
new six-base-point reconstruction, caller recovery or independent historical
prediction was executed in this study. The connection remains proposed.

For a prime n and a nonzero residue a, write n−1=2^s d with d odd. Fermat's
theorem and the two roots of X²−1 imply the strong probable-prime test accepts.
All generated small bases here are below 2^26, far below every original order.
Thus rejection would require implementation/input behavior, not the ordinary
mathematical test rejecting a true prime. This standard argument is conditional
on correct arithmetic; it is not a formal verification of the recovered C.
The actual replay checks this recovered configuration on the complete census.

## Frozen workload, controls and actual execution

The [human protocol](PROTOCOL.md) and [structured version-3 protocol](protocol.json)
preceded the 84-call cohort. The canonical non-freeze protocol hash is
6cddd67e331ed5ae990c90cfa7c8ee94fdfe00a84a1dccf239fa6fc179dcd0ca.
JSON Schema validation and native-source/file bindings are recorded in
[protocol admission](evidence/protocol-admission.json). Labels were already
visible; there is no blinded preregistration, p-value or holdout claim.
The 600-second field is a conservative enforced resource ceiling, not a
measured or forecast algorithm runtime. The original Python protocol checker
was not executed; schema/file admission and manual semantic review are named
precisely rather than presented as its certification.

| Version | Routine | Published accepted | Skipped accepted | Unknown |
|---|---|---|---|---|
| 1.0 | zprobprime(n,7) | 10/10 | 11/11 | 0 |
| 1.0 | zprime(n,8,2) | 10/10 | 11/11 | 0 |
| 1.1 | zprobprime(n,7) | 10/10 | 11/11 | 0 |
| 1.1 | zprime(n,8,2) | 10/10 | 11/11 | 0 |

Each call starts a fresh process with the unchanged default RNG initialization
7157891. All 21 exact decimal inputs were byte-copied from previously proved
original-order packets, with the complete parameter table retained in the
parent study. The three originally supplied skipped P-224 curves and skipped
P-256 candidate 876 pass every tested configuration. No control was omitted.

The modern compatibility build uses the original arithmetic bodies, Zig
0.15.2 x86-linux-musl, GNU89/O0/fcommon/fwrapv, SINGLE_MUL, 26-bit limbs and
DOUBLES_LOW_HIGH. The strict C99 adapter asserts long=4, double=8, char=8 and
the IEEE word layout. A typed dependency shim isolates legacy header-private
declarations without patching the original library. Undefined-behavior
instrumentation is retained with halt_on_error=1. Both versions passed 13
fixed generic prime/composite fixtures through both routes: 52 development
observations before the historical cohort. This is a recovered-source modern
build, not an authenticated historical executable or compiler/libc replay.

Source commit 02e5bfc524a0a2564d8e353f32c6c9bd06f731d7 launched the immutable
script after a successful S3 gate. Serial QEMU i386 7.2.0 calls used a 256 MiB
guest reservation, 768 MiB host virtual-address cap, 60-second per-call and
600-second total watchdog, with no retries and disabled core dumps. All 84
processes exited zero. No clock, CPU/rusage measurement, performance isolation,
speed comparison, DLP experiment or security conclusion is admitted.

## Output correction preserved separately

The initial observer required a single-line decimal root. Original zwriteln
uses zfwrite_c, which emits backslash-newline continuations at its line width.
Consequently all 12 larger orders per configuration were initially
indeterminate: 4 published and 5 skipped were admitted, with no rejection.
Every original routine had actually returned 1.

[Saved validation 002](VALIDATION_PROTOCOL.md) corrected only the observer's
source-defined continuation grammar after seeing those outputs. It requires
the complete requested decimal root, exact ABI/method, one final result and no
trailing data. Three positive grammar controls and seven malformed/root-bound
controls pass. All 84 saved input/status/stdout/stderr packets are hash-bound
before and after validation. No original routine was rerun, and no original
input or output changed. The frozen initial source and 48 initial unknown
observations remain in the raw archive. This is a disclosed post hoc I/O
correction, not a new hypothesis or 84 new samples.

The [unified native export](export.json) has 84 planned, attempted and completed
calls, zero incomplete/interrupted/resumed/excluded. Unmeasured host, clock,
rusage and execution-time fields remain null. Its verification concerns the
input-bound software observation, not a new mathematical prime certificate.
The corrected analysis hash is explicitly different from the initially frozen
observer. [Export admission](evidence/export-admission.json) checks structure
and all referenced bytes; it does not claim the canonical Python checker ran.

## Backend and development failures

Current repository policy defaults new CPU experiments to EKS Spot. The actual
configured account matches, but CodeBuild receipt reads and EKS cluster access
are denied; no custom-command MCP is installed, and the portable bridge admits
the ecbench benchmark contract rather than this legacy command. No Spot job or
paid resource was launched. A prospective amendment records the bounded local
fallback within the existing authorized native-analysis workflow, with the
same inputs/controls and no timing claim. No IAM, network policy or service
protection was broadened, and no unrelated fixture substituted for the task.

| Development outcome | Attempts |
|---|---|
| Strict driver exposed legacy header warnings/private declarations | 5 |
| Unadmitted LP64 platform configuration timed out on generic fixture 2 | 2 |
| ILP32 link, executable-mode or missing endian setting failed | 3 |
| Final explicit ILP32 configurations passed all fixed fixtures | 2 |

All 12 development attempts are preserved, including every error. The LP64
SINGLE_MUL union has incompatible unsigned-long words; the original configurator
requires DOUBLES_LOW_HIGH on x86. These are platform-control failures, not
rejected historical curves. The first ILP32 link lacked Zig's O0 UBsan runtime;
the runtime was linked rather than disabled. The corrected observer, native
exporter and operational metric repair have separate strict build checks.
[Descriptive reports and PDFs](reports/INDEX.md) cover development, actual
cohort execution and saved validation.

## What this changes, and what it leaves open

This excludes the tested FreeLIP versions/configurations as a universal
reject-all-skips explanation when supplied the correct original orders. It
does not exclude a caller feeding different or incorrect point counts,
unsupported historical ABI/compiler behavior, an order-computation failure,
a different library/configuration, or another acceptance step. The exact
original generation/acceptance program is still missing.

The complete-factor comparison remains 84 proved complete records over all
21 curves. The existing two-test generation and three-test structural fits
remain empirical, with their original calibration and root-convention limits.
Exact embedding and SEA-proxy tests also remain separate negative evidence.
No trait difference or recovered software behavior establishes an attack on
selected or skipped curves. Neither weakness hypothesis is resolved.

| Requirement | Status |
|---|---|
| Original ICV1/DiSSECT assessment and four factorizations | Preserved complete; all 21 controls retained. |
| Actual FreeLIP full-census replay | Complete: 84/84 calls; every configuration accepts all 21. |
| Seven-call/base-point relationship | Source-supported proposed mechanism; original caller and exact new reconstruction unverified. |
| Historical skip criterion | Unresolved; no supported rule explains every skip. |
| New attack or measured DLP | Not established; no DLP experiment performed. |

Checked the canonical IC scoreboard, progress timeline, curve diagrams and
performance-gain pages: this functional order-only result supplies no attack,
speed or crossover value to promote. Those graphs remain unchanged. Study
figures and the neutral distinct-order coverage metric are updated. The
[existing problem record](../../../problems/nist-candidate-selection-weakness-hypotheses.md)
preserves checkable historical-evidence and attack obligations.

## Evidence custody and reproduction

The preparation archive was stored and fully downloaded before any historical
call: 417 source/development/prepared-package files verified. The final archive
contains the original 84 execution packets, corrected saved validation,
initial/final summaries, snapshots and native exporter. [Preparation receipt](evidence/preparation-delivery.json)
and [final receipt](evidence/final-delivery.json) bind complete bytes and every
manifest member. No credentials or whole workspace were archived. The
[reproduction guide](REPRODUCE.md) explains hydration, extraction and hashes.
Current PDFs/PNGs are stored with full GET verification; old studies stay frozen.
