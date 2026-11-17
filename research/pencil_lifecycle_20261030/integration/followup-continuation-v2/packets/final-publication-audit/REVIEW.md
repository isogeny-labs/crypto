# Read-only publication custody and adapter contract audit

Initial custody pass checked70 publication maps and1,716 included-entry hashes:1,715 matched. The one mismatch is the musl-compiler-qualification producer custody-map.json: its publication map claims the original map at the same path that now holds the new publication map. Original bytes remain recoverable at /var/tmp/pencil-musl-publication-view-20261031/custody-map.json, SHA25698b0c63a479046ab674d868248f639075a5bcdea768809d5d74c9a07c7332deb. Preserve those bytes under a distinct producer-map name and update the publication mapping explicitly. This is a custody-path collision, not evidence of changed compiler qualification results.

All17 hash-bound links in the latest report-branch-completion index matched. All19 registered canonical Lean modules bind to retained source snapshots. Constructor and Add11 are matched only after the documented removal of surplus terminal LF bytes, leaving one final newline; their original reviewed sources remain separately retained. Other modules match exact bytes. This audit checks source custody, not fresh Lean execution.

There are66 local compiled/render cache candidates, principally relation-adapter/target and .aux files. None appears in the1,957 narrowly enumerated tracked paths for the two requested publication scopes, and none is claimed included by a custody map. No canonical .lake/target or compiled file was found at its top level. This supports exclusion from the inspected intended publication set, not a claim that local build caches do not exist. Retained .tex/.log files are render sources/receipts; they were not classified as executable caches. Future staging must preserve this distinction.

## Boxed request static contract

Against source checkpoint da1464aea27534f6daf0bc3de0cdc687030b66d2, diagnostic_file_adapter.rs differs in exactly one line: actual: Actual becomes actual: Box<Actual>. The actual JSON member, stage discriminator, names, required fields and deny_unknown_fields annotations remain unchanged. The resolved local serde_core1.0.229 implementation forwards Box<T> deserialization to T::deserialize and then Box::new, adding no JSON wrapper. Existing matching and field access dereference the box; loader limits, hashes, custody distinction, owner authentication and scientific-admission results remain unchanged in source.

Boxing changes memory representation and introduces allocation for that variant; this review does not infer byte allocation counts or performance. No native test/build was run here. Root's live boxed-request-software-checks tree was excluded throughout, and its eventual results require separate binding. Static JSON contract preservation is supported by the exact diff plus dependency implementation, not asserted as a passing native test.

## Audit boundary

Only private audit files were written. The audit used narrow git show/diff and ls-files, no broad Git status, index mutation, build, cloud operation or scientific execution. custody-checks.json retains the original mismatch; canonical-bindings.json, aggregate-links.json, intended-cache-check.json and source snapshots provide exact evidence. Any subsequent correction should be recorded additively, not erase this initial result.

## Additive post-repair readback

Root restored producer-custody-map.json with the exact expected98b0c63a... digest and redirected the original producer-map entry to that distinct path. All included entries in the affected map now match. publication-custody-correction.json records the collision and correction; its exact bytes are retained here. The original1,715/1,716 observation remains in custody-checks.json. postrepair.json records the repaired map separately; other initially matching entries were not rehashed. No unresolved custody mismatch remains in the inspected set after this scoped repair readback.
