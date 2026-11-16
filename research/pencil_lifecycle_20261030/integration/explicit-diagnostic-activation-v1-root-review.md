# Root review of explicit diagnostic activation v1

Patch SHA256: 6a05f2f3129f47475f14372ffd77e6802731527ce3b691c12297256d234cc582. Reviewed independently of the author against the current runner.rs and record.rs.

The pre-plan gate hashes configured executable bytes and checks the exact spec identity before workload planning. The existing execute_session gate independently checks the captured build hash and complete resolved plan. Child admission checks the actual running Linux inode, then the rebuilt full workload binding before the lifecycle callback. Absent context retains its old dispatch; mixed contexts and legacy v1 still reject. Fixture changes cover actual binary mismatch, wrong full workload across all three arms, and pre-plan file/spec mismatch. No correctness defect found in this delta by source review.

This review is not compilation or runtime validation. Named final-tree tests and full regression remain required. Non-Linux current_exe is a pathname observation, not Linux inode custody. The launch source digest is still an external manifest binding. Software activation supplies neither a prospective protocol freeze nor cumulative worker admission; no scientific execution is approved by this review.
