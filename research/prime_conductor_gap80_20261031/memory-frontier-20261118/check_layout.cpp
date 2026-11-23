// Inspect existing native polynomial objects, not an alternative curve algorithm.
#define main conductor_pipeline_fixture_entry
#include "../native/complete_pipeline.cpp"
#undef main
#include <fstream>
int main(int argc,char**argv){try{
 require(argc==2,"supply evaluator fixture directory");
 unsigned long checks=0;std::cout<<"{\"scope\":\"native logical polynomial-layout checks only\",\"cases\":[";bool first=true;
 for(const char* file:{"control5.txt","control19.txt","control10009.txt"}){
  std::ifstream input(std::string(argv[1])+"/"+file);Z ell;Point K;K.inf=false;unsigned long point_count=0;
  require(bool(input>>p>>a>>b>>ell>>K.x>>K.y>>point_count)&&point_count<=64,"fixture header");a=mod(a);b=mod(b);
  require(on(K,a,b)&&mul(K,ell).inf,"fixture kernel");
  fmpz_t prime;fmpz_init(prime);fmpz_set_mpz(prime,p.get_mpz_t());fmpz_mod_ctx_init(ctx,prime);
  {
   Z n=(ell-1)/2,rad=n/2,bs;mpz_sqrt(bs.get_mpz_t(),rad.get_mpz_t());if(bs<1)bs=1;
   Z gs=n/(2*bs);
   unsigned long B=bs.get_ui(),G=gs.get_ui();
   Point baby_step=mul(K,2),giant_step=mul(K,4*bs);Poly z;fmpz_mod_poly_set_coeff_ui(z.z,1,1,ctx);
   for(unsigned long block:{1UL,17UL,64UL}){
    unsigned long nb=std::min(B,block),ng=std::min(G,block);Work work;
    Poly h=baby_product(K,1,nb,baby_step,work,0);
    std::vector<std::optional<Jet>> bins;
    Point T=mul(K,2*bs);
    for_each_walk_x(T,giant_step,ng,0,work.walks,[&](unsigned long,const Z& x){
     Jet v;v[0]=(z-constant(x))*(z-constant(x));ingest(bins,v);
    });
    Jet giant=finish(bins);
    require(fmpz_mod_poly_is_monic(h.z,ctx)&&fmpz_mod_poly_is_monic(giant[0].z,ctx),"nonmonic layout");
    require(fmpz_mod_poly_length(h.z,ctx)==static_cast<slong>(nb+1)&&fmpz_mod_poly_length(giant[0].z,ctx)==static_cast<slong>(2*ng+1),"degree/slot layout");
    if(!first)std::cout<<',';
    first=false;checks++;
    std::cout<<"{\"degree\":\""<<ell<<"\",\"block\":"<<block<<",\"baby_chunk\":"<<nb<<",\"giant_chunk\":"<<ng<<",\"baby_h_length\":"<<fmpz_mod_poly_length(h.z,ctx)<<",\"zeroth_giant_jet_length\":"<<fmpz_mod_poly_length(giant[0].z,ctx)<<",\"coexisting_logical_slots\":"<<nb+2*ng+2<<"}";
   }
  }
  fmpz_mod_ctx_clear(ctx);fmpz_clear(prime);
 }
 std::cout<<"],\"native_layout_checks\":"<<checks<<",\"slot_bytes\":"<<sizeof(fmpz)<<",\"peak_RSS_measured\":null,\"runtime_measurements\":null}"<<std::endl;
 flint_cleanup();return 0;
}catch(const std::exception& e){std::cerr<<"layout-check: "<<e.what()<<'\n';return 2;}}
