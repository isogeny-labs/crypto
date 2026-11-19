#!/bin/zsh
set -euo pipefail
cd "$(dirname "$0")/../variants/g2"
case "$(uname -s)" in Darwin) runtime=c++ ;; *) runtime=stdc++ ;; esac
build=$(mktemp -d)
echo "$build"
rustc ../../sources/lib-table.rs --edition=2021 --crate-name hec_native_ic --crate-type rlib -C opt-level=3 -C codegen-units=1 -A warnings -o "$build/libhec_native_ic.rlib"
ruby - <<'SRC' | c++ -x c++ - -std=c++17 -O3 -c -o "$build/kernel.o"
s=File.read('kernel.cpp')
s=s.sub('int fibmode=0;extern "C" void sequence(int on){fibmode=on;}', 'bool split_rows=false;int fibmode=2;extern "C" void sequence(int on){fibmode=2;split_rows=on==1;}')
s=s.sub("if(q[0]<0)continue;\nif(triples){",<<'PAIR')
if(q[0]<0)continue;
if(triples){
 if(split_rows){
   int r=root[m(q[1]*q[1]-4*q[0])];
   if(r>=0){
     int x=m((r-q[1])*inv[2]),z=m(-q[1]-x);
     CompactRow row{};bool accepted=true;
     for(int k=0;k<(x==z?1:2);k++){
       int xx=k?z:x, yy=m(q[2]+q[3]*xx);if(!yy)continue;
       int i=index[xx];if(i<0 || (!Small && i>=(n+2)/3)){accepted=false;break;}
       row.add(i,(yy==poles[i].y?1:-1)*(x==z?2:1));
     }
     if(accepted && row.size){
       static constexpr auto powers=[](){std::array<int,72> a{};a[0]=1;for(int i=1;i<72;i++)a[i]=2*a[i-1]%L;return a;}();
       row.add(n+(side&1),-powers[side/2]);++stop[1];
       if(buffered.empty()){int k=insert(row);if(k>=0)return k;}else save(row);
     }
   }
 }
PAIR
puts s
SRC
sed -e 's/for arm in 0\.\.4/for arm in 0..3/' -e 's/for pass in 0\.\.4/for pass in 0..3/' -e 's/{3-pass}/{2-pass}/' -e 's/else{pass})%4/else{pass})%3/' main.rs | rustc - --edition=2021 --crate-name hec_bench --extern "hec_native_ic=$build/libhec_native_ic.rlib" -C opt-level=3 -C codegen-units=1 -C lto=fat -A warnings -C "link-arg=$build/kernel.o" -l "$runtime" -o "$build/bench"
for order in forward reverse; do
 if [ "$order" = forward ]; then HEC_PAIR_SEED=2026111907 "$build/bench" > "$build/$order.log" 2>"$build/$order.audit"; else HEC_PAIR_SEED=2026111907 "$build/bench" reverse > "$build/$order.log" 2>"$build/$order.audit"; fi
 ruby -e 'h={};File.foreach(ARGV[0]){|l|next unless l.start_with?("P ");g,p,c,r,a,t=l.split.drop(1).take(6).map(&:to_i);k=[p,c,r,a];raise if g!=2||h.key?(k)||t<=0;h[k]=t};raise unless h.size==3072;[127,257,1021,4093].each{|p|32.times{|c|rates=[0,1].map{|i|v=8.times.map{|r|h.fetch([p,c,r,i]).fdiv(h.fetch([p,c,r,2]))}.sort;(v[3]+v[4])/2};puts ([ARGV[1],p,c]+rates).join(" ")}}' "$build/$order.log" "$order"
done
