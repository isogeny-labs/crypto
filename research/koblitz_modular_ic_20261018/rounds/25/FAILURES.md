# Round25 failed attempts and controls

Lean01 fails four elaboration goals because the untyped simp rewrite does
not rewrite the ring numeral2. The explicit typed equality (2:R)=0 replaces
that invocation. The failed source and output remain retained; sorryAx in
that rejected compile is not an accepted proof. No native or Lean failure
is interpreted as nonexistence. R24's sealed source/70 raw files are unchanged.

Deck25-positive-01 and -02 are rejected. Initial inspection attributed -01
to intermediate n1,l1,n2,l2 names and changed these before -02. Full diagnostic
inspection of both logs identifies the earlier primary error: `kernel` is a
reserved Singular name. Its declaration fails, as does kernel invariance;
following parse errors invalidate subsequent expressions in -02. Some other
blocks in -01 print zero, but neither attempt certifies the complete chain.
Rename the ideal to kernelCoefficients before -03. This corrects the earlier
resume note's inaccurate statement that only involution failed. Preserve both
exact scripts/logs; native exit0 and a late marker do not pass the wrapper.

The first6.3KiB checkpoint transport snapshot was accidentally staged with
source. Preserve its exact committed bytes and history. Subsequent mutable
transport uses checkpoint-live.tar.xz, untracked and explicitly unignored for
the checkpoint helper; never overwrite the committed initial snapshot.

Positive03 removes the reserved-name error and checks all5 source residuals,
8 entries and3 invariants with zero residuals. It is still rejected because
Singular's map application requires a named polynomial, rather than the
bare ring variable n or l. Bind sourceNu/sourceLambda and map those in04.
Cascading later parse errors are not mathematical failures or certificates.

Q25-registered01 is rejected at the ring name quotient, also a reserved
Singular identifier. Its late marker does not count. Rename that ring qr25
and rerun the unchanged thirteen source-bound pullback certificates in02.

Q25-registered02 is refused before native launch. A concurrent artifact
manifest update changed the checkpoint fingerprint while gate was running;
the runner had no upload credentials to refresh it. Only gate.log exists,
with no native input/output. Record one gate refusal separately from native
attempt counts. Finish storage metadata, checkpoint again, then launch03
without concurrent repository mutations until input.sing has been copied.

Q25-registered03 passes the gate and begins native execution, then rejects
reserved polynomial name numerator. Rename it qrNumerator before04. No
thirteen-generator conclusion is accepted from this partial run.
Q25-registered04 is also refused before native launch. The preceding commit
had not finished when checkpoint hashing began; the timestamp/head in the
upload does not prove its earlier fingerprint saw the final HEAD. Preserve
this second gate-only directory. Wait for the completed commit, then sequence
checkpoint push and fresh registered05 in one shell, with no concurrent
repository mutation. Native equation source is unchanged from8db5f5c0b.

The checkpoint before the corrupt control fails before any runner launch:
/tmp has1048576/1048576 inodes used. Persistent research files are intact;
no native call is counted. Retain the error log and set the checkpoint's
TMPDIR to the task-owned /var/tmp/koblitz-r25-checkpoint. Do not delete or
change other jobs' temporary files. Native limits remain4GiB/1200s.
