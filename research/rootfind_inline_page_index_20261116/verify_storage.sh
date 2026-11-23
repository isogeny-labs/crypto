#!/bin/sh
# Independent integer-arithmetic check of the emitted backing request fields.
set -eu
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
jq -e '
  .schema == "rootfind-inline-page-results/v1" and
  .status == "passed" and
  ([.cells[].field_degree] == [13,19,23]) and
  all(.cells[];
    .inline_requested_backing_bytes ==
      (4 * .inline_directory_words + 64 * .inline_spill_capacity) and
    .inline_singleton_pages + .inline_spill_pages == .used_pages and
    .inline_spill_pages <= (.first_witness_keys - .used_pages) and
    .inline_spill_pages <= .inline_spill_capacity and
    .inline_requested_backing_bytes < .paged_requested_backing_bytes
  )
' "$here/RESULTS.json"
