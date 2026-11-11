#!/bin/bash
# Runs the job image's own gawk (Amazon Linux 2023, GNU Awk 5.1.0). LD_LIBRARY_PATH is set only for this one process, so no
# other tool picks up the image's libraries. IMG_ROOT is the unpacked image base layer.
IMG_ROOT=${IMG_ROOT:-/tmp/claude-0/-home-user/354dbe26-6a93-506a-a404-2fc6ef137fdc/scratchpad/imgbase/x}
LD_LIBRARY_PATH=$IMG_ROOT/usr/lib64 exec "$IMG_ROOT/usr/bin/gawk" "$@"
