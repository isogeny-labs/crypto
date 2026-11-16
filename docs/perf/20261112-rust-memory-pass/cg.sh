#!/bin/bash
# usage: cg.sh LABEL NAME -> callgrind of the nodump copy of LABEL on NAME.in; prints total Ir and malloc-family share
S=${S:-$PWD}; L=$1; n=$2; export RAYON_NUM_THREADS=1
cd $S/m-prof; rm -f cg.$L.$n*
valgrind --tool=callgrind --combine-dumps=yes --callgrind-out-file=cg.$L.$n $S/m-$L-nodump exec < $n.in > cg.$L.$n.stdout 2> cg.$L.$n.err
tot=$(grep -h '^summary:\|^totals:' cg.$L.$n* | awk '{s+=$2} END{print s}')
bash $(dirname "$0")/../20261112-rust-deep-audit/cg-top.sh cg.$L.$n 40 > cg.$L.$n.top 2>/dev/null
al=$(awk '$1=="E" && $3 ~ /^(_int_malloc|_int_free|malloc|free|realloc|cfree|_int_realloc|malloc_consolidate|unlink_chunk|__memset|__memcpy|__memmove|_int_free_merge_chunk|_int_free_create_chunk|_int_free_maybe_consolidate)/ {s+=$2} END{print s+0}' cg.$L.$n.agg)
tot=$(awk '$1=="T"{print $2}' cg.$L.$n.agg)
echo "$L $n total_Ir=$tot alloc_family_excl_Ir=$al"
