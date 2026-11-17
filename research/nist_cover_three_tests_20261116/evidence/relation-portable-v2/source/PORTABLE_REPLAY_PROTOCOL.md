# Portable replay comparison boundary

This qualifier compares existing native outputs; it performs no curve
arithmetic and does not run another relation/performance experiment.

Check a portable SHA256 manifest containing repository-relative paths for
the retained integrated sources, inputs and raw results. Require every
manifest subject to stay inside the specified checkout. Local binaries are
not in Git and are not claimed to be remotely custodied. The historical
pre-call manifests remain unchanged, including their original absolute paths.

Compare all four native result files from an expected integrated directory
and an observed directory. First use the separately retained standalone
relation and transfer outputs against their integrated equivalents. Preserve
every JSON type, object key, array order and scalar value. Do not equate an
empty array, object and string. Ignore only nonnegative numeric wall_us and
cpu_us fields in the root checker object or root stages array. Record the
two OpenSSL version strings and their equality separately; version equality
is not a mathematical result. All other fields must agree exactly.

Controls must reject: a changed source-image coordinate; a changed rank;
an omitted stage name; an empty array replaced by an empty object; a changed
manifest digest; and a manifest path escaping the checkout. A timing-only
change must remain acceptable and be counted as an excluded field.

Limits:1MiB per JSON file,128 nesting levels,60 seconds and1GiB for the
native qualifier. Copy and hash its source, binary, protocol, input manifest
and raw input files before calling. Keep every failed build/call in a fresh
directory. Output is producer-unverified comparison metadata; it is neither
a signed replay certificate nor proof that the recorded processes executed.

Sources: existing native result schemas and immutable pre-call manifests;
PCC verify_claim, docs/SABOTAGE_RESISTANCE.md sections4/5, and the IsoLab
access/cost guides read in this continuation. No novelty claim. The missing
external verifier and budget/capacity readbacks remain full-goal obligations.
