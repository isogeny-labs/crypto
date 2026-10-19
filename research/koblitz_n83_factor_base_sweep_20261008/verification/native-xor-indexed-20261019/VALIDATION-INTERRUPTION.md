# Validation runtime interruption

The first full serial release-library validation started before the runtime
restart observed at2026-10-19T16:32Z. Its log stops during
koblitz_groebner::tests::inherited_and_f5_engines_walk_the_same_tree_as_matrix_f4;
there is no terminal test result and its process/session no longer exists.
Treat it as INTERRUPTED, never passed. Preserve its exact log in the raw
bundle as full-lib-tests.interrupted.log. The solver-focused27-test run,
example tests19+2 and native runner19 tests completed separately before the
restart; they do not replace the required full suite.

A fresh full serial release suite starts on the restored32-vCPU host using
identical library source. Docker25.0.16 and the frozen immutable image are
available. No matched timing or natural continuation arm ran before this
restart. Freeze their specs and binary/host records again before execution.
This validation restart does not replace any scientific observation.

For root CPU reservations, cards now explicitly declare root_executor:true;
the native prompt/check/run validate sudo's exact digest as root and capture
actual stdout verbatim with noclobber shell redirection. No sudo permissions
or setuid executable copies are changed. Unexecuted earlier draft cards stay
labelled drafts and are never relayed.
