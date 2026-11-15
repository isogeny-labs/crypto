#!/usr/bin/env bash
# Offline stand-in for curl (installed as "curl" first on PATH by run_tests.sh). It emulates the few RunPod REST,
# GraphQL and pod-proxy calls the route makes, from files in $STUB_DIR, and logs every call's argv to
# $STUB_DIR/calls.log. With $STUB_DIR/deny present it fails every call (network blocked). It never opens a socket.
set -u
S=${STUB_DIR:?}
printf '%s\n' "$*" >> "$S/calls.log"
[ -e "$S/deny" ] && { echo "curl: (7) blocked by the offline test stub" >&2; exit 7; }
method=GET url="" out="" wfmt="" data="" fail=0
while [ $# -gt 0 ]; do
  case $1 in
    -X) method=$2; shift ;;
    -o) out=$2; shift ;;
    -w) wfmt=$2; shift ;;
    -d|--data-binary) data=$2; shift ;;
    -H|-m) shift ;;
    -f) fail=1 ;;
    -sS|-s|-S) ;;
    http*) url=$1 ;;
  esac; shift
done
[ "$method" = GET ] && [ -n "$data" ] && method=POST
body() { if [ -n "$data" ]; then case $data in @-) cat ;; @*) cat "${data#@}" ;; *) printf '%s' "$data" ;; esac; fi; }
emit() { # emit CODE FILE|-
  local code=$1 src=$2
  if [ "$fail" = 1 ] && [ "$code" -ge 400 ]; then echo "curl: (22) The requested URL returned error: $code" >&2; exit 22; fi
  if [ -n "$out" ]; then if [ "$src" = - ]; then cat > "$out"; else cat "$src" > "$out"; fi
  else if [ "$src" = - ]; then cat; else cat "$src"; fi; fi
  [ -n "$wfmt" ] && printf '%s' "$code"
  exit 0
}
rpdate() { date -u -d "@$1" '+%Y-%m-%d %H:%M:%S.000 +0000 UTC'; }
[ -f "$S/pods.json" ] || echo '[]' > "$S/pods.json"
case $url in
  https://rest.runpod.io/v1/pods)
    if [ "$method" = POST ]; then
      body > "$S/create-body.json"
      name=$(jq -r .name "$S/create-body.json"); id=${STUB_POD_ID:-stubpod1}
      ca=$(( $(date +%s) - ${STUB_CREATED_AGO:-0} ))
      jq --arg id "$id" --arg n "$name" --arg ca "$(rpdate "$ca")" \
        --slurpfile b "$S/create-body.json" \
        '. + {id:$id, name:$n, createdAt:$ca, lastStartedAt:$ca, lastStatusChange:("Rented by User: " + $ca), desiredStatus:"RUNNING", consumerUserId:"user_stub", env:$b[0].env}' \
        "$S/create-template.json" > "$S/pod-$id.json"
      jq --slurpfile p "$S/pod-$id.json" '. + [$p[0] | del(.machine) | . + {machine:{}}]' "$S/pods.json" > "$S/pods.tmp" && mv "$S/pods.tmp" "$S/pods.json"
      jq '.machine = {}' "$S/pod-$id.json" > "$S/pod-$id.rec" && mv "$S/pod-$id.rec" "$S/pod-$id.json.rec"
      jq 'del(.env)' "$S/pod-$id.json" > "$S/pod-$id.tmp" && mv "$S/pod-$id.tmp" "$S/pod-$id.json"   # the create response omits env
      if [ -e "$S/create-timeout" ]; then sleep 0; echo "curl: (28) Operation timed out" >&2; exit 28; fi
      if [ -e "$S/create-error" ]; then rm -f "$S/pod-$id.json" "$S/pod-$id.json.rec"; echo '[]' > "$S/pods.json"
        emit 400 - <<<'{"error":"no instances available"}'; fi
      emit 200 "$S/pod-$id.json"
    fi
    emit 200 "$S/pods.json" ;;
  https://rest.runpod.io/v1/pods/*)
    id=${url##*/}
    if [ "$method" = DELETE ]; then
      [ -e "$S/delete-fail" ] && { emit 500 - <<<'{"error":"x"}'; }
      if jq -e --arg i "$id" 'any(.[]; .id == $i)' "$S/pods.json" >/dev/null; then
        jq --arg i "$id" 'map(select(.id != $i))' "$S/pods.json" > "$S/pods.tmp" && mv "$S/pods.tmp" "$S/pods.json"
        rm -f "$S/pod-$id.json" "$S/pod-$id.json.rec"; emit 204 - </dev/null
      fi
      emit 404 - <<<'{"error":"pod not found"}'
    fi
    if [ -f "$S/pod-$id.json.rec" ]; then emit 200 "$S/pod-$id.json.rec"; fi
    emit 404 - <<<'{"error":"pod not found"}' ;;
  https://api.runpod.io/graphql)
    q=$(body)
    case $q in
      *ramMultiplier*) emit 200 - <<<'{"data":{"cpuFlavors":[{"ramMultiplier":8}]}}' ;;
      *) if [ -f "$S/spec.json" ]; then emit 200 "$S/spec.json"; fi
         emit 200 - <<<'{"data":{"cpuFlavors":[{"specifics":{"stockStatus":"High","securePrice":1.04}}]}}' ;;
    esac ;;
  https://*-8000.proxy.runpod.net/*)
    path=/${url#https://*/}
    [ -e "$S/unreachable" ] && { emit 502 - <<<'<html>502 Bad Gateway</html>'; }
    case $path in
      /status) if [ -f "$S/status.json" ]; then emit 200 "$S/status.json"; fi
               emit 200 - <<<'{"state":"idle","exit":null,"started":null,"ended":null,"stages":null,"progress":"","meminfo":[],"nproc":48}' ;;
      /upload/*|/append/*)
        n=${path##*/}; mkdir -p "$S/up"
        case $path in /upload/*) body > "$S/up/$n" ;; *) body >> "$S/up/$n" ;; esac
        emit 200 - < <(jq -nc --arg n "$n" --arg h "$(sha256sum "$S/up/$n" | cut -d' ' -f1)" '{name:$n, sha256:$h}') ;;
      /start)
        body > "$S/start-body.json"
        st=$(date -u +%FT%TZ)
        jq -nc --arg s "$st" --slurpfile b "$S/start-body.json" '{state:"running",exit:null,started:$s,ended:null,stages:$b[0].stages}' > "$S/status.json"
        emit 200 "$S/status.json" ;;
      /results.tar.gz)
        src="$S/tree"; [ -d "$src" ] || src="$S/pretree"   # before start the pod's /results holds only the cgroup copy
        [ -d "$src" ] || { emit 404 - <<<'{"error":"path"}'; }
        if [ "$src" = "$S/tree" ]; then n=0; [ -f "$S/fetchcount" ] && n=$(cat "$S/fetchcount"); n=$((n + 1)); echo "$n" > "$S/fetchcount"
          [ -e "$S/tree-varies" ] && echo "$n" > "$S/tree/varying.txt"; fi
        tmp=$(mktemp -d); cp -a "$src" "$tmp/results"; tar -czf "$tmp/r.tgz" -C "$tmp" results
        emit 200 "$tmp/r.tgz" ;;
    esac ;;
esac
echo "curl: stub has no route for $method $url" >&2; exit 6
