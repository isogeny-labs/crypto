# R89 all-generator transport: rejected first bound

The R89 source and protocol were frozen and pushed at
`a624ea54235799a37e5c2488679d3e1ea7323b5e` before execution. The
single native attempt `transport89-01` tested the first of 32 R50 PID
generators under each direction of the accepted rational diamond map.
At pole power at most 51, both directions emitted
`RR89_POLE_BOUND_EXHAUSTED_DIRECTION_{1,2}_COLUMN_1`. Singular returned
process exit 0 but emitted two `?` errors. The driver correctly rejected
the attempt (`FAIL transport89-01 native=0`).

The output subsequently printed `full_products=64` and
`TRANSPORT89_ALL_CHECKS_PASS`; these are **false positives** caused by
`ERROR` inside the `require89` procedure returning to the top level.
The exported certificate contains only its header. No full column image,
corruption rejection, or 64-product claim is accepted from this run.
The recorded failure is a bound/procedure defect, not evidence that the
normalized ring map is impossible.

`BOUND-ERRATUM.md` derives a conservative bound 61 from the saved R27
cusp valuations, the R50 degree-37 denominator, and relative degree 15
in `T`. The next attempt must use a new frozen source/protocol and an
unambiguous fatal stop. That attempt will test the actual 64 generator
images and full module products.

The exact raw member hashes are in `runs.sha256`. Their deterministic
archive is SHA-256 `2419cdde8b3c028a7d0c7f9f11bef2be5e689e48daf8ddf7d78159c74213cf86`
at `s3://setec-astronomy-artifacts/store/sha256/24/2419cdde8b3c028a7d0c7f9f11bef2be5e689e48daf8ddf7d78159c74213cf86`.
The checksum-protected PUT, fresh GET, archive digest, and all 18 member
hashes were checked (`raw-verification.txt`, `raw-member-check.log`).
