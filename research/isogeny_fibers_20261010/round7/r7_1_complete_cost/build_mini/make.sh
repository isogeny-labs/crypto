#!/bin/bash
# Usage: build_mini/make.sh <repo-root> <work-dir>
# Creates <work-dir> (a crate that includes the repository's isogeny_eval and
# isogeny_walk sources by #[path]) and builds the R7.1 and R6.4 examples:
#   cd <work-dir> && cargo build --release --offline --examples
set -e
REPO=$(cd "$1" && pwd); W=$2
mkdir -p "$W/src" "$W/examples"
cp "$REPO/research/isogeny_fibers_20261010/round7/r7_1_complete_cost/build_mini/Cargo.toml" "$W/Cargo.toml"
# The library's modpoly.rs without its test module (which names a module of
# the full crate).
awk '/^#\[cfg\(test\)\]$/ {exit} {print}' "$REPO/src/cryptanalysis/isogeny_walk/modpoly.rs" > "$W/modpoly_notests.rs"
cat > "$W/src/lib.rs" <<LIB
pub mod cryptanalysis {
    #[path = "$REPO/src/cryptanalysis/isogeny_eval/mod.rs"]
    pub mod isogeny_eval;
    pub mod isogeny_walk {
        #[path = "$REPO/src/cryptanalysis/isogeny_walk/field.rs"]
        pub mod field;
        #[path = "$REPO/src/cryptanalysis/isogeny_walk/curve.rs"]
        pub mod curve;
        #[path = "$REPO/src/cryptanalysis/isogeny_walk/poly.rs"]
        pub mod poly;
        #[path = "$W/modpoly_notests.rs"]
        pub mod modpoly;
        #[path = "$REPO/src/cryptanalysis/isogeny_walk/kernel.rs"]
        pub mod kernel;
    }
}
LIB
ln -sf "$REPO/examples/r7_1_complete_cost.rs" "$W/examples/r7_1_complete_cost.rs"
ln -sf "$REPO/examples/r6_4_vertical_margin_2.rs" "$W/examples/r6_4_vertical_margin_2.rs"
echo "now: cd $W && cargo build --release --offline --examples   (and: cargo test --release --offline isogeny_eval)"
