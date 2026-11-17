#define main cm_native_main
#include "native.cpp"
#undef main
static Json scalar_token(const std::string& value){Json j;j.kind=Json::String;j.value=value;return j;}
int main(int argc,char** argv){try{
 require(argc==2,"qualification-tests FIXTURE_DIR");auto originals=fixtures(argv[1]);require(originals.front().name=="P-192","primary P192 mutation panel");unsigned rejected=0;
 auto reject=[&](const Fixture& f,const char* expected){try{qualify(std::vector<Fixture>{f});}catch(const std::exception& e){require(std::string(e.what())==expected,"mutation failed at unexpected check");++rejected;std::cout<<"{\"mutation_rejected_at\":"<<json_quote(e.what())<<"}\n";return;}throw std::runtime_error("mutated certificate accepted");};
 auto f=originals.front();f.marked.object["alpha_a"]=scalar_token("-1");reject(f,"CM label a+b modulo n");
 f=originals.front();int opposite_sign=f.marked.at("degree").integer()-f.marked.at("orientation_root").integer();Json signed_root;signed_root.kind=Json::Number;signed_root.value=std::to_string(opposite_sign);f.marked.object["orientation_root"]=signed_root;f.closure.object["first"].object["edge"].object["frobenius_eigenvalue"]=signed_root;reject(f,"signed kernel Frobenius y action");
 f=originals.front();f.closure.object["closing_isomorphism"].object["y_scale"]=scalar_token("1");reject(f,"positive differential closing scales");
 f=originals.front();Field field(f.curve.at("p").str());auto& coefficient=f.closure.object["first"].object["edge"].object["kernel_polynomial"].array[0];coefficient=scalar_token(field.add(Big(coefficient.str()),Big(1)).str());reject(f,"map denominator is kernel square");
 f=originals.front();f.marked.object["orbit_checks"].array[1].object["transporter"]=scalar_token("0");reject(f,"saved representative transporter");
 f=originals.front();f.marked.object["orbit_checks"].array.clear();reject(f,"complete orbit fixture panel");
 f=originals.front();f.closure.object["trace"]=scalar_token("1");reject(f,"trace from field and group order");
 f=originals.front();f.closure.object["opposite_root"]=scalar_token("1");reject(f,"opposite characteristic root");
 std::cout<<"{\"native_certificate_mutations_rejected\":"<<rejected<<",\"timing_performed\":false}\n";return 0;
}catch(const std::exception& e){std::cerr<<"qualification control failed: "<<e.what()<<'\n';return 1;}}
