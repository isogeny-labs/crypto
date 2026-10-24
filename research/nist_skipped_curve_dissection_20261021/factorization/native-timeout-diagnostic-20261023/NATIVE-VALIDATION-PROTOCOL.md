# Frozen full-tool diagnostic after extracted-statement controls

Preserve the R05 archive and original executable unchanged. Copy the pinned
native source/build tree to a separate directory, fix only the two timeout
printf conversions for uint32_t arguments, retain the patch, source hashes
and build output, and rebuild with the original AVX2/BMI2/libecm options.
This is a separate validation allocation, not an extension of R05's budget.

At most two fresh, independent native sieving diagnostics: original and
patched binaries, exact R05 cofactor and selected native polynomial,
two threads, seed 20261022, native timeout one second, fixed algebraic
Q range 2935000 through 2935500, sieving phase only. Each command gets a
30-second outer bound, five-second kill grace, 4 GiB address-space limit,
64 MiB per output file, and no core dump. Do not import or alter R05's
relation files, range ledger or checkpoint. Retain every command and output.

Pass criterion: the original reaches the defective timeout path and fails
with exit 139 while the patched native tool logs its integer timeout value
and exits without that crash. If the path is not reached within the bound,
record the control as inconclusive; do not infer that the patch fixes the
full pipeline. Neither diagnostic establishes completion, relation rank,
a factor, or a production backtrace. Stop after these two diagnostics.
