# Benign PARI stack growth, version 1

The first corrected p=59 divisor replay completed all its equations and
printed its completion marker, but the conservative native gate rejected
PARI's automatic stack-growth warnings. Preserve that rejection and its raw
session. Accept only the explicitly recognized arithmetic stack-growth
warning format, a decimal byte size no greater than 4,000,002,048, and the
already permitted initial maximum-stack warning. Continue rejecting all
other stderr, any error appended after a benign warning, missing markers,
nonzero exits and input/output hash mismatches. Supporting checks include
an oversized growth warning and an error mixed with a benign warning.

This changes the transport adjudication, not the mathematical CAS program
or its result. Re-adjudicate unchanged raw output into a fresh directory
with the committed gate. Do not rewrite or relabel the earlier rejection.
