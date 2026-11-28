# Separate result readback verifier, development receipt

`verify-result-readback.py` checks the exact downloaded file set and its SHA-256 list, all 86 public inputs, pod/job destination, frozen source and image identities. A full-mode path requires 1,682 validated rows, replays every retained certificate with the exact arm binary, checks every tiny curve independently, and re-analyzes the paired rows with the exact panel binary. It emits no speedup verdict. [pcc:unverified]

Offline structural controls accepted a synthetic exact file closure and rejected changed, extra or omitted files, changed public input, wrong S3 prefix and checkpoint identity. An incomplete tree was rejected in full mode. The toy oracle was checked on all 84 public cases; a changed target was rejected. The certificate JSON digest format was checked against the existing genuine p5 receipt fixture. These controls ran no solver, benchmark or S3 call. The full-mode path has no admitted live result yet and cannot be reported as a passing independent audit. [pcc:unverified]

`development.tar.xz` retains complete local syntax, structural, checkpoint, publisher, workflow and gate logs; `source.sha256` pins all changed controls and the exact receipt fixture. The archive was extracted and byte-compared to its source logs. [pcc:unverified]
