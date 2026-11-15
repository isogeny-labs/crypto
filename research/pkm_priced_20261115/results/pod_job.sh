set -u
mkdir -p /work && cd /work
curl -fsS -o src.tgz "$GET_URL" || { echo DOWNLOAD_FAIL; sleep infinity; }
echo "359cb4f85458d8a8574bbb2f23ce6c37b4f97daa6e1def617225c8b1a47a5097  src.tgz" | sha256sum -c || { echo SHA_FAIL; sleep infinity; }
mkdir -p tree && tar xzf src.tgz -C tree
WORK=/work SRC=/work/src.tgz bash tree/research/pkm_priced_20261115/pod/build.sh > /work/build_stdout.txt 2>&1; echo "build exit $?" >> /work/build_stdout.txt
WORK=/work OUT=/work/out bash tree/research/pkm_priced_20261115/pod/run.sh > /work/run_stdout.txt 2>&1; echo "run exit $?" >> /work/run_stdout.txt
mkdir -p /work/out; cp /work/build_stdout.txt /work/run_stdout.txt /work/build.log /work/ids.txt /work/out/ 2>/dev/null
tar czf /work/results.tgz -C /work out
curl -fsS -X PUT -T /work/results.tgz "$PUT_URL" && echo PKM_DONE
sleep infinity
