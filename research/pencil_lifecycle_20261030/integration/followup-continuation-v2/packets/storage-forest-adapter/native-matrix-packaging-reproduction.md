# Reconstruct omitted duplicate matrix-review inputs

Packaging-only inspection, 2026-11-01. The frozen review is unchanged. The aggregate publisher omits duplicate review input/fixture trees while retaining their custody entries, original byte hashes, verifier source, fixed input manifest and receipts. Its current root `custody-map.json` indexes JSON parts of up to 400 entries, each with SHA256 and entry count. This supersedes the intermediate gzip-map proposal. Some included payloads still use their individually declared `gzip` or `base64` encoding; decode those according to their custody entry.

Authoritative producer input source: commit `80b98db381`, path `research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/union-division-runtime/matrix-bindings/`. Its parent `union-division-runtime/custody-map.json` supplies exact original-to-stored filenames and decoding metadata. The native verifier's `native-matrix/published-input-binding.json` fixes all 133 reconstructed input files; its SHA256 must be `8f6c33e483a6ceffe8009004470afd7699c4ccd330327804d496e7ea95966772`.

The following commands are reconstruction instructions, **not rerun during this packaging follow-up**. Earlier native replay/control receipts remain the execution evidence. Run in a fresh private directory, with `REPO` set to the repository and `AGGREGATE_COMMIT` set to the final published aggregate SHA.

```bash
set -euo pipefail
REPRO=$(mktemp -d /var/tmp/matrix-reproduction.XXXXXX)
PACKETS=research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets
mkdir -p "$REPRO/old" "$REPRO/new" "$REPRO/subject" "$REPRO/decoded"
git -C "$REPO" archive 80b98db381 "$PACKETS/union-division-runtime" | tar -x -C "$REPRO/old"
git -C "$REPO" archive "$AGGREGATE_COMMIT" "$PACKETS/storage-forest-adapter" | tar -x -C "$REPRO/new"
OLD="$REPRO/old/$PACKETS/union-division-runtime"
NEW="$REPRO/new/$PACKETS/storage-forest-adapter"
# Authenticate each indexed custody part before combining its entries.
while IFS=$'\t' read -r path sha entries; do
  printf '%s  %s\n' "$sha" "$NEW/$path" | sha256sum -c -
  test "$(jq '.files|length' "$NEW/$path")" -eq "$entries"
done < <(jq -r '.parts[]|[.path,.sha256,.entries]|@tsv' "$NEW/custody-map.json")
jq -s '{files:[.[].files[]]}' $(jq -r --arg root "$NEW" '.parts[].path|$root+"/"+.' "$NEW/custody-map.json") > "$REPRO/new-custody.json"
test "$(jq '.files|length' "$REPRO/new-custody.json")" -eq "$(jq '.entry_count' "$NEW/custody-map.json")"

# Decode an included subtree and authenticate BOTH stored and original bytes.
restore_subtree() {
  local archive=$1 map=$2 prefix=$3 destination=$4
  while IFS=$'\t' read -r original included encoding stored_sha original_sha; do
    local rel=${original#"$prefix"} output="$destination/${original#"$prefix"}"
    mkdir -p "$(dirname "$output")"
    printf '%s  %s\n' "$stored_sha" "$archive/$included" | sha256sum -c -
    case "$encoding" in
      identity) cp "$archive/$included" "$output" ;;
      gzip) gzip -dc "$archive/$included" > "$output" ;;
      base64) base64 -d "$archive/$included" > "$output" ;;
      *) printf 'Unsupported encoding: %s\n' "$encoding" >&2; return 1 ;;
    esac
    printf '%s  %s\n' "$original_sha" "$output" | sha256sum -c -
  done < <(jq -r --arg prefix "$prefix" '.files[]|select(.original_path|startswith($prefix))|select(.included_path!=null)|[.original_path,.included_path,(.encoding//"identity"),(.stored_sha256//.sha256),.sha256]|@tsv' "$map")
}
restore_subtree "$OLD" "$OLD/custody-map.json" matrix-bindings/ "$REPRO/subject"
restore_subtree "$NEW" "$REPRO/new-custody.json" native-matrix/ "$REPRO/decoded/native-matrix"
restore_subtree "$NEW" "$REPRO/new-custody.json" native-matrix-review/ "$REPRO/decoded/native-matrix-review"
BINDING="$REPRO/decoded/native-matrix/published-input-binding.json"
printf '8f6c33e483a6ceffe8009004470afd7699c4ccd330327804d496e7ea95966772  %s\n' "$BINDING" | sha256sum -c -
while IFS=$'\t' read -r path sha; do
  printf '%s  %s\n' "$sha" "$REPRO/subject/$path" | sha256sum -c -
done < <(jq -r '.files[]|[.path,.sha256]|@tsv' "$BINDING")

# Native replay; bypass check.sh's original absolute paths.
CHECKER_ROOT="$REPRO/decoded/native-matrix"
cargo build --offline --locked --release --manifest-path "$CHECKER_ROOT/Cargo.toml" --target-dir "$REPRO/build"
CHECKER="$REPRO/build/release/pencil-readonly-accounting"
"$CHECKER" audit "$REPRO/subject" "$BINDING" > "$REPRO/audit.json"
"$CHECKER" controls "$REPRO/subject" "$BINDING" > "$REPRO/controls.json"
jq -e '.accepted and .bound_files==133 and .actions==105 and .source_pins==12' "$REPRO/audit.json"
jq -e '.accepted and .case_count==10' "$REPRO/controls.json"

# Recreate the two actual CLI corruption fixtures in new copies.
cp -a "$REPRO/subject" "$REPRO/changed-source"
printf 'unrelated source\n' > "$REPRO/changed-source/walk-source.rs"
printf 'altered binding\n' > "$REPRO/altered-binding.json"
set +e
"$CHECKER" audit "$REPRO/changed-source" "$BINDING" > "$REPRO/changed-source-result.json"
SOURCE_EXIT=$?
"$CHECKER" audit "$REPRO/subject" "$REPRO/altered-binding.json" > "$REPRO/changed-binding-result.json"
BINDING_EXIT=$?
set -e
test "$SOURCE_EXIT" -eq 1
test "$BINDING_EXIT" -eq 1
jq -e '.accepted==false' "$REPRO/changed-source-result.json"
jq -e '.accepted==false' "$REPRO/changed-binding-result.json"
# Independent reviewer boundary tests are Rust and need no curve/point replay.
cargo test --offline --locked --release --manifest-path "$REPRO/decoded/native-matrix-review/Cargo.toml" --target-dir "$REPRO/reviewer-build" --bin reviewer_boundaries
```

For a reconstructed review-tree layout, its `subject/` is the exact 133-file producer subject above. Its `negative/source/subject/` is a copy with only `walk-source.rs` replaced by `unrelated source\n`; `negative/altered-binding.json` contains `altered binding\n`. The earlier review's nested duplicate `subject/negative/.../inputs/` directories are already represented inside the authoritative producer snapshot and its binding. Do not substitute newly generated scientific inputs. The verifier's ten in-memory mutations are regenerated by its `controls` mode.

Important limits: a missing archived payload or unresolved artifact-store pointer must stop reconstruction; hydrate through the repository's authorized artifact mechanism, then verify both custody hashes before continuing. A SHA256 match supplies exact bytes, not runtime provenance. `native-matrix/check.sh` is retained but hardcodes original `/var/tmp` roots; the explicit relocated commands above avoid modifying frozen scripts. This follow-up did not execute these reconstruction commands, rebuild, rerun controls, invoke Python checking, perform scientific computation or edit the repository.
