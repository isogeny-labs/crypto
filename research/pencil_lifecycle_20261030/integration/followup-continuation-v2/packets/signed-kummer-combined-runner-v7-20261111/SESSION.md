# Source session

T-2170, fence 1, 2026-11-11. Preserved the v6 source, terminal compile failure,
all eight overlays, and all 50 driver source bindings. The donor cache inventory
contained 2,165 regular files: 2,082 mode 0600, one mode 0644, 78 mode 0700,
and four mode 0755; 82 were executable and none had special permission bits.
The v6 copy retained bytes but created the first cached build script at 0600,
while the donor was 0700. V7 assigns fresh workspace/unit identities and adds a
compact, byte-bound mode inventory plus exact `fchmod` and post-copy checks.
No compiler, test, resolver, or payload was run while preparing v7.
