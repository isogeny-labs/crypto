#!/usr/bin/env bash
# Where a validator-built binary differs from the producer's: byte offsets mapped to ELF
# sections, and the differing symbol names.  Usage: binary_diff.sh A B
A=$1; B=$2
sha256sum $A $B; ls -l $A $B | awk '{print $5, $9}'
n=$(cmp -l $A $B | wc -l); echo "differing bytes: $n"
python3 -I - "$A" "$B" <<'PY'
import subprocess, sys, collections
a, b = open(sys.argv[1], "rb").read(), open(sys.argv[2], "rb").read()
secs = []
for line in subprocess.run(["readelf", "-S", "-W", sys.argv[1]], capture_output=True, text=True).stdout.splitlines():
    if "]" in line and not line.strip().startswith("[Nr]"):
        f = line.split("]", 1)[1].split()
        if len(f) >= 5:
            secs.append((f[0], int(f[3], 16), int(f[4], 16)))
cnt = collections.Counter()
for i, (x, y) in enumerate(zip(a, b)):
    if x != y:
        cnt[next((n for n, o, sz in secs if o <= i < o + sz), "<outside sections>")] += 1
print("differing bytes by section:", dict(cnt))
PY
echo "symbols present in one binary only:"
diff <(nm $A | awk '{print $NF}' | sort) <(nm $B | awk '{print $NF}' | sort) | grep '^[<>]' | head -10
for s in .text .rodata .data .data.rel.ro .eh_frame .gcc_except_table; do
  a=$(objcopy -O binary --only-section=$s $A /dev/stdout 2>/dev/null | sha256sum | cut -c1-16)
  b=$(objcopy -O binary --only-section=$s $B /dev/stdout 2>/dev/null | sha256sum | cut -c1-16)
  echo "section $s: $([ $a = $b ] && echo identical || echo DIFFERENT) ($a)"
done
