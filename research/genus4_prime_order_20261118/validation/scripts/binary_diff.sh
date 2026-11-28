#!/usr/bin/env bash
# C4a: compare the producer's genus_ladder (copied and hashed at dispatch) with the validator rebuild,
# by allocated section (lesson L-7f8877029be4: build-id and .llvm.<hash> suffixes follow the build path).
set -u
V=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/val4po
P=$V/genus_ladder.po.copy
B=$V/bin/genus_ladder.val
O=/home/user/crypto/research/genus4_prime_order_20261118/validation/out/binary_diff.txt
T=$(mktemp -d -p $V)
{
echo "# Binary comparison (validator, $(date -u +%FT%TZ))"
echo "producer binary (copied and hashed at dispatch 2026-11-18T21:15:08Z, before any validator build): $(sha256sum $P | cut -d' ' -f1)  size $(stat -c %s $P)"
echo "binary.sha256 (frozen at 6da1e3c5b2): $(git -C /home/user/crypto show 6da1e3c5b2:research/genus4_prime_order_20261118/binary.sha256)"
echo "validator rebuild from git archive ca72267899 (paths.py, $(wc -l < $V/paths_ca7.txt) paths, list sha256 $(sha256sum $V/paths_ca7.txt | cut -d' ' -f1)): $(sha256sum $B | cut -d' ' -f1)  size $(stat -c %s $B)"
echo "toolchain: $(rustc --version); $(cargo --version)"
echo "build log: $(grep 'Compiling crypto v0.1.0' $V/build.log | tail -1) / $(grep Finished $V/build.log | tail -1)"
echo
echo "## allocated sections (objcopy -O binary --only-section, sha256 prefix)"
for s in .text .rodata .data .data.rel.ro .eh_frame .eh_frame_hdr .gcc_except_table .init_array .got .note.gnu.build-id; do
  objcopy -O binary --only-section=$s $P $T/p.bin 2>/dev/null; objcopy -O binary --only-section=$s $B $T/v.bin 2>/dev/null
  a=$(sha256sum $T/p.bin | cut -c1-16); b=$(sha256sum $T/v.bin | cut -c1-16)
  echo "$s prod=$a val=$b $([ "$a" = "$b" ] && echo SAME || echo DIFF)"
done
echo
echo "## section size table diff (readelf -SW name size)"
diff <(readelf -SW $P | awk '/\]/{print $2, $6}') <(readelf -SW $B | awk '/\]/{print $2, $6}') && echo "(none)"
echo
nm $P | sort -k3 > $T/p.nm; nm $B | sort -k3 > $T/v.nm
echo "## symbols: $(wc -l < $T/p.nm) / $(wc -l < $T/v.nm); address+type columns differ in $(diff <(cut -d' ' -f1,2 $T/p.nm) <(cut -d' ' -f1,2 $T/v.nm) | grep -c '^<') lines; names differ in $(diff <(cut -d' ' -f3 $T/p.nm) <(cut -d' ' -f3 $T/v.nm) | grep -c '^<') symbols; differing names with the .llvm.<hash> suffix removed: $(diff <(cut -d' ' -f3 $T/p.nm | sed 's/\.llvm\.[0-9]*$//') <(cut -d' ' -f3 $T/v.nm | sed 's/\.llvm\.[0-9]*$//') | grep -c '^<')"
} > $O
rm -rf $T
cat $O
