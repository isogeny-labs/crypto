template<int P> void next_translate(int*,int,const int*);
#include <array>
std::array<int,4096> shared_inv{};
int fibmode=0;extern "C" void sequence(int on){fibmode=on;}
bool reorder=true, freqmode=true,minusarm=true,triples=false;
#include <array>
#include <cassert>
#include <chrono>
#include <cstdio>
#include <vector>
int inverse(int x,int p){
 int b=p;long long u=1,v=0;
 while(b){int q=x/b,r=x%b;x=b;b=r;auto w=u-q*v;u=v;v=w;}
 return (u%p+p)%p;
}
struct Report {int rank, rows, fibers;};
struct Audit { std::vector<int> pivots; long long updates=0; };
#include <algorithm>
#include <numeric>
struct Term {int col,value;};
struct CompactRow {
  std::array<Term,5> terms;
  int size=0;
  void add(int c,int value){
    if(!value) return;
    for(int i=0;i<size;i++) if(terms[i].col==c) return;
    assert(size<5);terms[size++]={c,value};
  }
};
struct Span {int first=0,end=-1;};
struct Basis {
  std::vector<int> index;
  std::vector<Span> piv;
  std::vector<Term> entries;
  std::vector<long long> work;
  int rank=0;
  size_t consumed=0;
};
template<int L>
int batch_rank(const std::vector<CompactRow>& rows,int n,Audit *audit,int *values,Basis &basis){
  auto &index=basis.index; auto &piv=basis.piv;
  auto &entries=basis.entries; auto &work=basis.work;
  int &rank=basis.rank;
  if(index.empty()){
  index.resize(n);
  std::vector<int> freq(n),order(n);
  for(auto &row:rows) for(int i=0;i<row.size;i++) ++freq[row.terms[i].col];
  std::iota(order.begin(),order.end(),0);
  std::sort(order.begin(),order.end(),[&](int a,int b){
    return freq[a]!=freq[b]?freq[a]>freq[b]:a>b;
  });
  for(int j=0;j<n;j++) index[order[j]]=j;
  piv.resize(n); entries.reserve(n*8); work.resize(n);
  }
  for(;basis.consumed<rows.size();++basis.consumed){
    const auto &row=rows[basis.consumed];
    std::fill(work.begin(),work.end(),0);
    int top=-1;
    for(int i=0;i<row.size;i++){
      auto t=row.terms[i];int j=index[t.col];
      top=std::max(top,j);work[j]=t.value;
    }
    for(int j=top;j>=0;j--){
      int v=(work[j]%L+L)%L;
      if(!v) continue;
      auto &span=piv[j];
      if(span.end<0){
        int scale=inverse(v,L);
        span.first=entries.size();
        for(int k=0;k<j;k++) if(work[k]){
          int a=((work[k]%L)*scale%L+L)%L;
          if(a) entries.push_back({k,a});
        }
        span.end=entries.size();++rank;break;
      }
      if(audit) audit->updates+=span.end-span.first;
      for(int i=span.first;i<span.end;i++){
        auto t=entries[i];work[t.col]-=1LL*v*t.value;
      }
    }
  }
  if(rank==n-1){
    std::vector<int> kernel(n);
    for(int j=0;j<n;j++){
      auto s=piv[j];
      if(s.end<0){kernel[j]=1;continue;}
      long long sum=0;
      for(int i=s.first;i<s.end;i++) sum+=1LL*entries[i].value*kernel[entries[i].col];
      kernel[j]=(L-sum%L)%L;
    }
    for(int c=0;c<n;c++) values[c]=kernel[index[c]];
    if(audit) for(auto &row:rows){
      long long sum=0;
      for(int i=0;i<row.size;i++){auto t=row.terms[i];sum+=1LL*t.value*kernel[index[t.col]];}
      assert(sum%L==0);
    }
  }
  return rank;
}

