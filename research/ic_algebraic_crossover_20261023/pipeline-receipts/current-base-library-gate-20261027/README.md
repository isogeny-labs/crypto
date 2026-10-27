# First current-base library attempt: sparse fixtures omitted

Exact source974f2e9cd46b000021cb20e6a78c3f54063f8882 exits101 before test
execution:32include errors name27unique tracked fixture files omitted by the
custom sparse checkout. Every file was restored from its recorded Git blob;
`hydrated-exact-git-blobs.json` records exact equality and byte counts. The
73,935,397-byte total is dominated by inherited docs/curves/traits.json.
No test, assertion, budget or source code was changed. The raw failed log and
status remain unchanged. Attempt-2 is a separate bounded development gate.

The maintained sparse helper's add command requires one of its own applied
profiles. This checkout has custom exact-path patterns, so native
`git sparse-checkout add --stdin` preserved that scope while materializing the
27files; no broad harness reset or fabricated fixture was used. The shared
busy lock serialized the attempt with scientific children, and its timings
carry no scientific performance claim.
