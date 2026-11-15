#!/usr/bin/env bash
# P3 of backlog/XP-20261029-packed-filter/APPROVAL.md (read-only)
set -u
EB=/dev/shm/pf-bundle/ecbench
PF=/dev/shm/src-pf/backlog/XP-20261029-packed-filter
DB=/dev/shm/pf-work/dgb-ref/backlog/XP-20261029-dg-blocked   # spec-P*.json of cd04cfd2d1 (XP-20261029-dg-blocked freeze) via git show
fail=0
chk() { if [ "$1" = 0 ]; then echo "  OK   $2"; else echo "  FAIL $2"; fail=1; fi; }
for b in 24 28 32; do
  echo "== b=$b"
  for k in "" "-rho"; do
    s=$PF/spec-P$b$k.json
    $EB plan --spec $s > /dev/shm/pf-work/plan-P$b$k.txt 2> /dev/shm/pf-work/plan-P$b$k.err
    e=$?
    chk $e "ecbench plan spec-P$b$k.json exit=$e; $(head -1 /dev/shm/pf-work/plan-P$b$k.txt | grep -o '[0-9]* executions')"
    echo "     slugs: $(grep -o 'icv1-fp[0-9]*-[a-z0-9-]*' /dev/shm/pf-work/plan-P$b$k.txt | sort -u | tr '\n' ' ')"
    echo "     arms (spec order): $(grep '^  arm ' /dev/shm/pf-work/plan-P$b$k.txt | awk '{print $2":"$3":"$4}' | tr '\n' ' ')"
  done
  diff <(grep '^  workload ' /dev/shm/pf-work/plan-P$b.txt) <(grep '^  workload ' /dev/shm/pf-work/plan-P$b-rho.txt) > /dev/null
  chk $? "workload lines of wall plan == counted plan"
  W=$PF/spec-P$b.json
  R=$PF/spec-P$b-rho.json
  jq -e '[.arms[].name]==["dgb","dgb-aa","dgbf","dgbf-edge"] and [.arms[].role]==["baseline","control","candidate","reference"]' $W >/dev/null
  chk $? "wall arms names/roles in order dgb,dgb-aa,dgbf,dgbf-edge = baseline,control,candidate,reference"
  jq -e '.arms[1].method==.arms[0].method and .arms[3].method==.arms[2].method' $W >/dev/null
  chk $? "dgb-aa method == dgb method; dgbf-edge method == dgbf method"
  jq -e '(.arms[2].method.params|del(.oracle))==(.arms[0].method.params|del(.oracle)) and .arms[2].method.params.oracle==(.arms[0].method.params.oracle+",filter=1") and .arms[0].method.params.oracle=="mitm-prime-blocked:m=2" and .arms[2].method.id==.arms[0].method.id' $W >/dev/null
  chk $? "dgbf params == dgb params except oracle, oracle = dgb oracle + ',filter=1'"
  diff <(jq -S '.arms[]|select(.name=="dgb")|.method' $W) <(jq -S '.arms[]|select(.name=="dgb")|.method' $DB/spec-P$b.json) >/dev/null
  chk $? "dgb method (jq -S) == dg-blocked spec-P$b.json dgb arm"
  # counted spec
  for a in dgb dgbf; do
    diff <(jq -S ".arms[]|select(.name==\"$a\")" $R) <(jq -S ".arms[]|select(.name==\"$a\")" $W) >/dev/null
    chk $? "counted $a arm (jq -S) == wall $a arm"
  done
  for a in rho-neg rho-neg-aa; do
    diff <(jq -S ".arms[]|select(.name==\"$a\")" $R) <(jq -S ".arms[]|select(.name==\"$a\")" $DB/spec-P$b-rho.json) >/dev/null
    chk $? "counted $a arm (jq -S) == dg-blocked spec-P$b-rho.json $a arm"
  done
  diff <(jq -S .workloads $R) <(jq -S .workloads $W) >/dev/null; chk $? "counted workloads == wall workloads"
  diff <(jq -S .measurement $R) <(jq -S .measurement $W) >/dev/null; chk $? "counted measurement == wall measurement"
  echo "     counted arms: $(jq -r '[.arms[]|.name+":"+.role]|join(" ")' $R)"
  echo "     measurement: $(jq -c .measurement $W)"
  echo "     wall dgb-vs-dgb(dg-blocked) workloads equal: $(diff <(jq -S .workloads $W) <(jq -S .workloads $DB/spec-P$b.json) >/dev/null && echo yes || echo no-expected-fresh-curves)"
done
echo "P3_FAIL=$fail"
