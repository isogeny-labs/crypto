#!/usr/bin/env bash
# Print the command argument for ephemeral run --no-source -- /bin/bash -lc.
# This transfers only the checked-in preflight script, not GitHub credentials.
set -Eeuo pipefail

script_dir="$(cd "$(dirname "$0")" && pwd -P)"
script="$script_dir/linux_source_preflight.sh"
test -f "$script"
script_sha="$(shasum -a 256 "$script" | cut -d ' ' -f 1)"
payload="$(gzip -c "$script" | base64 | tr -d '\r\n')"

cat <<EOF
printf '%s' '$payload' | base64 -d | gzip -dc > /tmp/n83-d18-linux-source-preflight.sh
printf '%s  %s\n' '$script_sha' /tmp/n83-d18-linux-source-preflight.sh | sha256sum -c -
exec bash /tmp/n83-d18-linux-source-preflight.sh
EOF
