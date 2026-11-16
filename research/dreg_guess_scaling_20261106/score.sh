#!/usr/bin/env bash
# Tallies the committed records against PREREGISTRATION.md (+ Amendment 1) and
# PREREG_SUMMAND.md. Counting only; every verdict per branch is the record's own.
set -u
cd "$(dirname "$0")/runs"
deg() { grep -o '"closure_degree":[a-z0-9]*' "$1" | cut -d: -f2; }
ref4() { grep -q '"degree":4,"xl_refuted":false,"closure_refuted":true' "$1"; }
sat4() { grep -q '"degree":4,"xl_refuted":false,"closure_refuted":false,"saturated":true' "$1"; }
rec() { [ -s "pod/$1.jsonl" ] && echo "pod/$1.jsonl" || echo "$1.jsonl"; }
echo "== Gate A"
echo "A1 closure_degree=$(deg A1.jsonl) (committed dense: 4)"
echo "A2 closure_degree=$(deg A2.jsonl) closure_rank=$(grep -o '"closure_rank":[0-9]*' A2.jsonl | tail -1 | cut -d: -f2) (committed dense: null, 131588)"
echo "== Positive control P (refutes at 4?)"
p=0; for d in 1 4 5 7; do f=P.d$d.jsonl; ref4 "$f" && p=$((p+1)); echo "P.d$d $(deg $f) identity=$(grep -o '"identity_check":[a-z]*' $f | cut -d: -f2)"; done
echo "P refutes on $p of 4"
echo "== S5 (all 32 branches per draw)"
suff=0
for d in 1 4 5 7; do r=0; s=0; u=0; ids=0
  for b in $(seq 0 31); do f=$(rec S5.d$d.b$b)
    if [ ! -s "$f" ]; then u=$((u+1)); continue; fi
    grep -q '"identity_check":true' "$f" && ids=$((ids+1))
    if ref4 "$f"; then r=$((r+1)); elif sat4 "$f"; then s=$((s+1)); else u=$((u+1)); fi
  done
  v="does not suffice"; [ $r -eq 32 ] && { v="suffices"; suff=$((suff+1)); }
  echo "draw $d: refute $r, saturated-unrefuted $s, unmeasured $u, identity_check true $ids/32 -> $v"
done
echo "S5 suffices on $suff of 4 draws"
if [ $p -ge 3 ]; then
  if [ $suff -ge 3 ]; then echo "VERDICT: k* <= 5 at l = 9 (saturation reading)"
  elif [ $suff -le 1 ]; then echo "VERDICT: S5 does not suffice (consistent with K_marginal)"
  else echo "VERDICT: mixed"; fi
else echo "VERDICT: controls invalid"; fi
echo "== Whole-summand residual (PREREG_SUMMAND.md)"
for l in 7 8 9; do
  echo "l=$l: $(for f in $(ls W.l$l.*.jsonl pod/W.l$l.*.jsonl 2>/dev/null); do [ -s $f ] && deg $f; done | sort | uniq -c | tr '\n' ' ')"
  echo "  nonzero x1: $(for f in $(ls W.l$l.*.jsonl pod/W.l$l.*.jsonl 2>/dev/null | grep -v '\.h0\.'); do [ -s $f ] && deg $f; done | sort | uniq -c | tr '\n' ' ')"
done
