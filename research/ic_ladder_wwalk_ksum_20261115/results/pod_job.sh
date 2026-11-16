set -u
apt-get update -qq >/dev/null 2>&1 && apt-get install -y -qq nodejs jq util-linux >/dev/null 2>&1; node --version || { echo NO_NODE; sleep infinity; }
mkdir -p /work && cd /work
curl -fsS -o src.tgz "$GET_URL" || { echo DOWNLOAD_FAIL; sleep infinity; }
echo "5b8d9f23a6337aec137da69340eddc0ea563a51600e1db8fecbecfad6eb9cbb9  src.tgz" | sha256sum -c || { echo SHA_FAIL; sleep infinity; }
mkdir -p tree && tar xzf src.tgz -C tree
( while true; do sleep 1800; tar czf /work/progress.tgz -C /work out build_stdout.txt run_stdout.txt 2>/dev/null; curl -fsS -X PUT -T /work/progress.tgz "$PROG_URL" >/dev/null 2>&1; done ) &
bash tree/research/ic_ladder_wwalk_ksum_20261115/pod/build.sh /work/src.tgz /work > /work/build_stdout.txt 2>&1; echo "build exit $?" >> /work/build_stdout.txt
bash tree/research/ic_ladder_wwalk_ksum_20261115/pod/run.sh /work/tree/research/ic_ladder_wwalk_ksum_20261115 /work > /work/run_stdout.txt 2>&1; echo "run exit $?" >> /work/run_stdout.txt
mkdir -p /work/out; cp /work/build_stdout.txt /work/run_stdout.txt /work/out/ 2>/dev/null; cp /work/bin/BINARY.sha256 /work/out/ 2>/dev/null
tar czf /work/results.tgz -C /work out
curl -fsS -X PUT -T /work/results.tgz "$PUT_URL" && echo LADDER_DONE
sleep infinity
