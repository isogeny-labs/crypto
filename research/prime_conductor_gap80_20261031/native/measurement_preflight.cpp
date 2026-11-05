// Exact metadata audit; does not allocate a kernel polynomial or time a map.
#include <gmpxx.h>
#include <flint/fmpz.h>
#include <iostream>
#include <sstream>
#include <cstdint>
#include <algorithm>
int main(int argc,char **argv) {
  if(argc!=3) {std::cerr<<"usage: measurement_preflight DEGREE MEMORY_BYTES\n";return 2;}
  mpz_class ell(argv[1]),cap(argv[2]);
  if(ell<3||mpz_even_p(ell.get_mpz_t())||cap<=0)return 2;
  mpz_class n=(ell-1)/2, rad=n/2, B;
  mpz_sqrt(B.get_mpz_t(),rad.get_mpz_t());if(B<1)B=1;
  mpz_class rootbytes=B*sizeof(fmpz);
  // Reproduce the legacy constructor's unsigned-degree extraction, without
  // invoking its huge allocation or truncating the target to another degree.
  std::istringstream input(argv[1]);uint64_t degree=0;input>>degree;
  bool width_ok=!input.fail()&&mpz_class(std::to_string(degree))==ell;
  bool memory_ok=rootbytes<=cap;
  std::cout<<"{\n\"scope\":\"Exact static eligibility audit; no performance measurement\",\n"
    <<"\"ell\":\""<<ell<<"\",\n\"ell_bits\":"<<mpz_sizeinbase(ell.get_mpz_t(),2)
    <<",\n\"legacy_uint64_parser_accepts_target\":"<<(width_ok?"true":"false")
    <<",\n\"baby_points_exact_floor\":\""<<B<<"\",\n\"fmpz_slot_bytes\":"<<sizeof(fmpz)
    <<",\n\"baby_root_vector_bytes_lower_bound\":\""<<rootbytes
    <<"\",\n\"memory_budget_bytes\":\""<<cap
    <<"\",\n\"root_slots_alone_fit_budget\":"<<(memory_ok?"true":"false")
    <<",\n\"legacy_constructor_has_point_evaluator\":false,\n"
    <<"\"construction_seconds\":null,\n\"point_evaluation_seconds\":null,\n"
    <<"\"matched_endpoint_ecdlp_seconds\":null,\n"
    <<"\"status\":\""<<(width_ok&&memory_ok?"metadata_checks_pass_only":"unchanged_constructor_not_launchable")<<"\"\n}\n";
}
