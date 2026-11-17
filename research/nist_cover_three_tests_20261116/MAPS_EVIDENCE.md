# Map evidence layout and reproduction

The six successful native development units are complete-maps-v1,
transpose-maps-v1,closed-transpose-v1,moving-transpose-v1,pole-cases-v1 and
complete-km-v3 under evidence/. complete-km-v1/v2 preserve the rejected build
and rejected comparison. These are source-qualified producer records,not
independent execution verification. [pcc:unverified]

Each directory retains source/,build and pre/post-call hashes,logs and exits.
Successful directories also retain result.json. The largest new JSON result
is762107bytes. Code remains loose because the immutable manifests bind its
individual paths (AGENTS section13a's path-resolving exception). JSON files
remain below the1MiB policy threshold;the historical artifact tool confirms
that .json is not a64KiB DATA_EXT. [pcc:unverified]

Native qualifier binaries are ignored local build products,their hashes in
the pre-call manifests remain. Rebuild with the named check script rather
than claiming a separately uploaded executable. Object-store tooling/config
and AWS CLI are absent;no S3 custody is asserted. Existing historical pending
large files remain excluded,as in the preceding quotient PR.

The cancelled IC preparation is retained as
evidence/cancelled-plane-ic-20261117.tar.gz (64549bytes,below65536).
It includes three failed preparations and the cancelled source/protocol.
No arithmetic was executed in that abandoned direction. Its raw compiler
diagnostic is preserved;it is not a scalar-recovery result.

Example,from this study directory,with the archived kernel3 study extracted:

    study=/absolute/path/to/kernel3-replay-study
    global=/absolute/path/to/evidence/global-correspondence-v2/result.json
    bridge=/absolute/path/to/evidence/generic-bridge-v1/result.json
    formal=/absolute/path/to/evidence/formal-series-v1/result.json
    sh check-complete-maps.sh "$study" "$global" "$bridge" evidence/NEW-forward
    sh check-transpose-maps.sh "$study" "$global" "$bridge" "$formal" evidence/NEW-transpose
    sh check-closed-transpose.sh "$study" "$global" "$bridge" "$formal" evidence/NEW-closed
    sh check-moving-transpose.sh "$study" "$global" "$bridge" "$formal" evidence/NEW-projective
    sh check-pole-cases.sh "$study" "$global" "$bridge" evidence/NEW-poles
    sh check-complete-km.sh "$study" "$global" "$bridge" "$formal" evidence/NEW-km

Each directory must be new;the qualifier sources are copied before compile
and calls run against that immutable copy. No benchmark times are admitted
or recorded. The original proofs/reports are unchanged;MAPS_STATUS is their
successor,not a retroactive rewrite of their open-implementation statements.
