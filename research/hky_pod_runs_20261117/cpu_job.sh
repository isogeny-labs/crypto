#!/usr/bin/env bash
# CPU pod: (A) optlevel on a quiet host, (B) hky2 depth 13 both draws, (C) hky2 depth 14, c <= 6.
# Stages run in this order so that the calibration sees an idle machine.
set -uo pipefail
W=/workspace; O=$W/out; B=$W/in/pilot_candidate
mkdir -p $O/optlevel $O/t10 $O/t13 $O/t14
exec > >(tee -a $O/cpu_job.log) 2>&1
date -u +%FT%TZ; nproc; free -g | head -2; lscpu | head -30; uptime
echo "47d6c281ecf9cc85ac338fae9ee3ddb4012f88d159ed996cdb855ee0fdeda543  $B" | sha256sum -c - || exit 9
chmod +x $B
export DEBIAN_FRONTEND=noninteractive
(apt-get update -qq && apt-get install -y -qq gcc make linux-tools-common linux-tools-generic) > $O/apt.log 2>&1; echo "apt rc=$?"
mkdir -p $W/src && tar xzf $W/in/optlevel.tar.gz -C $W/src && cd $W/src
OPT="python3 tools/optlevel/optlevel.py"
CELL="--p 2013265921 --ladder-t none --planted 0 --engine hky2 --hky-max-cols 2000000 --hky-max-entries 14000000000 --kinds kummer --m 2 --controls tower"

echo "=== A1 probe"; $OPT probe | tee $O/optlevel/probe.json
cat /proc/sys/kernel/perf_event_paranoid; (perf stat -e cycles,instructions true 2>&1 | tail -8) || true
echo "=== A2 controls (quiet host)"; uptime
$OPT controls --out $O/optlevel/controls --repeat 5; echo "controls rc=$?"; uptime

echo "=== A3 hky2 at t = 10 under optlevel measure"
cat > $W/run_t10.sh <<'S'
#!/usr/bin/env bash
out=$(mktemp); /workspace/in/pilot_candidate --p 2013265921 --ladder-t none --planted 0 --engine hky2 --hky-max-cols 2000000 --hky-max-entries 14000000000 --kinds kummer --m 2 --controls tower --random 1 --hky-cmax 6 --budget 3600 --t-min 10 --max-t 10 --out $out >/dev/null 2>&1
python3 -c "import json,sys;print('OPS',sum(c['muladds'] for l in open(sys.argv[1]) if l.strip() for c in json.loads(l).get('closures',[])))" $out; rm -f $out
S
chmod +x $W/run_t10.sh
$OPT measure --out $O/optlevel/hky2_t10.measure.json --label hky2-t10 --repeat 3 --ops-regex 'OPS (\d+)' --op-class modmuladd_p31 --scale env:RAYON_NUM_THREADS:1,$(nproc) --timeout 7200 -- $W/run_t10.sh; echo "measure rc=$?"
CAL=$(ls $O/optlevel/controls/*calibration*.json 2>/dev/null | head -1); echo "cal=$CAL"
[ -n "$CAL" ] && $OPT score --calibration $CAL --measurement $O/optlevel/hky2_t10.measure.json --out $O/optlevel/hky2_t10.record.json && $OPT plot --calibration $CAL --out $O/optlevel/hky2_t10.svg $O/optlevel/hky2_t10.record.json
tar czf $O/optlevel.tar.gz -C $O optlevel; echo "=== A done"; date -u +%FT%TZ

echo "=== B t = 13, draws 0 and 1, c <= 6"; uptime
/usr/bin/time -v $B $CELL --random 2 --hky-cmax 6 --budget 14400 --t-min 13 --max-t 13 --hky-progress $O/t13/progress.jsonl --out $O/t13/hky2.jsonl > $O/t13/run.log 2>&1; echo "t13 rc=$?"; date -u +%FT%TZ
tail -25 $O/t13/run.log

echo "=== C t = 14, draw 0, c <= 6"; uptime
/usr/bin/time -v $B $CELL --random 1 --hky-cmax 6 --budget 79200 --t-min 14 --max-t 14 --hky-progress $O/t14/progress.jsonl --out $O/t14/hky2.jsonl > $O/t14/run.log 2>&1; echo "t14 rc=$?"; date -u +%FT%TZ
tail -25 $O/t14/run.log
echo "=== ALL DONE"
