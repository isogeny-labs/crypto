# Next mechanism and source-version discipline

First read the primary complete Edwards/projective addition and Montgomery
conversion sources. Then compose several member modules against a public
target, handling identity, equal-point and inverse intermediates. An
affine-addition-only restriction would need its own analyzed protocol.
Keep normalized point/index recovery and complete costs explicit.

Current Circuit::image evaluates numeric output coordinates but does not
export their symbolic roles. Add a public symbolic-output interface rather
than guessing variable slots or names. This changes Rust source and requires
source-v2/native-v2 qualification. Preserve all v1 snapshot/manifest/record
bytes; source-v1 is a historical snapshot, not a hash assertion about future
working code. Direct Lean645f28 source/configuration stays immutable.

Acceptance for aggregation: public parameters/target only in compilation;
complete group-law cases; valid original-relation witnesses; fixed native
controls including cancellation/doubling/identity; explicit chart guards
and independent native point checking. It is still a construction until
an actual decomposition algorithm and cost/yield/family bounds are supplied.
No new solver campaign, experiment or paid run is authorized by this note.