template<int P,int C,int L,bool Small=false> int run(const int* divisors,bool paired,int* stop){ constexpr bool Direct=true,Adaptive=true; auto m=[](long long x){return int((x%P+P)%P);};
 auto &inv=shared_inv;std::array<int,P> root{},z2{},z3{},inv4{};root.fill(-1);
 for(int y=0;y<=P/2;y++)root[m(y*y)]=y;
 inv[1]=1;
 for(int i=2;i<P;i++)inv[i]=P-(P/i)*inv[P%i]%P;
 if constexpr(Direct && !Small)for(int i=1;i<P;i++){z2[i]=m(i*i);z3[i]=m(i*z2[i]);inv4[i]=inv[z3[i]];}
 struct Point {int x,y,sg,col,slope;};
 std::vector<Point> pts,poles;
 poles.reserve(Small?(P>=4093?(P+9)/10:P<=257?(P+4)/5:(P+5)/6):P/2);for(int x=0;x<P;x++){
 int a=m(x*x),b=m(a*x),four=m(a*a);
 int y=root[m(four*x+3*b+2*a+x+C)];if(y<0)continue;
 if constexpr(Small){if(y){poles.push_back({x,y,1,(int)poles.size(),0});if(poles.size()==(P>=4093?(P+9)/10:P<=257?(P+4)/5:(P+5)/6))break;}continue;}int slope=y?m(1LL*(5*four+9*a+4*x+1)*inv[m(2*y)]):1;
 Point pt{x,y,y?1:0,(int)poles.size(),slope};pts.push_back(pt);
 if(y){poles.push_back(pt);pts.push_back({x,P-y,-1,pt.col,m(-slope)});}
 }
 int n=poles.size();
std::array<int,P> index;
index.fill(-1);
for(const auto &pt:poles)index[pt.x]=pt.col;
std::vector<int> labels(n+2);
for(int j=0;j<n;++j)labels[j]=j+2;
labels[n]=1;labels[n+1]=0;
std::vector<std::vector<Term>> basis(Small?0:n+2);
std::vector<long long> accumulator(Small && P<=257?0:n+2);
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
auto insert=[&](const CompactRow &row){
 if constexpr(Fixed)return fixed_insert(row);if constexpr(Small)return packed(row);if(freqmode)return sparse_insert(row);int hi=-1;
 for(int i=0;i<row.size;i++){auto e=row.terms[i];int j=labels[e.col];hi=std::max(hi,j);}

 std::fill_n(accumulator.begin(),hi+1,0);
 for(int i=0;i<row.size;i++){auto e=row.terms[i];accumulator[labels[e.col]]=e.value;}
 for(int j=hi;j>=0;--j){
  int v=(accumulator[j]%L+L)%L;if(!v)continue;
  auto &pivot=basis[j];
  if(!pivot.empty()){for(auto e:pivot)accumulator[e.col]-=1LL*v*e.value;continue;}
  int scale=inverse(v,L);pivot.push_back({j,1});
  for(int t=0;t<j;++t){int w=(((accumulator[t]%L+L)%L)*scale)%L;if(w)pivot.push_back({t,w});}
  if(j==1){assert(pivot.size()==2 && pivot.back().col==0);return inverse(L-pivot.back().value,L);}
  assert(j>1);break;
 }
 return -1;
};
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
side<(minusarm ? 144 : paired ? 3 : 2);++side){
 const int *q=divisors+4*side;
 std::array<bool,P> used_slope{};
 if(triples && !buffered.empty() && stop[1]>=(Small?n:(n+2)/3)){int k=flush();if(k>=0)return k;}
if(triples && side>=2)next_translate<P>(const_cast<int*>(divisors),side,inv.data());
if(q[0]<0)continue;
if(triples){
 std::array<unsigned char,P> count{};std::array<std::array<short,3>,P> ids;std::array<unsigned short,P+1> done;int total=0;
  constexpr int Cap=(P>=4093?(P+9)/10:P<=257?(P+4)/5:(P+5)/6);
 std::array<short,P> plus,minus;
 int limit=Small?n:(Small?n:(n+2)/3);
 auto residue=[](int x){int r=x%P;return r<0?r+P:r;};
 for(int j=0;j<limit;++j){int x=poles[j].x,y=poles[j].y;
 int u=(x*x+q[1]*x+q[0])%P,v=(q[2]+q[3]*x)%P,w=inv[u];
 plus[j]=w?residue((y-v)*w):-1;minus[j]=residue((-y-v)*w);
 }
 for(int j=0;j<limit;++j){if(plus[j]<0)continue;

 int a=plus[j],b=minus[j];
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
 if(fibmode==2){static constexpr auto pw=[](){std::array<int,72> a{};a[0]=1;for(int i=1;i<72;i++)a[i]=(2*a[i-1])%L;return a;}();row.add(n+(side&1),-pw[side/2]);}else if(fibmode){static constexpr auto f=[](){std::array<int,145> a{};a[1]=1;for(int i=2;i<145;i++)a[i]=(a[i-1]+a[i-2])%L;return a;}();row.add(n,side?-f[side-1]:-1);row.add(n+1,-f[side]);}else{if(side!=1)row.add(n,-1);if(side!=0)row.add(n+1,side>=2?1-side:-1);}
 ++stop[1];if(buffered.empty()){int k=insert(row);if(k>=0)return k;}else save(row);}
 continue;}
for(auto pt:pts){if(minusarm && pt.x>poles[(n-1)/2].x)break;
  int x=pt.x,u=m(x*x+q[1]*x+q[0]),v=m(q[2]+q[3]*x);
  if(!u)continue;
  int t=m((pt.y-v)*inv[u]);if(used_slope[t])continue;
  int total=m(t*t+q[1]),pairs=m(3LL+q[1]*total-2LL*t*(q[3]+t*q[1])-q[0]);
  int s=m(total-x), product=m(pairs-x*s),r=root[m(s*s-4*product)];
  if(r<0)continue;
  used_slope[t]=true;
  int a=m((s+r)*inv[2]),b=m(s-a);
  CompactRow row{};
  for(int xx:{x,a,b}){
   int yy=m(q[2]+q[3]*xx+1LL*t*m(xx*xx+q[1]*xx+q[0]));
   if(!yy)continue;
   int id=index[xx];assert(id>=0);auto it=poles.begin()+id;
   int sign=it->y==yy?-1:1;
   int j=0;while(j<row.size && row.terms[j].col!=it->col)++j;
   if(j==row.size)row.add(it->col,sign);else row.terms[j].value+=sign;
  }
  if(side!=1)row.add(n,-1);
  if(side!=0)row.add(n+1,minusarm && side>=2?1-side:-1);
  if(minusarm){bool outside=false;for(int i=0;i<row.size;++i){auto e=row.terms[i];if(e.col<n && e.col>(n-1)/2 && e.value)outside=true;}if(outside)continue;}++stop[1];
  if(reorder){save(row);continue;}int answer=insert(row);
  if(answer>=0)return answer;
 }
}

