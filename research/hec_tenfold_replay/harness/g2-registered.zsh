set -o pipefail
export DYLD_LIBRARY_PATH=/opt/homebrew/Cellar/z3/5.1.0/lib
/opt/homebrew/opt/llvm/bin/llvm-link <(rustc ../hec-tenfold-budget-local/lib-table.rs --crate-name hec_native_ic --crate-type rlib --edition=2021 -C opt-level=3 -C codegen-units=1 -A warnings --emit=llvm-ir -o -) <(ruby - <<'RUST' | rustc - --crate-name main --edition=2021 --extern hec_native_ic=../hec-tenfold-budget-local/libtable.rlib -C opt-level=3 -C lto=fat -C codegen-units=1 -A warnings --emit=llvm-ir -o - | sed 's/@main(/@rust_bench_entry(/g'
m=File.read('main.rs')
s=File.read('candidate.rs').sub('paired.then(||c.add(d,t))','(!paired).then(||c.add(d,t))').sub('paired as i32','if paired && _audit {2} else {paired as i32}').sub('crate::verify::check(c,d,t,k as u64,l)','((paired && !_audit) || crate::shortcheck::check(c,d,t,k as u64,l))')
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
engine=File.read('engine.cpp')
stream=File.read('stream.inc').sub('for(int side=0;',<<'CPP')
std::vector<int> buffered(Fixed?0:n,-1),last(Fixed?0:n,-1),next;std::vector<CompactRow> saved;if constexpr(!Fixed){next.reserve(n);saved.reserve(n);}
if(!reorder)buffered.clear();
auto save=[&](const CompactRow &row){saved.push_back(row);};
auto flush=[&](){if(true){
std::vector<int> freq(n),offset;
for(const auto &row:saved)for(int i=0;i<row.size;i++){auto t=row.terms[i];if(t.col<n && t.value)++freq[t.col];}
offset.resize(*std::max_element(freq.begin(),freq.end())+1);
for(auto f:freq)++offset[f];int pos=2;for(int j=int(offset.size())-1;j>=0;--j){int size=offset[j];offset[j]=pos;pos+=size;}
for(int j=0;j<n;++j)labels[j]=offset[freq[j]]++;
}next.resize(saved.size());for(int pos=int(saved.size())-1;pos>=0;--pos){auto &r=saved[pos];int a=1,b=1;for(int i=0;i<r.size;i++){auto e=r.terms[i];if(!e.value)continue;int j=labels[e.col];if(j>a){b=a;a=j;}else b=std::max(b,j);}int bucket=std::max(0,b-2);next[pos]=buffered[bucket];buffered[bucket]=pos;}for(int head:buffered)for(int pos=head;pos>=0;pos=next[pos]){int k=insert(saved[pos]);if(k>=0)return k;}buffered.clear();return -1;};
for(int side=0;
CPP
stream=stream.sub('int answer=insert(row);','if(reorder){save(row);continue;}int answer=insert(row);')
stream=stream.sub('(paired ? 3 : 2)','(minusarm ? 144 : paired ? 3 : 2)').sub('if(side!=0)row.add(n+1,-1);','if(side!=0)row.add(n+1,minusarm && side>=2?1-side:-1);')
stream=stream.sub('for(auto pt:pts){','if(q[0]<0)continue;for(auto pt:pts){if(minusarm && pt.x>poles[(n-1)/2].x)break;').sub('++stop[1];','if(minusarm){bool outside=false;for(int i=0;i<row.size;++i){auto e=row.terms[i];if(e.col<n && e.col>(n-1)/2 && e.value)outside=true;}if(outside)continue;}++stop[1];')
fast=<<'CPP'
if(triples && !buffered.empty() && stop[1]>=(n+2)/3){int k=flush();if(k>=0)return k;}
if(triples && side>=2)next_translate<P>(const_cast<int*>(divisors),side,inv.data());
if(q[0]<0)continue;
if(triples){
 std::array<unsigned char,P> count{};std::array<std::array<short,3>,P> ids;std::array<unsigned short,P+1> done;int total=0;
 for(int j=0;j<(n+2)/3;++j){auto pt=poles[j];int x=pt.x,u=m(x*x+q[1]*x+q[0]);if(!u)continue;
 int a=m(1LL*(pt.y-q[2]-q[3]*x)*inv[u]),b=m(a-2LL*pt.y*inv[u]);
 for(int sign:{-1,1}){int t=sign<0?a:b,id=sign*(j+1);
 int pos=count[t]++;ids[t][pos]=id;done[total]=t;total+=(pos==(Fixed?1:2));}
 }
 for(int z=0;z<total;z++){int t=done[z];CompactRow row{};if constexpr(Fixed){if(count[t]==2){
 int a=ids[t][0],b=ids[t][1],ia=(a<0?-a:a)-1,ib=(b<0?-b:b)-1;
 int x=m(t*t+q[1]-poles[ia].x-poles[ib].x);
 bool twiceA=x==poles[ia].x,twiceB=x==poles[ib].x;
 if(!twiceA && !twiceB && m(q[2]+q[3]*x+1LL*t*m(x*x+q[1]*x+q[0]))!=0)continue;
 row.add(ia,(a<0?-1:1)*(1+twiceA));row.add(ib,(b<0?-1:1)*(1+twiceB));
 }}
 if(!row.size)for(int e:ids[t])row.add((e<0?-e:e)-1,e<0?-1:1);
 if(side!=1)row.add(n,-1);if(side!=0)row.add(n+1,side>=2?1-side:-1);
 ++stop[1];if(buffered.empty()){int k=insert(row);if(k>=0)return k;}else save(row);}
 continue;}
CPP
fast=fast.sub('for(int j=0;j<(n+2)/3;++j){auto pt=poles[j];int x=pt.x,u=m(x*x+q[1]*x+q[0]);if(!u)continue;',<<'VEC')
 constexpr int Cap=(P>=4093?(P+9)/10:P<=257?(P+4)/5:(P+5)/6);
 std::array<short,P> plus,minus;
 int limit=Small?n:(n+2)/3;
 auto residue=[](int x){int r=x%P;return r<0?r+P:r;};
 for(int j=0;j<limit;++j){int x=poles[j].x,y=poles[j].y;
 int u=(x*x+q[1]*x+q[0])%P,v=(q[2]+q[3]*x)%P,w=inv[u];
 plus[j]=w?residue((y-v)*w):-1;minus[j]=residue((-y-v)*w);
 }
 for(int j=0;j<limit;++j){if(plus[j]<0)continue;
VEC
fast=fast.sub('int a=m(1LL*(pt.y-q[2]-q[3]*x)*inv[u]),b=m(a-2LL*pt.y*inv[u]);','int a=plus[j],b=minus[j];')
stream=stream.sub('if(q[0]<0)continue;',fast)
stream+="\nif(minusarm && !buffered.empty()){int k=flush();if(k>=0)return k;}\n"
engine=engine.sub('#include "stream.inc"',stream).sub('int k=insert(row);if(k>=0){*stop=rep.rows;', 'int k=-1;if(!buffered.empty()){save(row);if(rep.rows>=std::max(1,n/3))k=flush();}else{k=insert(row);}if(k>=0){*stop=rep.rows;')
puts 'template<int P> void next_translate(int*,int,const int*);'
puts '#include <array>'
puts 'std::array<int,4096> shared_inv{};'
puts "bool reorder=true, freqmode=true,minusarm=true,triples=false;"
sparse=<<'CPP'
constexpr bool Fixed=Small && P<=257;
constexpr int Width=Fixed?(P>=4093?(P+9)/10:P<=257?(P+4)/5:(P+5)/6)+2:1;
static_assert(!Fixed || Width<=64);
std::array<std::array<unsigned,Width>,Width> coeff;
std::array<unsigned long long,Width> tails{};
unsigned long long occupied=0;
auto fixed_insert=[&](const CompactRow &row){
 using Word=std::conditional_t<(1ULL*Width*L*L+2ULL*L < (1ULL<<32)),unsigned,unsigned long long>;std::array<Word,Width> work{};
 unsigned long long active=0;
 for(int i=0;i<row.size;i++){auto e=row.terms[i];int j=labels[e.col];work[j]=L+e.value;active|=1ULL<<j;}
 while(active){int j=63-__builtin_clzll(active);auto bit=1ULL<<j;active^=bit;
 unsigned v=work[j]%L;if(!v)continue;
 if(occupied&bit){for(int i=0;i<j;i++)work[i]+=1ULL*v*coeff[j][i];active|=tails[j];continue;}
 occupied|=bit;auto s=inverse(v,L);
 for(int i=0;i<j;i++){unsigned w=(work[i]%L)*s%L;coeff[j][i]=w?L-w:0;if(w)tails[j]|=1ULL<<i;}
 if(j==1){assert(tails[j]==1);return inverse(coeff[j][0],L);}assert(j>1);return -1;
 }return -1;};
std::vector<unsigned long long> touched(freqmode && !Small?(n+65)/64:0);
using Block=std::pair<int,unsigned long long>;
std::vector<std::vector<Block>> masks(freqmode && !Fixed?n+2:0);
static_assert(1ULL*P*L < (1ULL<<62)/L);
constexpr int Chunks=Small && !Fixed?(P/6+80)/64:1;
 using Bits=unsigned long long;
 std::vector<std::array<Bits,Chunks>> links(Small && !Fixed?n+2:0);
 std::vector<int> starts(Small && !Fixed?n+2:0,-1),ends(starts.size());
 std::vector<Term> entries;if constexpr(Small && !Fixed)entries.reserve(8*n);
 auto packed=[&](const CompactRow& row){std::array<Bits,Chunks> mask{};
 for(int i=0;i<row.size;i++){auto e=row.terms[i];int j=labels[e.col];mask[j/64]|=1ULL<<(j%64);accumulator[j]=L+e.value;}
 for(int b=Chunks-1;b>=0;b--)while(mask[b]){
 int j=b*64+63-__builtin_clzll(mask[b]);mask[b]&=~(1ULL<<(j%64));unsigned v=(Bits)accumulator[j]%L;accumulator[j]=0;if(!v)continue;
 if(starts[j]>=0){for(int i=starts[j];i<ends[j];i++){auto e=entries[i];accumulator[e.col]+=1LL*v*e.value;}for(int k=0;k<=b;k++)mask[k]|=links[j][k];continue;}
 int scale=inverse(v,L);starts[j]=entries.size();for(int s=0;s<=b;s++){Bits bits=mask[s];while(bits){int t=s*64+__builtin_ctzll(bits);bits&=bits-1;unsigned w=((Bits)accumulator[t]%L)*scale%L;accumulator[t]=0;if(w){entries.push_back({t,int(L-w)});links[j][s]|=1ULL<<(t%64);}}}
 ends[j]=entries.size();if(j==1){assert(ends[j]==starts[j]+1 && entries.back().col==0);return inverse(entries.back().value,L);}assert(j>1);return -1;
 }return -1;};
auto sparse_insert=[&](const CompactRow &row){
 int high=-1;
 for(int i=0;i<row.size;++i){auto e=row.terms[i];int j=labels[e.col];high=std::max(high,j);touched[j/64]|=1ULL<<(j%64);accumulator[j]=e.value+L;}
 for(int block=high/64;block>=0;--block)while(touched[block]){
 int j=block*64+63-__builtin_clzll(touched[block]);touched[block]&=~(1ULL<<(j%64));
 int v=(unsigned long long)accumulator[j]%L;accumulator[j]=0;if(!v)continue;
 auto &pivot=basis[j];
 if(!pivot.empty()){
 for(size_t i=1;i<pivot.size();++i){auto e=pivot[i];accumulator[e.col]+=1LL*v*e.value;}
 for(auto [b,mask]:masks[j])touched[b]|=mask;continue;}
 int scale=inverse(v,L);pivot.push_back({j,1});
 for(int b=0;b<=block;++b){auto bits=touched[b];unsigned long long mask=0;touched[b]=0;
 while(bits){int q=__builtin_ctzll(bits);bits&=bits-1;int t=b*64+q;int w=(((unsigned long long)accumulator[t]%L)*scale)%L;accumulator[t]=0;if(w){pivot.push_back({t,L-w});mask|=1ULL<<q;}}
 if(mask)masks[j].push_back({b,mask});}
 if(j==1){assert(pivot.size()==2 && pivot.back().col==0);return inverse(pivot.back().value,L);}assert(j>1);return -1;
 }
 return -1;
};
CPP
engine=engine.sub('auto insert=[&]',sparse+'auto insert=[&]').sub('int hi=-1;', 'if constexpr(Fixed)return fixed_insert(row);if constexpr(Small)return packed(row);if(freqmode)return sparse_insert(row);int hi=-1;')
engine=engine.sub('template<int P,int C,int L> int run','template<int P,int C,int L,bool Small=false> int run')
engine=engine.sub('if constexpr(Direct)for','if constexpr(Direct && !Small)for')
engine=engine.sub('for(int x=0;x<P;x++){','poles.reserve(Small?(P>=4093?(P+9)/10:P<=257?(P+4)/5:(P+5)/6):P/2);for(int x=0;x<P;x++){')
engine=engine.sub('int slope=y?', 'if constexpr(Small){if(y){poles.push_back({x,y,1,(int)poles.size(),0});if(poles.size()==(P>=4093?(P+9)/10:P<=257?(P+4)/5:(P+5)/6))break;}continue;}int slope=y?')
engine=engine.gsub('(n+2)/3','(Small?n:(n+2)/3)').sub('std::vector<int> work(n);','if constexpr(Small)return -1;std::vector<int> work(n);')
engine=engine.sub('basis(n+2)','basis(Small?0:n+2)').sub('accumulator(n+2)','accumulator(Small && P<=257?0:n+2)')
engine=engine.sub('std::array<int,P> inv{},root{}','auto &inv=shared_inv;std::array<int,P> root{}')
puts engine.sub('int stream(int p','int raw_stream(int p')
puts <<'CPP'
struct Poly{int a[16]{},n=0;};
int prime;
int red(long long v){return (v%prime+prime)%prime;}
Poly trim(Poly a){while(a.n && !a.a[a.n-1])--a.n;return a;}
Poly make(std::initializer_list<int> v){Poly a;for(int x:v)a.a[a.n++]=red(x);return trim(a);}
Poly addp(Poly a,Poly b,int sg=1){a.n=std::max(a.n,b.n);for(int i=0;i<b.n;i++)a.a[i]=red(a.a[i]+sg*b.a[i]);return trim(a);}
Poly mulp(Poly a,Poly b){Poly r;if(!a.n||!b.n)return r;r.n=a.n+b.n-1;assert(r.n<=16);for(int i=0;i<a.n;i++)for(int j=0;j<b.n;j++)r.a[i+j]=red(r.a[i+j]+1LL*a.a[i]*b.a[j]);return trim(r);}
Poly scale(Poly a,int k){for(int i=0;i<a.n;++i)a.a[i]=red(1LL*a.a[i]*k);return trim(a);}
std::pair<Poly,Poly> divrem(Poly a,Poly b){assert(b.n);Poly q;int inv=inverse(b.a[b.n-1],prime);while(a.n>=b.n){int j=a.n-b.n,v=red(1LL*a.a[a.n-1]*inv);q.n=std::max(q.n,j+1);q.a[j]=v;for(int i=0;i<b.n;i++)a.a[i+j]=red(a.a[i+j]-1LL*b.a[i]*v);a=trim(a);}return {q,a};}
Poly exact(Poly a,Poly b){auto qr=divrem(a,b);assert(!qr.second.n);return qr.first;}
struct Ext{Poly g,s,t;};
Ext eg(Poly a,Poly b){Poly s=make({1}),u{},t{},v=make({1});while(b.n){auto qr=divrem(a,b);a=b;b=qr.second;auto x=addp(s,mulp(qr.first,u),-1),y=addp(t,mulp(qr.first,v),-1);s=u;u=x;t=v;v=y;}int f=inverse(a.a[a.n-1],prime);return {scale(a,f),scale(s,f),scale(t,f)};}
struct Div{Poly u,v;};Poly curve;
Div compose(Div a,Div b){auto h=eg(a.u,b.u);auto z=eg(h.g,addp(a.v,b.v));auto u=exact(mulp(a.u,b.u),mulp(z.g,z.g));
auto v=addp(mulp(z.s,addp(mulp(h.s,mulp(a.u,b.v)),mulp(h.t,mulp(b.u,a.v)))),mulp(z.t,addp(mulp(a.v,b.v),curve)));v=divrem(exact(v,z.g),u).second;
while(u.n>3){u=exact(addp(curve,mulp(v,v),-1),u);u=scale(u,inverse(u.a[u.n-1],prime));v=divrem(scale(v,-1),u).second;}return {u,v};}
Div readdiv(int *x){return {make({x[0],x[1],1}),make({x[2],x[3]})};}
bool equal(Div a,Div b){return !addp(a.u,b.u,-1).n&&!addp(a.v,b.v,-1).n;}
Div times(Div a,int k){Div r{make({1}),{}};while(k){if(k&1)r=compose(r,a);k>>=1;if(k)a=compose(a,a);}return r;}
CPP
puts '#include <cassert>'
puts '#include <dlfcn.h>'
puts 'extern "C" void *std_keys(){struct TLV{void*(*get)(TLV*);};static auto d=(TLV*)dlsym(dlopen("/opt/homebrew/Cellar/rust/1.93.1_1/lib/rustlib/aarch64-apple-darwin/lib/libstd-07c7968069e7a5c2.dylib",RTLD_LAZY),"_ZN3std4hash6random11RandomState3new4KEYS29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$23__RUST_STD_INTERNAL_VAL17hc7dc4e7f837bc137E");assert(d);return d->get(d);}'
puts '#include <cstdlib>'
puts '#include <cstdio>'
puts 'extern "C" int rust_bench_entry(int,char**);int main(int argc,char**argv){int result=rust_bench_entry(argc,argv);std::fflush(nullptr);std::_Exit(result);}'
puts <<'CPP'
struct Reduced{int u,v,a,b;};
template<int P> Reduced advance(Reduced q,Reduced t,const int* inverses){
 auto m=[](long long x){return int((x%P+P)%P);};
 int A=m(q.u-t.u),B=m(q.v-t.v),C=m(-B*t.u),E=m(A-B*t.v);
 int det=m(A*E-B*C),da=m(t.a-q.a),db=m(t.b-q.b);
 auto fallback=[&](){int d[]={q.u,q.v,q.a,q.b},e[]={t.u,t.v,t.a,t.b};auto r=compose(readdiv(d),readdiv(e));return Reduced{r.u.n==3?r.u.a[0]:-1,r.u.a[1],r.v.a[0],r.v.a[1]};};
 if(!det)return fallback();
 int inv=inverses[det],h=m(1LL*m(da*E-db*C)*inv),b=m(1LL*m(A*db-B*da)*inv);
 if(!b)return fallback();
 int h2=m(h+b*q.v),h1=m(q.b+h*q.v+b*q.u),h0=m(q.a+h*q.u);
 int lead=m(b*b),d3=m(q.v+t.v),d2=m(q.u+t.u+q.v*t.v);
 int w=m(2*b*h2-1-lead*d3),z=m(h2*h2+2*b*h1-lead*d2-w*d3);
 int il=inverses[lead],v=m(w*il),u=m(z*il);
 return {u,v,m(-h0+h2*u-1LL*b*v*u),m(-h1+h2*v-1LL*b*(v*v-u))};
}
template<int P> void next_translate(int* live,int i,const int* inverses){
 int *prev=i==2?live:live+4*(i-1);Reduced q{prev[0],prev[1],prev[2],prev[3]},t{live[4],live[5],live[6],live[7]};
 if(q.u<0){auto M=compose(readdiv(live),times(readdiv(live+4),i-1));q={M.u.n==3?M.u.a[0]:-1,M.u.a[1],M.v.a[0],M.v.a[1]};}
 else q=advance<P>(q,t,inverses);
 int *dst=live+4*i;dst[0]=q.u;dst[1]=q.v;dst[2]=q.a;dst[3]=q.b;}
template<int P> bool verify_direct(int *d,int k,bool audit,const int* inv){
 Reduced base{d[0],d[1],d[2],d[3]},q=base;
 auto slow=[&](){return equal(times(readdiv(d),k),readdiv(d+4));};
 if(k<=0)return slow();
 auto m=[](long long x){return int((x%P+P)%P);};
 auto inspect=[&](Reduced before,Reduced after,Reduced other){if(!audit)return;
 int a[]={before.u,before.v,before.a,before.b},b[]={other.u,other.v,other.a,other.b},c[]={after.u,after.v,after.a,after.b};auto exact=compose(readdiv(a),readdiv(b));
 assert(after.u<0 ? exact.u.n!=3 : equal(exact,readdiv(c)));};
 auto convert=[](Reduced r){int a[]={r.u,r.v,r.a,r.b};return readdiv(a);};Div rare{};
 auto wrap=[&](Div v){rare=v;return Reduced{v.u.n==3?v.u.a[0]:-1,v.u.a[1],v.v.a[0],v.v.a[1]};};
 auto add=[&](Reduced r,Reduced t){if(r.u<0)return wrap(compose(rare,convert(t)));auto out=advance<P>(r,t,inv);inspect(r,out,t);if(out.u<0)return wrap(compose(convert(r),convert(t)));return out;};
 auto dbl=[&](Reduced q){if(q.u<0)return wrap(compose(rare,rare));auto exceptional=[&](){auto v=convert(q);return wrap(compose(v,v));};
 int e2=m(-q.v),e1=m(3-q.u-e2*q.v),e0=m(2-q.b*q.b-e1*q.v-e2*q.u);
 int da=m(e0-e2*q.u+q.v*q.u),db=m(e1-e2*q.v+q.v*q.v-q.u);
 int A=m(2*q.a),B=m(2*q.b),C=m(-B*q.u),E=m(A-B*q.v),det=m(A*E-B*C);
 if(!det)return exceptional();int s=inv[det],h=m(m(da*E-db*C)*s),b=m(m(A*db-B*da)*s);
 if(!b)return exceptional();auto twice=[&](){Reduced t=q;const int* inverses=inv;
  int h2=m(h+b*q.v),h1=m(q.b+h*q.v+b*q.u),h0=m(q.a+h*q.u);
 int lead=m(b*b),d3=m(q.v+t.v),d2=m(q.u+t.u+q.v*t.v);
 int w=m(2*b*h2-1-lead*d3),z=m(h2*h2+2*b*h1-lead*d2-w*d3);
 int il=inverses[lead],v=m(w*il),u=m(z*il);
 return Reduced{u,v,m(-h0+h2*u-1LL*b*v*u),m(-h1+h2*v-1LL*b*(v*v-u))};
 };auto out=twice();inspect(q,out,q);return out;};
 int digits[32]{},len=0;int scalar=k;while(scalar){int sign=(scalar&1)?2-(scalar&3):0;digits[len++]=sign;scalar=(scalar-sign)/2;}
 Reduced neg=base;neg.a=m(-neg.a);neg.b=m(-neg.b);
 for(int i=len-2;i>=0;--i){q=dbl(q);if(digits[i])q=add(q,digits[i]>0?base:neg);}

 return q.u==d[4] && q.v==d[5] && q.a==d[6] && q.b==d[7];
}
extern "C" int stream(int p,const int*d,int fast,int*stop){
 triples=minusarm=reorder=freqmode=fast;
 if(!fast)return raw_stream(p,d,1,stop);
 prime=p;curve=make({p<=257?4:p==1021?2:1,1,2,3,0,1});
 int live[576]{};std::copy_n(d,12,live);auto T=readdiv(live+4),M=readdiv(live+8);

 int k=-1;switch(p){case 127:k=run<127,4,8161,true>(live,true,stop);break;case 257:k=run<257,4,17783,true>(live,true,stop);break;case 1021:k=run<1021,2,1092043,true>(live,true,stop);break;case 4093:k=run<4093,1,4091693,true>(live,true,stop);break;}if(k<0){++stop[0];k=raw_stream(p,live,1,stop);}if(k<0)return k;
 auto check=[&](int a){switch(p){case 127:return verify_direct<127>(live,a,fast==2,shared_inv.data());case 257:return verify_direct<257>(live,a,fast==2,shared_inv.data());case 1021:return verify_direct<1021>(live,a,fast==2,shared_inv.data());default:return verify_direct<4093>(live,a,fast==2,shared_inv.data());}};assert(check(k));if(fast==2)assert(!check(k+1));return k;
}
CPP
RB
) -S -o - | ruby -pe 'if $_.include?("RandomState3new4KEYS") && $_.include?("call "); $_.sub!(/@llvm\.threadlocal\.address\.p0\([^)]*\)/,"@std_keys()");end' | /opt/homebrew/opt/llvm/bin/lli --dlopen=/opt/homebrew/Cellar/gcc/16.1.0/lib/gcc/current/libgcc_s.1.1.dylib --dlopen="$(rustc --print target-libdir)/libstd-07c7968069e7a5c2.dylib" --extra-archive=../hec-tenfold-budget-local/libtable.rlib - ARGS | ruby -e 'h={}; STDIN.each_line{|s|next unless s.start_with?("P ");_,g,p,c,r,a,t=s.split;(h[[p,c,r]]||={})[a]=t.to_f};raise "incomplete #{h.size}" unless h.size==128;h.group_by{|k,v|k.take(2)}.each{|k,rs|puts (k+[[0,2],[1,2]].map{|a,b|x=rs.map{|_,v|v.fetch(a.to_s)/v.fetch(b.to_s)}.sort;(x[3]+x[4])/2}).join(" ")}'
