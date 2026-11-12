#!/bin/bash
# usage: dhsum.sh DHAT.json [N=6] -> totals, short-lived churn, top sites by count and bytes (site = first non-allocator frame)
F=$1; N=${2:-6}
jq -r --argjson n $N '
  .ftbl as $f |
  def site: [ .fs[] | $f[.] | sub("^0x[0-9A-F]+: ";"") | sub(" \\(in [^)]*\\)$";"") | sub(" \\([^)]*:[0-9]+\\)$";"")
     | select(test("^(malloc|calloc|realloc|free|__rust_|alloc::|<alloc::|hashbrown::|<hashbrown::|core::|<core::|std::sys|<std::collections|std::collections|<T as alloc|__libc|_int_|operator new)") | not) ][0] // "?"
     | gsub("::h[0-9a-f]{16}$";"") | .[0:110];
  [.pps[] | {s: site, tb, tbk, tl}] as $p |
  ($p | map(.tb) | add) as $TB | ($p | map(.tbk) | add) as $TBK |
  ($p | map(select(.tbk>0 and (.tl/.tbk) < 500))) as $s5 |
  ($p | map(select(.tbk>0 and (.tl/.tbk) < 100000))) as $s100k |
  "total_bytes=\($TB) total_blocks=\($TBK) short<500instr: blocks=\($s5|map(.tbk)|add // 0) bytes=\($s5|map(.tb)|add // 0); short<100k instr: blocks=\($s100k|map(.tbk)|add // 0) bytes=\($s100k|map(.tb)|add // 0)",
  ($p | group_by(.s) | map({s: .[0].s, tb: (map(.tb)|add), tbk: (map(.tbk)|add), tl: (map(.tl)|add)}) ) as $g |
  "-- by count", ($g | sort_by(-.tbk) | .[0:$n][] | "\(.tbk)\t\(.tb)\tavg_life=\((.tl/.tbk)|floor)\t\(.s)"),
  "-- by bytes", ($g | sort_by(-.tb) | .[0:$n][] | "\(.tbk)\t\(.tb)\tavg_life=\((.tl/.tbk)|floor)\t\(.s)")
' $F
