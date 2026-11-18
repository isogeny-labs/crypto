#!/bin/sh
set -eu
# Pin the reused arithmetic; never read a mutable worktree source.
git show 2b8dd06a8cecf6e3707bd5f131c75a9977fa253a:src/cryptanalysis/ecbench/plane_quartic.rs |
awk '
/^use std::collections::HashMap/ { print; next }
/^pub const QUARTIC_MIN_P/ { copying=1 }
/    \/\/\/ `\[k\]x` by double-and-add/ { copying=0; print "}" }
/^pub fn rational_points/ { copying=1 }
/^\/\/\/ `\(a₁, a₂\)`/ { copying=0 }
copying { print }
' | sed '$d' > research/hec_transport13_20261115/reused.rs
