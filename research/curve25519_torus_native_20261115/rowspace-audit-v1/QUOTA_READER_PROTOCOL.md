# Existing-output quota reader, frozen before code/calls

Metadata-only Rust reader of the SEVEN existing complete Audit JSON files in
native-v1.tar.xz. It performs no polynomial/field/rank/root computation and
reruns none of the original controls. Preserve hashes of the old whole archive
and all seven input files before calls. New source/binary identity is separate.

Count row-term plus origin-weight records across projection,baseline,combined
rank certificates,plus image terms. Report each phase and total. Compare total
to1million represented entries per complete old Audit. Do not substitute file
bytes for entry counts; one certificate exceeds1MiB but may have few entries.
The v1 source guard is per RankCertificate,not a globally shared memory bound.

Require exact schema,all three phases,array shapes,exact seven fixed inputs,
<=32MiB/file,checked integer addition. Retain positive synthetic count and
missing-array/schema refusals as native unit controls. This counts metadata
records,not cryptanalytic operation costs or RSS. Freeze source/Cargo.lock,
copied executable/compiled bindings,source/input hashes and distinct pre-call
record before one metadata-read dispatch. No old source/protocol/report edit.
