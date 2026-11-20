#define main cm_native_main
#include "native.cpp"
#undef main
#include <filesystem>
static Json token(const std::string& s){Json j;j.kind=Json::String;j.value=s;return j;}
static Json number(long n){Json j;j.kind=Json::Number;j.value=std::to_string(n);return j;}
static Json object(std::initializer_list<std::pair<const std::string,Json>> fields){Json j;j.kind=Json::Object;j.object=fields;return j;}
static Json array(const Poly& p){Json j;j.kind=Json::Array;for(auto& c:p)j.array.push_back(token(c.str()));return j;}
static Poly gcd_poly(Field& f,Poly a,Poly b){while(!b.empty()){Poly rest=poly_divmod(f,a,b).second;a=b;b=rest;}require(!a.empty(),"zero gcd");return scale(f,a,f.inv(a.back()));}
static Poly oriented_kernel(Field& f,const Big& a,const Big& b,unsigned ell,unsigned rho){
 DivisionPolynomials d(f,a,b);Poly torsion=d.get(ell);torsion=scale(f,torsion,f.inv(torsion.back()));KernelRing ring{f,torsion};
 Poly denominator=poly_power(f,d.get(rho),2),term=times(f,d.get(rho-1),d.get(rho+1));
 if(rho&1)term=scale(f,times(f,d.cubic,term),Big(4));else denominator=scale(f,times(f,d.cubic,denominator),Big(4));
 Poly numerator=plus(f,times(f,Poly{Big(),Big(1)},denominator),term,true);
 Poly h=plus(f,times(f,deriv(f,numerator),denominator),times(f,numerator,deriv(f,denominator)),true);
 Poly scalar_x=ring.mul(numerator,ring.inverse(denominator));
 Poly scalar_y=ring.mul(h,ring.inverse(scale(f,times(f,denominator,denominator),Big(long(rho)))));
 Big half=f.p;require(BN_sub_word(half.p,1)&&BN_rshift1(half.p,half.p),"half exponent");
 Poly x_difference=plus(f,ring.power(Poly{Big(),Big(1)},f.p),scalar_x,true);
 Poly y_difference=plus(f,ring.power(d.cubic,half),scalar_y,true);
 Poly kernel=gcd_poly(f,gcd_poly(f,torsion,x_difference),y_difference);
 require(kernel.size()==size_t((ell+1)/2),"oriented eigenspace kernel degree");return kernel;
}
static std::pair<Json,Json> velu(Field& f,const Big& a,const Big& b,unsigned ell,unsigned rho){
 Poly k=oriented_kernel(f,a,b,ell,rho),kd=deriv(f,k),kdd=deriv(f,kd),k2=times(f,k,k),cubic={b,a,Big(),Big(1)};
 size_t m=k.size()-1;Big c1=k[m-1],c2=m>=2?k[m-2]:Big(),c3=m>=3?k[m-3]:Big();
 Big s1=f.sub(Big(),c1),s2=f.sub(f.sub(Big(),f.mul(c1,s1)),f.mul(Big(2),c2));
 Big s3=f.sub(f.sub(f.sub(Big(),f.mul(c1,s2)),f.mul(c2,s1)),f.mul(Big(3),c3));
 Poly base={f.mul(Big(-2),s1),Big(long(2*m+1))};
 Poly numerator=plus(f,plus(f,times(f,base,k2),scale(f,times(f,deriv(f,cubic),times(f,kd,k)),Big(2)),true),scale(f,times(f,cubic,plus(f,times(f,kdd,k),times(f,kd,kd),true)),Big(4)),true);
 Poly h=plus(f,times(f,deriv(f,numerator),k2),times(f,numerator,deriv(f,k2)),true);
 Big target_a=f.sub(a,f.mul(Big(10),f.add(f.mul(Big(3),s2),f.mul(a,Big(long(m))))));
 Big target_b=f.sub(b,f.mul(Big(14),f.add(f.add(f.mul(Big(5),s3),f.mul(Big(3),f.mul(a,s1))),f.mul(Big(long(2*m)),b))));
 Json target=object({{"a",token(target_a.str())},{"b",token(target_b.str())}});
 Json edge=object({{"degree",number(ell)},{"frobenius_eigenvalue",number(rho)},{"kernel_polynomial",array(k)},{"x_numerator",array(numerator)},{"x_denominator",array(k2)},{"derivative_numerator",array(h)}});
 Map check(edge,target);check.verify(f,a,b);return {edge,target};
}
struct Parameters {
 std::string name;std::unique_ptr<EC_GROUP,decltype(&EC_GROUP_free)> group;Field f;Big a,b,n,t,gx,gy;
 explicit Parameters(const std::string& name):name(name),group(curve_group(name),EC_GROUP_free),f("3"){
  require(bool(group),"curve group");require(EC_GROUP_get_curve(group.get(),f.p.p,a.p,b.p,f.ctx),"parameters");require(EC_GROUP_get_order(group.get(),n.p,f.ctx),"order");require(BN_check_prime(n.p,f.ctx,nullptr)==1,"prime subgroup");require(EC_POINT_get_affine_coordinates(group.get(),EC_GROUP_get0_generator(group.get()),gx.p,gy.p,f.ctx),"generator");require(BN_copy(t.p,f.p.p)&&BN_add_word(t.p,1)&&BN_sub(t.p,t.p,n.p),"trace");
 }
 Json curve(){return object({{"name",token(name)},{"p",token(f.p.str())},{"a",token(a.str())},{"b",token(b.str())},{"n",token(n.str())},{"t",token(t.str())},{"gx",token(gx.str())},{"gy",token(gy.str())}});}
};
static std::pair<unsigned,Big> marked_label(Parameters& p,unsigned rho,unsigned ell,Big& lift_c){
 Big nm1=p.n;require(BN_sub_word(nm1.p,1),"n-1");Field subgroup(p.n.str());
 for(long j:{0,-1,1,-2,2}){
  lift_c=Big(long(rho)+j*long(ell));Big initial;require(BN_sub(initial.p,Big(1).p,lift_c.p),"initial label");initial=subgroup.mod(initial);
  for(unsigned s:{3,5,7,11,13,17,19,23,29,31})if(BN_mod_word(nm1.p,s)==0){Big exponent=nm1,mu;require(BN_div_word(exponent.p,s)==0,"projection exponent");require(BN_mod_exp(mu.p,initial.p,exponent.p,p.n.p,p.f.ctx),"projection");if(!BN_is_one(mu.p)&&!mu.zero())return {s,mu};}
 }
 throw std::runtime_error("no nonidentity odd prime projection; preserve initial label and amend scope before changing root");
}
static unsigned select_root(Parameters& p,unsigned ell,unsigned chosen){
 unsigned tp=unsigned(BN_mod_word(p.t.p,ell)),pp=unsigned(BN_mod_word(p.f.p.p,ell));std::vector<unsigned> roots;
 for(unsigned r=1;r<ell;++r)if((r*r+ell*ell-tp*r+pp)%ell==0)roots.push_back(r);
 require(roots.size()==2,"requested distinct-split horizontal prime is not distinct split");
 if(!chosen)chosen=roots[0];
 require(std::find(roots.begin(),roots.end(),chosen)!=roots.end(),"specified root not characteristic root");return chosen;
}
static Json orbit_fixtures(Parameters& p,const Big& mu,unsigned order){
 Json result;result.kind=Json::Array;
 for(long input:{0,1,17}){
  auto P=point(p.group.get());Big k(input);require(EC_POINT_mul(p.group.get(),P.get(),k.p,nullptr,nullptr,p.f.ctx),"orbit input");
  Big power(1),best_label;Affine best;bool have=false;
  for(unsigned j=0;j<(input?order:1);++j){for(int sign:{1,-1}){
   Big label=power;if(sign<0)require(BN_mod_sub(label.p,Big().p,power.p,p.n.p,p.f.ctx),"orbit sign");auto Y=point(p.group.get());require(EC_POINT_mul(p.group.get(),Y.get(),nullptr,P.get(),label.p,p.f.ctx),"reference orbit");Affine a;if(!EC_POINT_is_at_infinity(p.group.get(),Y.get())){a.infinity=false;require(EC_POINT_get_affine_coordinates(p.group.get(),Y.get(),a.x.p,a.y.p,p.f.ctx),"orbit affine");}
   if(!have||(!a.infinity&&(BN_cmp(a.x.p,best.x.p)<0||(a.x==best.x&&BN_cmp(a.y.p,best.y.p)<0)))){best=a;best_label=label;have=true;}
  }require(BN_mod_mul(power.p,power.p,mu.p,p.n.p,p.f.ctx),"orbit power");}
  Json representative;if(!best.infinity){representative.kind=Json::Array;representative.array={token(best.x.str()),token(best.y.str())};}
  result.array.push_back(object({{"k",number(input)},{"representative",representative},{"transporter",token(best_label.str())}}));
 }return result;
}
int main(int argc,char**argv){try{
 if(argc==4&&std::string(argv[1])=="--compare"){
  auto generated=read_json(argv[2]),saved=read_json(argv[3]);
  for(auto key:{"p","n","trace"})require(Big(generated.at(key).str())==Big(saved.at(key).str()),"positive control parameter");
  for(auto key:{"kernel_polynomial","x_numerator","x_denominator","derivative_numerator"}){
   require(coeffs(generated.at("first").at("edge").at(key))==coeffs(saved.at("first").at("edge").at(key)),"positive forward polynomial");
   require(coeffs(generated.at("dual_normalized").at(key))==coeffs(saved.at("dual_normalized").at(key)),"positive dual polynomial");
  }
  for(auto key:{"a","b"})require(Big(generated.at("scaled_endpoint").at(key).str())==Big(saved.at("scaled_endpoint").at(key).str()),"positive endpoint");
  for(auto key:{"x_scale","y_scale"})require(Big(generated.at("closing_isomorphism").at(key).str())==Big(saved.at("closing_isomorphism").at(key).str()),"positive scale");
  std::cout<<"{\"native_constructor_positive_control\":true,\"exact_polynomial_tokens\":true,\"timing_performed\":false}\n";return 0;
 }
 require(argc==5||argc==6,"construct --preflight CURVE ELL ROOT | construct --construct CURVE ELL ROOT OUTDIR");
 std::string mode=argv[1],name=argv[2];require((mode=="--preflight"&&argc==5)||(mode=="--construct"&&argc==6),"mode");unsigned ell=std::stoul(argv[3]),chosen=std::stoul(argv[4]);require(ell>=3&&ell<=19&&(ell&1),"bounded degree");Parameters p(name);unsigned rho=select_root(p,ell,chosen);Big initial_c;auto label=marked_label(p,rho,ell,initial_c);unsigned s=label.first;Big mu=label.second;
 std::cout<<"{\"curve\":"<<json_quote(name)<<",\"degree\":"<<ell<<",\"root\":"<<rho<<",\"initial_c\":"<<json_quote(initial_c.str())<<",\"marked_order\":"<<s<<",\"preflight_pass\":true,\"timing_performed\":false}"<<std::endl;if(mode=="--preflight")return 0;
 std::filesystem::path out(argv[5]);require(!std::filesystem::exists(out),"write-once construction directory");
 auto first=velu(p.f,p.a,p.b,ell,rho);unsigned other=0;for(unsigned r=1;r<ell;++r)if(r*rho%ell==BN_mod_word(p.f.p.p,ell))other=r;require(other&&other!=rho,"opposite root");auto back=velu(p.f,Big(first.second.at("a").str()),Big(first.second.at("b").str()),ell,other);
 Big l{long(ell)},l2=p.f.mul(l,l),l3=p.f.mul(l2,l),c;
 require(BN_mod_sub(c.p,Big(1).p,mu.p,p.n.p,p.f.ctx),"CRT initial");bool crt=false;for(unsigned j=0;j<ell;++j){if(BN_mod_word(c.p,ell)==rho){crt=true;break;}require(BN_add(c.p,c.p,p.n.p),"CRT lift");}require(crt,"CRT solution");Big modulus,twice;require(BN_mul(modulus.p,p.n.p,l.p,p.f.ctx)&&BN_lshift1(twice.p,c.p),"CRT modulus");if(BN_cmp(twice.p,modulus.p)>0)require(BN_sub(c.p,c.p,modulus.p),"centered CRT");Big aa=c;BN_set_negative(aa.p,!BN_is_negative(aa.p)&&!aa.zero());Big norm,ct;require(BN_mul(norm.p,c.p,c.p,p.f.ctx)&&BN_mul(ct.p,c.p,p.t.p,p.f.ctx)&&BN_sub(norm.p,norm.p,ct.p)&&BN_add(norm.p,norm.p,p.f.p.p),"norm");Big residual,remainder;require(BN_div(residual.p,remainder.p,norm.p,l.p,p.f.ctx)&&remainder.zero(),"quotient degree");
 Json marked=object({{"name",token(name)},{"s",number(s)},{"mu",token(mu.str())},{"projection_lift_c",token(initial_c.str())},{"alpha_a",token(aa.str())},{"alpha_b",number(1)},{"norm",token(norm.str())},{"residual_degree",token(residual.str())},{"differential",token(p.f.mod(aa).str())},{"orientation_root",number(rho)},{"degree",number(ell)},{"orbit_checks",orbit_fixtures(p,mu,s)}});
 Json closure=object({{"name",token(name)},{"p",token(p.f.p.str())},{"n",token(p.n.str())},{"trace",token(p.t.str())},{"first",object({{"edge",first.first},{"target",first.second}})},{"dual_normalized",back.first},{"scaled_endpoint",back.second},{"opposite_root",number(other)},{"closing_isomorphism",object({{"x_scale",token(p.f.inv(l2).str())},{"y_scale",token(p.f.inv(l3).str())}})}});
 Fixture fixture(name,p.curve(),closure,marked);qualify(std::vector<Fixture>{fixture});
 std::filesystem::create_directories(out);for(const auto& item:std::vector<std::pair<std::string,Json>>{{"curve.json",p.curve()},{"closure.json",closure},{"marked.json",marked}}){std::ofstream file(out/item.first);require(bool(file),"output file");file<<canonical_json(item.second)<<'\n';require(bool(file),"output write");}return 0;
}catch(const std::exception& e){std::cerr<<"native construction refused/failed: "<<e.what()<<'\n';return 1;}}
