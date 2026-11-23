# Private runtime source integration

The private assembly supervisor and corrected qualifier have matching source interfaces and a non-author source review. The reviewer replayed 21 supervisor controls and 14 qualifier controls, all passing. Root checked all eight reviewed source hashes. These are mocked/static and temporary-filesystem checks; no service, mount, daemon, image build or container was started.

The construction uses separate 600-second assembly and 300-second qualification services. Proposed common limits are 2 CPU quota, 2 GiB RAM, no swap, 256 tasks and a 4 GiB filesystem. Each phase has its own admission and stop reserve. The private daemon loads the already verified base archive; the shared daemon is unused. Retained image storage transfers only after exact disk custody and predecessor termination checks. Both drafts remain unadmitted.

A held no-fork gate waits for an exact nonce while the host checks container PID lifetime and cgroup placement. The source review corrected directory/token modes masked by 0077, private disk access, source-bundle closure, phase expiry, readiness and transient-unit lifecycle. Root separately checked the exact gate source with GCC 11.5.0, -Wall -Wextra -Werror -fsyntax-only: exit 0. Native held-gate and Docker copy behavior remain separate controls.

The prior qualifier source/report is preserved. Its own-scope review did not cover the later-discovered service umask interaction; qualifier-v2 and its real temporary-file mode test supersede that interface claim. All failed or intermediate source checks remain in producer packets.

Next: build and check the exact held-gate candidate, complete prospective runtime admission and actual bounded service/resource readback, then image-bound qualification. The full matched cohort and all mathematical follow-up obligations remain unchanged. No scientific result or cost ratio is added by this source-integration packet.
