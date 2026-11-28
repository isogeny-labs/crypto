# Root-finding baseline errata

Status: arithmetic correction from the committed raw rows, `[pcc:unverified]`.

## A = 1024 table-build equivalent

`RESULTS.md` says the 32-bit prime-field table build costs as much as about
1,100 default targets. The committed row records a 0.110216525 second median
build and 195751.72265625 nanoseconds per default target. Their quotient is
563.042 targets.

The correction changes only that explanatory equivalent. The frozen timing
rows, tables and raw data remain unchanged. The native recomputation in
[`VALIDATION.md`](VALIDATION.md) checks the inputs and reports this correction.
