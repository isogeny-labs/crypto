#!/usr/bin/env bash
# README.md = README.tmpl.md with the @@file@@ markers replaced by the generated tables (awk).
set -eu
D=$(cd "$(dirname "$0")" && pwd)
awk -v d="$D" '/^@@.*@@$/{f=substr($0,3,length($0)-4); while((getline l < (d "/" f))>0) print l; close(d "/" f); next}1' "$D/README.tmpl.md" > "$D/../README.md"
