#include "json.hpp"
#include <openssl/bn.h>
#include <openssl/ec.h>
#include <openssl/obj_mac.h>
#include <openssl/crypto.h>
#include <algorithm>
#include <iostream>
#include <memory>
#include <numeric>
#include <stdexcept>
#include <vector>
#include <set>
static void require(bool b,const char* s){if(!b)throw std::runtime_error(s);}
struct Big {
 BIGNUM* p=BN_new(); Big(){require(p,"BN_new");BN_zero(p);} explicit Big(const std::string& s):Big(){require(!s.empty(),"empty BIGNUM");size_t first=s[0]=='-'?1:0;require(first<s.size(),"BIGNUM digits");for(size_t i=first;i<s.size();++i)require(s[i]>='0'&&s[i]<='9',"noninteger BIGNUM");require(BN_dec2bn(&p,s.c_str())>0,"decimal BIGNUM");}
 explicit Big(long v):Big(std::to_string(v)){} Big(const Big& b):Big(){require(BN_copy(p,b.p),"BN_copy");} Big(Big&& b)noexcept:p(b.p){b.p=nullptr;}
 Big& operator=(Big b){std::swap(p,b.p);return *this;}~Big(){BN_free(p);}std::string str()const{char* s=BN_bn2dec(p);require(s,"BN_bn2dec");std::string v(s);OPENSSL_free(s);return v;}
 bool operator==(const Big& b)const{return BN_cmp(p,b.p)==0;} bool zero()const{return BN_is_zero(p);} };
