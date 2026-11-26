# Review of retained attempt-01 receipts

2026-11-24 UTC. Separate-agent review of retained files and reported results.
The reviewer did not rebuild the package, rerun its tests or checker, or
recompute finite-field ranks, root jets, or elliptic point arithmetic.
This is an evidence review, not an independent-host execution or signed
scientific verdict. Standing remains unsigned producer development evidence.
[pcc:unverified]

## Disposition

The retained records support the bounded statement that the frozen native
M31 component built, its four selected binary unit tests passed, and its
fixed checker reported all sixteen supplied-witness cases and four synthetic
controls passing. Every fixed case reports original collapsed rank one and
six-polar rank three. The complete eighty-coefficient arrays from the two
jet constructions are retained and agree in every case.

No source difference was detected between the frozen snapshot and the
current working template among the nine files in the source manifest.
The checker executable also still matches its recorded pre-call hash.
The provenance limits below remain part of this finding. [pcc:unverified]

## Inputs inspected and hash bindings

The review read the [frozen manifest](evidence/attempt-01/frozen-source-manifest.json),
all three command/exit/source-check receipts, their toolchain records and
stdout/stderr, the checker's binary-hash records, and the complete
[37,948-byte checker output](evidence/attempt-01/check/stdout.txt).
The recorder source was read to distinguish performed checks from default
status fields.

The reviewer independently recomputed SHA-256 digests of all nine retained
source files and their corresponding working-template files. Each matched
the frozen manifest. The nine entries cover the native main, Cargo manifest
and lockfile, frozen protocol, command recorder, dependency metadata
adapter, original native library, and its two included field/solver modules.
The three pre-command checksum inventories have exactly these same nine
paths and digests. Their post-command reports each contain the same nine
paths marked `OK`, and all three source-check exit records are zero.

| Bound item | SHA-256 |
| --- | --- |
| Native `main.rs`, both frozen and template | `4246bf15ca2d5a8484ccd80d051b757245198bddd89787869484a3cd63c41559` |
| Frozen source manifest | `05441516d5ff4244d75c8007154d7ceb12af561a5996ab533e6f51b5329f52e8` |
| Checker executable | `c421988eb0dab0787497ddc7880f158a3380507e86f2dd2262bd4f9929c01f8b` |
| Raw checker stdout | `c6c9da55547b469db1d702aa9db725032d6e9e3cd2a4d6c3389e2e8cd50c4827` |
| Raw unit-test stdout | `b405ca49089e8da4f1cc7040db2157c54f4425da58431953f4364284cf5c86b6` |
| Raw build stderr | `7a7660a064074d42790fe41e6bba5ad1c91c2d4148b6bbf445bd2a97a84b93ec` |

The checker binary's pre-call record and post-call `OK` record agree, its
binary-check exit is zero, and the executable present at review time has
the same digest. This is a local producer provenance chain; it is not a
hermetic or remotely attested build.

The build receipt records parent commit
`62c856c14e5dae68b520022db3debdeb3090ba64`. The unit-test and checker
receipts record `72024a791ec48780c2c57c9e0bdaf9d0b54fceaf`. The differing
commit labels do not indicate a change in the declared source closure:
the nine content hashes are identical across all three invocations. The
session log records the intervening explicit tracking of the two unchanged
lockfiles.

## Retained execution records

| Invocation | Recorded command scope | Retained outcome |
| --- | --- | --- |
| [Build](evidence/attempt-01/build/command.txt) | `timeout 300s cargo build --release --locked --offline` against the frozen manifest path | Exit 0; release compilation completed; source check 0 |
| [Unit tests](evidence/attempt-01/unit-tests/command.txt) | `timeout 300s cargo test --release --locked --offline`, selected binary, `--test-threads=1` | Exit 0; 4 passed, 0 failed, 0 ignored, 0 filtered; source check 0 |
| [Fixed checker](evidence/attempt-01/check/command.txt) | `timeout 300s` applied to the retained release executable | Exit 0; valid complete JSON, `all_pass=true`; empty stderr; source and binary checks 0 |

All three receipts record rustc 1.100.0, compiler commit
`0ef1bc386059596dada8fb65b50425aab2ea637a`, LLVM 23.1.5, and Cargo
1.100.0 on `x86_64-unknown-linux-gnu`. The build's recorded UTC interval
is 06:22:22 to 06:22:45; the unit-test and checker calls have 06:23:56
timestamps. These timestamps and Cargo's diagnostic duration are execution
metadata, not performance measurements or comparative runtime results.

