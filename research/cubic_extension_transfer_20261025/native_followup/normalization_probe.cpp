// A bounded attempt to simplify the symbolic theta normalization. This probe
// is kept whether the proposed formula succeeds or fails.
#define main obstruction_reference_main
#include "obstruction.cpp"
#undef main
int main(int argc,char**argv){try{
 require(argc==2,"usage: normalization_probe FOLLOWUP_DIR");std::string d=argv[1];json rows=json::array();
 for(int p:{109,157}){field_start(p,18);{
  std::ifstream input(d+"/fixtures/p"+std::to_string(p)+".fixture");std::vector<std::string>v;std::string line;while(std::getline(input,line))v.push_back(line);std::vector<F>mod;for(auto c:parse_poly(v[3]))mod.push_back(F(c));F z=roots(mod)[0];auto ab=parse_curve(v[4],z);EC E{ab.first,ab.second};
  // The p=157 fixture source is unprepared, but geometric full E[3] still
  // exists in the chosen splitting field, so the normalization test applies.
  Normal n=normal3(E);int match=0;json cs=json::array();
  for(int c=1;c<9;c++){F guess=-F(1)/(F(2)*n.points[c].y);bool same=guess==n.sections[c][2];match+=same;cs.push_back({{"coordinate",{c/3,c%3}},{"candidate_matches",same}});}
  rows.push_back({{"p",p},{"matching_nonzero_sections",match},{"sections",cs}});
 }field_stop();}
 write_json(d+"/results/normalization-probe.json",{{"status","complete"},{"candidate","alpha_P=-1/(2*y(P)) for every nonzero three-torsion point"},{"cases",rows},{"scope","Finite test of a proposed symbolic simplification; not a universal identity"}});return 0;
 }catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 2;}}
