#!/bin/bash
# f4_fp.rs at HEAD does not compile (duplicate definitions from merge #4538); the pilot needs it only
# for types. Materialise the file and its submodules as they were at 94479f7e3e (the commit the first
# speed pass was built on) into src/cryptanalysis/ (git-ignored). Nothing of the HKY engines is pinned.
set -eu
D="$(cd "$(dirname "$0")" && pwd)/src/cryptanalysis"
cd "$(git -C "$D" rev-parse --show-toplevel)"
C=94479f7e3e
mkdir -p "$D/f4_fp"
git show $C:src/cryptanalysis/f4_fp.rs > "$D/f4_fp.rs"
for f in $(git ls-tree --name-only $C:src/cryptanalysis/f4_fp); do
  git show $C:src/cryptanalysis/f4_fp/$f > "$D/f4_fp/$f"
done
