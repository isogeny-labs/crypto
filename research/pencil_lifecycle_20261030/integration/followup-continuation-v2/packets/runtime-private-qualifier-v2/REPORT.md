# Private qualifier token-mode correction

The new qualifier-v2 explicitly sets token mode0444 after writing and flushing, then fsyncs and verifies regular file type, exact root uid/gid, mode, size37 and linkcount1 before Docker copy. This closes the combined-review defect: service UMask0077 changed the original os.open request0444 into0400, which the held gate correctly rejected.

Fourteen source/mock and real-tempfile tests passed. The new real-file control sets umask077, runs the exact token writer with explicit current test uid/gid because the worker is unprivileged, and verifies final mode0444 and complete nonce bytes. A wrong-custody control rejects. The production call supplies no uid/gid override and requires0:0. Existing endpoint, ancestry, release ordering and final-boundary checks remain. No Docker, systemd, sudo, namespace, mount or whole-gate execution occurred.

The independent reviewer replayed all14 checks and found no remaining required source correction. The supervisor and qualifier-v2 interface was judged source-compatible at the reviewed snapshots; this is not runtime integration validation. Exact source hashes and the original immutable commit are retained.

The gate.c source remains SHA2566548d8d41a37975bb30c1a225583113ea6c25056753b3d455d90f1608d8e65bc. All Q0-Q4, PID/CPU/memory controls, strict empty diff and head_contract binding are preserved. New gate binary/control evidence and runtime/image admission remain separate and unset. The API is unchanged from runtime-private-qualifier/INTERFACE.md; supervisor now targets this v2 source path.

Original packet and its historical review are preserved. correction.patch shows only the implementation/test change; prior-sources.json binds the sources before correction. This software correction changes no scientific result or planning authority.
