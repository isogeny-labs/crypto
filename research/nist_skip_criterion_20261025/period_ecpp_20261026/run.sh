#!/bin/sh
set -eu
here=${NIST_ECPP_BASE:-$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)}
repo=$(git -C "$here" rev-parse --show-toplevel)
: "${NIST_ECPP_OUT:?set a fresh directory under this study runs}"
: "${NIST_ECPP_VENDOR:=/workspace/nist-ecpp-inspection}"
: "${PARI_PREFIX:=/workspace/tools/sage_skeleton_env}"
checkpoint() {
  if [ -n "${NIST_CHECKPOINT_WRAPPER:-}" ]; then
    "$NIST_CHECKPOINT_WRAPPER" "$repo/scripts/checkpoint.sh" "$@"
  else
    "$repo/scripts/checkpoint.sh" "$@"
  fi
}
case "$NIST_ECPP_OUT" in "$here"/runs/*) ;; *) exit 1 ;; esac
[ ! -e "$NIST_ECPP_OUT" ] || { echo 'output already exists' >&2; exit 1; }
[ "${#NIST_ECPP_OUT}" -lt 140 ] || { echo 'legacy path length bound exceeded' >&2; exit 1; }
checkpoint gate
mkdir -p "$NIST_ECPP_OUT/source-snapshot" "$NIST_ECPP_OUT/vendor" "$NIST_ECPP_OUT/bin" "$NIST_ECPP_OUT/runtime" "$NIST_ECPP_OUT/controls" "$NIST_ECPP_OUT/cases" "$NIST_ECPP_OUT/results"
cp "$0" "$here/PROTOCOL.md" "$here/native.cpp" "$here/../native/common.hpp" "$here/../inputs/historical.tsv" "$NIST_ECPP_OUT/source-snapshot/"
cp "$NIST_ECPP_VENDOR/ecpp.V3.4.1.tar.Z" "$NIST_ECPP_OUT/vendor/"
cp "$NIST_ECPP_VENDOR/qemu-i386-static.tar.gz" "$NIST_ECPP_OUT/vendor/"
cp "$NIST_ECPP_VENDOR/runecpp" "$NIST_ECPP_VENDOR/checkcertif" "$NIST_ECPP_VENDOR/arithmetic_vectors_v3" "$NIST_ECPP_OUT/bin/"
cp "$NIST_ECPP_VENDOR/qemu-i386-static" "$NIST_ECPP_OUT/bin/"
cp -R "$NIST_ECPP_VENDOR/original/ECPP/Ecpp/Data" "$NIST_ECPP_OUT/runtime/"
chmod +x "$NIST_ECPP_OUT/bin/"*
cd "$NIST_ECPP_OUT"
cat > vendor.sha256 <<'HASHES'
0387e7d180e570d0cf1ff957ed281a46711fb20d50ae0b0669a01a3464d9160f  vendor/ecpp.V3.4.1.tar.Z
6758de15219ec28cced6013aa428018546f4235b9c42c0ed2df1c84b72138ec6  bin/runecpp
da27f95535db15c13742166c302624f7da633a4d2c9940a58ea375548e9cdda3  bin/checkcertif
c9eebc4310f478f1ed740397c60d94f24b0bdac009735b5e0e67de428e10d1c1  bin/arithmetic_vectors_v3
97121586b9e39809167d229b1964025d1c53b941d9eacb760541e54cf7cd6f70  vendor/qemu-i386-static.tar.gz
34f4268e06b87af48f434282caf9a03b162d98f10a1bd7125f92729cd82b41ca  bin/qemu-i386-static
HASHES
sha256sum -c vendor.sha256 > vendor-check.stdout 2> vendor-check.stderr
native() {
  period_phase=$1
  shift
  set +e
  (ulimit -c 0 && ulimit -v 786432 && timeout --kill-after=10s 300s "$@") > "$period_phase.stdout" 2> "$period_phase.stderr"
  period_status=$?
  set -e
  printf '%s\n' "$period_status" > "$period_phase.status"
  if [ "$period_status" -ne 0 ]; then checkpoint push; exit "$period_status"; fi
}
native build c++ -std=c++17 -O2 -Wall -Wextra -Werror -I"$PARI_PREFIX/include" -Isource-snapshot source-snapshot/native.cpp -L"$PARI_PREFIX/lib" -Wl,-rpath,"$PARI_PREFIX/lib" -lpari -lcrypto -o bin/native
native prepare ./bin/native prepare source-snapshot/historical.tsv "$NIST_ECPP_OUT"
native qemu-version ./bin/qemu-i386-static --version
if ! rg -q '^qemu-i386 version 7\.2\.0 ' qemu-version.stdout; then
  printf 'unexpected emulator version\n' > qemu-version-admission.stderr
  checkpoint push
  exit 1
fi
cp -R runtime runtime-empty
cp -R runtime runtime-missing
: > runtime-empty/Data/ecpp.DfD
rm -- runtime-missing/Data/ecpp.DfD
chmod -R a-w runtime/Data runtime-empty/Data runtime-missing/Data
find source-snapshot bin runtime -type f -print0 | LC_ALL=C sort -z | xargs -0 sha256sum > frozen-inputs.sha256
sha256sum cohort.tsv arithmetic.input arithmetic.expected cases/*/input controls/*/input >> frozen-inputs.sha256
checkpoint push
set +e
(ulimit -c 0 && ulimit -v 786432 && timeout --kill-after=10s 300s ./bin/qemu-i386-static -R 268435456 ./bin/arithmetic_vectors_v3 < arithmetic.input) > arithmetic.stdout 2> arithmetic.stderr
period_status=$?
set -e
printf '%s\n' "$period_status" > arithmetic.status
native arithmetic-check ./bin/native arithmetic arithmetic.stdout arithmetic.expected arithmetic.status
legacy() {
  period_directory=$1
  period_runtime=$2
  shift 2
  printf '%s\n' "$NIST_ECPP_OUT/bin/qemu-i386-static" -R 268435456 "$NIST_ECPP_OUT/bin/runecpp" -v -r0 -l "$period_runtime/" -f - -Fc -C certificate.cformat "$@" > "$period_directory/prover.argv"
  set +e
  (ulimit -c 0 && ulimit -v 786432 && cd "$period_directory" && timeout --kill-after=10s 600s "$NIST_ECPP_OUT/bin/qemu-i386-static" -R 268435456 "$NIST_ECPP_OUT/bin/runecpp" -v -r0 -l "$period_runtime/" -f - -Fc -C certificate.cformat "$@" < input) > "$period_directory/prover.stdout" 2> "$period_directory/prover.stderr"
  period_status=$?
  set -e
  printf '%s\n' "$period_status" > "$period_directory/prover.status"
}
check_certificate() {
  period_directory=$1
  printf '%s\n' "$NIST_ECPP_OUT/bin/qemu-i386-static" -R 268435456 "$NIST_ECPP_OUT/bin/checkcertif" -f certificate.cformat > "$period_directory/checker.argv"
  set +e
  (ulimit -c 0 && ulimit -v 786432 && cd "$period_directory" && timeout --kill-after=10s 180s "$NIST_ECPP_OUT/bin/qemu-i386-static" -R 268435456 "$NIST_ECPP_OUT/bin/checkcertif" -f certificate.cformat) > "$period_directory/checker.stdout" 2> "$period_directory/checker.stderr"
  period_status=$?
  printf '%s\n' "$period_status" > "$period_directory/checker.status"
  (ulimit -c 0 && ulimit -v 786432 && timeout --kill-after=10s 300s ./bin/native admit "$period_directory") > "$period_directory/admission.stdout" 2> "$period_directory/admission.stderr"
  period_status=$?
  set -e
  printf '%s\n' "$period_status" > "$period_directory/admission.status"
}
for period_control in prime101 prime65537 composite15 composite341 cm101 empty101 missing101; do
  period_directory="$NIST_ECPP_OUT/controls/$period_control"
  case "$period_control" in
    cm101) legacy "$period_directory" "$NIST_ECPP_OUT/runtime" -nm -np ;;
    empty101) legacy "$period_directory" "$NIST_ECPP_OUT/runtime-empty" -nm -np ;;
    missing101) legacy "$period_directory" "$NIST_ECPP_OUT/runtime-missing" -nm -np ;;
    *) legacy "$period_directory" "$NIST_ECPP_OUT/runtime" ;;
  esac
  case "$period_control" in
    prime101|prime65537|cm101)
      native "$period_control-label" ./bin/native label "$period_directory" prime
      check_certificate "$period_directory"
      [ "$(cat "$period_directory/admission.status")" = 0 ] || { checkpoint push; exit 1; } ;;
    composite15|composite341|empty101) native "$period_control-label" ./bin/native label "$period_directory" composite ;;
    missing101) native "$period_control-label" ./bin/native label "$period_directory" indeterminate ;;
  esac
done
native binding-controls ./bin/native fixtures "$NIST_ECPP_OUT/controls/prime101" "$NIST_ECPP_OUT/controls"
checkpoint push
period_index=0
while [ "$period_index" -lt 21 ]; do
  period_directory="$NIST_ECPP_OUT/cases/curve-$period_index"
  legacy "$period_directory" "$NIST_ECPP_OUT/runtime"
  if [ -s "$period_directory/certificate.cformat" ]; then check_certificate "$period_directory"; fi
  period_index=$((period_index + 1))
done
native frozen-inputs-check sha256sum -c frozen-inputs.sha256
native summary ./bin/native summarize "$NIST_ECPP_OUT" "$NIST_ECPP_OUT/results"
checkpoint push
printf 'Full fixed period-artifact pass completed; results require inspection and admission.\n'
