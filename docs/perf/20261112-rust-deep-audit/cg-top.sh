#!/bin/bash
# usage: cg-top.sh CGFILE [N] -> markdown: exclusive top N, inclusive top N below the process frames
N=${2:-15}; d=$(dirname $0)
awk -f $d/cgagg.awk "$1" > "$1.agg"
T=$(awk '$1=="T"{print $2}' "$1.agg")
fmt='{n=$3; for(i=4;i<=NF;i++) n=n" "$i; gsub(/crypto_lib::cryptanalysis::|ic_boundary::|ic_framework::|core::iter::adapters::/,"",n); if(length(n)>110) n=substr(n,1,110)"…"; printf "| %\047d | %.1f | `%s` |\n",$2,100*$2/T,n}'
echo "Total Ir (all parts) $T."; echo
echo "| exclusive Ir | % | function |"; echo "|---:|---:|---|"
awk '$1=="E"' "$1.agg" | sort -k2 -nr | head -$N | awk -v T=$T "$fmt"
echo; echo "| inclusive Ir | % | function |"; echo "|---:|---:|---|"
awk '$1=="I"' "$1.agg" | sort -k2 -nr | awk -v T=$T '$2<=T*1.0001' | grep -vE ' (0x[0-9a-f]+|\(below main\)|__libc_start_main.*|main|_dl_.*|_start)$| std::| ecbench::main$| ecbench::run$|record::child_main|solve_measured$|solve_inner$' | head -$N | awk -v T=$T "$fmt"