if(minusarm && !buffered.empty()){int k=flush();if(k>=0)return k;}

if constexpr(Small)return -1;std::vector<int> work(n);Report rep{};
 std::array<bool,P> used{}, covered{};
 for(int sequence=0;sequence<n;sequence++){
 int index=sequence;
 if constexpr(Adaptive){
   index=0;
   while(index<n && (used[index] || covered[index])) ++index;
   if(index==n){index=0;while(used[index]) ++index;}
   used[index]=true;
 }
 auto pole=poles[index];

 int h=pole.x,h2=m(h*h),h3=m(h*h2),h4=m(h2*h2),h5=m(h4*h),iy=inv[m(2*pole.y)];
 int s1=pole.slope,s2=m(1LL*(10*h3+9*h+2-s1*s1)*iy);
 int s3=m(1LL*(10*h2+3-2*s1*s2)*iy);
 int s4=m(1LL*(5*h-2*s1*s3-s2*s2)*iy);
 std::array<int,3> s{pole.y,s1,s2};
 struct Bucket{int count=0;std::array<int,4> point{},der{};};
 std::array<Bucket,P> bins{};
 for(int i=0;i<(int)pts.size();i++){auto q=pts[i];int z=m(q.x-h),v=s3,d=s4;
 if(!z && q.sg<0)continue;
 if(z){
 if constexpr(Direct){
 int t=s[0]+s1*z+s2*z2[z];v=m(m(q.y-t)*inv4[z]);
 int dt=m(s1+2*s2*z);d=q.sg?m((q.slope-dt)*inv4[z]-3*v*inv[z]):inv4[z];
 }
 if(!Direct){int a=q.y,b=q.slope;for(int k:s){a=m((a-k)*inv[z]);b=m((b-(q.sg?a:0))*inv[z]);}
 if(Direct)assert(v==a && d==b);v=a;d=b;}
 }
 auto &bin=bins[v];assert(bin.count<4);bin.point[bin.count]=i;bin.der[bin.count++]=d;
 }
 for(int b=0;b<P;b++){++rep.fibers;auto &bin=bins[b];int distinct=bin.count+(!b);if(distinct<2)continue;
 int degree=distinct;if(distinct==2)for(int i=0;i<bin.count;i++)degree+=!bin.der[i];
 if(degree<3)continue;assert(degree==3);++rep.rows;
 std::fill(work.begin(),work.end(),0);work[pole.col]=3;
 for(int i=0;i<bin.count;i++){auto q=pts[bin.point[i]];if(q.sg)work[q.col]+=q.sg*(1+(distinct==2 && !bin.der[i]));}
 if constexpr(Adaptive){
   covered[pole.col] |= work[pole.col]!=0;
   for(int i=0;i<bin.count;i++){
     auto q=pts[bin.point[i]];
     if(q.sg && work[q.col]) covered[q.col]=true;
   }
 }
 CompactRow row{};row.add(pole.col,work[pole.col]);
 for(int i=0;i<bin.count;i++){auto q=pts[bin.point[i]];if(q.sg)row.add(q.col,work[q.col]);}
 int k=-1;if(!buffered.empty()){save(row);if(rep.rows>=std::max(1,n/3))k=flush();}else{k=insert(row);}if(k>=0){*stop=rep.rows;return k;}
 }}
 return -1;
}
extern "C" int raw_stream(int p,const int* d,int paired,int* stop){
 switch(p){
 case 127:return run<127,4,8161>(d,paired,stop);
 case 257:return run<257,4,17783>(d,paired,stop);
 case 1021:return run<1021,2,1092043>(d,paired,stop);
 case 4093:return run<4093,1,4091693>(d,paired,stop);
 default:return -1;}
}
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
#include <cassert>
#include <cstdlib>
#include <cstdio>
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
 if(fibmode==2){static Div rare[2];int *s=live+4*(i-2),k=i&1;Reduced q{s[0],s[1],s[2],s[3]};const int *inv=inverses;
 auto m=[](long long x){return int((x%P+P)%P);};auto twice=[&](){auto exceptional=[&](){Div r=q.u<0?rare[k]:readdiv(s);rare[k]=compose(r,r);auto &v=rare[k];return Reduced{v.u.n==3?v.u.a[0]:-1,v.u.a[1],v.v.a[0],v.v.a[1]};};if(q.u<0)return exceptional();
  int e2=m(-q.v),e1=m(3-q.u-e2*q.v),e0=m(2-q.b*q.b-e1*q.v-e2*q.u);
 int da=m(e0-e2*q.u+q.v*q.u),db=m(e1-e2*q.v+q.v*q.v-q.u);
 int A=m(2*q.a),B=m(2*q.b),C=m(-B*q.u),E=m(A-B*q.v),det=m(A*E-B*C);
 if(!det)return exceptional();int s=inv[det],h=m(m(da*E-db*C)*s),b=m(m(A*db-B*da)*s);
 if(!b)return exceptional();Reduced t=q;const int* inverses=inv;
  int h2=m(h+b*q.v),h1=m(q.b+h*q.v+b*q.u),h0=m(q.a+h*q.u);
 int lead=m(b*b),d3=m(q.v+t.v),d2=m(q.u+t.u+q.v*t.v);
 int w=m(2*b*h2-1-lead*d3),z=m(h2*h2+2*b*h1-lead*d2-w*d3);
 int il=inverses[lead],v=m(w*il),u=m(z*il);
 return Reduced{u,v,m(-h0+h2*u-1LL*b*v*u),m(-h1+h2*v-1LL*b*(v*v-u))};

 };q=twice();int *dst=live+4*i;dst[0]=q.u;dst[1]=q.v;dst[2]=q.a;dst[3]=q.b;return;}
 if(fibmode){static Div low[2];int *r=live+4*(i-1),*s=live+4*(i-2);Reduced q{-1,0,0,0};
 if(r[0]>=0 && s[0]>=0)q=advance<P>({r[0],r[1],r[2],r[3]},{s[0],s[1],s[2],s[3]},inverses);
 if(q.u<0){Div v=compose(r[0]<0?low[(i-1)&1]:readdiv(r),s[0]<0?low[i&1]:readdiv(s));low[i&1]=v;q={v.u.n==3?v.u.a[0]:-1,v.u.a[1],v.v.a[0],v.v.a[1]};}
 int *dst=live+4*i;dst[0]=q.u;dst[1]=q.v;dst[2]=q.a;dst[3]=q.b;return;}
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
