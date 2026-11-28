#!/bin/sh
set -eu
XEDNI_FORMAL_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
export XEDNI_FORMAL_DIR
mkdir -p "$XEDNI_FORMAL_DIR/.build"
cd /Volumes/SSD990/cryptanalysis/experiments/xedni-public-first-large-holdout-20261115/formal
lake env sh -c '
  set -eu
  export LEAN_PATH="$XEDNI_FORMAL_DIR/.build:$LEAN_PATH"
  lean -R "$XEDNI_FORMAL_DIR" -o "$XEDNI_FORMAL_DIR/.build/UniformQuotient.olean" "$XEDNI_FORMAL_DIR/UniformQuotient.lean"
  lean -R "$XEDNI_FORMAL_DIR" -o "$XEDNI_FORMAL_DIR/.build/FreeTorsionQuotient.olean" "$XEDNI_FORMAL_DIR/FreeTorsionQuotient.lean"
  lean -R "$XEDNI_FORMAL_DIR" -o "$XEDNI_FORMAL_DIR/.build/ConcreteRelation.olean" "$XEDNI_FORMAL_DIR/ConcreteRelation.lean"
  lean -R "$XEDNI_FORMAL_DIR" "$XEDNI_FORMAL_DIR/HeightRadius.lean"
'
