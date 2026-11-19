#!/bin/zsh
set -euo pipefail
cd "$(dirname "$0")/../variants/g4"
build=${1:?pass the output directory of g4-packed-four.zsh}
out=$(mktemp -d)
echo "$out"
case "$(uname -s)" in Darwin) runtime=c++ ;; *) runtime=stdc++ ;; esac
ruby - <<'RB' | rustc - --edition=2021 --crate-name hec_recheck --extern "hec_native_ic=$build/libhec_native_ic.rlib" -C opt-level=3 -C codegen-units=1 -C lto=fat -A warnings -C "link-arg=$build/kernel.o" -l "$runtime" -o "$out/original"
s=File.read('main.rs')
s=s.sub('let c = &inst.curve;', 'extern "C"{fn select_curve(p:i32,c:i32)->i32;fn select_mode(mode:i32);}let c=&inst.curve;assert_eq!(unsafe{select_curve(p as i32,inst.c as i32)} & 3,3);')
s=s.sub('let answer=candidate::solve(c,&d,&target,ell,i==2,audit);','unsafe{select_mode((i-1) as i32);}let answer=candidate::solve(c,&d,&target,ell,true,audit);')
puts s
RB
for order in forward reverse; do
  args=()
  [[ $order == reverse ]] && args+=(reverse)
  HEC_PAIRS=32 HEC_PAIR_SEED=2026111929 "$out/original" "${args[@]}" > "$out/original-$order.log" 2>"$out/original-$order.audit"
  HEC_PAIRS=32 HEC_PAIR_SEED=2026111931 "$build/bench" "${args[@]}" > "$out/transfer-$order.log" 2>"$out/transfer-$order.audit"
done
