# Registered small-field row experiment

Frozen design, 2026-11-23, before collecting timing data. Source is the `chained_s3_prime31_pilot` Rust example in PR #4920. This is a component experiment. The full cold ECDLP and Prime31 goals remain separate gates.

## Instance and controls

Curve `y²=x³+7x+4` over F101 has 97 points by direct enumeration, discriminant `-16(4a³+27b²) != 0`, trace 5, and is ordinary. The group is prime order 97. The factor base is the first B rational x coordinates in increasing order with canonical y, for B=4 and 8. Targets are all 97 group points, in increasing x order with both signs and the point at infinity. This complete cohort has no target sampling variance.

Run m=3 and m=4 for each B. Both arms use the same curve arithmetic, target sequence, signed factor-base points, relation-row format, and literal group verification. The group arm prepares all signed two-point sums keyed by complete point. The chain arm prepares S3 roots of two factor-base x coordinates and keys by projective x, then lifts candidates and checks complete points. For m=4 both arms prepare the signed right pair once, join left and right groups, and charge all table building, right states, probes, candidate lifting, validation and deduplication. The auxiliary x coordinate is the x coordinate of the intermediate sum and is unrestricted by the factor base. Intermediate infinity is a separate key. A shorter relation caused by cancellation remains an accepted row.

## Measurements and decision

Each process runs one arm/m/B cell from a fresh process. Register five repetitions per cell, 40 processes total, plus a same-binary A/A control repeating the group arm. The complete rows for every target must match the independent exhaustive signed-tuple oracle before timing is interpreted. Primary component measure is the total elapsed nanoseconds divided by distinct verified rows across all targets. Record total elapsed time, counts of successful targets and verified rows, group additions, doublings and field operations, S3 inversion calls, probes, lifted candidates, table keys and entries, largest bucket, right states, and process peak RSS. Wall measurements include one construction of both required tables and every target query. Compilation is excluded. One CPU core, release profile, same executable and hardware for all cells. Alternate arm order across repetitions. Failures and timeouts count against the arm. The finite budget is 40 runs, 60 seconds per process, and 1 GiB peak memory. Stop if this ceiling is reached. Record source SHA, Rust version, host CPU, raw stdout/stderr, exit status, and command for every process.

The comparison is descriptive at this small size. Report each m/B cell and all repetitions. The chain arm qualifies for further study only if it yields the identical row sets and has lower median total work per verified row than the group arm without exceeding the memory ceiling. A wall improvement on this toy field does not qualify a cold Prime31 or rho claim. If the comparison fails, identify the dominant measured cost from root preparation, bucket growth, candidate lifts, or group verification and revise the structure hypothesis.

## Reproduction commands

`cargo test --release --example chained_s3_prime31_pilot`

`cargo build --release --example chained_s3_prime31_pilot`

`target/release/examples/chained_s3_prime31_pilot --small-field group 3 4`

`target/release/examples/chained_s3_prime31_pilot --small-field s3 4 8`

Use the registered EKS Spot experiment route for timing runs and bind a checkpoint before launch. Keep raw files with the result note.
