# Inspect and reproduce the source-check evidence

Hydrate `evidence.tar.gz` using the repository artifact tool and `.artifacts.json`. Verify the complete archive SHA256/byte count in artifact-delivery.json, then every regular member against archive-members.json before extraction into a fresh private directory. The archive contains failed/ and passed/ source/vendor/config/attempt trees plus the selected test executable; inspecting its bytes does not require executing it.

The passing raw stdout is `passed/attempt1/stdout`. Its SHA256 is in successor/attempt1/stdout.pointer.json. `gzip -dc counter-pairs.csv.gz` yields the ten-column extraction; counter-pairs-custody.json binds compressed and original CSV bytes. Root independently re-parsed the complete raw row bodies, including all other arithmetic and meter fields, as recorded in root-postreview.json.

Review the original and successor command.json/run.py/freeze.json separately. Frozen absolute paths describe the original private closures. Replaying on a fresh path requires an explicitly recorded path rebind and new finite admission; do not claim identical command bytes after relocating paths. Vendor files and source closure are archived, while compiler/sysroot byte pins refer to the installed Rust1.99.1 toolchain. System libraries and compiler itself are not freshly source-built by this check.

The selected test is progress_native::finite_difference_integration_tests::finite_difference_progress_route_pairs. The private lib.rs adds only test wiring for the complete actual progress module and sink. The candidate file excludes appended tests; canonical-application.json binds its exact application. Neither an unfiltered test suite nor solve_one/scientific execution is needed to inspect these records.
