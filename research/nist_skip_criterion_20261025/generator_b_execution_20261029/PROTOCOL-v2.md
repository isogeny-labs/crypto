# Version 2: raise the virtual-address ceiling

Version 1 remains frozen and its failed attempt is preserved. Its positive
control passed, but production timed out at 300 seconds before writing point
traits. Verification did not start. Partial files are unadmitted.

Development diagnostics locate the stall at proof of the fixed public P-192
field prime. The exact same diagnostic binary times out under 768 MiB and
completes all its prime/group checks under 1,536 MiB. A separate resource check
then proves all 21 already-known p/n pairs, covering every one of the six field
sizes, under that larger ceiling. These observations concern execution setup;
they do not explain a historical skip or establish curve weakness.

Change only the address-space budget to 1,536 MiB, uniformly for all phases.
Keep the native producer/verifier binaries, primality algorithm, full inputs,
trait catalog, point/coefficient controls, fixed envelopes and scalar recipes
unchanged. Keep the 30/300/300-second watchdogs and no-clock claim scope.
Use fresh production-run-002 outputs and one explicitly amended attempt.

The amendment follows the observed timeout and resource diagnostics, before
affected point-comparison output. Already-seen material consists of old
inherited-feature differences, the four supplied-point checks and known-input
primality checks. Labels were already visible. No point-trait matrix, recipe
results or fitted envelope outcomes existed from version 1.

Freeze and commit protocol-v2.json, plan-v2.json and run-v2.sh before launch,
then push the required S3 checkpoint. Retain the version-1 failure, absent
verification and every diagnostic. All 21 choices remain required. An exact
finite fit remains exploratory; historical and vulnerability claims stay open.
