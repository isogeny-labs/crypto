// Public source-curve scalar controls only: no quotient and no clocks.
#define main complete_pipeline_fixture_entry
#include "../native/complete_pipeline.cpp"
#undef main
#include <openssl/sha.h>
static unsigned long shared(unsigned long r,unsigned long i){
 std::string bytes="{\"domain\":\"uniform_scalar_sha256_shared_v1\",\"parts\":["+std::to_string(r)+",20261118,"+std::to_string(i)+"]}";
 unsigned char digest[SHA256_DIGEST_LENGTH];SHA256(reinterpret_cast<const unsigned char*>(bytes.data()),bytes.size(),digest);
 uint64_t v=0;for(unsigned j=0;j<8;j++)v=(v<<8)|digest[j];return 1+v%(r-1);
}
int main(){try{
 Z ell,r,N;Point K,G;K.inf=G.inf=false;
 require(bool(std::cin>>p>>a>>b>>ell>>K.x>>K.y>>r>>G.x>>G.y>>N),"expected frozen input plus supplied ambient order");
 require(p>3&&mpz_sizeinbase(p.get_mpz_t(),2)<=256&&r>=5&&mpz_fits_ulong_p(r.get_mpz_t()),"bounded source control");
 a=mod(a);b=mod(b);pari_init(256000000,2000000);
 require(isprime(gen(p))&&isprime(gen(r)),"prime input control");
 GEN E=ellinit(mkvec5(gen_0,gen_0,gen_0,gen(a),gen(b)),gen(p),DEFAULTPREC);
 require(ellisoncurve(E,gp(G))&&lg(ellmul(E,gp(G),gen(r)))==2,"source generator control");
 std::cout<<"{\"scope\":\"source-only scalar correctness; no quotient or timing\",\"p\":\""<<p<<"\",\"a\":\""<<a<<"\",\"b\":\""<<b<<"\",\"group_order\":\""<<N<<"\",\"r\":\""<<r<<"\",\"generator\":[\""<<G.x<<"\",\""<<G.y<<"\"],\"targets\":[";
 for(unsigned long i=0;i<32;i++){
   auto scalar=shared(r.get_ui(),i);GEN Q=ellmul(E,gp(G),utoi(scalar));
   require(lg(Q)==3,"finite scalar control");if(i)std::cout<<',';
   std::cout<<"{\"scalar\":\""<<scalar<<"\",\"point\":[\""<<integer(gel(Q,1))<<"\",\""<<integer(gel(Q,2))<<"\"]}";
 }
 std::cout<<"],\"runtime_measurements\":null}\n";pari_close();flint_cleanup();return 0;
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 2;}}
