# Source session

T-2182, fence 1, 2026-11-11. Preserved the v7 source and terminal evidence,
including successful materialization of 4,485 source files and 2,165 cache
files with 82 executable modes. The v7 compiler ran for 287.088 seconds before
the monitor encountered a concurrently removed Rust temporary directory;
tests did not run.

V8 introduces one shared fail-closed tree scanner and one charged control. The
scanner accepts only confirmed `ENOENT` within explicitly mutable compiler
trees or exact atomic receipt-temp names. The control is bound into the packet
and worker command sequence but was not executed during source preparation.
No compiler, test, resolver, scientific payload, or cloud action was run.
