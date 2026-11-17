# Verification and custody after worker loss

The original worker compiled the standalone three-module project using:

```sh
env PATH=/workspace/prime-degree-lean/bin:$PATH LEAN_NUM_THREADS=4 /workspace/prime-degree-lean/bin/lake --no-cache build > verification/development/build-attempt03.log 2>&1
```

The command completed with exit status 0. The recorded tool output reported
all three modules and twelve `#print axioms` statements successfully. The
new modules reported only propext, Classical.choice and Quot.sound; the
copied ring identities reported propext and Quot.sound.

Before that build, the additional pinned Mathlib dependencies were built
from source with this command (exit 0):

```sh
env PATH=/workspace/prime-degree-lean/bin:$PATH LEAN_NUM_THREADS=4 taskset -c 0-3 /workspace/prime-degree-lean/bin/lake --no-cache build Mathlib.AlgebraicGeometry.EllipticCurve.Group Mathlib.Tactic.LinearCombination > /workspace/scratch/secant-group-mathlib-source-build.log 2>&1
```

## Failed development and incomplete integration

Initial elaboration attempts failed and were corrected before successful
build03. The worker retained the first attempted source snapshot and build
attempt logs under `verification/development/`. Those files became
inaccessible with the worker. Their complete bytes and diagnostic output
cannot be published from this continuation; no replacement raw logs are
invented. The recovered successful output is explicitly labeled as an
excerpt rather than an original log.

After build03, an optional integration changed the Lake dependency layout
to reference `../proof`, moved the duplicate identity source aside and ran
`MATHLIB_NO_CACHE_ON_UPDATE=1 lake --no-cache update secant_formalization`.
That update exited 0; no subsequent build or F17 coupling theorem completed.
The published project instead uses the standalone configuration that built
successfully. The abandoned layout is not reported as tested.

## Outage and recovery

The execution transport disconnected and recovery timed out. The selected
environment was then explicitly marked unavailable; a later command
returned "No usable execution environment is available." No environment is
marked starting. GitHub connector access remained functional.

The source text was recovered from the recorded edits. The identity module
and dependency manifest were fetched from merged prior proof sources. The
published manifest changes only the root package name and JSON formatting.
SHA256SUMS identifies the recovered published payload; it does not assert a
pre-outage file-byte checksum that was never captured.

## Outstanding verification

- Rebuild the exact published source bytes in a live native environment.
- Run the unchanged whole-module repository axiom gate and retain its output.
- Run warnings-as-errors and lean4checker --fresh when available.
- Add a direct theorem combining the existing concrete F17 fixture with this
  generic group bridge if a single bundled statement is desired.
- Render and inspect the supplied report PDF.

The GitHub connector exposes no manual workflow-dispatch operation and no
browser-control tool is available. The unchanged Lean workflow discovers this
Lake project on the matching main-branch push. Its future status is not
reported as a completed check.
