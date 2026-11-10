// Reuse the frozen bounded polynomial parser/arithmetic; do not alter it.
#define main legacy_multiplicity_main
#include "../../../solver_multiplicity_audit_20261109/native/analyze.cpp"
#undef main

std::vector<Poly> expected(bool sat) {
  return list(sat ? "x-1,y" : "x*y,x^2-x,y^2-y");
}
bool field101(const std::string& text) {
  std::stringstream in(text); std::string s; int matches=0;
  while(std::getline(in,s)) if(s.rfind("#field characteristic:",0)==0) {
    if(compact(s.substr(s.find(':')+1))!="101") return false;
    matches++;
  }
  return matches==1;
}
int main(int argc,char** argv) {
  p=101;
  if(argc==2 && std::string(argv[1])=="--selftest") {
    auto a=expected(false),b=expected(true);
    auto wrong=list("x+1,y"); bool zeroRejected=false;
    try { (void)monic(Poly{}); } catch(const std::exception&) {zeroRejected=true;}
    bool ok=buchberger(a)&&buchberger(b)&&standardMonomials(a)==3&&
      standardMonomials(b)==1&&!sameBasis(b,wrong)&&zeroRejected&&
      field101("#field characteristic: 101\n")&&
      !field101("#field characteristic: 7\n")&&
      !field101("#field characteristic: 101\n#field characteristic: 101\n");
    std::cout<<"{\"calibration\":\"synthetic_expected_rows_only\",\"passed\":"
      <<(ok?"true":"false")<<"}\n";
    return ok?0:1;
  }
  if(argc!=2) {std::cerr<<"usage: analyze STUDY_DIR | --selftest\n";return 2;}
  bool all=true;
  for(bool sat:{false,true}) {
    std::string arm=sat?"saturation":"ordinary";
    std::string prefix=std::string(argv[1])+"/evidence/runs/"+arm;
    try {
      bool exitOk=compact(read(prefix+".exit"))=="0";
      bool characteristic=field101(read(prefix+".out"));
      auto g=groebner(prefix+".out");
      bool basis=sameBasis(g,expected(sat)),sp=buchberger(g),inputs=true;
      for(const auto& f:expected(false)) if(!remainder(f,g).empty()) inputs=false;
      int staircase=standardMonomials(g),points=pointCount(g);
      bool hasY=std::find(g.begin(),g.end(),polynomial("y"))!=g.end();
      bool vanishes=true;
      for(const auto& f:g) if(value(f,0,1)!=0) vanishes=false;
      bool ok=exitOk&&characteristic&&basis&&sp&&inputs&&
        staircase==(sat?1:3)&&points==(sat?1:3)&&
        (sat?hasY&&!vanishes:vanishes);
      all&=ok;
      std::cout<<"{\"arm\":\""<<arm<<"\",\"exit_ok\":"<<(exitOk?"true":"false")
        <<",\"field101\":"<<(characteristic?"true":"false")
        <<",\"basis_matches\":"<<(basis?"true":"false")
        <<",\"buchberger\":"<<(sp?"true":"false")
        <<",\"inputs_reduce\":"<<(inputs?"true":"false")
        <<",\"standard_monomials\":"<<staircase<<",\"points\":"<<points
        <<",\"contains_y\":"<<(hasY?"true":"false")
        <<",\"vanishes_at_zero_one\":"<<(vanishes?"true":"false")
        <<",\"verdict\":\""<<(ok?"fixture_pass":"discrepancy")<<"\"}\n";
    } catch(const std::exception& e) {
      all=false;
      std::cerr<<arm<<": "<<e.what()<<"\n";
      std::cout<<"{\"arm\":\""<<arm<<"\",\"verdict\":\"not_validated\"}\n";
    }
  }
  return all?0:1;
}
