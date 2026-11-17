#include "poly.hpp"
#include <filesystem>
#include <regex>
#include <openssl/sha.h>
#include <iomanip>
#include <sstream>
#include <time.h>
using Algebra::Poly;
using Algebra::Mon;
U cpu_clock(){timespec t{};need(clock_gettime(CLOCK_PROCESS_CPUTIME_ID,&t)==0,"CPU clock");return U(t.tv_sec)*1000000000+U(t.tv_nsec);}
std::string bytes(const std::string&p){std::ifstream f(p,std::ios::binary);need(bool(f),"read "+p);return {std::istreambuf_iterator<char>(f),{}};}
std::string sha(const std::string&p){auto s=bytes(p);unsigned char h[32];SHA256(reinterpret_cast<const unsigned char*>(s.data()),s.size(),h);std::ostringstream o;for(auto c:h)o<<std::hex<<std::setfill('0')<<std::setw(2)<<unsigned(c);return o.str();}
Poly sum(Poly a,const Poly&b){for(auto [m,c]:b)Algebra::term(a,m,c);return a;}
Poly negative(const Poly&a){return Algebra::scale(a,{0,0},PRIME-1);}
Poly difference(Poly a,const Poly&b){return sum(std::move(a),negative(b));}
Poly product(const Poly&a,const Poly&b){Poly r;for(auto [u,c]:a)for(auto [v,d]:b)Algebra::term(r,{u.x+v.x,u.y+v.y},Algebra::times(c,d));return r;}
Poly linear(bool y,U a){Poly p{{Mon{y?0:1,y?1:0},1}};Algebra::term(p,{0,0},Algebra::minus(0,a));return p;}
Poly factor(const std::vector<U>&roots,bool y,const std::set<int>&omit={}){Poly p{{Mon{0,0},1}};for(size_t i=0;i<roots.size();i++)if(!omit.count(int(i)))p=product(p,linear(y,roots[i]));return p;}
U pow_counted(U a,int e){U r=1;while(e){if(e&1)r=Algebra::times(r,a);a=Algebra::times(a,a);e>>=1;}return r;}
U counted_evaluate(const Poly&p,U x,U y){U r=0;for(auto [m,c]:p)r=Algebra::plus(r,Algebra::times(c,Algebra::times(pow_counted(x,m.x),pow_counted(y,m.y))));return r;}
Poly parse(const std::string&s){Poly p;std::regex term(R"(([0-9]+)(?:\*x\^([0-9]+))?(?:\*y\^([0-9]+))?)");size_t pos=0;while(pos<s.size()){auto end=s.find('+',pos);if(end==std::string::npos)end=s.size();std::smatch m;auto token=s.substr(pos,end-pos);need(std::regex_match(token,m,term),"polynomial token");U c=std::stoull(m[1]);need(c<PRIME,"coefficient residue");int x=m[2].matched?std::stoi(m[2]):0,y=m[3].matched?std::stoi(m[3]):0;need(x>=0&&y>=0&&x+y<=12,"input exponents");Algebra::term(p,{x,y},c);pos=end+1;}return p;}
std::array<Poly,3> parse_line(const std::string&s){need(s.rfind("F=",0)==0,"case prefix");auto g=s.find(";G="),h=s.find(";H="),end=s.find(";I=F,G,H;");need(g!=s.npos&&h!=s.npos&&end!=s.npos&&g<h&&h<end,"case delimiters");return{parse(s.substr(2,g-2)),parse(s.substr(g+3,h-g-3)),parse(s.substr(h+3,end-h-3))};}
std::array<Poly,2> divide_grid(Poly p,const Poly&A,const Poly&B){std::array<Poly,2> q;std::array<Poly,2> basis{A,B};while(!p.empty()){auto [m,c]=*p.begin();bool did=false;for(int j=0;j<2;j++){auto lm=basis[j].begin()->first;if(!Algebra::divides(lm,m))continue;need(basis[j].begin()->second==1,"monic grid");Mon shift{m.x-lm.x,m.y-lm.y};Algebra::term(q[j],shift,c);p=difference(std::move(p),Algebra::scale(basis[j],shift,c));if(Algebra::counts)Algebra::counts->reductions++;did=true;break;}need(did,"nonzero grid remainder");}return q;}
std::string gp_poly(const Poly&p,int homogeneous_degree=-1){std::string s;need(homogeneous_degree<0||Algebra::degree(p)<=homogeneous_degree,"homogenization bound");for(auto [m,c]:p){if(!s.empty())s+="+";s+="Mod("+std::to_string(c)+",p)";if(m.x)s+="*x^"+std::to_string(m.x);if(m.y)s+="*y^"+std::to_string(m.y);if(homogeneous_degree>=0&&homogeneous_degree-m.degree())s+="*z^"+std::to_string(homogeneous_degree-m.degree());}return s.empty()?"0":s;}
json counts(const Algebra::Count&c){return{{"Fp_multiply_including_inverse_power",c.multiply},{"Fp_add_sub",c.add_sub},{"Fp_inverse",c.inverse},{"polynomial_leading_reductions",c.reductions},{"max_temporary_affine_degree",c.max_actual_degree}};}
struct Certificate{int reg_ideal,reg_quotient;std::vector<std::vector<int>>shifts;json roots,proof;};
Certificate certify(const Poly&F,const Poly&A,const Poly&B,const std::vector<U>&xs,int within){
 need(Algebra::degree(F)==4&&F.count({2,2})&&F.at({2,2})==1,"quartic shape");
 for(auto [m,c]:F){(void)c;need(m.degree()<4||m==Mon{2,2},"unique quartic top term");}
 std::vector<std::pair<int,int>> zeros;for(int i=0;i<12;i++)for(int j=0;j<12;j++)if(counted_evaluate(F,xs[i],xs[j])==0)zeros.emplace_back(i,j);
 if(within>=2){need(zeros.empty(),"unexpected random grid zero");return{26,25,{{0},{4,12,12},{16,16,24},{28}},zeros,{{"kind","complete_intersection_12_12_4"},{"no_projective_infinity_points",true}}};}
 auto expected=within==0?std::vector<std::pair<int,int>>{{0,1},{1,0}}:std::vector<std::pair<int,int>>{{0,0}};
 need(zeros==expected,"unsupported planted zero pattern");
 std::array<Poly,3> c0,c1;std::array<int,3>d0,d1;Poly T;
 if(within==1){auto Ux=factor(xs,false,{0}),Vy=factor(xs,true,{0});T=product(Ux,Vy);c0={negative(Vy),Poly{},linear(false,xs[0])};c1={Poly{},negative(Ux),linear(true,xs[0])};d0={11,11,1};d1=d0;}
 else{auto Ux=factor(xs,false,{0,1}),Vy=factor(xs,true,{0,1});Poly xm_y{{Mon{1,0},1},{Mon{0,1},PRIME-1}};T=product(xm_y,product(Ux,Vy));Poly xy{{Mon{1,0},1},{Mon{0,1},1}};Algebra::term(xy,{0,0},Algebra::minus(0,Algebra::plus(xs[0],xs[1])));auto X=linear(false,xs[0]),Y=linear(true,xs[0]);c0={negative(Vy),Ux,xy};c1={product(Y,Vy),negative(product(X,Ux)),negative(product(X,Y))};d0={10,10,1};d1={11,11,2};}
 std::array<Poly,3> minors{difference(product(c0[0],c1[1]),product(c1[0],c0[1])),difference(product(c0[0],c1[2]),product(c1[0],c0[2])),difference(product(c0[1],c1[2]),product(c1[1],c0[2]))};
 need(minors[0]==T&&minors[1]==negative(B)&&minors[2]==A,"Hilbert-Burch minors");
 for(int i=0;i<12;i++)for(int j=0;j<12;j++)need((counted_evaluate(T,xs[i],xs[j])!=0)==(counted_evaluate(F,xs[i],xs[j])==0),"residual grid");
 int td=within==1?22:21;need(Algebra::degree(T)==td,"separator degree");
 auto FT=product(F,T);auto q=divide_grid(FT,A,B);need(sum(product(q[0],A),product(q[1],B))==FT,"membership identity");need(Algebra::degree(q[0])<=td-8&&Algebra::degree(q[1])<=td-8,"membership degree");
 json matrix=json::array();for(int i=0;i<3;i++){need(Algebra::degree(c0[i])<=d0[i]&&Algebra::degree(c1[i])<=d1[i],"matrix degree");matrix.push_back({Algebra::wire(c0[i]),Algebra::wire(c1[i])});}
 std::vector<std::vector<int>> shifts=within==1?std::vector<std::vector<int>>{{0},{4,12,12},{16,16,24,26},{27,27}}:std::vector<std::vector<int>>{{0},{4,12,12},{16,16,24,25},{26,27}};
 int reg=-1;for(size_t i=0;i<shifts.size();i++)for(int d:shifts[i])reg=std::max(reg,d-int(i));
 for(size_t i=1;i<shifts.size();i++)for(int a:shifts[i-1])for(int b:shifts[i])need(a!=b,"nonminimal mapping cone");
 need(reg==24,"resolution shift check");json proof={{"kind","residual_grid_Hilbert_Burch_mapping_cone"},{"F",Algebra::wire(F)},{"A",Algebra::wire(A)},{"B",Algebra::wire(B)},{"separator",Algebra::wire(T)},{"separator_degree",td},{"quotients",{Algebra::wire(q[0]),Algebra::wire(q[1])}},{"HB_matrix",matrix},{"HB_column_0_degrees",d0},{"HB_column_1_degrees",d1},{"all_identities_checked",true},{"no_projective_infinity_points",true},{"residual_grid_checked",true}};
 return{reg+1,reg,shifts,zeros,proof};
}
Poly from_wire(const json&j){Poly p;for(const auto&t:j)Algebra::term(p,{t[0].get<int>(),t[1].get<int>()},val(t[2]));return p;}
void export_control(std::ostream&o,const json&p){
 int td=p["separator_degree"];o<<"FF="<<gp_poly(from_wire(p["F"]),4)<<";AA="<<gp_poly(from_wire(p["A"]),12)<<";BB="<<gp_poly(from_wire(p["B"]),12)<<";TT="<<gp_poly(from_wire(p["separator"]),td)<<";QQ="<<gp_poly(from_wire(p["quotients"][0]),td-8)<<";RR="<<gp_poly(from_wire(p["quotients"][1]),td-8)<<";";
 for(int i=0;i<3;i++)for(int j=0;j<2;j++)o<<"M"<<i<<j<<"="<<gp_poly(from_wire(p["HB_matrix"][i][j]),p[j?"HB_column_1_degrees":"HB_column_0_degrees"][i])<<";";
 o<<"if(FF*TT!=QQ*AA+RR*BB||M00*M11-M01*M10!=TT||M00*M21-M01*M20!=-BB||M10*M21-M11*M20!=AA,error(\"homogeneous certificate\"));control_checks++;\n";
}
void toy(const std::string&out){need(std::filesystem::create_directory(out),"claim toy output");std::ofstream s(out+"/toy.sing");s<<"ring r=1000003,(x,y,z),dp;\nideal T=x^2,y^2,z^2;resolution M=mres(T,0);print(\"CONVENTION \"+string(regularity(M)));\npoly A=1;poly B=1;int i;for(i=0;i<12;i++){A=A*(x-i*z);B=B*(y-i*z);}\npoly F;ideal J;\n";for(int i=0;i<3;i++){s<<"F=x^2*y^2+"<<(i==2?"(x+y-z)^2*z^2":"x^2*z^2+y^2*z^2")<<(i==0?"+z^4":"")<<";J=A,B,F;M=mres(J,0);print(\"TOY "<<i<<" REG_IDEAL \"+string(regularity(M)));print(betti(M));\n";}s<<"print(\"TOY_COMPLETE 3\");quit;\n";}
int main(int argc,char**argv){std::string out;try{
 need(argc==4,"MODE ROOT OUT");std::string mode=argv[1],root=argv[2];out=argv[3];
 if(mode=="toy"){toy(out);std::cout<<"TOY_INPUT_PREPARED\n";return 0;}
 if(mode=="audit"){auto gp=bytes(out+"/gp.stdout"),ge=bytes(out+"/gp.stderr"),sg=bytes(out+"/toy.stdout"),se=bytes(out+"/toy.stderr");need(gp=="REGULARITY_GRID_CHECKS=14720 CONTROL_IDENTITIES=920\n"&&ge.empty(),"independent GP audit");need(se.empty()&&sg.find('?')==sg.npos&&sg.find("CONVENTION 4\n")!=sg.npos&&sg.find("TOY 0 REG_IDEAL 26\n")!=sg.npos&&sg.find("TOY 1 REG_IDEAL 25\n")!=sg.npos&&sg.find("TOY 2 REG_IDEAL 25\n")!=sg.npos&&sg.find("TOY_COMPLETE 3\n")!=sg.npos,"independent small-field resolution convention");auto sum=load(out+"/summary.json");need(sum["cases"]==14720&&sum["controls"]==920,"native coverage");save(out+"/independent_audit.json",{{"passed",true},{"actual_field_grid_systems",14720},{"actual_field_homogeneous_control_certificates",920},{"separate_small_prime_resolution_controls",3},{"scalar_witness_inputs",false},{"full_random_IC_recovery_complete",false}});std::cout<<"REGULARITY_AUDIT_PASSED\n";return 0;}
 need(mode=="run","mode");auto binding=load(root+"/REGULARITY_CERT_FROZEN.json");for(auto [p,h]:binding["files"].items())need(sha(root+"/"+p)==h.get<std::string>(),"binding "+p);
 need(prime(PRIME),"actual field prime");need(std::filesystem::create_directory(out),"claim output");
 auto family=load(root+"/data/family.json"),bases=load(root+"/data/ic_factorbases.json");std::ifstream input(root+"/validation/singular_all.sing");need(bool(input),"public systems");
 std::ofstream records(out+"/cases.jsonl"),certs(out+"/controls.jsonl"),setups(out+"/setups.jsonl"),gp(out+"/verify.gp");gp<<"p="<<PRIME<<";x='x;y='y;z='z;grid_checks=0;control_checks=0;\n";
 std::string line;size_t id=0;int lastgroup=-1;std::vector<U>xs;Poly A,B;U wall=ns();int controls=0;
 while(std::getline(input,line)){if(line.rfind("F=",0)!=0)continue;need(id<14720,"too many systems");int curve=int(id/320),g=int(id%320)/32,within=int(id%32),group=curve*10+g;
  if(group!=lastgroup){xs.clear();for(const auto&p:bases[std::to_string(curve)][g])xs.push_back(val(p[0]));need(xs.size()==12&&std::set<U>(xs.begin(),xs.end()).size()==12,"distinct roots");Algebra::Count c;Algebra::counts=&c;U t=ns(),cpu=cpu_clock();A=factor(xs,false);B=factor(xs,true);cpu=cpu_clock()-cpu;t=ns()-t;Algebra::counts=nullptr;setups<<json({{"curve",curve},{"generator",g},{"wall_ns",t},{"process_cpu_ns",cpu},{"counters",counts(c)}}).dump()<<'\n';gp<<"xs=[";for(size_t k=0;k<xs.size();k++){if(k)gp<<",";gp<<"Mod("<<xs[k]<<",p)";}gp<<"];GA=prod(i=1,12,x-xs[i]);HB=prod(i=1,12,y-xs[i]);\n";lastgroup=group;}
  U t=ns();auto polys=parse_line(line);U parsing=ns()-t;Algebra::Count count;Algebra::counts=&count;t=ns();U cpu=cpu_clock();need(polys[1]==A&&polys[2]==B,"factor polynomial match");auto n=family["nodes"][curve];U cross=polys[0].count({2,1})?polys[0].at({2,1}):0;U rx=Algebra::times(Algebra::minus(0,cross),(PRIME+1)/2);need(polys[0]==Algebra::f3(rx,val(n["a"]),val(n["b"])),"actual curve f3");
  auto cert=certify(polys[0],A,B,xs,within);cpu=cpu_clock()-cpu;t=ns()-t;Algebra::counts=nullptr;
  json row={{"system",id},{"curve",curve},{"generator",g},{"stage",within<2?"positive_control":within<22?"setup":"target"},{"index",within<2?within:within<22?within-2:within-22},{"roots",cert.roots},{"reg_generator_homogenized_ideal",cert.reg_ideal},{"reg_quotient",cert.reg_quotient},{"quotient_resolution_shifts",cert.shifts},{"parse_wall_ns",parsing},{"certificate_wall_ns",t},{"certificate_process_cpu_ns",cpu},{"counters",counts(count)},{"verified_native",true}};records<<row.dump()<<'\n';records.flush();
  gp<<"F="<<gp_poly(polys[0])<<";if(GA!="<<gp_poly(A)<<"||HB!="<<gp_poly(B)<<",error(\"grid polynomials\"));zz=[];for(i=1,12,for(j=1,12,if(subst(subst(F,x,xs[i]),y,xs[j])==0,zz=concat(zz,[[i-1,j-1]]))));if(zz!="<<cert.roots.dump()<<",error(\"grid zeros\"));grid_checks++;\n";
  if(within<2){cert.proof["system"]=id;cert.proof["curve"]=curve;cert.proof["generator"]=g;cert.proof["quotient_resolution_shifts"]=cert.shifts;certs<<cert.proof.dump()<<'\n';certs.flush();export_control(gp,cert.proof);controls++;}
  id++;
 }
 need(id==14720&&controls==920,"full case coverage");gp<<"print(\"REGULARITY_GRID_CHECKS=\",grid_checks,\" CONTROL_IDENTITIES=\",control_checks);quit;\n";gp.close();
 save(out+"/summary.json",{{"cases",id},{"controls",controls},{"random_setup_and_target_systems",13800},{"native_certificate_complete",true},{"independent_audit_pending",true},{"reg_ideal_random_systems",26},{"reg_ideal_positive_controls",25},{"definition","ideal generated by separate homogenizations of the supplied F,G,H"},{"whole_native_wall_ns",ns()-wall},{"actual_prime_field",std::to_string(PRIME)},{"private_scalar_inputs",false},{"original_IC_measurements_unchanged",true},{"binding",binding}});
 std::cout<<"REGULARITY_NATIVE_PASSED cases="<<id<<" controls="<<controls<<"\n";return 0;
 }catch(const std::exception&e){Algebra::counts=nullptr;std::cerr<<"REGULARITY_ERROR "<<e.what()<<'\n';if(!out.empty()&&std::filesystem::exists(out))std::ofstream(out+"/failure.txt",std::ios::app)<<e.what()<<'\n';return 2;}}