struct Field {
 Big p; BN_CTX* ctx=BN_CTX_new(); explicit Field(const std::string& prime):p(prime){require(ctx,"BN_CTX_new");}~Field(){BN_CTX_free(ctx);}
 Big mod(const Big&a){Big r;require(BN_nnmod(r.p,a.p,p.p,ctx),"mod");return r;}
 Big add(const Big&a,const Big&b){Big r;require(BN_mod_add(r.p,a.p,b.p,p.p,ctx),"add");return r;}
 Big sub(const Big&a,const Big&b){Big r;require(BN_mod_sub(r.p,a.p,b.p,p.p,ctx),"sub");return r;}
 Big mul(const Big&a,const Big&b){Big r;require(BN_mod_mul(r.p,a.p,b.p,p.p,ctx),"mul");return r;}
 Big inv(const Big&a){Big r;require(BN_mod_inverse(r.p,a.p,p.p,ctx),"inverse");return r;}
 Big power(const Big&a,const Big&b){Big r;require(BN_mod_exp(r.p,a.p,b.p,p.p,ctx),"power");return r;}
};
using Poly=std::vector<Big>;
static Poly coeffs(const Json& j){Poly p;for(auto&c:j.array)p.emplace_back(c.str());return p;}
static void trim(Poly&a){while(!a.empty()&&a.back().zero())a.pop_back();}
static Poly plus(Field&f,const Poly&a,const Poly&b,bool sub=false){Poly c;for(size_t i=0;i<std::max(a.size(),b.size());++i)c.push_back(sub?f.sub(i<a.size()?a[i]:Big(),i<b.size()?b[i]:Big()):f.add(i<a.size()?a[i]:Big(),i<b.size()?b[i]:Big()));trim(c);return c;}
static Poly times(Field&f,const Poly&a,const Poly&b){if(a.empty()||b.empty())return {};Poly c(a.size()+b.size()-1);for(size_t i=0;i<a.size();++i)for(size_t j=0;j<b.size();++j)c[i+j]=f.add(c[i+j],f.mul(a[i],b[j]));trim(c);return c;}
static Poly scale(Field&f,Poly a,const Big& b){for(auto&v:a)v=f.mul(v,b);trim(a);return a;}
static Poly deriv(Field&f,const Poly&a){Poly b;for(size_t i=1;i<a.size();++i)b.push_back(f.mul(a[i],Big(long(i))));trim(b);return b;}
static Big evaluate(Field&f,const Poly&a,const Big&x){Big v;for(auto i=a.rbegin();i!=a.rend();++i)v=f.add(f.mul(v,x),*i);return v;}
struct Affine {Big x,y;bool infinity=true;};
struct Map {
 Poly N,D,H; Big target_a,target_b; int degree;
 explicit Map(const Json&j,const Json&target):N(coeffs(j.at("x_numerator"))),D(coeffs(j.at("x_denominator"))),H(coeffs(j.at("derivative_numerator"))),target_a(target.at("a").str()),target_b(target.at("b").str()),degree(j.at("degree").integer()){}
 Affine apply(Field& f,const Affine&P)const{if(P.infinity)return {};Big d=f.inv(evaluate(f,D,P.x));Affine Q;Q.infinity=false;Q.x=f.mul(evaluate(f,N,P.x),d);Q.y=f.mul(f.mul(P.y,evaluate(f,H,P.x)),f.mul(d,d));return Q;}
 void verify(Field&f,const Big&a,const Big&b)const{
  Poly curve={b,a,Big(),Big(1)};Poly lhs=times(f,curve,times(f,H,H));Poly dd=times(f,D,D);Poly rhs=times(f,D,plus(f,plus(f,times(f,times(f,N,N),N),scale(f,times(f,N,dd),target_a)),scale(f,times(f,dd,D),target_b)));
  require(lhs==rhs,"full polynomial curve identity");require(H==plus(f,times(f,deriv(f,N),D),times(f,N,deriv(f,D)),true),"derivative identity");require(int(N.size())==degree+1&&int(D.size())==degree,"degree");
 }
};
using Point=std::unique_ptr<EC_POINT,decltype(&EC_POINT_free)>;
static Point point(const EC_GROUP*g){Point p(EC_POINT_new(g),EC_POINT_free);require(bool(p),"EC_POINT_new");return p;}
struct Fixture {
 std::string name; Json curve,closure,marked;
 Fixture(std::string n,Json c,Json z,Json m):name(std::move(n)),curve(std::move(c)),closure(std::move(z)),marked(std::move(m)){}
};
static int curve_nid(const std::string& name){
 if(name=="P-192")return NID_X9_62_prime192v1;
 if(name=="P-224")return NID_secp224r1;
 if(name=="P-256")return NID_X9_62_prime256v1;
 if(name=="P-384")return NID_secp384r1;
 if(name=="P-521")return NID_secp521r1;
 throw std::runtime_error("unsupported exact curve name");
}
struct Runtime {
 const Fixture& f; Field field; Big n,mu,correction,xscale,yscale; std::unique_ptr<Map> forward,back;
 EC_GROUP* group=nullptr;
 Runtime(const Fixture&v,bool relation=true):f(v),field(v.curve.at("p").str()),n(v.curve.at("n").str()),mu(v.marked.at("mu").str()){
  group=EC_GROUP_new_by_curve_name(curve_nid(v.name));require(group,"group");Big actual;require(EC_GROUP_get_order(group,actual.p,field.ctx),"group order");require(actual==n,"exact subgroup order");if(relation){ forward=std::make_unique<Map>(v.closure.at("first").at("edge"),v.closure.at("first").at("target")); back=std::make_unique<Map>(v.closure.at("dual_normalized"),v.closure.at("scaled_endpoint")); xscale=Big(v.closure.at("closing_isomorphism").at("x_scale").str()); yscale=Big(v.closure.at("closing_isomorphism").at("y_scale").str()); Big ell(forward->degree),ei;require(BN_mod_inverse(ei.p,ell.p,n.p,field.ctx),"degree inverse");require(BN_mod_mul(correction.p,mu.p,ei.p,n.p,field.ctx),"correction");}
 }
 ~Runtime(){EC_GROUP_free(group);}
 Affine affine(const EC_POINT* P){if(EC_POINT_is_at_infinity(group,P))return {};Affine a;a.infinity=false;require(EC_POINT_get_affine_coordinates(group,P,a.x.p,a.y.p,field.ctx),"get affine");return a;}
 void set(EC_POINT* P,const Affine&a){if(a.infinity)require(EC_POINT_set_to_infinity(group,P),"set identity");else require(EC_POINT_set_affine_coordinates(group,P,a.x.p,a.y.p,field.ctx),"set affine");}
 Point transported(const EC_POINT* P){Affine a=back->apply(field,forward->apply(field,affine(P)));if(!a.infinity){a.x=field.mul(a.x,xscale);a.y=field.mul(a.y,yscale);}auto q=point(group);set(q.get(),a);return q;}
};
#include "relation_verify.hpp"
struct Counts {uint64_t doubles=0,adds=0,negations=0,table_points=0;};
static Big centered(const Big&k,const Big&n){Big r=k,twice;require(BN_lshift1(twice.p,k.p),"shift");if(BN_cmp(twice.p,n.p)>0)require(BN_sub(r.p,k.p,n.p),"center");return r;}
static std::vector<int> wnaf(const Big& scalar,unsigned width){require(width>=2&&width<=8,"window range");Big k=scalar;BN_set_negative(k.p,0);std::vector<int> out;while(!k.zero()){int d=0;if(BN_is_odd(k.p)){d=int(BN_mod_word(k.p,1u<<width));if(d>=(1<<(width-1)))d-=1<<width;if(d>0)require(BN_sub_word(k.p,d),"NAF subtract");else require(BN_add_word(k.p,-d),"NAF add");}out.push_back(d);require(BN_rshift1(k.p,k.p),"NAF shift");}return out;}
static void public_mul(Runtime&r,EC_POINT*out,const EC_POINT*P,const Big& scalar,unsigned width,Counts&cnt){
 Big k=centered(scalar,r.n);auto digits=wnaf(k,width);if(digits.empty()||EC_POINT_is_at_infinity(r.group,P)){require(EC_POINT_set_to_infinity(r.group,out),"identity");return;}
 size_t size=1u<<(width-2);std::vector<Point> table;table.reserve(size);auto two=point(r.group);if(size>1){require(EC_POINT_dbl(r.group,two.get(),P,r.field.ctx),"table double");++cnt.doubles;}
 table.push_back(point(r.group));require(EC_POINT_copy(table.back().get(),P),"table copy");for(size_t i=1;i<size;++i){table.push_back(point(r.group));require(EC_POINT_add(r.group,table[i].get(),table[i-1].get(),two.get(),r.field.ctx),"table add");++cnt.adds;}cnt.table_points=size;if(size>1){std::vector<EC_POINT*> raw;for(auto&entry:table)raw.push_back(entry.get());require(EC_POINTs_make_affine(r.group,raw.size(),raw.data(),r.field.ctx),"affine odd-multiple table");}
 require(EC_POINT_set_to_infinity(r.group,out),"zero");auto selected=point(r.group);bool started=false;
 for(auto i=digits.rbegin();i!=digits.rend();++i){if(started){require(EC_POINT_dbl(r.group,out,out,r.field.ctx),"double");++cnt.doubles;}if(*i){int d=*i;require(EC_POINT_copy(selected.get(),table.at((std::abs(d)-1)/2).get()),"select");if(d<0){require(EC_POINT_invert(r.group,selected.get(),r.field.ctx),"negate");++cnt.negations;}if(started){require(EC_POINT_add(r.group,out,out,selected.get(),r.field.ctx),"add");++cnt.adds;}else{require(EC_POINT_copy(out,selected.get()),"first digit");started=true;}}}
 if(BN_is_negative(k.p)){require(EC_POINT_invert(r.group,out,r.field.ctx),"center sign");++cnt.negations;}
}
static void external_public(Runtime&r,EC_POINT*out,const EC_POINT*P,const Big&k){
 // Two-point public multiscalar API avoids OpenSSL's mandatory single-point ladder.
 auto identity=point(r.group);require(EC_POINT_set_to_infinity(r.group,identity.get()),"identity");Big zero;
 Big public_scalar=centered(k,r.n);const EC_POINT* points[]={P,identity.get()};const BIGNUM* scalars[]={public_scalar.p,zero.p};require(EC_POINTs_mul(r.group,out,nullptr,2,points,scalars,r.field.ctx),"external public multiscalar");
}
static std::vector<Fixture> fixtures(const std::string&dir){auto cs=read_json(dir+"/published-curves.json");auto ms=read_json(dir+"/marked-relations.json");std::vector<Fixture> v;for(auto&c:cs.at("rows").array){std::string name=c.at("name").str();std::string file=name=="P-384"?"p-384-closure.json":"p-521-closure.json";const Json* marked=nullptr;for(auto&m:ms.at("rows").array)if(m.at("name").str()==name)marked=&m;require(marked,"marked fixture");v.emplace_back(name,c,read_json(dir+"/"+file),*marked);}return v;}
#include "orbit_verify.hpp"
static void qualify(const std::vector<Fixture>&v){
 uint64_t cases=0;for(auto&f:v){Runtime r(f);Big a=r.field.mod(Big(f.curve.at("a").str())),b(f.curve.at("b").str());Big actual_p,actual_a,actual_b;require(EC_GROUP_get_curve(r.group,actual_p.p,actual_a.p,actual_b.p,r.field.ctx),"backend curve");require(actual_p==r.field.p&&actual_a==a&&actual_b==b,"exact backend curve parameters");r.forward->verify(r.field,a,b);r.back->verify(r.field,r.forward->target_a,r.forward->target_b);verify_relation(r);verify_orbits(r);
  Big seventh(f.marked.at("s").str()),res;require(BN_mod_exp(res.p,r.mu.p,seventh.p,r.n.p,r.field.ctx),"seventh power");require(BN_is_one(res.p)&&!BN_is_one(r.mu.p),"primitive prime-order root");Big ell(r.forward->degree),reconstruction;require(BN_mod_mul(reconstruction.p,r.correction.p,ell.p,r.n.p,r.field.ctx),"scalar reconstruction");require(reconstruction==r.mu,"scalar label");
  for(long input:{0,1,2,17,-1}){Big k(input),positive=k;if(input<0)require(BN_sub(positive.p,r.n.p,Big(-input).p),"negative point scalar");auto P=point(r.group);require(EC_POINT_mul(r.group,P.get(),positive.p,nullptr,nullptr,r.field.ctx),"fixture point");auto ref=point(r.group);require(EC_POINT_mul(r.group,ref.get(),nullptr,P.get(),r.mu.p,r.field.ctx),"reference");auto closed=r.transported(P.get());auto expected=point(r.group);require(EC_POINT_mul(r.group,expected.get(),nullptr,P.get(),ell.p,r.field.ctx),"closure reference");require(EC_POINT_cmp(r.group,closed.get(),expected.get(),r.field.ctx)==0,"closed dual label");
   for(unsigned w=2;w<=7;++w){Counts c,d,e;auto direct=point(r.group),candidate=point(r.group),small=point(r.group),fused=point(r.group);public_mul(r,small.get(),P.get(),ell,2,e);public_mul(r,fused.get(),small.get(),r.correction,w,e);require(EC_POINT_cmp(r.group,fused.get(),ref.get(),r.field.ctx)==0,"native fused output");public_mul(r,direct.get(),P.get(),r.mu,w,c);public_mul(r,candidate.get(),closed.get(),r.correction,w,d);require(EC_POINT_cmp(r.group,direct.get(),ref.get(),r.field.ctx)==0,"native direct output");require(EC_POINT_cmp(r.group,candidate.get(),ref.get(),r.field.ctx)==0,"native candidate output");require(EC_POINT_is_on_curve(r.group,candidate.get(),r.field.ctx)==1,"output on source curve");++cases;}
   auto ext=point(r.group);external_public(r,ext.get(),P.get(),r.mu);require(EC_POINT_cmp(r.group,ext.get(),ref.get(),r.field.ctx)==0,"external public output");if(input!=0){Big wrong=r.mu;require(BN_add_word(wrong.p,1),"negative control scalar");auto bad=point(r.group);require(EC_POINT_mul(r.group,bad.get(),nullptr,P.get(),wrong.p,r.field.ctx),"negative control");require(EC_POINT_cmp(r.group,bad.get(),ref.get(),r.field.ctx)!=0,"negative control must fail");}
  }std::cout<<"{\"curve\":\""<<f.name<<"\",\"full_map_identities\":true,\"global_composition_identity\":true,\"signed_kernel_frobenius\":true,\"cm_label_norm_orientation\":true,\"orbit_fixtures\":3,\"closure_point_checks\":5,\"native_output_checks\":30,\"negative_control_rejected\":true}"<<std::endl;
 }std::cout<<"{\"qualification_pass\":true,\"cases\":"<<cases<<",\"openssl\":\""<<OpenSSL_version(OPENSSL_VERSION)<<"\",\"timing_performed\":false}"<<std::endl;
}
int main(int argc,char**argv){try{require(argc==3&&std::string(argv[1])=="--qualify","usage: native --qualify FIXTURE_DIR");require(JsonParser("{\"n\":123456789012345678901234567890}").read().at("n").str()=="123456789012345678901234567890","exact JSON number");qualify(fixtures(argv[2]));return 0;}catch(const std::exception&e){std::cerr<<"qualification failed: "<<e.what()<<std::endl;return 1;}}
