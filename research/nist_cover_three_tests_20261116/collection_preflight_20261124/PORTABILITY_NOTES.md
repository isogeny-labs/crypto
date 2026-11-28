# Replaying driver transport with a fresh native build

Keep the original transport source and recorded proof subjects byte-equal.
Its first-run helper uses the successful qualification artifact's local bin
path. Add a metadata-only replay wrapper that accepts any fresh qualification
directory and stages a minimal isolated mirror of the required source/data
layout,then runs the exact original helper in that mirror. It must not modify
or replace the original evidence directory or require its ignored binary.

Before testing the wrapper,freeze the same two transport outcomes:recorded
input reaches both sentinels;literal rank-field drift stops before them.
Require exact staged driver/helper bytes and retain native binary/source/input
bindings. All scientific dependencies remain stubs. Use120seconds/1GiB and a
fresh write-once output. No actual mathematical run or admission is asserted.
[pcc:unverified]
