# Independent V3 corrective review

Patch SHA256 `b303bb932aa629cc6a10d203f91bbfbdf6be0fbe27bf908318b4ab279b1c4d7e`; all three proposed source hashes match source-manifest.json on independent readback.

Reviewed changes against the V2 findings. No remaining blocking defect identified in this corrective slice. Campaign supervisor captures one CLOCK_MONOTONIC anchor before sampling UTC, retains fractional remaining time, and passes the same Copy Deadline to watchdog, action checks and subprocess supervision. It does not recapture a larger duration after a wall-clock rollback. UTC forward changes still stop execution; invalid monotonic reads fail closed. Legacy noncampaign entry points remain UTC-only and are outside this campaign qualification.

Environment comparison adds only omitted ObjectFieldSelector apiVersion=v1 on actual objects, leaving every other value/key/order unchanged. Init and ephemeral container entries are rejected unless absent or empty. Existing root PID1/private namespace checks and single regular container checks remain. The code cannot prove that a separately authorized operator will not inject a debug container after inspection; that remains an external custody assumption.

Inspected fake-clock coverage and source wiring; reviewer did not run watchdog, compiler, cloud commands or scientific work. This review does not establish scheduling bounds, physical worker deallocation, billing, runtime image delivery or a scientific result. The prior V2 review and failing qualification remain historical evidence. Fresh-owner Job API default reconciliation and fresh native stop-clock correction are separate patches and are not supplied by this V3 patch.
