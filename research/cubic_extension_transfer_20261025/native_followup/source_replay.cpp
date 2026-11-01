// Recheck all recorded answers with FLINT elliptic arithmetic, independently
// of the Rust source-control arithmetic. Target answers share a public known
// scalar, not an executed elliptic-to-Jacobian map.
#include "ff.hpp"
int main(int argc,char**argv){try{
 require(argc==2,"usage: source_replay NATIVE_FOLLOWUP_DIR");std::string d=argv[1];auto receipt=read_json(d+"/results/pinned-diagnostic/reservation.json");json checks=json::array();
 for(int p:{109,157}){field_start(p,18);{
  std::ifstream input(d+"/fixtures/p"+std::to_string(p)+".fixture");std::vector<std::string>v;std::string line;while(std::getline(input,line))v.push_back(line);require(v.size()==10,"fixture line count");std::vector<F>mod;for(auto c:parse_poly(v[3]))mod.push_back(F(c));auto z=roots(mod)[0];auto ab=parse_curve(v[4],z);EC E{ab.first,ab.second};Point P(eval_poly(parse_poly(v[5]),z),eval_poly(parse_poly(v[6]),z)),Q(eval_poly(parse_poly(v[7]),z),eval_poly(parse_poly(v[8]),z));uint64_t r=std::stoull(v[1]);require(E.mul(r,P).infinity&&!P.infinity,"source order witness");
  for(auto&run:receipt["runs"]){if(run["job"]["p"]!=p)continue;auto row=read_json(d+"/results/pinned-diagnostic/"+run["stdout"].get<std::string>());require(row["verified"]==true&&run["exit_status"]==0,"failed run cannot be replayed as success");uint64_t scalar=row["scalar"];require(E.mul(scalar,P)==Q,"FLINT source scalar relation failed");checks.push_back({{"p",p},{"job_index",run["index"]},{"scalar",scalar},{"source_relation_verified",true}});}
 }field_stop();}
 require(checks.size()==58,"all requested runs must replay");write_json(d+"/results/source-replay.json",{{"status","passed"},{"arithmetic","C++/FLINT versus Rust source arithmetic; same host/session"},{"checks",checks},{"scope","All recovered scalar values satisfy the original public source relation. Native target inputs are synthetic with the same known answer; this does not certify a forward map."}});std::cout<<"FLINT source replay passed "<<checks.size()<<'\n';return 0;
 }catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 2;}}
