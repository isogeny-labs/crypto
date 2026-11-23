#!/bin/sh
set -eu
XEDNI_FORMAL_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
SOURCE_FORMAL_DIR=$(CDPATH= cd -- "$XEDNI_FORMAL_DIR/../../xedni_height_radius_20261120/formal" && pwd)
export XEDNI_FORMAL_DIR SOURCE_FORMAL_DIR
check_source() {
  actual=$(shasum -a 256 "$SOURCE_FORMAL_DIR/$1" | cut -d ' ' -f 1)
  test "$actual" = "$2"
}
check_source UniformQuotient.lean 761c5cd589ee9bdb240a4444c7984a633b7b7f0313e02cb4aff59ce33c2bf14b
check_source FreeTorsionQuotient.lean 903a85887e20213a4f52b957f921f91483e87d7145e203196df180b9cbf74c8c
check_source ConcreteRelation.lean 868616ec5ff3e85c0e762fb1c2c9d2bed7d3816d7c0bdb58c459d41336482c81
check_source HeightRadius.lean 085cd582ac0c83d56840f9ba301cf6401997269f6e14f8cd2ada2cb5c4d5d0b1
mkdir -p "$XEDNI_FORMAL_DIR/.build"
cd /Volumes/SSD990/cryptanalysis/experiments/xedni-public-first-large-holdout-20261115/formal
lake env sh -c '
  set -eu
  export LEAN_PATH="$XEDNI_FORMAL_DIR/.build:$LEAN_PATH"
  lean -R "$SOURCE_FORMAL_DIR" -o "$XEDNI_FORMAL_DIR/.build/UniformQuotient.olean" "$SOURCE_FORMAL_DIR/UniformQuotient.lean"
  lean -R "$SOURCE_FORMAL_DIR" -o "$XEDNI_FORMAL_DIR/.build/FreeTorsionQuotient.olean" "$SOURCE_FORMAL_DIR/FreeTorsionQuotient.lean"
  lean -R "$SOURCE_FORMAL_DIR" -o "$XEDNI_FORMAL_DIR/.build/ConcreteRelation.olean" "$SOURCE_FORMAL_DIR/ConcreteRelation.lean"
  lean -R "$SOURCE_FORMAL_DIR" -o "$XEDNI_FORMAL_DIR/.build/HeightRadius.olean" "$SOURCE_FORMAL_DIR/HeightRadius.lean"
  lean -R "$XEDNI_FORMAL_DIR" "$XEDNI_FORMAL_DIR/UniformConstant.lean"
'
