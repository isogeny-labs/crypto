set -eu
cd /tmp
curl -fsS -o b.tgz "$GET_URL"
echo "b9a9d4ae5df9295f477a31775fed0eefd478b11bb23055a6271389aff6d8c3ab  b.tgz" | sha256sum -c
tar xzf b.tgz && cd r1c
printf '%s  %s\n' 4693ab67a5f730c1dc5b41518669b412852b13ef6de193d3824d297dde6284a1 r1_carry c962966ceb6bb4ae576c7869113ed78fb3ba066aaa6d1d9178cf885a26db0324 r1b 1b3837bd2f04211ee029399fdf0d37d6bc4cbd7f5fb847b11f1114f038da22c2 r1c | sha256sum -c
O=/tmp/out; mkdir -p $O
{ date -u +%FT%TZ; uname -a; nproc; grep -m1 'model name' /proc/cpuinfo || true; } > $O/host.txt
./r1_carry 20261115 12 4 > $O/armA_r1_seed20261115.jsonl
./r1b 20261115 > $O/armA_r1b_seed20261115.jsonl
./r1c 20261115 6 2097152 > $O/armB_r1c_seed20261115.jsonl
date -u +%FT%TZ >> $O/host.txt
(cd $O && sha256sum armA_r1_seed20261115.jsonl armA_r1b_seed20261115.jsonl armB_r1c_seed20261115.jsonl host.txt > SHA256SUMS)
cat $O/SHA256SUMS
tar czf /tmp/results.tgz -C /tmp out
curl -fsS -X PUT -T /tmp/results.tgz "$PUT_URL"
echo R1C_RUNPOD_DONE
sleep infinity
