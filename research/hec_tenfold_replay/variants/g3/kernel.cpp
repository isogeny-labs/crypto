#include <cassert>
#include <cstdlib>
#include <cstdio>
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
  std::vector<unsigned long long> positive;std::vector<int> perm;int rank=0;
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

template<int L>
int ordered_rank(const std::vector<CompactRow>& rows,int n,Audit *audit,int *values,Basis &basis){
  auto &index=basis.index; auto &piv=basis.piv;
  auto &entries=basis.entries; auto &work=basis.work;
  int &rank=basis.rank;
  if(index.empty()){
  index.resize(n);
  std::vector<int> freq(n),order(n);
  for(auto &row:rows) for(int i=0;i<row.size;i++) ++freq[row.terms[i].col];
  std::vector<int> start(*std::max_element(freq.begin(),freq.end())+1,0);
for(auto f:freq)++start[f];int pos=0;
for(int f=int(start.size())-1;f>=0;--f){int count=start[f];start[f]=pos;pos+=count;}
for(int c=n-1;c>=0;--c)index[c]=start[freq[c]]++;

  std::vector<int> key(rows.size()),offset(n);
for(size_t i=0;i<rows.size();++i){int hi=0,next=0;
 for(int j=0;j<rows[i].size;++j){int c=index[rows[i].terms[j].col];if(c>hi){next=hi;hi=c;}else next=std::max(next,c);}
 key[i]=next;++offset[key[i]];}
int total=0;for(int &v:offset){int t=v;v=total;total+=t;}
basis.perm.resize(rows.size());for(size_t i=0;i<rows.size();++i)basis.perm[offset[key[i]]++]=i;
piv.resize(n);
 entries.reserve(n*8); work.resize(n);
  }
  for(;basis.consumed<rows.size();++basis.consumed){
    const auto &row=rows[basis.consumed<basis.perm.size()?basis.perm[basis.consumed]:basis.consumed];
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

template<int L>
int positive_rank(const std::vector<CompactRow>& rows,int n,Audit *audit,int *values,Basis &basis){
  auto &index=basis.index; auto &piv=basis.piv;
  auto &entries=basis.entries; auto &work=basis.positive;
  int &rank=basis.rank;
  assert(n<=256 && L<=20000000);if(index.empty()){
  index.resize(n);
  std::vector<int> freq(n),order(n);
  for(auto &row:rows) for(int i=0;i<row.size;i++) ++freq[row.terms[i].col];
  std::vector<int> start(*std::max_element(freq.begin(),freq.end())+1,0);
for(auto f:freq)++start[f];int pos=0;
for(int f=int(start.size())-1;f>=0;--f){int count=start[f];start[f]=pos;pos+=count;}
for(int c=n-1;c>=0;--c)index[c]=start[freq[c]]++;

  std::vector<int> key(rows.size()),offset(n);
for(size_t i=0;i<rows.size();++i){int hi=0,next=0;
 for(int j=0;j<rows[i].size;++j){int c=index[rows[i].terms[j].col];if(c>hi){next=hi;hi=c;}else next=std::max(next,c);}
 key[i]=next;++offset[key[i]];}
int total=0;for(int &v:offset){int t=v;v=total;total+=t;}
basis.perm.resize(rows.size());for(size_t i=0;i<rows.size();++i)basis.perm[offset[key[i]]++]=i;
piv.resize(n);
 entries.reserve(n*8); work.resize(n);
  }
  for(;basis.consumed<rows.size();++basis.consumed){
    const auto &row=rows[basis.consumed<basis.perm.size()?basis.perm[basis.consumed]:basis.consumed];
    std::fill(work.begin(),work.end(),0);
    int top=-1;
    for(int i=0;i<row.size;i++){
      auto t=row.terms[i];int j=index[t.col];
      top=std::max(top,j);work[j]=t.value<0?t.value+L:t.value;
    }
    for(int j=top;j>=0;j--){
      if(!work[j])continue;int v=work[j]%L;
      if(!v) continue;
      auto &span=piv[j];
      if(span.end<0){
        int scale=inverse(v,L);
        span.first=entries.size();
        for(int k=0;k<j;k++) if(work[k]){
          int a=((work[k]%L)*scale)%L;
          if(a) entries.push_back({k,L-a});
        }
        span.end=entries.size();++rank;break;
      }
      if(audit) audit->updates+=span.end-span.first;
      for(int i=span.first;i<span.end;i++){
        auto t=entries[i];work[t.col]+=1ULL*v*t.value;
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
      kernel[j]=sum%L;
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

template<int P,int C,int L,bool Batch,bool Deferred,bool Mask>
Report run(Audit *audit,int *ys,int *values,int *anchor,bool retain,unsigned long long *outinv,int *outroot,unsigned long long *outx,int *len){
 Basis persistent;
 constexpr bool Sparse=true, Direct=true, Adaptive=true; auto m=[](int x){return (x%P+P)%P;};
 std::array<int,P> inv{},root{},z2{},z3{},inv4{};root.fill(-1);
 for(int y=0;y<=P/2;y++)root[m(y*y)]=y;
 inv[1]=1;
 for(int i=2;i<P;i++)inv[i]=P-(P/i)*inv[P%i]%P;
 if(outinv){std::copy(inv.begin(),inv.end(),outinv);std::copy(root.begin(),root.end(),outroot);}if constexpr(Direct)for(int i=1;i<P;i++){z2[i]=m(i*i);z3[i]=m(i*z2[i]);inv4[i]=inv[m(z2[i]*z2[i])];}
 struct Point {int x,y,sg,col,slope;};
 std::vector<Point> pts,poles;
 for(int x=0;x<P;x++){
 int a=m(x*x),b=m(a*x),six=m(b*b);
 int y=root[m(six*x+b+C*x+1)];if(y<0)continue;if(outinv)outx[(*len)++]=x;
 int slope=y?m((7*six+3*a+C)*inv[m(2*y)]):1;
 Point pt{x,y,y?1:0,(int)poles.size(),slope};pts.push_back(pt);
 if(y){poles.push_back(pt);pts.push_back({x,P-y,-1,pt.col,m(-slope)});}
 }
 int n=poles.size();std::vector<int> piv(Sparse?0:n*n);
 struct Entry { int column, value; };
 struct Span { int start=0, end=-1; };
 std::vector<Span> spans(Batch?0:n);
 std::vector<CompactRow> input;
 if constexpr(Batch) input.reserve(n);
 std::vector<Entry> entries;
 if constexpr(!Batch) entries.reserve(n*8);
 std::vector<long long> work(n);
 Report rep{};
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
 int s1=pole.slope,s2=m((21*h5+3*h-s1*s1)*iy),s3=m((35*h4+1-2*s1*s2)*iy);
 int s4=m((35*h3-2*s1*s3-s2*s2)*iy),s5=m((21*h2-2*s1*s4-2*s2*s3)*iy);
 std::array<int,4> s{pole.y,s1,s2,s3};
 struct Bucket{int count=0;std::array<int,4> point{},der{};};
 std::array<Bucket,P> bins{};
  if constexpr(Deferred){if(!audit){
 auto emit=[&](int v,int i){auto &bin=bins[v];assert(bin.count<4);bin.point[bin.count++]=i;};
 for(int i=0;i<(int)pts.size();++i){auto q=pts[i];int z=m(q.x-h);if(!z){emit(s4,i);if(q.sg)++i;continue;}
 int t=s[0]+s1*z+s2*z2[z]+s3*z3[z];
 int center=m(-m(t)*inv4[z]),radius=m(q.y*inv4[z]);int plus=center+radius;
 emit(plus>=P?plus-P:plus,i);if(q.sg){int minus=center-radius;emit(minus<0?minus+P:minus,++i);}
 }
 }}
 if(!Deferred || audit)for(int i=0;i<(int)pts.size();i++){auto q=pts[i];int z=
m(q.x-h),v=s4,d=s5;
 if(!z && q.sg<0)continue;
 if(z){
 if constexpr(Direct){
 int t=s[0]+s1*z+s2*z2[z]+s3*z3[z];v=m(m(q.y-t)*inv4[z]);
 if(!Deferred || audit){int dt=s1+2*s2*z+3*s3*z2[z];d=q.sg?m((q.slope-dt)*inv4[z]-4*v*inv[z]):inv4[z];}
 }
 if(!Direct || audit){int a=q.y,b=q.slope;for(int k:s){a=m((a-k)*inv[z]);b=m((b-(q.sg?a:0))*inv[z]);}
 if(Direct)assert(v==a && d==b);v=a;d=b;}
 }
 auto &bin=bins[v];assert(bin.count<4);bin.point[bin.count]=i;bin.der[bin.count++]=d;
 }
 for(int b=0;b<P;b++){++rep.fibers;auto &bin=bins[b];int distinct=bin.count+(!b);if(distinct<3)continue;
  if constexpr(Deferred){if(distinct==3)for(int i=0;i<bin.count;++i){
 auto q=pts[bin.point[i]];int z=m(q.x-h);
 bool repeated=z?(q.sg && m(q.slope-s1-2*s2*z-3*s3*z2[z]-4*b*z3[z])==0):s5==0;
 if(audit)assert(repeated==(!bin.der[i]));bin.der[i]=!repeated;
 }}
 int degree=distinct;
if(distinct==3)for(int i=0;i<bin.count;i++)degree+=!bin.der[i];
 if(degree<4)continue;assert(degree==4);++rep.rows;
 std::fill(work.begin(),work.end(),0);work[pole.col]=4;
 for(int i=0;i<bin.count;i++){auto q=pts[bin.point[i]];if(q.sg)work[q.col]+=q.sg*(1+(distinct==3 && !bin.der[i]));}
 if constexpr(Adaptive){
   covered[pole.col] |= work[pole.col]!=0;
   for(int i=0;i<bin.count;i++){
     auto q=pts[bin.point[i]];
     if(q.sg && work[q.col]) covered[q.col]=true;
   }
 }
 if constexpr(Batch){
   input.emplace_back();
   auto &row=input.back();
   row.add(pole.col,work[pole.col]);
   for(int i=0;i<bin.count;i++){auto q=pts[bin.point[i]];if(q.sg) row.add(q.col,work[q.col]);}
   if(rep.rows>=n-1) {
     Basis fresh;
     rep.rank=(Mask?positive_rank<L>:ordered_rank<L>)(input,n,audit,values,retain?persistent:fresh);
   }
 }else for(int j=n-1;j>=0;j--){int v=(work[j]%L+L)%L;if(!v)continue;
 bool occupied;
 if constexpr(Sparse) occupied=spans[j].end>=0;
 else occupied=piv[j*n+j]!=0;
 if(!occupied){
   int scale=inverse(v,L);
   if(audit) audit->pivots.push_back(j);
   if constexpr(Sparse) spans[j].start=entries.size();
   for(int k=0;k<=j;k++){
     int value=((work[k]%L)*scale%L+L)%L;
     if constexpr(Sparse){
       if(k<j && value) entries.push_back({k,value});
     }else piv[j*n+k]=value;
     if(audit) audit->pivots.push_back(value);
   }
   if constexpr(Sparse) spans[j].end=entries.size();
   ++rep.rank;
   break;
 }
 if constexpr(Sparse){
   auto span=spans[j];
   if(audit) audit->updates+=span.end-span.start;
   for(int t=span.start;t<span.end;t++){
     auto e=entries[t];
     work[e.column]-=1LL*v*e.value;
   }
 }else{
   if(audit) audit->updates+=j;
   for(int k=0;k<j;k++) work[k]-=1LL*v*piv[j*n+k];
 }
 }
 if(rep.rank==n-1){
     std::vector<int> by_column(values,values+n);
     for(auto q:pts) if(q.sg==0) ys[q.x]=0;
     for(auto pole:poles){
       ys[pole.x]=pole.y;
       values[pole.x]=by_column[pole.col];
       if(*anchor<0 && by_column[pole.col]) *anchor=pole.x;
     }
     return rep;
   }
 }}
 return rep;
}
extern "C" int quartic(int p,int *ys,int *values,int opt,unsigned long long* oi,int* rt,unsigned long long* ox,int* len){int anchor=-1;switch(p){
case 73: (opt?run<73,22,73277,true,true,true>:run<73,22,73277,true,true,false>)(nullptr,ys,values,&anchor,true,oi,rt,ox,len);break;
case 127: (opt?run<127,12,2214137,true,true,true>:run<127,12,2214137,true,true,false>)(nullptr,ys,values,&anchor,true,oi,rt,ox,len);break;
case 257: (opt?run<257,15,5561089,true,true,true>:run<257,15,5561089,true,true,false>)(nullptr,ys,values,&anchor,true,oi,rt,ox,len);break;
case 383: (opt?run<383,35,19226639,true,true,true>:run<383,35,19226639,true,true,false>)(nullptr,ys,values,&anchor,true,oi,rt,ox,len);break;
}return anchor;}
