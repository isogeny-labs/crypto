#!/bin/sh
set -eu

if [ "$#" -ne 1 ] || [ -e "$1" ]; then
    echo 'usage: prepare-source.sh NEW_SNAPSHOT_DIRECTORY' >&2
    exit 2
fi

study=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(git rev-parse --show-toplevel)
revision=$(git rev-parse HEAD)
snapshot=$1
mkdir -p "$snapshot"

for file in \
    examples/koblitz_n83_factor_base_export.rs \
    research/koblitz_n83_factor_base_sweep_20261008/primary_adapter.rs \
    src/cryptanalysis/binary_semaev_chain_sat.rs \
    src/cryptanalysis/sat.rs \
    src/cryptanalysis/koblitz_index_calculus.rs \
    research/koblitz_n83_factor_base_sweep_20261008/size-frontier-v2.json
do
    if ! git diff --quiet "$revision" -- "$file"; then
        echo "compiled source differs from $revision: $file" >&2
        exit 1
    fi
    mkdir -p "$snapshot/$(dirname -- "$file")"
    git show "$revision:$file" > "$snapshot/$file"
done

jq -n --arg revision "$revision" '{schema:"n83.chain-s3-source-attestation/v1",source_commit:$revision,status_clean:true}' > "$snapshot/attestation.json"
(
    cd "$root"
    sha256sum \
        examples/koblitz_n83_factor_base_export.rs \
        research/koblitz_n83_factor_base_sweep_20261008/primary_adapter.rs \
        src/cryptanalysis/binary_semaev_chain_sat.rs \
        src/cryptanalysis/sat.rs \
        src/cryptanalysis/koblitz_index_calculus.rs \
        research/koblitz_n83_factor_base_sweep_20261008/size-frontier-v2.json
) > "$study/source-files.sha256"
(
    cd "$root"
    sha256sum -c "$study/source-files.sha256"
) > "$study/source-check.log"
(
    cd "$snapshot"
    sha256sum -c "$study/source-files.sha256"
) > "$study/source-snapshot-check.log"

jq -n --arg revision "$revision" --arg snapshot "$snapshot" \
    '{schema:"n83.selector-only-source/v1",source_commit:$revision,snapshot:$snapshot,source_files:6}' \
    > "$study/source-attestation.json"
