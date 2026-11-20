# Completed Runpod timing matrix

Sourcec0176f942; podwg53vkdhjsv6lk; worker exit0; DELETE204. Read
../RESULTS.md and summary.json for the scoped native observations.
The complete archive is3298986bytes with SHA-256
0b27d27910807fc9e5b047e4f64999ba71d603178537c78ac7b7f1cb06be81fa.

Both upload and full readback matched before the artifact credential failed.
The approved, per-run source object remains named in custody.json:

s3://setec-astronomy-artifacts/store/runpod/trace-incidence/c0176f942c03a549a98cb680b816e864b8820333/43907109841/results.tar.gz

Later publication calls return InvalidAccessKeyId, so the requested CAS copy
is pending. No .artifacts.json falsely advertises that missing CAS object.
Use a functioning authorized AWS identity to fetch the source object and
check its digest before replay; preserve its original bytes. The collected
archive/readback copies also remain in the originating executor's private
result directory. No data was replaced or measurement rerun for this error.

Native and rho pre/post qualifier hashes are retained here. Summary and
analysis remain unsigned statements, not signed PCC verification.
