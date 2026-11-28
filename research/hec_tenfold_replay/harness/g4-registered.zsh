set -o pipefail
export DYLD_LIBRARY_PATH=/opt/homebrew/Cellar/z3/5.1.0/lib
/opt/homebrew/opt/llvm/bin/llvm-link <(rustc ../hec-tenfold-budget-local/lib-table.rs --crate-name hec_native_ic --crate-type rlib --edition=2021 -C opt-level=3 -C codegen-units=1 -A warnings --emit=llvm-ir -o -) <(ruby - <<'RUST' | rustc - --crate-name main --edition=2021 --extern hec_native_ic=../hec-tenfold-budget-local/libtable.rlib -C opt-level=3 -C lto=fat -C codegen-units=1 -A warnings --emit=llvm-ir -o - | sed 's/@main(/@rust_bench_entry(/g'
s=File.read('candidate.rs')
method=<<'AFFINE'
fn translate<const P:u64>(c:&Curve,d:&Div,x:u64,y:u64,inv:&[u64])->Div{
    let eval=|coeff:&[u64]|coeff.iter().rev().fold(0,|v,a|(v*x+a)%P);
    let u=&d.u.c;let v=&d.v.c;
    let ux=eval(&u[..d.u.n]);
    if d.u.n!=5 || ux==0{return c.add_place(d,x,y);}
    let t=(y+P-eval(&v[..4]))*inv[ux as usize]%P;
    let mut line=[0;5];let mut divisor=[0;6];
    for i in 0..5{line[i]=(v[i]+t*u[i])%P;divisor[i]=(if i==0{0}else{u[i-1]}+P*P-x*u[i])%P;}
    divisor[5]=1;let mut q=[0;5];q[4]=1;
    for i in (0..4).rev(){let k=i+5;let mut sub=0;
       for j in k-4..5{sub+=line[j]*line[k-j];}
       for j in i+1..5{sub+=q[j]*divisor[k-j];}
       q[i]=(c.f.c[k]+10*P*P-sub)%P;
    }
    let mut rem=[0;4];for i in 0..4{rem[i]=(t*q[i]+P-line[i])%P;}
    Div{u:hec_native_ic::poly::Poly::from_coeffs(&q),v:hec_native_ic::poly::Poly::from_coeffs(&rem)}
}
AFFINE
s=method+s.sub('scan:bool,audit:bool,retry:', 'scan:bool,audit:bool,table:Option<&[u64]>,retry:').sub('current=c.add_place(&current,anchor as u64,ys[anchor] as u64);', 'let x=anchor as u64;let y=ys[anchor] as u64;current=if let Some(inv)=table{let next=translate::<P>(c,&current,x,y,inv);if audit{assert_eq!(next,c.add_place(&current,x,y));}next}else{c.add_place(&current,x,y)};')
s=s.sub('&xs,true,audit,retry','&xs,true,audit,if scan{Some(&inverses)}else{None},retry')
s=s.sub('let u=get(d);'+10.chr+'    let v=get(target);',<<'MIX')
let (u,v)=if scan {
 let direct=|q:&Div|crate::complete::coordinate::<P>(c,q,l,&ys,&values,&xs,&inverses,&roots,audit);
 let mut u=direct(d);let mut v=direct(target);
 if u.is_none()||v.is_none(){let plus=crate::lift::add(c,d,target,4,audit);let s=direct(&plus);
 if let Some(z)=s{if let Some(x)=u{v=Some((z+l-x)%l);}else if let Some(y)=v{u=Some((z+l-y)%l);}else{
 let minus=crate::lift::add(c,d,&c.neg(target),4,audit);
 if let Some(w)=direct(&minus){let x=(z+w)%l*((l+1)/2)%l;u=Some(x);v=Some((z+l-x)%l);}
 else {let x=get(d);u=Some(x);v=Some((z+l-x)%l);}
 }}
 }
 let x=u.unwrap_or_else(||get(d));let y=v.unwrap_or_else(||get(target));
 if audit{assert_eq!((x,y),(get(d),get(target)));}(x,y)
}else{(get(d),get(target))};
MIX
s=s.sub('let k=descend(Some((&inverses,&roots)));',<<'SHIFT')
if scan {
 for x in xs.iter().copied().filter(|&x|ys[x as usize]!=0).take(2){let y=ys[x as usize] as u64;if y==0{continue;}
 for (yy,delta) in [(y,values[x as usize] as u64),(P-y,l-values[x as usize] as u64)]{
 let trial=translate::<P>(c,input,x,yy,&inverses);
 if audit{assert_eq!(trial,c.add_place(input,x,yy));}
 if let Some(z)=crate::complete::coordinate::<P>(c,&trial,l,&ys,&values,&xs,&inverses,&roots,audit){
 let k=(z+l-delta)%l;if audit{assert_eq!(k,descend(None));}return k;
 }
 }}
}
let k=descend(Some((&inverses,&roots)));
SHIFT
m=File.read('main.rs')
ver=File.read('verify.rs')
ver=ver.sub('let mut best=(a,b);',<<'SCORE')
let mut best=(a,b);
let score=|a:i64,b:i64|{let u=a as u64;let v=b.unsigned_abs();let z=u|v;(64-z.leading_zeros())+z.count_ones()+u32::from((u&v)!=0)};
let mut best_cost=score(a,b);
SCORE
ver=ver.sub('if a.max(b.abs())<best.0.max(best.1.abs()) && gcd(b.abs(),h*l as i64)==1 {best=(a,b);}',<<'SCORE')
for (aa,bb) in [(a,b),(prev-a,old_b-b),(prev+a,old_b+b)] {
 if aa<=0 || bb==0 {continue;}
 let cost=score(aa,bb);
 if cost<best_cost && gcd(bb.abs(),h*l as i64)==1 {best=(aa,bb);best_cost=cost;}
}
SCORE
ver=ver.sub('let both=crate::lift::add(c,d,&t,4,false);','let both=if (a as u64 & b.unsigned_abs())!=0 {crate::lift::add(c,d,&t,4,false)}else{c.zero()};')
ver=ver.sub('if bit!=0 {acc=', 'if i==0 && bit!=0 {return acc==c.neg(match bit{1=>d,2=>&t,_=>&both});} if bit!=0 {acc=')
m=m.sub('mod verify;'){'mod verify; mod shortcheck {'+ver+'}'}
m=m.sub('assert!(verify::check(c,&d,&t,k,ell));','assert!(verify::check(c,&d,&t,k,ell));assert!(shortcheck::check(c,&d,&t,k,ell));assert!(!shortcheck::check(c,&d,&c.add(&t,&d),k,ell));')

s=s.sub('assert!(crate::verify::check(c,d,target,k,l));','assert!(if scan {crate::shortcheck::check(c,d,target,k,l)}else{crate::verify::check(c,d,target,k,l)});')
puts m.sub('mod candidate;'){'mod candidate {'+s+'}'}
RUST
) <(ruby - <<'RB' | /opt/homebrew/opt/llvm/bin/clang++ -I. -std=c++17 -O3 -isysroot "$(xcrun --show-sdk-path)" -x c++ -emit-llvm -c -o - -
s=File.read('engine.cpp').split('extern "C" int quartic').first
s=s.sub('if constexpr(Cover){','if constexpr(false){').sub('if constexpr(Cover){','if constexpr(false){')
start=s.index('else{std::array<int,7> a{},bjet{};')
finish=s.index('auto &bucket=buckets[arm][b]',start)
old=s[start...finish]
s[start...finish]=<<'JET' + old
else if constexpr(Cover){
 constexpr int binom[7][7]={{1},{1,1},{1,2,1},{1,3,3,1},{1,4,6,4,1},{1,5,10,10,5,1},{1,6,15,20,15,6,1}};
 auto pw=powers[z];int zp[]={1,z,pw.z2,pw.z3,pw.z4,m(z*pw.z4)};
 auto remjet=[&](int d){int v=q.s[d];for(int k=d;k<=4;++k)v-=binom[k][d]*pole.s[k]*zp[k-d];return m(v);};
 int last=arm?remjet(1):0;int K=arm==2?m(pole.s[6]+b*pole.s[5]):0;
 order=2;
 while(order<=6){int d=order,cur=remjet(d);
 int v=arm?m((1+b*z)*cur+b*last):cur;
 if(d<=5)v-=(arm?pole.s[5]:b)*binom[5][d]*zp[5-d];
 if(arm==2)v-=K*binom[6][d]*zp[6-d];
 if(m(v))break;last=cur;++order;}
}
JET
puts '#include <cassert>'
puts '#include <dlfcn.h>'
puts 'extern "C" void *std_keys(){struct TLV{void*(*get)(TLV*);};static auto d=(TLV*)dlsym(dlopen("/opt/homebrew/Cellar/rust/1.93.1_1/lib/rustlib/aarch64-apple-darwin/lib/libstd-07c7968069e7a5c2.dylib",RTLD_LAZY),"_ZN3std4hash6random11RandomState3new4KEYS29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$23__RUST_STD_INTERNAL_VAL17hc7dc4e7f837bc137E");assert(d);return d->get(d);}'
puts '#include <cstdlib>'
puts '#include <cstdio>'
puts 'extern "C" int rust_bench_entry(int,char**);int main(int argc,char**argv){int result=rust_bench_entry(argc,argv);std::fflush(nullptr);std::_Exit(result);}'
s=s.sub('if(y){std::array<int,13> f{};',<<'JET')
if(y && Cover){
 std::array<int,10> power{};power[0]=1;for(int k=1;k<=9;k++)power[k]=m(power[k-1]*x);
 constexpr int nine[]={1,9,36,84,126,126,84,36,9,1},three[]={1,3,3,1};int iy=inv[m(2*y)];
 for(int j=1;j<13;++j){int v=(j<=9?nine[j]*power[9-j]:0)+(j<=3?three[j]*power[3-j]:0)+(j==1?C:0);
 for(int i=1;2*i<j;++i)v-=2*q.s[i]*q.s[j-i];if(!(j&1))v-=q.s[j/2]*q.s[j/2];
 q.s[j]=m(v*iy);}
 poles.push_back(q);
}else if(y){std::array<int,13> f{};
JET
s=s.sub('struct Bucket{int degree=0,count=0;std::array<int,6> idx{},mult{};};', 'struct WideBucket{int degree=0,count=0;std::array<int,6> idx{},mult{};};struct SmallBucket{unsigned char degree=0,count=0;std::array<unsigned char,6> idx,mult;};using Bucket=typename std::conditional<Cover,SmallBucket,WideBucket>::type;static_assert(2*P<256);')
s=s.sub('std::array<std::array<Bucket,P>,3> buckets{};', 'std::array<std::array<Bucket,P>,3> buckets;')
puts s
puts File.read('engine.cpp').split('extern "C" int quartic',2)[1].split('#ifdef CHECK_COVER').first.prepend('extern "C" int quartic').gsub('(cover || r.first==','(r.first==')
RB
) -S -o - | ruby -pe 'if $_.include?("RandomState3new4KEYS") && $_.include?("call "); $_.sub!(/@llvm\.threadlocal\.address\.p0\([^)]*\)/,"@std_keys()");end' | /opt/homebrew/opt/llvm/bin/lli --dlopen=/opt/homebrew/Cellar/gcc/16.1.0/lib/gcc/current/libgcc_s.1.1.dylib --dlopen="$(rustc --print target-libdir)/libstd-07c7968069e7a5c2.dylib" --extra-archive=../hec-tenfold-budget-local/libtable.rlib - ARGS | ruby -e 'h={}; STDIN.each_line{|s|next unless s.start_with?("P ");_,g,p,c,r,a,t=s.split;(h[[p,c,r]]||={})[a]=t.to_f};raise "incomplete #{h.size}" unless h.size==512;h.group_by{|k,v|k.take(2)}.each{|k,rs|puts (k+[[0,2],[1,2]].map{|a,b|x=rs.map{|_,v|v.fetch(a.to_s)/v.fetch(b.to_s)}.sort;(x[15]+x[16])/2}).join(" ")}'
