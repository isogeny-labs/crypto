set -eu
cd /tmp
curl -fsS -o b.tgz "$GET_URL"
echo "c0c7450893159d54592d280054120a83830d237aca6cfd626ca16c759d777f3f  b.tgz" | sha256sum -c
tar xzf b.tgz && cd r1d
echo "c56b9f4e73b4c8fae55dbe89fa3fed46e264ab7b146312da709c301d25059399  r1d" | sha256sum -c
O=/tmp/out; mkdir -p $O
{ date -u +%FT%TZ; uname -a; nproc; grep -m1 'model name' /proc/cpuinfo || true; } > $O/host.txt
./r1d 20261116 6 2097152 3 > $O/part1_w3_seed20261116.jsonl
date -u +%FT%TZ >> $O/host.txt
./r1d 20261116 2 16777216 2 > $O/part2_largep_seed20261116.jsonl
date -u +%FT%TZ >> $O/host.txt
(cd $O && sha256sum part1_w3_seed20261116.jsonl part2_largep_seed20261116.jsonl host.txt > SHA256SUMS)
cat $O/SHA256SUMS
tar czf /tmp/results.tgz -C /tmp out
curl -fsS -X PUT -T /tmp/results.tgz "$PUT_URL"
echo R1D_RUNPOD_DONE
sleep infinity
