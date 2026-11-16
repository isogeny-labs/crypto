# Prompt-compatible serialization and fail-closed execution, before request003

Request002 is retained as a second failed qualification. A leading `>;`
line did not close the relation ideal on this service. Execution continued
with the earlier two-variable probe ideal, emitted the expected-looking
relation header, then failed when projecting thirteen-variable coordinates.
Strict response admission rejected it. PCC-20261116-a4d489 is unsigned
[pcc:unverified].

Primary source inspection identifies the service wrapper's
SetIgnorePrompt(true): Magma-Maths/calculator app/magma_cmd.py at
1e6267e22a19e75a6a3b6a3cc43bae3ce675e4a7. The Magma environment handbook
documents leading `>` prompt handling. For public unit input only, join
each standalone `>;` closing line onto its preceding polynomial line.
This whitespace-only change preserves the frozen polynomial system and
the original input is retained. Prepend SetQuitOnError(true) so that an
error cannot continue into a stale ring/ideal. Both native exact replay
and strict response admission remain mandatory.

Request003 combines the corrected field probe and dense m2/d1 fixture.
After it passes, the other three raw arms may share one sequential public
request, delimited by exact BEGIN_ARM/END_ARM markers. Likewise the four
optional preconditioned units may share one request after native certificate
tests. Each batch stays within51200 input bytes,20480 output bytes, one
request at a time and25 seconds. This reduces request count and makes no
timing comparison. The original total cap remains nine requests; no
failed bytes are retried automatically [pcc:unverified].
