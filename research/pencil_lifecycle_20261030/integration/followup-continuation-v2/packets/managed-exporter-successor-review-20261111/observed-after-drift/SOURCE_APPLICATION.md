# Source application

Prerequisite: first apply the sealed predecessor patch:

`packets/pencil-managed-stream-integration-20261110/integration.patch`

Then apply this packet's `integration.patch` at repository root. It:

1. adds the existing native-price crate as a path dependency;
2. registers `retained_attempt_join_v2` beside, not over, v1;
3. adds the v2 library, adversarial control module, and postrun CLI.

The intended binary name is `ecbench_retained_attempt_export_v2`. It takes
exactly two arguments: a v2 request JSON path and a fresh absolute output path.
The output parent must be canonical, caller-owned, and mode 0700 or stricter;
the output is created once at mode 0400.

This packet was not applied to a source worktree. No build or test command is
authorized by this document.
