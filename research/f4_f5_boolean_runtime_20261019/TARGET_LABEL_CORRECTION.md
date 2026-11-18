# Correction: the legacy “random” flag is a generator control

The previous instruction report and the frozen continuation protocols call
the driver's `--random 1` fixture “random”. That wording is inaccurate for
these single-target invocations. In `examples/f4_f5_boolean_perf.rs` the
flag computes `1 + (i * 2654435761) % 1000003`, with target index `i=0`.
The resulting scalar is1, so the target is the registered curve generator.
It is an **unplanted generator control**, not a random group sample.

The native report uses that name. Raw `random` booleans and old directory
names stay unchanged for exact replay. Both arms still use the same original
target, input digest and verified root/decomposition output; no timing or
input was changed to make this correction. The seed230 planted fixture and
fresh planted seeds231/232 keep their existing definitions.

All statements about the old “random” n17/n23 rows refer to these particular
generator controls. They do not establish a random-target distribution or
an average-case solver ranking. Earlier wording is superseded by this note.
