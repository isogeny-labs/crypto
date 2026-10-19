# Execution log

The first two ephemeral attempts, `20261019-011623-boolean-native-compariso-9fc034`
and `20261019-012707-boolean-native-optimizat-abf14c`, terminated during source
preparation. The working-tree patch included existing PR changes outside the
sparse checkout, so `git apply` failed before compilation or timing. Both
instances terminated automatically. Their job/status/log receipts remain in
`preparation-failures/`. Neither supplies native measurements. Corrected runs
use a complete source checkout. No isolation or scientific gate is changed.

Build/test receipts and accepted/rejected native samples are added only after
they actually finish. The frozen baseline files are copied verbatim from
`1340a6176`; the new study does not modify the prior instruction archive.
