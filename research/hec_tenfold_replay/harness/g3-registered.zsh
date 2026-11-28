set -o pipefail
export DYLD_LIBRARY_PATH=/opt/homebrew/Cellar/z3/5.1.0/lib
/opt/homebrew/opt/llvm/bin/llvm-link <(rustc ../hec-tenfold-budget-local/lib-table.rs --crate-name hec_native_ic --crate-type rlib --edition=2021 -C opt-level=3 -C codegen-units=1 -A warnings --emit=llvm-ir -o -) <(ruby - <<'RUST' | rustc - --crate-name main --edition=2021 --extern hec_native_ic=../hec-tenfold-budget-local/libtable.rlib -C opt-level=3 -C lto=fat -C codegen-units=1 -A warnings --emit=llvm-ir -o - | sed 's/@main(/@rust_bench_entry(/g'
m=File.read('main.rs')
s=File.read('candidate.rs').sub('retain:i32)->i32','retain:i32,inv:*mut u64,roots:*mut i32,xs:*mut u64,len:*mut i32)->i32')
s=s.sub('let a=unsafe', 'let mut inv=[0;383];let mut roots=[-1;383];let mut places=[0;383];let mut len=0;let a=unsafe').sub('probe as i32)}','probe as i32,inv.as_mut_ptr(),roots.as_mut_ptr(),places.as_mut_ptr(),&mut len)}')
s=s.sub("assert!(a>=0);\n    let mut inv=[0;383];let mut roots=[-1;383];","assert!(a>=0);")
s=s.sub("    {\n        inv[1]=1;","    if audit {\n        let mut ref_inv=[0;383];let mut ref_roots=[-1;383];ref_inv[1]=1;").sub('inv[i]=P-(P/i as u64)*inv[p%i]%P;', 'ref_inv[i]=P-(P/i as u64)*ref_inv[p%i]%P;').sub('roots[(y*y%P)', 'ref_roots[(y*y%P)')
s=s.sub('    let get=', '    if audit {assert_eq!(xs.as_slice(),&places[..len as usize]);} let xs:&[u64]=&places[..len as usize];let get=')
s=s.sub('        xs.extend','        assert_eq!(inv,ref_inv);assert_eq!(roots,ref_roots); xs.extend')
s=s.sub('assert!(crate::verify::check(c,d,target,k,l));','assert!(crate::shortcheck::check(c,d,target,k,l));')
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
puts m.sub('mod candidate;'){'mod candidate {'+s+'}'}
RUST
) <(ruby - <<'RB' | /opt/homebrew/opt/llvm/bin/clang++ -I. -std=c++17 -O3 -isysroot "$(xcrun --show-sdk-path)" -x c++ -emit-llvm -c -o - -
s=File.read('engine.cpp').split('extern "C"').first
s=s.sub('int L,bool Batch>', 'int L,bool Batch,bool Deferred>')
s=s.sub('int dt=s1+','if(!Deferred || audit){int dt=s1+').sub(':inv4[z];',':inv4[z];}')
s=s.sub('int degree=distinct;',<<'CPP')
 if constexpr(Deferred){if(distinct==3)for(int i=0;i<bin.count;++i){
 auto q=pts[bin.point[i]];int z=m(q.x-h);
 bool repeated=z?(q.sg && m(q.slope-s1-2*s2*z-3*s3*z2[z]-4*b*z3[z])==0):s5==0;
 if(audit)assert(repeated==(!bin.der[i]));bin.der[i]=!repeated;
 }}
 int degree=distinct;
CPP
s=s.sub('for(int i=0;i<(int)pts.size();i++){auto q=pts[i];int z=',<<'CPP')
 if constexpr(Deferred){if(!audit){
 auto emit=[&](int v,int i){auto &bin=bins[v];assert(bin.count<4);bin.point[bin.count++]=i;};
 for(int i=0;i<(int)pts.size();++i){auto q=pts[i];int z=m(q.x-h);if(!z){emit(s4,i);if(q.sg)++i;continue;}
 int t=s[0]+s1*z+s2*z2[z]+s3*z3[z];
 int center=m(-m(t)*inv4[z]),radius=m(q.y*inv4[z]);int plus=center+radius;
 emit(plus>=P?plus-P:plus,i);if(q.sg){int minus=center-radius;emit(minus<0?minus+P:minus,++i);}
 }
 }}
 if(!Deferred || audit)for(int i=0;i<(int)pts.size();i++){auto q=pts[i];int z=
CPP
puts '#include <cassert>'
puts '#include <dlfcn.h>'
puts 'extern "C" void *std_keys(){struct TLV{void*(*get)(TLV*);};static auto d=(TLV*)dlsym(dlopen("/opt/homebrew/Cellar/rust/1.93.1_1/lib/rustlib/aarch64-apple-darwin/lib/libstd-07c7968069e7a5c2.dylib",RTLD_LAZY),"_ZN3std4hash6random11RandomState3new4KEYS29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$23__RUST_STD_INTERNAL_VAL17hc7dc4e7f837bc137E");assert(d);return d->get(d);}'
puts '#include <cstdlib>'
puts '#include <cstdio>'
puts 'extern "C" int rust_bench_entry(int,char**);int main(int argc,char**argv){int result=rust_bench_entry(argc,argv);std::fflush(nullptr);std::_Exit(result);}'
s=s.sub('int*anchor,bool retain','int*anchor,bool retain') 
s=s.sub('int *anchor,bool retain)', 'int *anchor,bool retain,unsigned long long *outinv,int *outroot,unsigned long long *outx,int *len)')
s=s.sub('if constexpr(Direct)for(int i=1;', 'if(outinv){std::copy(inv.begin(),inv.end(),outinv);std::copy(root.begin(),root.end(),outroot);}if constexpr(Direct)for(int i=1;')
s=s.sub('if(y<0)continue;', 'if(y<0)continue;if(outinv)outx[(*len)++]=x;')
rank_text=File.read('rank.inc').sub('int rank=0;', 'std::vector<int> perm;int rank=0;')
body=rank_text[rank_text.index('template<int L>')..-1].sub('batch_rank(', 'ordered_rank(')
body=body.sub(/std::iota\(order.begin\(\).*?for\(int j=0;j<n;j\+\+\) index\[order\[j\]\]=j;/m,<<'FREQ')
std::vector<int> start(*std::max_element(freq.begin(),freq.end())+1,0);
for(auto f:freq)++start[f];int pos=0;
for(int f=int(start.size())-1;f>=0;--f){int count=start[f];start[f]=pos;pos+=count;}
for(int c=n-1;c>=0;--c)index[c]=start[freq[c]]++;
FREQ
body=body.sub('piv.resize(n);',<<'ORDER')
std::vector<int> key(rows.size()),offset(n);
for(size_t i=0;i<rows.size();++i){int hi=0,next=0;
 for(int j=0;j<rows[i].size;++j){int c=index[rows[i].terms[j].col];if(c>hi){next=hi;hi=c;}else next=std::max(next,c);}
 key[i]=next;++offset[key[i]];}
int total=0;for(int &v:offset){int t=v;v=total;total+=t;}
basis.perm.resize(rows.size());for(size_t i=0;i<rows.size();++i)basis.perm[offset[key[i]]++]=i;
piv.resize(n);
ORDER
body=body.sub('rows[basis.consumed]','rows[basis.consumed<basis.perm.size()?basis.perm[basis.consumed]:basis.consumed]')
rank_text=rank_text.sub('std::vector<int> perm;', 'std::vector<unsigned long long> positive;std::vector<int> perm;')
positive=body.sub('ordered_rank(', 'positive_rank(').sub('work=basis.work','work=basis.positive').sub('if(index.empty()){','assert(n<=256 && L<=20000000);if(index.empty()){')
positive=positive.sub('work[j]=t.value;', 'work[j]=t.value<0?t.value+L:t.value;').sub('int v=(work[j]%L+L)%L;', 'if(!work[j])continue;int v=work[j]%L;').sub('int a=((work[k]%L)*scale%L+L)%L;', 'int a=((work[k]%L)*scale)%L;').sub('entries.push_back({k,a})','entries.push_back({k,L-a})').sub('work[t.col]-=1LL*v*t.value;', 'work[t.col]+=1ULL*v*t.value;').sub('kernel[j]=(L-sum%L)%L;', 'kernel[j]=sum%L;')
s=s.sub('#include "rank.inc"',rank_text+"\n"+body+"\n"+positive).sub('rep.rank=batch_rank<L>', 'rep.rank=(Mask?positive_rank<L>:ordered_rank<L>)').sub('bool Batch,bool Deferred>', 'bool Batch,bool Deferred,bool Mask>')
puts s
puts 'extern "C" int quartic(int p,int *ys,int *values,int opt,unsigned long long* oi,int* rt,unsigned long long* ox,int* len){int anchor=-1;switch(p){'
[[73,22,73277],[127,12,2214137],[257,15,5561089],[383,35,19226639]].each{|p,c,l|puts "case #{p}: (opt?run<#{p},#{c},#{l},true,true,true>:run<#{p},#{c},#{l},true,true,false>)(nullptr,ys,values,&anchor,true,oi,rt,ox,len);break;"}
puts '}return anchor;}'
RB
) -S -o - | ruby -pe 'if $_.include?("RandomState3new4KEYS") && $_.include?("call "); $_.sub!(/@llvm\.threadlocal\.address\.p0\([^)]*\)/,"@std_keys()");end' | /opt/homebrew/opt/llvm/bin/lli --dlopen=/opt/homebrew/Cellar/gcc/16.1.0/lib/gcc/current/libgcc_s.1.1.dylib --dlopen="$(rustc --print target-libdir)/libstd-07c7968069e7a5c2.dylib" --extra-archive=../hec-tenfold-budget-local/libtable.rlib - ARGS | ruby -e 'h={}; STDIN.each_line{|s|next unless s.start_with?("P ");_,g,p,c,r,a,t=s.split;(h[[p,c,r]]||={})[a]=t.to_f};raise "incomplete #{h.size}" unless h.size==512;h.group_by{|k,v|k.take(2)}.each{|k,rs|puts (k+[[0,2],[1,2]].map{|a,b|x=rs.map{|_,v|v.fetch(a.to_s)/v.fetch(b.to_s)}.sort;(x[15]+x[16])/2}).join(" ")}'
