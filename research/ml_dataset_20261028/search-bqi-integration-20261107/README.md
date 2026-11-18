# Search V2 and independent BQI1 integration

This standalone test connects exact Search V2 nodes and committed proof rows to the reviewed scheduler-BQI bridge and existing independent checker. All source donors are copied without modification; receipt.json records their hashes.

18 tests passed, zero failed/ignored: two new cross-component controls plus16 unchanged checker/sparse controls. For original equations1+x0 and1+x1+x2, search retains a false-x0 child with a constant-one refutation involving the explicit branch generator. The independent checker accepts that exact node identity on the same cumulative owner/deadline. Replacing the branch generator x0 with1+x0 makes independent verification fail with EqualityMismatch. A separate original-unsatisfiable fixture independently checks its root constant-one row.

This verifies arithmetic binding to the supplied exact node equations. It does not authenticate a CheckedMap, validate whole branch coverage, perform signed group lifting, make original-owner admission, or publish an ECDLP outcome. Those remain integration requirements. Search's own18-test suite is separate evidence, not counted as executed by this harness. No scientific data or cloud execution occurred.

Reproduce with `rustc +1.98 --edition=2021 --test harness.rs -o /tmp/search-bqi-tests`, then `timeout 60 /tmp/search-bqi-tests --test-threads=1`.
