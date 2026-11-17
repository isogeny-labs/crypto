#!/bin/bash
# usage: nodump.sh BIN OUT -> copy of BIN with ic_measurement::dump patched to `ret` (callgrind profiling only)
B=$1; O=$2; cp $B $O
a=$(nm $B | grep -E 'ic_measurement4dump' | awk '{print $1}' | head -1)
seg=$(readelf -lW $B | awk '$1=="LOAD" && / R E / {print $2, $3}')
off=$(( 16#$a - $(printf '%d' $(echo $seg | awk '{print $2}')) + $(printf '%d' $(echo $seg | awk '{print $1}')) ))
printf '\xc3' | dd of=$O bs=1 seek=$off conv=notrunc 2>/dev/null
objdump -d --no-show-raw-insn --start-address=0x$a --stop-address=$((16#$a+1)) $O | tail -1
