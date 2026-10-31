#!/usr/bin/env bash
# aggregate.sh -- derive every reported number from the recorded rows in out/ (awk only); also writes results.json
D=/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E0-separation-lemma
cd "$D" || exit 2
FILES=$(ls out/T1*.txt out/T2*.txt out/T3.txt out/T4.txt out/T5.txt out/controls.txt 2>/dev/null)
awk -f aggregate.awk $FILES > out/results_kv.txt
# operation counts derived from the same raw rows (see opcounts.awk); appended to the kv file and hence to results.json
awk -f opcounts.awk out/T1*.txt out/T2*.txt out/T3.txt >> out/results_kv.txt
cat out/results_kv.txt
echo "--- timing"
cat out/timing.txt
echo "--- done markers"
grep -H "^DONE" out/*.txt
echo "--- errors in raw outputs (lines containing ***)"
grep -c '\*\*\*' out/*.txt | grep -v ':0$' || echo "none"
# JSON (flat, dotted keys)
awk -F= 'BEGIN{printf "{\n"; first=1}
  { k=$1; v=substr($0,length($1)+2); gsub(/\\/,"\\\\",v); gsub(/"/,"\\\"",v);
    if(!first)printf ",\n"; first=0;
    if(v ~ /^-?[0-9]+(\.[0-9]+)?$/) printf "  \"%s\": %s",k,v; else printf "  \"%s\": \"%s\"",k,v }
  END{printf "\n}\n"}' out/results_kv.txt > results.json
