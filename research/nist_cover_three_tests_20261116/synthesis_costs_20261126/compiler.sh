#!/bin/bash
set -euo pipefail
output=
previous=
for arg in "$@"; do
    if test "$previous" = -o; then output=$arg; fi
    previous=$arg
done
test -n "$output"
"$P192_CAPTURE/instrumentation/meter.sh" build "$P192_COMPILER" "$@"
binary=$(mktemp "$P192_CAPTURE/bin/native.XXXXXX")
cp "$output" "$binary"
chmod 0555 "$binary"
sha256sum "$binary" > "$binary.sha256"
printf '%s\n' '#!/bin/bash' > "$output"
printf 'exec %q native %q "$@"\n' "$P192_CAPTURE/instrumentation/meter.sh" "$binary" >> "$output"
chmod 0555 "$output"