The four test names in the retained stdout cover arbitrary-root Graeffe
composition, a prescribed nonlinear implicit root lift through order three,
mixed and repeated Taylor derivative factors, and the deliberately
rank-deficient normal outside the regular hypothesis.

## Raw-result accounting

The JSON records the unchanged field p=2147483647, curve coefficients
A=847514846 and B=2087444724, group-order field 2147446831, and generator
(625037715,2100627251). Its depth-ten selector has recorded terminal
tau=1616683971, 1024 raw selector roots, and 509 lifted base points.
Those are retained fixture results; this audit did not recalculate the
curve order or rebuild the factor base.

Structural inspection of the full output confirmed:

- Exactly the case IDs 0 through 15 occur, in order, with no replacement.
  Case k has indices `[4*k,4*k+1,4*k+2,4*k+3]` and sign mask k, matching
  the frozen protocol. Every case retains four signed points, its target,
  RR5 parameters, quartic, tangent matrix and normal.
- Every case has status `pass`, `original_collapsed_rank=1`,
  `f3_hessian_jacobian_rank=3`, and a retained six-by-three polar matrix.
- Each of the two centered-jet arrays has four rows of twenty coefficients.
  Direct comparison of the stored integers confirms array equality for
  every case: eighty comparisons per case, 1,280 across the panel. This is
  a comparison of retained outputs, not a fresh mathematical replay.
- All four centered constant coefficients are zero. The centered e3
  coefficients of degree two are zero in all sixteen records. The original
  linear data have a nonzero first row and zero remaining rows, consistent
  with the recorded collapsed rank one.
- The independent-terminal-coordinate control reports rank zero in all
  cases. The wrong-center control retains e3 value 2147483643, namely
  p-4, in all cases. The unmodified zero-centered residual is retained
  separately.

The four synthetic records retain both the expected success cases and the
hypothesis boundary:

| Normal | e1 plus gradient-e2 rank | e3 six-polar rank | Scope |
| --- | ---: | ---: | --- |
| (1,1,1,1) | 3 | 3 | Nonzero normal |
| (1,2,3,4) | 3 | 3 | Nonzero normal |
| (1,0,0,1) | 3 | 2 | Deliberately outside the all-nonzero hypothesis |
| (1,5,1326042065,821441576) | 2 | 3 | Nonzero normal with recorded sum and square-sum both zero |

The exceptional construction is recorded as fixed b=5, discriminant p-88,
and the smaller canonical square root. It is a labeled algebraic control,
not an additional elliptic witness, a replacement case, or a sample of the
M31 relation population. [pcc:unverified]

## Provenance and interpretation limits

Only the fixed checker call has an actual pre/post executable digest. For
build and unit tests the recorder was passed its no-binary sentinel; their
`binary-check-exit.txt` value zero is the recorder's default and must not be
described as a binary-hash verification. Their source hashes and retained
compiler/test outputs are available, but the unit-test executable was not
separately bound by a pre-call checksum.

The retained command lines explicitly bind offline locked Cargo resolution,
the 300-second outer limits, and one test thread. The recorder does not
capture environment variables; a one-job build setting supplied externally
through `CARGO_BUILD_JOBS` cannot be independently established from these
receipts alone. The toolchain is version-recorded, not bound to an immutable
runner image or complete dependency-cache digest.

The reviewer has checked file hashes, manifest consistency, output shape,
fixed-case accounting and equality of stored arrays. Matrix ranks and
elliptic/jet arithmetic remain the producer's native observations, supported
by the earlier separate source and written-proof review. They were not
re-executed by this reviewer. The raw arithmetic counters describe the
original fixture helper only; they are not total charged operations for the
new jet computation.

This attempt establishes bounded component evidence on supplied witnesses
from the pure Dickson B509 sector. It does not measure discovery, F4,
finite-field root extraction, useful relation yield, global elimination
cost, or the mixed Y7 sectors of B677. It does not qualify the full root
crate, import a formal proof, execute the M61-only v1 identity checker, or
create signed PCC standing. These limits do not require a rerun of the
completed fixed control. [pcc:unverified]
