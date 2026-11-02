# Retained failed CI and interrupted local selftest

The sanitized CI log records a disk-exhaustion failure before Lean's
positive fixtures could be verified. The first local exact-subtree check
was interrupted without a terminal receipt. Its processes and tool handle
were reconciled as absent before a separate, explicitly single-thread
verification was launched. See `interruption.txt` and the retained Empty
axiom-audit log for the observed thread-creation error. Positive fixture
abort logs do not locate their precise cause. This directory is not a
successful or completed test run. The complete corrected local validation
is `../lean-selftest-sparse-v2/`.
