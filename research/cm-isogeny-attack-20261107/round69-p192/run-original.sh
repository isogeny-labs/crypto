set -u
r=/home/ec2-user/research/cm-isogeny-attack-20261107/round69-p192
timeout 120 /tmp/p192-round69-target/release/signed-baby-response --batch-controls > "$r/batch-controls.json" 2> "$r/batch-controls.log"
code=$?; echo "$code" > "$r/controls-exit.txt"; if [ "$code" -ne 0 ]; then exit "$code"; fi
for mode in candidate per-block; do
 extra=''
 if [ "$mode" = per-block ]; then extra='--per-block-norm'; fi
 timeout 120 /tmp/p192-round69-target/release/signed-baby-response $extra > "$r/$mode-response.json" 2> "$r/$mode-response.log"
 code=$?; echo "$code" > "$r/$mode-exit.txt"; if [ "$code" -ne 0 ]; then exit "$code"; fi
 timeout 120 /tmp/p192-round69-target/release/cached-p192 "$r" $extra > "$r/$mode-P192.json" 2> "$r/$mode-P192.log"
 code=$?; echo "$code" > "$r/$mode-P192-exit.txt"; if [ "$code" -ne 0 ]; then exit "$code"; fi
done
