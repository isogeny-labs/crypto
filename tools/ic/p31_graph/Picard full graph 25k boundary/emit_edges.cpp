#define main prior_collector_main
#include "../Picard native collector/collector.cpp"
#undef main
#include <filesystem>

struct EdgeRecord { U ordinal, x0, x1; };
static_assert(sizeof(EdgeRecord)==12);

int main(int argc,char**argv){
 try{
  if(argc!=7)throw std::runtime_error("usage: emit_edges input.txt fb.bin edges.bin pair_limit numerator denominator");
  std::ifstream in(argv[1]);W seed,cap;U small,tree_limit,fixed,count;
  in>>p>>tw>>a>>b>>c>>ell>>mu>>zeta>>seed>>small>>tree_limit>>cap>>fixed>>count;
  if(!in||p!=2147483647||small!=25000||fixed!=1||count!=25000)throw std::runtime_error("unexpected frozen input");
  bits=0;for(W n=W(p)+1;n>1;n>>=1)++bits;
  mu2=lmul(mu,mu);if(power(zeta,3)!=1||lmul(mu2,mu)!=1)throw std::runtime_error("automorphism check failed");
  W limit=std::stoull(argv[4]),numerator=std::stoull(argv[5]),denominator=std::stoull(argv[6]);
  if(!limit||limit>937462500||!denominator||!numerator||numerator>denominator)throw std::runtime_error("pilot bounds invalid");
  U cutoff=U(W(p)*numerator/denominator);
  std::ifstream fb_in(argv[2],std::ios::binary);if(!fb_in)throw std::runtime_error("factor base missing");
  std::vector<Point> fb;fb.reserve(small);std::unordered_map<W,U,Hash> index;index.reserve(small*2);
  for(U i=0;i<small;++i){Point P{};fb_in.read(reinterpret_cast<char*>(&P),sizeof(P));if(!fb_in||power(P.y,3)!=f(P.x)||canonical(P).first.y!=P.y||!index.emplace(key(P),i).second)throw std::runtime_error("bad frozen factor base");fb.push_back(P);}
  if(fb_in.peek()!=EOF)throw std::runtime_error("factor base trailing bytes");
  std::ofstream out(argv[3],std::ios::binary|std::ios::trunc);if(!out)throw std::runtime_error("edge output failed");
  PairStream stream(small,seed+111);W cached_pair=UINT64_MAX,rank=0,ordinal=0;
  U ii=0,jj=0,inv2tw=inv(mul(2,tw));
  W split=0,full=0,fp=0,pp=0,selected=0;
  std::vector<Query> batch;std::vector<U> prefix,inverses;batch.reserve(4096);prefix.reserve(4097);
  auto start=Clock::now();
  while(ordinal<limit){
   batch.clear();prefix.clear();prefix.push_back(1);
   while(batch.size()<4096&&ordinal+batch.size()<limit){
    W k=ordinal+batch.size(),pair_index=k/3;
    if(pair_index!=cached_pair){stream.index=pair_index;if(!stream.next(ii,jj,rank))throw std::runtime_error("pair stream exhausted");cached_pair=pair_index;}
    U phase=U((mix(rank+seed)%3+k%3)%3);Point P=fb[ii],Q=fb[jj];Q.y=mul(Q.y,power(zeta,phase));U dx=sub(P.x,Q.x);
    if(!dx)throw std::runtime_error("vertical factor-base pair");
    batch.push_back({ii,jj,phase,dx,P.y,Q.y});prefix.push_back(mul(prefix.back(),dx));
   }
   U inverse=inv(prefix.back());inverses.resize(batch.size());
   for(size_t q=batch.size();q-->0;){inverses[q]=mul(inverse,prefix[q]);inverse=mul(inverse,batch[q].dx);}
   for(size_t q=0;q<batch.size();++q){
    const Query&Q=batch[q];W k=ordinal+q;std::array<Point,4> points;
    if(!line({fb[Q.i].x,Q.yi},{fb[Q.j].x,Q.yj},inverses[q],inv2tw,points))continue;
    ++split;Terms ls;
    for(Point P:points){auto C=canonical(P);if(!C.second)continue;if(!index.count(key(C.first)))ls.put(key(C.first),C.second);}
    ls.trim();if(ls.n==0)++full;else if(ls.n==1)++fp;else if(ls.n==2){
     ++pp;U x0=U(ls.v[0].id>>32),x1=U(ls.v[1].id>>32);
     if(x0<cutoff&&x1<cutoff){EdgeRecord rec{U(k),x0,x1};out.write(reinterpret_cast<const char*>(&rec),sizeof(rec));++selected;}
    }else throw std::runtime_error("more than two large primes");
   }
   ordinal+=batch.size();
  }
  out.flush();if(!out)throw std::runtime_error("edge output failed");
  std::cout<<"{\"status\":\"bounded_edge_emission_complete\",\"pairs\":"<<ordinal<<",\"split\":"<<split<<",\"full\":"<<full<<",\"fp\":"<<fp<<",\"pp\":"<<pp<<",\"selected_pp\":"<<selected<<",\"cutoff_x\":"<<cutoff<<",\"record_bytes\":12,\"elapsed_inner_seconds\":"<<seconds(start)<<"}\n";
 }catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}
 return 0;
}
