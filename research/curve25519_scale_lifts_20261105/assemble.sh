#!/bin/bash
cd "$(dirname "$0")"
analysis/make_tables.sh >/dev/null && analysis/make_svg.sh
{ cat README.head.md; cat README.mid.md; cat README.perrel.md; cat README.tail.md; } > README.md
