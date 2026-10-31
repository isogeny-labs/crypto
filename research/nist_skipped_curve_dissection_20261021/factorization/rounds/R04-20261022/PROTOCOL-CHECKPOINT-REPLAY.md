# Final checkpoint recognition diagnostic

Recorded after the single production allocation stopped and before this
diagnostic. Native GP has validated 329 saved candidate records and highest
leading coefficient 900. Copy the finalized state to a separate control
directory; never mutate the preserved production checkpoint for this test.

Repeat R03's input/master recognition diagnostic for at most three seconds,
one-second kill grace, pinned native YAFU and the same input/seed/options,
with verbosity enabled and polynomial batch 120. Require native messages
that the input matches and the last leading coefficient is 900. Any
incidental arithmetic is setup work and is not reported as additional
production coverage or a factorization result. Preserve stdout, stderr,
status and command. No further production allocation is authorized by
this diagnostic record.
