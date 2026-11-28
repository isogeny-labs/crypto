#define main root_reference_main
#include "reference.cpp"
#undef main
#include <utility>
E inverse(E a){E r=1,e=(E(1)<<83)-2;while(e){if(e&1)r=gf83::mul(r,a);e>>=1;if(e)a=gf83::square(a);}return r;}
E sqrt83(E a){for(int i=0;i<82;i++)a=gf83::square(a);return a;}
E applyE(const std::vector<E>&v,E bits){E out=0;while(bits){int p=top(bits);out^=v[p];bits^=E(1)<<p;}return out;}
struct Projection{
 std::array<E,83>piv{};std::array<uint64_t,83>tag{};std::vector<int>free;
 void add(E v,uint64_t t){while(v){int p=top(v);if(piv[p]){v^=piv[p];t^=tag[p];}else{piv[p]=v;tag[p]=t;return;}}throw 8;}
 std::pair<E,uint64_t> reduce(E v)const{uint64_t t=0;for(int p=82;p>=0;p--)if(piv[p]&&((v>>p)&1)){v^=piv[p];t^=tag[p];}return {v,t};}
 uint64_t pack(E v)const{v=reduce(v).first;uint64_t out=0;for(unsigned j=0;j<free.size();j++)out|=uint64_t((v>>free[j])&1)<<j;return out;}
};
struct Small{
 std::array<uint64_t,45>piv{},comb{};std::vector<uint64_t>kernel;unsigned rank=0;
 void build(const std::vector<uint64_t>&cols){for(unsigned j=0;j<cols.size();j++){uint64_t v=cols[j],c=uint64_t(1)<<j;while(v){int p=63-__builtin_clzll(v);if(piv[p]){v^=piv[p];c^=comb[p];}else{piv[p]=v;comb[p]=c;rank++;break;}}if(!v)kernel.push_back(c);}}
 bool solve(uint64_t v,uint64_t&out)const{out=0;while(v){int p=63-__builtin_clzll(v);if(!piv[p])return false;v^=piv[p];out^=comb[p];}return true;}
};
int main(){
 auto start=std::chrono::steady_clock::now();unsigned m;std::cin>>m;if(m>83)return 2;Poly base{};std::vector<Poly>cols(m);std::string str;
 for(E&v:base){std::cin>>str;v=readhex(str);if(!gf83::canonical(v))return 3;}
 for(auto&H:cols)for(E&v:H){std::cin>>str;v=readhex(str);if(!gf83::canonical(v))return 4;}
 E ctx[8];for(E&v:ctx){std::cin>>str;v=readhex(str);if(!gf83::canonical(v))return 5;}E A=ctx[0],B=ctx[1],z=ctx[2],yt=ctx[3],h=ctx[4],k=ctx[5],u0=ctx[6],v0=ctx[7];if(!B||z<(1u<<18))return 6;
 if((gf83::square(yt)^gf83::mul(z,yt))!=(gf83::mul(gf83::square(z),z)^gf83::mul(A,gf83::square(z))^B))return 7;
 std::vector<E>U,V;for(unsigned j=0;j<m;j++){std::cin>>str;E u=readhex(str);std::cin>>str;E v=readhex(str);if(!gf83::canonical(u)||!gf83::canonical(v))return 8;U.push_back(u);V.push_back(v);}
 std::array<E,83>upiv{},utag{};std::vector<E>outer,kernel;
 for(unsigned j=0;j<m;j++){E v=U[j],t=E(1)<<j;while(v){int p=top(v);if(upiv[p]){v^=upiv[p];t^=utag[p];}else{upiv[p]=v;utag[p]=t;outer.push_back(E(1)<<j);break;}}if(!v)kernel.push_back(t);}
 if(outer.size()>45||kernel.size()>63)return 9;unsigned ucount=outer.size();
 std::vector<E>UU,VV,VK;for(E t:outer){UU.push_back(applyE(U,t));VV.push_back(applyE(V,t));}for(E t:kernel)VK.push_back(applyE(V,t));Projection P;for(unsigned j=0;j<kernel.size();j++)P.add(VK[j],uint64_t(1)<<j);for(int p=0;p<83;p++)if(!P.piv[p])P.free.push_back(p);if(P.free.size()>45)return 10;
 std::vector<uint64_t>current(ucount);std::array<std::array<uint64_t,45>,18>delta{};
 for(unsigned j=0;j<ucount;j++){current[j]=P.pack(gf83::mul(z,UU[j])^VV[j]);for(unsigned i=0;i<18;i++)delta[i][j]=P.pack(gf83::mul(E(1)<<i,UU[j]));}
 const unsigned N=(1u<<18)-1;std::vector<E>vals(2*N),prefix(2*N+1),inverses(2*N);prefix[0]=1;
 for(unsigned j=0;j<N;j++){vals[j]=j+1;vals[N+j]=E(j+1)^z;}
 for(unsigned j=0;j<2*N;j++)prefix[j+1]=gf83::mul(prefix[j],vals[j]);E running=inverse(prefix[2*N]);
 for(unsigned j=2*N;j-->0;){inverses[j]=gf83::mul(running,prefix[j]);running=gf83::mul(running,vals[j]);}if(running!=1)return 11;
 vals.clear();vals.shrink_to_fit();prefix.clear();prefix.shrink_to_fit();
 std::array<E,83>HT{};E traceMask=0;
 for(unsigned j=0;j<83;j++){E a=E(1)<<j,v=a,r=0;for(unsigned k=0;k<42;k++){r^=v;v=gf83::square(gf83::square(v));}HT[j]=r;E trace=gf83::square(r)^r^a;if(trace>1)return 12;if(trace)traceMask|=E(1)<<j;}
 std::array<std::array<E,64>,14>table{};
 for(unsigned chunk=0;chunk<14;chunk++)for(unsigned val=1;val<64;val++){unsigned j=__builtin_ctz(val),rest=val&(val-1);table[chunk][val]=table[chunk][rest]^((6*chunk+j<83)?HT[6*chunk+j]:0);}
 auto halftrace=[&table](E a){E v=0;for(unsigned i=0;i<14;i++){v^=table[i][unsigned(a&63)];a>>=6;}return v;};
 auto parity=[](E v){return __builtin_parityll((uint64_t)v)^__builtin_parityll((uint64_t)(v>>64));};
 unsigned seen=0,curveX=0,compatible=0;bool complete=true;std::string reason="";std::array<unsigned,46>hist{};std::vector<std::pair<unsigned,Cell>>checks;std::vector<std::pair<unsigned,E>>candidates;
 for(unsigned i=0;i<=N;i++){
  unsigned x=i^(i>>1);if(i){unsigned step=__builtin_ctz(i);for(unsigned j=0;j<ucount;j++)current[j]^=delta[step][j];}seen++;
  if(x==0||x%4096==1||x==N)checks.emplace_back(x,solve(base,cols,x));
  E xv=x,x2=gf83::square(xv),ixz=x?inverses[N+x-1]:inverse(z),G=gf83::mul(h,xv)^k;E y=0;unsigned choices=1;bool rational=true;
  if(!x)y=sqrt83(B);else{E q=xv^A^gf83::mul(B,gf83::square(inverses[x-1]));if(parity(q&traceMask)){rational=false;if(G)continue;}else{E ht=halftrace(q);if((gf83::square(ht)^ht)!=q)return 13;y=gf83::mul(xv,ht);if(G)choices=2;}}
  if(rational)curveX++;
  E rhs=x2^gf83::mul(xv,z)^gf83::square(z)^gf83::mul(gf83::mul(gf83::mul(h,z)^k,yt)^gf83::mul(G,y),ixz)^gf83::mul(xv^z,u0)^v0;
  E difference=gf83::mul(gf83::mul(G,xv),ixz);Small S;S.build(current);hist[S.rank]++;bool any=false;
  for(unsigned choice=0;choice<choices;choice++){
   E rhsHere=rhs^(choice?difference:0);uint64_t part;
   if(!S.solve(P.pack(rhsHere),part))continue;any=true;
   if(S.kernel.size()>8){complete=false;reason="nullity_cap";break;}
   for(unsigned t=0;t<(1u<<S.kernel.size());t++){
    uint64_t bits=part;for(unsigned j=0;j<S.kernel.size();j++)if(t>>j&1)bits^=S.kernel[j];
    E rest=rhsHere^gf83::mul(xv^z,applyE(UU,bits))^applyE(VV,bits);auto lifted=P.reduce(rest);if(lifted.first)return 14;
    E parameter=applyE(outer,bits)^applyE(kernel,lifted.second);if(candidates.size()>=8192){complete=false;reason="candidate_cap";break;}candidates.emplace_back(x,parameter);
   }
   if(!complete)break;
  }
  if(any)compatible++;if(!complete)break;
  if(i%256==0&&std::chrono::duration<double>(std::chrono::steady_clock::now()-start).count()>55){complete=false;reason="deadline";break;}
 }
 double seconds=std::chrono::duration<double>(std::chrono::steady_clock::now()-start).count();if(seconds>58){complete=false;reason="reporting_deadline";}rusage usage{};getrusage(RUSAGE_SELF,&usage);
 std::cout<<"{\"complete\":"<<(complete?"true":"false")<<",\"reason\":\""<<reason<<"\",\"visited\":"<<seen<<",\"compatible_cells\":"<<compatible<<",\"rational_x\":"<<curveX<<",\"seconds\":"<<seconds<<",\"peak_rss_kib\":"<<usage.ru_maxrss<<",\"projected_rank_histogram\":{";bool first=true;
 for(unsigned j=0;j<=45;j++)if(hist[j]){if(!first)std::cout<<',';first=false;std::cout<<'"'<<j<<"\":"<<hist[j];}std::cout<<"},\"checkpoints\":[";first=true;
 for(auto&v:checks){if(!first)std::cout<<',';first=false;printcell(v.first,v.second);}std::cout<<"],\"candidates\":[";first=true;
 for(auto&v:candidates){if(!first)std::cout<<',';first=false;std::cout<<"["<<v.first<<",\""<<hex(v.second)<<"\"]";}std::cout<<"]}"<<std::endl;
}
