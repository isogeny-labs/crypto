set -u
r=/home/ec2-user/research/cm-isogeny-attack-20261107/round68-p192
timeout 120 /tmp/p192-round68-target/release/cancellation-identity > "$r/identity.json" 2> "$r/identity.log"
code=$?; echo "$code" > "$r/identity-exit.txt"; if [ "$code" -ne 0 ]; then exit "$code"; fi
for mode in candidate direct; do
 extra=''
 if [ "$mode" = direct ]; then extra='--direct-numerator'; fi
 timeout 120 /tmp/p192-round68-target/release/signed-baby-response $extra > "$r/$mode-response.json" 2> "$r/$mode-response.log"
 code=$?; echo "$code" > "$r/$mode-exit.txt"; if [ "$code" -ne 0 ]; then exit "$code"; fi
 timeout 120 /tmp/p192-round68-target/release/cached-p192 "$r" $extra > "$r/$mode-P192.json" 2> "$r/$mode-P192.log"
 code=$?; echo "$code" > "$r/$mode-P192-exit.txt"; if [ "$code" -ne 0 ]; then exit "$code"; fi
done
