#!/usr/bin/env bash
# Freeze integrity: (1) frozen.sha256 entries equal the blobs at the freeze commit;
# (2) the instrument blobs at every later study commit; (3) commit order and times;
# (4) run start = raw-file mtime - wall_s; (5) binary.sha256 vs the producer binaries.
cd /home/user/crypto
chk(){ c=$1; f=$2; echo "### freeze commit $c ($(git log -1 --format=%cd --date=iso $c)) file $f"; git show $c:$f | while read h p; do case "$p" in /tmp/*) echo "  (binary path outside repo: $p  $h)";; *) a=$(git show $c:$p | sha256sum | cut -d' ' -f1); [ "$a" = "$h" ] && echo "  OK $p" || echo "  MISMATCH $p frozen=$h actual=$a";; esac; done; }
chk 4feb188f47 research/ic_positive_control_lucas_20261116/frozen.sha256
chk caae5eca9f research/ic_positive_control_la_20261116/frozen.sha256
chk 551590e3b7 research/ic_positive_control_unit_t_20261116/frozen.sha256
chk 81ddcdc450 research/ic_positive_control_capped_20261116/frozen.sha256
echo "### binary.sha256 files and the commit that added each"
for d in ic_positive_control_la_20261116 ic_positive_control_unit_t_20261116 ic_positive_control_capped_20261116; do echo "$d: added in $(git log --diff-filter=A --format='%h %cd' --date=iso 79c91dd913 -- research/$d/binary.sha256) :: $(git show 79c91dd913:research/$d/binary.sha256)"; done
echo "### instrument blob hashes per commit"
for c in 4feb188f47 82b3d37a39 ba0c9c1233 caae5eca9f cc2d4d50c6 2bc7249bac fbd22482c7 be5e426df1 551590e3b7 3eaf62ab17 fcc5fc5e87 81ddcdc450 87afac5ff2 5576b7fac8 79c91dd913; do echo "$c $(git log -1 --format='%cd' --date=iso $c) ic_control=$(git show $c:src/cryptanalysis/chained_transfer/ic_control.rs | sha256sum | cut -c1-12) bin=$(git show $c:src/bin/ic_positive_control.rs | sha256sum | cut -c1-12) $(git log -1 --format=%s $c | cut -c1-70)"; done
echo "### study commit logs"
for d in ic_positive_control_lucas_20261116 ic_positive_control_la_20261116 ic_positive_control_unit_t_20261116 ic_positive_control_capped_20261116; do echo "== $d"; git log --format='%h %cd %s' --date=iso 79c91dd913 -- research/$d; done
echo "### run start = mtime - wall_s"
python3 -I - <<'PY'
import json, os, datetime
for f in ["research/ic_positive_control_lucas_20261116/run.json", "research/ic_positive_control_la_20261116/train_cc0.7.json",
          "research/ic_positive_control_la_20261116/train_cc1.0.json", "research/ic_positive_control_la_20261116/train_cc1.4.json",
          "research/ic_positive_control_la_20261116/train_cc2.0.json", "research/ic_positive_control_la_20261116/run.json",
          "research/ic_positive_control_unit_t_20261116/run.json", "research/ic_positive_control_capped_20261116/run_w.json",
          "research/ic_positive_control_capped_20261116/run_sr.json"]:
    w = json.load(open(f))["wall_s"]; m = os.stat(f).st_mtime
    print(f, "end", datetime.datetime.utcfromtimestamp(m).isoformat(timespec="seconds"), "wall_s", round(w, 1),
          "start", datetime.datetime.utcfromtimestamp(m - w).isoformat(timespec="seconds"))
PY
echo "### producer binaries (sha256 at validator dispatch 14:52 UTC)"
cat /tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/validator/producer_binaries_at_dispatch.sha256
