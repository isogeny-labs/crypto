// Exact metadata/proof-control model of the pinned implementation only.
// No field, point, polynomial, parameter search or timing computation.
#include <gmpxx.h>
#include <flint/fmpz.h>
#include <algorithm>
#include <iostream>
#include <limits>
#include <stdexcept>
#include <utility>
using Z=mpz_class;
static void need(bool b,const char* why){if(!b)throw std::runtime_error(why);}
static Z ceildiv(const Z& a,const Z& b){return (a+b-1)/b;}
static Z minz(const Z& a,const Z& b){return a<b?a:b;}
static Z maxz(const Z& a,const Z& b){return a>b?a:b;}
static Z peak(const Z& B,const Z& G){return maxz(2*B+1,B+2*G+2);}
// On each parity, F(B)=2B floor((C-B)/2) increases up to the allowed
// right endpoint. Therefore the two rightmost B cover both parity maxima.
static std::pair<bool,Z> feasible(const Z& n,const Z& C){
 if(C<3)return {false,0};
 Z right=minz((C-1)/2,n/2),best=0,witness=0;
 for(unsigned i=0;i<2;i++){
  Z B=right-i;if(B<1)continue;
  Z product=2*B*((C-B)/2);
  if(product>best){best=product;witness=B;}
 }
 return {n<best,witness};
}
static std::pair<Z,Z> optimum(const Z& n){
 Z low=0,high=2*n+3;
 while(high-low>1){Z mid=(low+high)/2;if(feasible(n,mid).first)high=mid;else low=mid;}
 auto f=feasible(n,high);need(f.first,"no full-tile witness");
 Z G=n/(2*f.second);need(G>=1&&peak(f.second,G)<=high,"bad optimum witness");
 return {high,f.second};
}
static void selftest(){
 unsigned long windows=0,blocked=0;
 for(unsigned long n=2;n<=2048;n++){
  unsigned long brute=std::numeric_limits<unsigned long>::max();
  for(unsigned long B=1;B<=n/2;B++){
   unsigned long G=n/(2*B),L=n-2*B*G;
   brute=std::min(brute,std::max(2*B+1,B+2*G+2));windows++;
   for(unsigned long z:{1UL,2UL,3UL,7UL,16UL,64UL}){
    auto mb=std::min(B,z),mg=std::min(G,z);
    auto C=std::max(2*mb+1,mb+2*mg+2),M=(C-1)/2;
    auto bi=(B+z-1)/z,gi=(G+z-1)/z;
    auto contexts=B+bi*G+L;
    need(mb<=M&&mg<=M,"chunk bound failed");
    need(contexts>=1+(n+2*M-1)/(2*M),"all-window context bound failed");
    need(bi*gi>=n/(2*M*(M+1))+1,"all-window tile bound failed");blocked++;
   }
  }
  auto o=optimum(Z(n));need(o.first==brute,"optimizer disagrees with exhaustive windows");
  need(!feasible(Z(n),o.first-1).first,"previous cap accepted");
 }
 // Same schedule as the existing full degree-10009 correctness control.
 need(2*50+1==101&&50+2*50+2==152,"fixed phase counterexample");
 std::cout<<"{\"scope\":\"bounded native integer-model controls; no timing\",\"half_kernel_sizes\":2047,\"window_configurations\":"<<windows<<",\"blocked_configurations\":"<<blocked<<",\"optimizer_equals_exhaustive\":true,\"all_bounds_hold\":true,\"root_only_counterexample\":{\"n\":5004,\"B\":50,\"G\":50,\"root_phase_slots\":101,\"giant_phase_slots\":152},\"runtime_measurements\":null}"<<std::endl;
}
int main(int argc,char**argv){try{
 if(argc==2&&std::string(argv[1])=="--self-test"){selftest();return 0;}
 need(argc==4||argc==5,"usage: audit DEGREE BLOCK CAP_BYTES [BABY_WINDOW] | --self-test");
 Z ell(argv[1]),block(argv[2]),cap(argv[3]);
 need(ell>=5&&mpz_odd_p(ell.get_mpz_t())&&block>0&&cap>0&&mpz_fits_ulong_p(block.get_mpz_t()),"invalid degree/block/cap");
 Z n=(ell-1)/2,B,rad=n/2;mpz_sqrt(B.get_mpz_t(),rad.get_mpz_t());if(B<1)B=1;
 if(argc==5)B=Z(argv[4]);need(B>0&&2*B<=n,"invalid baby window");
 Z G=n/(2*B),L=n-2*B*G,bi=ceildiv(B,block),gi=ceildiv(G,block);
 Z mb=minz(B,block),mg=minz(G,block),slots=sizeof(fmpz),C=cap/slots;
 Z root=2*mb+1,giant=mb+2*mg+2,necessary=maxz(root,giant),M=0;
 if(C>=3)M=(C-1)/2;
 auto optimal=optimum(n);Z OB=optimal.second,OG=n/(2*OB),OL=n-2*OB*OG;
 std::cout<<"{\"schema\":\"complete-memory-frontier/v1\",\"scope\":\"Necessary logical coefficient-slot bounds for pinned source only; not RSS or a general isogeny bound\",\"ell\":\""<<ell<<"\",\"block\":\""<<block<<"\",\"cap_bytes\":\""<<cap<<"\",\"slot_bytes\":"<<slots<<",\"cap_slots\":\""<<C<<"\",\"n\":\""<<n<<"\",\"B\":\""<<B<<"\",\"G\":\""<<G<<"\",\"L\":\""<<L<<"\",\"baby_blocks\":\""<<bi<<"\",\"giant_blocks\":\""<<gi<<"\",\"rectangles\":\""<<bi*gi<<"\",\"coordinate_contexts\":\""<<B+bi*G+L<<"\",\"baby_root_phase_slots\":\""<<root<<"\",\"giant_product_phase_slots\":\""<<giant<<"\",\"chosen_peak_lower_bound_bytes\":\""<<necessary*slots<<"\",\"chosen_lower_bound_fits_cap\":"<<(necessary<=C?"true":"false")<<",\"all_window_single_rectangle_minimum_slots\":\""<<optimal.first<<"\",\"all_window_single_rectangle_minimum_bytes\":\""<<optimal.first*slots<<"\",\"single_rectangle_witness_B\":\""<<OB<<"\",\"single_rectangle_witness_G\":\""<<OG<<"\",\"single_rectangle_witness_L\":\""<<OL<<"\",\"chunk_limit_by_necessary_bounds\":\""<<M<<"\",\"all_window_contexts_minimum_under_cap\":";
 if(M>0)std::cout<<"\""<<1+ceildiv(n,2*M)<<"\"";else std::cout<<"null";
 std::cout<<",\"all_window_rectangles_minimum_under_cap\":";
 if(M>0)std::cout<<"\""<<n/(2*M*(M+1))+1<<"\"";else std::cout<<"null";
 std::cout<<",\"gmp_limbs_included\":false,\"other_jets_caches_scratch_included\":false,\"primality_checked\":false,\"launch_admitted\":false,\"runtime_measurements\":null}"<<std::endl;
 return 0;
}catch(const std::exception& e){std::cerr<<"memory-audit: "<<e.what()<<'\n';return 2;}}
