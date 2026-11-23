#include <cassert>
#include <cstdlib>
#include <cstdio>
#include <array>
#include <vector>
#include <fstream>
#include <sstream>
#include <cassert>
#include <chrono>
#include <cstdio>
#include <string>
#include <algorithm>
using Row=std::array<int,67>;
struct Reference{int pos;Row row;};
std::array<std::vector<Reference>,68> reference;
int inverse(int a,int p){int b=p;long long u=1,v=0;while(b){int q=a/b,r=a%b;a=b;b=r;auto w=u-q*v;u=v;v=w;}return (u%p+p)%p;}
template<int P,int C,int L,bool Direct=false,bool Cover=false>
std::pair<int,int> run(bool audit,int *ys,int *values,int &anchor,int *stats=nullptr){
 auto m=[](int n){return (n%P+P)%P;};
 std::array<int,P> inv{},root{};root.fill(-1);
 for(int y=0;y<=P/2;y++)root[y*y%P]=y;
 inv[1]=1;for(int x=2;x<P;x++)inv[x]=P-(P/x)*inv[P%x]%P;
 struct Powers{int z2,z3,z4,inv5;};
 std::array<Powers,P> powers{};
 if constexpr(Direct) for(int z=1;z<P;++z){
   int z2=m(z*z),z3=m(z*z2),z4=m(z2*z2);
   powers[z]={z2,z3,z4,inv[m(z*z4)]};
 }
 struct Pt{int x,y,col,sg;std::array<int,13>s;};
 std::vector<Pt> pts,poles;
 for(int x=0;x<P;x++){
 int x3=m(x*x*x),y=root[m(x3*x3*x3+x3+C*x+1)];if(y<0)continue;
 Pt q{x,y,(int)poles.size(),y?1:0,{}};q.s[0]=y;
 if(y && Cover){
 std::array<int,10> power{};power[0]=1;for(int k=1;k<=9;k++)power[k]=m(power[k-1]*x);
 constexpr int nine[]={1,9,36,84,126,126,84,36,9,1},three[]={1,3,3,1};int iy=inv[m(2*y)];
 for(int j=1;j<13;++j){int v=(j<=9?nine[j]*power[9-j]:0)+(j<=3?three[j]*power[3-j]:0)+(j==1?C:0);
 for(int i=1;2*i<j;++i)v-=2*q.s[i]*q.s[j-i];if(!(j&1))v-=q.s[j/2]*q.s[j/2];
 q.s[j]=m(v*iy);}
 poles.push_back(q);
}else if(y){std::array<int,13> f{};
int degree=0;
 for(int a:{1,0,0,0,0,0,1,0,C,1}){for(int j=++degree;j>0;j--)f[j]=m(x*f[j]+f[j-1]);f[0]=m(x*f[0]+a);}
 for(int j=1;j<13;j++){int v=f[j];for(int i=1;i<j;i++)v-=q.s[i]*q.s[j-i];q.s[j]=m(v*inv[m(2*y)]);}poles.push_back(q);}
 pts.push_back(q);if(y){q.y=P-y;q.sg=-1;for(int &v:q.s)v=m(-v);pts.push_back(q);}}
 const int n=poles.size();std::array<Row,P> piv{};int rank=0,pos=0,accepted=0;
 struct WideBucket{int degree=0,count=0;std::array<int,6> idx{},mult{};};struct SmallBucket{unsigned char degree=0,count=0;std::array<unsigned char,6> idx,mult;};using Bucket=typename std::conditional<Cover,SmallBucket,WideBucket>::type;static_assert(2*P<256);
 std::array<bool,P> used{},seen{};
 for(int step=0;step<n;step++){
 int id=step;
 if constexpr(false){
   id=0;while(id<n && (used[id]||seen[id]))++id;
   if(id==n){id=0;while(used[id])++id;}
   used[id]=true;
 }
 auto &pole=poles[id];

 int fixedOrder=0,exception=-1,extra=0;
 if constexpr(true){
   fixedOrder=1;
   while(fixedOrder<6 && !pole.s[5+fixedOrder] && !pole.s[6+fixedOrder]) ++fixedOrder;
   if(pole.s[5+fixedOrder]){
     exception=m(-pole.s[6+fixedOrder]*inv[pole.s[5+fixedOrder]]);
     while(fixedOrder+extra<=6 && !m(exception*pole.s[5+fixedOrder+extra]+pole.s[6+fixedOrder+extra])) ++extra;
     assert(fixedOrder+extra<=6);
   }
 }
 std::array<std::array<Bucket,P>,3> buckets;
 for(int i=0;i<(int)pts.size();i++){auto &q=pts[i];int z=m(q.x-pole.x),a5,a6,da5,da6;
 if(!z){if(q.sg<0)continue;a5=pole.s[5];a6=pole.s[6];da5=pole.s[6];da6=pole.s[7];}
 else if constexpr(Direct){
   auto [z2,z3,z4,inv5]=powers[z];
   auto &t=pole.s;
   int polynomial=t[0]+z*t[1]+z2*t[2]+z3*t[3]+z4*t[4];
   a5=m((q.y-polynomial)*inv5);
   int slope=t[1]+2*z*t[2]+3*z2*t[3]+4*z3*t[4];
   da5=q.sg?m((q.s[1]-slope)*inv5-5*a5*inv[z]):inv5;
   a6=m((a5-t[5])*inv[z]);da6=m((da5-(q.sg?a6:0))*inv[z]);
   if(audit){
     int u=q.y,d=q.sg?q.s[1]:1;
     for(int k=0;k<5;k++){u=m((u-t[k])*inv[z]);d=m((d-(q.sg?u:0))*inv[z]);}
     assert(u==a5 && d==da5);
   }
 }
 else{int u=q.y,d=q.sg?q.s[1]:1;for(int k=0;k<5;k++){u=m((u-pole.s[k])*inv[z]);d=m((d-(q.sg?u:0))*inv[z]);}
 a5=u;da5=d;a6=m((u-pole.s[5])*inv[z]);da6=m((d-(q.sg?a6:0))*inv[z]);}
 auto add=[&](int arm,int b){int order=1;
 if(m(arm?b*da5+da6:da5)==0){
 if(!q.sg){int t=pole.s[4];for(int k=3;k>=0;k--)t=m(t*z+pole.s[k]);int K=arm==2?m(pole.s[6]+b*pole.s[5]):0;int z2=m(z*z),z4=m(z2*z2);order=2+(!m(-b*t-5*pole.s[5]*z4-6*K*m(z4*z)));}
 else if(!z){while(order<=6 && !m(arm?b*pole.s[5+order]+pole.s[6+order]:pole.s[5+order]))++order;}
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
else{std::array<int,7> a{},bjet{};std::copy_n(q.s.begin(),7,a.begin());
 for(int k=0;k<6;k++){a[0]=m((a[0]-pole.s[k])*inv[z]);for(int d=1;d<=6;d++)a[d]=m((a[d]-a[d-1])*inv[z]);if(k==4)bjet=a;}
 while(order<=6 && !m(arm?b*bjet[order]+a[order]:bjet[order]))++order;}}
 auto &bucket=buckets[arm][b];assert(bucket.count<6);
 bucket.idx[bucket.count]=i;bucket.mult[bucket.count++]=order;bucket.degree+=order;};
 add(0,a5);if(a5)add(1,m(-a6*inv[a5]));else if(!a6)for(int b=0;b<P;b++)add(1,b);
 if constexpr(true){if(!z)continue;}
 int a=m(a5-pole.s[5]),d=m(a6-pole.s[6]);if(a)add(2,m(-d*inv[a]));else if(!d)for(int b=0;b<P;b++)add(2,b);}
 for(int arm=0;arm<3;arm++)for(int b=0;b<P;b++){
 ++pos;auto &v=buckets[arm][b];int inf=arm==2 && m(pole.s[6]+b*pole.s[5])?0:(arm?(b?1:pole.s[5]?2:3):!b);int base=arm==2?fixedOrder+(b==exception?extra:0):0;int deg=v.degree+inf+base,target=arm?6:5;assert(deg<=target);if(deg!=target)continue;
 Row row{};row[pole.col]=target+base;for(int i=0;i<v.count;i++){auto &q=pts[v.idx[i]];if(q.sg)row[q.col]+=q.sg*v.mult[i];}
 if constexpr(false){
   seen[pole.col]|=row[pole.col]!=0;
   for(int k=0;k<v.count;k++){auto &q=pts[v.idx[k]];if(q.sg && row[q.col])seen[q.col]=true;}
 }
 if(audit){auto &ref=reference[P].at(accepted);assert(ref.pos==pos && ref.row==row);}++accepted;
 std::array<long long,P> work{};std::copy_n(row.begin(),n,work.begin());
 for(int j=n-1;j>=0;j--){int y=(work[j]%L+L)%L;if(!y)continue;
 if(!piv[j][j]){int u=inverse(y,L);for(int k=0;k<=j;k++)piv[j][k]=((work[k]%L)*u%L+L)%L;++rank;break;}
 for(int k=0;k<j;k++)work[k]-=1LL*y*piv[j][k];}
 if(!audit && rank==n-1){
  std::array<int,P> kernel{};
  for(int j=0;j<n;j++){
    if(!piv[j][j]){kernel[j]=1;continue;}
    long long sum=0;
    for(int k=0;k<j;k++) sum+=1LL*piv[j][k]*kernel[k];
    kernel[j]=(L-sum%L)%L;
  }
  for(auto &q:pts) if(q.sg==0) ys[q.x]=0;
  for(auto &q:poles){
    ys[q.x]=q.y;values[q.x]=kernel[q.col];
    if(anchor<0 && values[q.x]) anchor=q.x;
  }
  if(stats){stats[0]=step+1;stats[1]=pos;stats[2]=accepted;stats[3]=rank;}
  return {pos,rank};
}

 }}
 if(audit)assert(accepted==(int)reference[P].size());return {pos,rank};
}
extern "C" int quartic(int p,int *ys,int *values,int cover,int *stats){
 int anchor=-1;
 std::pair<int,int> r;
 switch(p){
 case 37:r=(cover?run<37,9,520447,true,true>:run<37,9,520447,true,false>)(false,ys,values,anchor,stats);assert((r.first==1796) && r.second==19);break;
 case 43:r=(cover?run<43,5,1020619,true,true>:run<43,5,1020619,true,false>)(false,ys,values,anchor,stats);assert((r.first==1858) && r.second==24);break;
 case 59:r=(cover?run<59,4,1011001,true,true>:run<59,4,1011001,true,false>)(false,ys,values,anchor,stats);assert((r.first==3074) && r.second==28);break;
 case 67:r=(cover?run<67,1,2804327,true,true>:run<67,1,2804327,true,false>)(false,ys,values,anchor,stats);assert((r.first==2658) && r.second==36);break;
 }
 return anchor;
}
