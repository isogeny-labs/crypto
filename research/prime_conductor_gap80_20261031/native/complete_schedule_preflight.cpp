// Exact metadata for the current complete_pipeline.cpp traversal. No root
// allocation, curve arithmetic, polynomial multiplication or timing occurs.
#include <gmpxx.h>
#include <flint/fmpz.h>
#include <iostream>
#include <stdexcept>
#include <limits>
using Z = mpz_class;
static Z ceil_div(const Z& a, const Z& b) { return (a+b-1)/b; }
static Z minimum(const Z& a, const Z& b) { return a < b ? a : b; }
int main(int argc, char** argv) {
  try {
    if (argc != 4 && argc != 5) throw std::runtime_error("usage: complete_schedule_preflight DEGREE BLOCK MEMORY_BYTES [BABY_WINDOW]");
    Z ell(argv[1]), block(argv[2]), cap(argv[3]);
    if (ell < 5 || mpz_even_p(ell.get_mpz_t()) || block <= 0 || cap <= 0 ||
        !mpz_fits_ulong_p(block.get_mpz_t())) throw std::runtime_error("invalid degree/block/cap");
    Z n=(ell-1)/2, rad=n/2, B;
    mpz_sqrt(B.get_mpz_t(),rad.get_mpz_t());
    if (B<1) B=1;
    if (argc==5) {B=Z(argv[4]);if(B<=0||2*B>n)throw std::runtime_error("baby window outside coverage range");}
    Z G=n/(2*B), bi=ceil_div(B,block), gi=ceil_div(G,block);
    Z tiles=bi*gi, left=n-2*B*G;
    Z slots=sizeof(fmpz), max_baby=minimum(B,block);
    // Roots remain owned by RootVector while the monic result of degree
    // max_baby has max_baby+1 coefficient slots. Both exist at call return.
    Z chosen_bytes=(2*max_baby+1)*slots, full_bytes=(2*B+1)*slots;
    Z allowed=0;
    if (cap>=3*slots) allowed=(cap-slots)/(2*slots);
    bool has_candidate=allowed>0;
    Z generous_block=allowed, generous_tiles=0;
    if (has_candidate) {
      if (allowed>=B) generous_block=B>G?B:G;
      generous_tiles=ceil_div(B,generous_block)*ceil_div(G,generous_block);
    }
    bool native_width=mpz_fits_ulong_p(B.get_mpz_t()) && mpz_fits_ulong_p(G.get_mpz_t());
    bool chunk_width=max_baby<=Z(std::numeric_limits<slong>::max());
    std::cout << "{\"schema\":\"complete-schedule-metadata/v1\","
      << "\"scope\":\"Derived current-source complete-execution counts and necessary slot bound only\","
      << "\"allocation_budget_scope\":\"Live logical slot storage; no RSS, swapping or external-memory model\","
      << "\"window_selection\":\""<<(argc==5?"explicit":"balanced_default")<<"\","
      << "\"ell\":\""<<ell<<"\",\"ell_bits\":"<<mpz_sizeinbase(ell.get_mpz_t(),2)
      << ",\"block\":\""<<block<<"\",\"memory_cap_bytes\":\""<<cap
      << "\",\"required_declared_window_guard\":\""<<(B>G?B:G)
      << "\",\"baby_count\":\""<<B<<"\",\"giant_count\":\""<<G
      << "\",\"half_kernel_pairs\":\""<<n<<"\",\"leftover_pairs\":\""<<left
      << "\",\"baby_blocks\":\""<<bi<<"\",\"giant_blocks\":\""<<gi
      << "\",\"rectangles\":\""<<tiles
      << "\",\"baby_products\":\""<<bi<<"\",\"baby_scalar_starts\":\""<<bi
      << "\",\"baby_recurrence_additions\":\""<<(B-bi)
      << "\",\"giant_scalar_starts\":\""<<tiles
      << "\",\"giant_recurrence_additions\":\""<<(bi*(G-gi))
      << "\",\"giant_contexts_regenerated\":\""<<(bi*G)
      << "\",\"leftover_scalar_starts\":\""<<(left>0?1:0)
      << "\",\"leftover_recurrence_additions\":\""<<(left>0?Z(left-1):Z(0))
      << "\",\"minimum_log_norm_calls\":\""<<tiles
      << "\",\"fmpz_slot_bytes\":"<<slots
      << ",\"chosen_baby_vectors_slot_bytes_lower_bound\":\""<<chosen_bytes
      << "\",\"chosen_baby_vectors_bound_fits_cap\":"<<(chosen_bytes<=cap?"true":"false")
      << ",\"full_baby_vectors_slot_bytes_lower_bound\":\""<<full_bytes
      << "\",\"full_baby_vectors_bound_fits_cap\":"<<(full_bytes<=cap?"true":"false")
      << ",\"max_baby_length_by_slot_bound_only\":\""<<allowed<<"\","
      << "\"best_case_block_by_slot_bound_only\":";
    if (has_candidate) std::cout<<"\""<<generous_block<<"\""; else std::cout<<"null";
    std::cout<<",\"minimum_rectangles_by_slot_bound_only\":";
    if (has_candidate) std::cout<<"\""<<generous_tiles<<"\""; else std::cout<<"null";
    std::cout<<",\"unsigned_long_window_indices_fit\":"<<(native_width?"true":"false")
      << ",\"root_chunk_signed_length_fits\":"<<(chunk_width?"true":"false")
      << ",\"primality_checked\":false,\"peak_memory_measured\":null,\"runtime_measurements\":null,"
      << "\"launch_admitted\":false,\"proof_scope\":\"Model of the pinned implementation; not a universal algorithm bound or independent mathematical verification\"}\n";
  } catch (const std::exception& e) {
    std::cerr<<"complete_schedule_preflight: "<<e.what()<<'\n'; return 2;
  }
}
