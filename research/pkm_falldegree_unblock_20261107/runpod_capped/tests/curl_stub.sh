#!/usr/bin/env bash
# Offline stand-in for curl (installed as "curl" first on PATH by run_tests.sh). It emulates the RunPod REST v2 calls
# (api.runpod.io/v2: pods list with its envelope and cursor pages, create, get, delete, the CPU catalog) and the
# pod-proxy calls the route makes, from files in $STUB_DIR, and logs every call's argv to $STUB_DIR/calls.log. With
# $STUB_DIR/deny present it fails every call (network blocked). It has no route for REST v1 or GraphQL, so a call to
# either fails (exit 6). It never opens a socket. The v2 shapes follow V2_FIELDS.md (read-only GETs of 2026-11-21).
set -u
S=${STUB_DIR:?}
printf '%s\n' "$*" >> "$S/calls.log"
[ -e "$S/deny" ] && { echo "curl: (7) blocked by the offline test stub" >&2; exit 7; }
method=GET url="" out="" wfmt="" data="" fail=0 getq=0 q=""
while [ $# -gt 0 ]; do
  case $1 in
    -X) method=$2; shift ;;
    -o) out=$2; shift ;;
    -w) wfmt=$2; shift ;;
    -d|--data-binary) data=$2; shift ;;
    --get) getq=1 ;;
    --data-urlencode) q="${q:+$q&}$2"; shift ;;
    -H|-m) shift ;;
    -f) fail=1 ;;
    -sS|-s|-S) ;;
    http*) url=$1 ;;
  esac; shift
done
[ "$method" = GET ] && [ -n "$data" ] && method=POST
[ "$getq" = 1 ] && [ -n "$q" ] && url="$url?$q"   # curl --get --data-urlencode: the pair goes into the query
body() { if [ -n "$data" ]; then case $data in @-) cat ;; @*) cat "${data#@}" ;; *) printf '%s' "$data" ;; esac; fi; }
emit() { # emit CODE FILE|-
  local code=$1 src=$2
  if [ "$fail" = 1 ] && [ "$code" -ge 400 ]; then echo "curl: (22) The requested URL returned error: $code" >&2; exit 22; fi
  if [ -n "$out" ]; then if [ "$src" = - ]; then cat > "$out"; else cat "$src" > "$out"; fi
  else if [ "$src" = - ]; then cat; else cat "$src"; fi; fi
  [ -n "$wfmt" ] && printf '%s' "$code"
  exit 0
}
isod() { date -u -d "@$1" '+%Y-%m-%dT%H:%M:%S.123Z'; }
notfound='{"detail":"pod not found","status":404,"title":"Not Found"}'
[ -f "$S/pods.json" ] || echo '[]' > "$S/pods.json"
V2=https://api.runpod.io/v2
case $url in
  "$V2/pods"|"$V2/pods?cursor="*)
    if [ "$method" = POST ]; then
      body > "$S/create-body.json"
      name=$(jq -r .name "$S/create-body.json"); id=${STUB_POD_ID:-stubpod1}
      ca=$(( $(date +%s) - ${STUB_CREATED_AGO:-0} ))
      # the stored record: the template, the request's env and command (v2 encodes entrypoint and cmd into args)
      jq --arg id "$id" --arg n "$name" --arg ca "$(isod "$ca")" --slurpfile b "$S/create-body.json" \
        '. + {id:$id, name:$n, createdAt:$ca, startedAt:$ca, status:"RUNNING", actions:["stop","restart","terminate"],
              env:$b[0].env, args:("bash -c " + ($b[0].cmd[0] | .[0:40])), image:$b[0].image, ports:$b[0].ports}' \
        "$S/create-template.json" > "$S/pod-$id.json.rec"
      jq --slurpfile p "$S/pod-$id.json.rec" '. + [$p[0] | del(.env, .args, .runtime)]' "$S/pods.json" > "$S/pods.tmp" && mv "$S/pods.tmp" "$S/pods.json"
      # the create response: the record without env, and dataCenterId null while the pod provisions
      jq 'del(.env) | .dataCenterId = null' "$S/pod-$id.json.rec" > "$S/pod-$id.json"
      [ -f "$S/create-response-patch.jq" ] && { jq -f "$S/create-response-patch.jq" "$S/pod-$id.json" > "$S/x" && mv "$S/x" "$S/pod-$id.json"; }
      [ -f "$S/record-patch.jq" ] && { jq -f "$S/record-patch.jq" "$S/pod-$id.json.rec" > "$S/x" && mv "$S/x" "$S/pod-$id.json.rec"; }
      if [ -e "$S/create-timeout" ]; then echo "curl: (28) Operation timed out" >&2; [ -n "$wfmt" ] && printf '000'; exit 28; fi
      if [ -e "$S/create-error-html" ]; then rm -f "$S/pod-$id.json" "$S/pod-$id.json.rec"; echo '[]' > "$S/pods.json"
        emit 502 - <<<"<html>502 Bad Gateway, request was: $(tr -d '\n' < "$S/create-body.json")</html>"; fi   # a non-JSON error that echoes the body
      if [ -e "$S/create-error" ]; then rm -f "$S/pod-$id.json" "$S/pod-$id.json.rec"; echo '[]' > "$S/pods.json"
        emit 400 - <<<'{"detail":"no instances available","status":400,"title":"Bad Request"}'; fi
      emit "${STUB_CREATE_CODE:-201}" "$S/pod-$id.json"
    fi
    # GET list: the envelope; with pods-page2.json present the first page points to a second one
    if [ -f "$S/pods-page2.json" ] && [ "$url" = "$V2/pods" ]; then
      emit 200 - < <(jq '{pods:., pagination:{hasNextPage:true, nextCursor:"c2"}}' "$S/pods.json")
    elif [ "$url" = "$V2/pods?cursor=c2" ]; then
      emit 200 - < <(jq '{pods:., pagination:{hasNextPage:false, nextCursor:null}}' "$S/pods-page2.json")
    fi
    emit 200 - < <(jq '{pods:., pagination:{hasNextPage:false, nextCursor:null}}' "$S/pods.json") ;;
  "$V2/pods/"*)
    id=${url##*/}
    if [ "$method" = DELETE ]; then
      [ -e "$S/delete-fail" ] && { emit 500 - <<<'{"detail":"x","status":500}'; }
      if jq -e --arg i "$id" 'any(.[]; .id == $i)' "$S/pods.json" >/dev/null; then
        jq --arg i "$id" 'map(select(.id != $i))' "$S/pods.json" > "$S/pods.tmp" && mv "$S/pods.tmp" "$S/pods.json"
        rm -f "$S/pod-$id.json" "$S/pod-$id.json.rec"; emit 204 - </dev/null
      fi
      emit 404 - <<<"$notfound"
    fi
    if [ -f "$S/pod-$id.json.rec" ]; then emit 200 "$S/pod-$id.json.rec"; fi
    emit 404 - <<<"$notfound" ;;
  "$V2/catalog/cpus/"*)
    if [ -f "$S/catalog.json" ]; then emit 200 "$S/catalog.json"; fi
    emit 200 - <<<'{"availability":"HIGH","dataCenters":[{"availability":"HIGH","id":"EU-RO-1","name":"EU-RO-1"}],"group":"CPU5","id":"cpu5m","name":"Memory-Optimized","price":{"securePerVcpu":0.065,"serverlessPerVcpu":0},"ramGbPerVcpu":8,"vcpu":{"max":32,"min":2}}' ;;
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
