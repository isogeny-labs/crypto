#!/bin/sh
set -eu
XEDNI_FORMAL_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
HEIGHT_DIR=$(CDPATH= cd -- "$XEDNI_FORMAL_DIR/../../xedni_height_radius_20261120/formal" && pwd)
CONSTANT_DIR=$(CDPATH= cd -- "$XEDNI_FORMAL_DIR/../../xedni_uniform_constant_20261120/formal" && pwd)
export XEDNI_FORMAL_DIR HEIGHT_DIR CONSTANT_DIR
check_source() {
  actual=$(shasum -a 256 "$1" | cut -d ' ' -f 1)
  test "$actual" = "$2"
}
check_source "$XEDNI_FORMAL_DIR/XedniObstruction.lean" 94d7f7af1f5644144bda652898939d10a7a62cfc7cc38b552d61f2c7ed28c882
check_source "$XEDNI_FORMAL_DIR/HeightPigeonhole.lean" 2ba68aba33d9e1219cfe081e2ee6e4cec07c55f27a0659dc588fdfe4788908eb
check_source "$XEDNI_FORMAL_DIR/XedniDependentLift.lean" b1d09d998b48f5fcb998a29ab7fae0c039e1eda7ca3b38914e65525c928162d3
check_source "$HEIGHT_DIR/UniformQuotient.lean" 761c5cd589ee9bdb240a4444c7984a633b7b7f0313e02cb4aff59ce33c2bf14b
check_source "$HEIGHT_DIR/FreeTorsionQuotient.lean" 903a85887e20213a4f52b957f921f91483e87d7145e203196df180b9cbf74c8c
check_source "$HEIGHT_DIR/ConcreteRelation.lean" 868616ec5ff3e85c0e762fb1c2c9d2bed7d3816d7c0bdb58c459d41336482c81
check_source "$HEIGHT_DIR/HeightRadius.lean" 085cd582ac0c83d56840f9ba301cf6401997269f6e14f8cd2ada2cb5c4d5d0b1
check_source "$CONSTANT_DIR/UniformConstant.lean" 43788eee9242992c0332316ed091da1d630c30d436fe75d0e34783244d883ce9
mkdir -p "$XEDNI_FORMAL_DIR/.build"
cd /Volumes/SSD990/cryptanalysis/experiments/xedni-public-first-large-holdout-20261115/formal
lake env sh -c '
  set -eu
  export LEAN_PATH="$XEDNI_FORMAL_DIR/.build:$LEAN_PATH"
  lean -R "$XEDNI_FORMAL_DIR" -o "$XEDNI_FORMAL_DIR/.build/XedniObstruction.olean" "$XEDNI_FORMAL_DIR/XedniObstruction.lean"
  lean -R "$XEDNI_FORMAL_DIR" -o "$XEDNI_FORMAL_DIR/.build/HeightPigeonhole.olean" "$XEDNI_FORMAL_DIR/HeightPigeonhole.lean"
  lean -R "$XEDNI_FORMAL_DIR" -o "$XEDNI_FORMAL_DIR/.build/XedniDependentLift.olean" "$XEDNI_FORMAL_DIR/XedniDependentLift.lean"
  lean -R "$HEIGHT_DIR" -o "$XEDNI_FORMAL_DIR/.build/UniformQuotient.olean" "$HEIGHT_DIR/UniformQuotient.lean"
  lean -R "$HEIGHT_DIR" -o "$XEDNI_FORMAL_DIR/.build/FreeTorsionQuotient.olean" "$HEIGHT_DIR/FreeTorsionQuotient.lean"
  lean -R "$HEIGHT_DIR" -o "$XEDNI_FORMAL_DIR/.build/ConcreteRelation.olean" "$HEIGHT_DIR/ConcreteRelation.lean"
  lean -R "$HEIGHT_DIR" -o "$XEDNI_FORMAL_DIR/.build/HeightRadius.olean" "$HEIGHT_DIR/HeightRadius.lean"
  lean -R "$CONSTANT_DIR" -o "$XEDNI_FORMAL_DIR/.build/UniformConstant.olean" "$CONSTANT_DIR/UniformConstant.lean"
  lean -R "$XEDNI_FORMAL_DIR" "$XEDNI_FORMAL_DIR/AdaptiveHeightObstruction.lean"
'
