#!/bin/bash
# Build the CMS 5.11.21 library sources (pycryptosat sdist) + cms_driver.cpp into one binary.
# usage: build_driver.sh SRC_DIR(the unpacked pycryptosat-5.11.21) BUILD_DIR OUT_BIN
set -euo pipefail
SRC=$1; B=$2; OUT=$3; HERE=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$B"
CXXSRC="bva cardfinder ccnr_cms ccnr clauseallocator clausecleaner cnf completedetachreattacher cryptominisat_c datasync distillerbin distillerlitrem distillerlong distillerlongwithimpl frat gatefinder gaussian get_clause_query hyperengine intree lucky matrixfinder occsimplifier packedrow propengine reducedb sccfinder searcher searchstats sls solutionextender solverconf solver str_impl_w_impl subsumeimplicit subsumestrengthen varreplacer xorfinder oracle/oracle"
FLAGS="-O3 -DNDEBUG -std=c++17 -I$SRC/src -I$SRC/src/picosat -DTRACE= -DCMS_FULL_VERSION=\"5.11.21\""
gcc -O3 -DNDEBUG -DTRACE=ON -I$SRC/src/picosat -c $SRC/src/picosat/picosat.c -o $B/picosat.o
gcc -O3 -DNDEBUG -DTRACE=ON -I$SRC/src/picosat -c $SRC/src/picosat/version.c -o $B/picover.o
OBJS="$B/picosat.o $B/picover.o"
for f in $CXXSRC; do
  o=$B/$(echo $f | tr / _).o; OBJS="$OBJS $o"
  echo "nice -n 10 g++ $FLAGS -c $SRC/src/$f.cpp -o $o"
done > $B/cmds.txt
for x in "$SRC/python/src/GitSHA1.cpp $B/gitsha.o" "$HERE/cms_shim.cpp $B/shim.o" "$HERE/cms_driver.cpp $B/driver.o"; do
  set -- $x; echo "nice -n 10 g++ $FLAGS -I$SRC -I$SRC/python/src -c $1 -o $2" >> $B/cmds.txt
  OBJS="$OBJS $2"
done
xargs -P 2 -I{} bash -c '{}' < $B/cmds.txt
g++ -O3 -o "$OUT" $OBJS -lpthread
