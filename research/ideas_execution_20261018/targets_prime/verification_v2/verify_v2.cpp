// Additive workload-bound certificate replay. Not a new scientific run.
#define main diagnostic_main_unused
#include "../diagnostic.cpp"
#undef main
#include "vendor/json.hpp"
#include <openssl/evp.h>
#include <memory>
#include <optional>

using Json=nlohmann::json;
static void need(bool value,const std::string& reason){if(!value)throw std::runtime_error(reason);}
static Json parse(const std::string& line){
 std::vector<std::set<std::string>> keys;
 auto callback=[&](int,Json::parse_event_t event,Json& parsed){
  if(event==Json::parse_event_t::object_start)keys.emplace_back();
  if(event==Json::parse_event_t::key)need(keys.back().insert(parsed.get<std::string>()).second,"duplicate JSON key");
  if(event==Json::parse_event_t::object_end)keys.pop_back();
  return true;
 };
 Json j=Json::parse(line,callback);need(j.is_object(),"expected JSON object");return j;
}
static U num(const Json& j,const char* key){need(j.contains(key)&&j[key].is_number_unsigned(),std::string("missing unsigned integer: ")+key);return j[key].get<U>();}
static bool flag(const Json& j,const char* key){need(j.contains(key),std::string("missing flag: ")+key);if(j[key].is_boolean())return j[key].get<bool>();U value=num(j,key);need(value<=1,"flag outside zero/one");return value!=0;}
static P point(const Json& j){need(j.is_array()&&j.size()==2&&j[0].is_number_unsigned()&&j[1].is_number_unsigned(),"invalid point encoding");return {j[0].get<U>(),j[1].get<U>(),false};}
static std::string hash_file(const std::filesystem::path& path){
 std::ifstream in(path,std::ios::binary);need(bool(in),"hash input open");
 auto ctx=std::unique_ptr<EVP_MD_CTX,decltype(&EVP_MD_CTX_free)>(EVP_MD_CTX_new(),EVP_MD_CTX_free);
 need(ctx&&EVP_DigestInit_ex(ctx.get(),EVP_sha256(),nullptr)==1,"SHA init");char bytes[65536];
 while(in){in.read(bytes,sizeof bytes);need(EVP_DigestUpdate(ctx.get(),bytes,size_t(in.gcount()))==1,"SHA update");}
 need(in.eof(),"hash input read");unsigned char digest[32];unsigned count=0;
 need(EVP_DigestFinal_ex(ctx.get(),digest,&count)==1&&count==32,"SHA final");std::ostringstream out;
 for(unsigned i=0;i<count;++i)out<<std::hex<<std::setw(2)<<std::setfill('0')<<unsigned(digest[i]);return out.str();
}
struct J {U x=0,y=1,z=0;};
// Jacobian formulas retained from the independent-coordinate verifier.
static J jd(const F&f,U a,J p){if(!p.z||!p.y)return {};U A=f.mul(p.x,p.x),B=f.mul(p.y,p.y),C=f.mul(B,B),D=f.mul(2,f.sub(f.sub(f.mul(f.add(p.x,B),f.add(p.x,B)),A),C)),zz=f.mul(p.z,p.z),E=f.add(f.mul(3,A),f.mul(a,f.mul(zz,zz))),FF=f.mul(E,E);U x=f.sub(FF,f.mul(2,D));return {x,f.sub(f.mul(E,f.sub(D,x)),f.mul(8,C)),f.mul(2,f.mul(p.y,p.z))};}
static J ja(const F&f,U a,J p,J q){if(!p.z)return q;if(!q.z)return p;U z1=f.mul(p.z,p.z),z2=f.mul(q.z,q.z),u1=f.mul(p.x,z2),u2=f.mul(q.x,z1),s1=f.mul(p.y,f.mul(q.z,z2)),s2=f.mul(q.y,f.mul(p.z,z1));if(u1==u2)return s1==s2?jd(f,a,p):J{};U h=f.sub(u2,u1),i=f.mul(f.mul(2,h),f.mul(2,h)),j=f.mul(h,i),r=f.mul(2,f.sub(s2,s1)),v=f.mul(u1,i),x=f.sub(f.sub(f.mul(r,r),j),f.mul(2,v));return {x,f.sub(f.mul(r,f.sub(v,x)),f.mul(2,f.mul(s1,j))),f.mul(f.sub(f.sub(f.mul(f.add(p.z,q.z),f.add(p.z,q.z)),z1),z2),h)};}
static P affine(const F&f,J p){if(!p.z)return {};U zi=f.inv(p.z),z2=f.mul(zi,zi);return {f.mul(p.x,z2),f.mul(p.y,f.mul(z2,zi)),false};}
static J jm(const F&f,U a,U k,P p){J out,base{p.x,p.y,p.inf?0ULL:1ULL};while(k){if(k&1)out=ja(f,a,out,base);base=jd(f,a,base);k>>=1;}return out;}
static bool is_prime(U value){if(value<2)return false;for(U divisor=2;divisor<=value/divisor;++divisor)if(value%divisor==0)return false;return true;}
static U checked_p(U p){need(p>=17&&p<=10000&&is_prime(p),"unsupported prime-field size");return p;}
struct Meta {
 U item,p,a,b,r,h,B,N;bool ext;F f;std::optional<P> generator;std::set<P> projected_base;
 explicit Meta(const Json& row):item(num(row,"item")),p(checked_p(num(row,"p"))),a(num(row,"a")),b(num(row,"b")),r(num(row,"r")),h(item==1?num(row,"cofactor"):1),B(item==14?num(row,"B"):0),N(0),ext(item==1),f(p,ext){
  need((item==1||item==14)&&p<=10000&&is_prime(p),"unsupported field/item");need(f.q<=1000000&&a<f.q&&b<f.q&&is_prime(r),"invalid curve parameters");
  // Integers must be embedded in the prime subfield. At p=17 the packed
  // extension encoding of integer 27 is 10+u, whereas this coefficient is 10.
  need(f.add(f.mul(4%p,f.mul(f.mul(a,a),a)),f.mul(27%p,f.mul(b,b)))!=0,"singular curve");
  if(row.contains("generator")){generator=point(row["generator"]);check_point(*generator);need(!jm(f,a,r,*generator).z,"generator outside r subgroup");}
  std::map<U,std::vector<U>> square_roots;for(U y=0;y<f.q;++y)square_roots[f.mul(y,y)].push_back(y);
  N=1;for(U x=0;x<f.q;++x){U rhs=f.add(f.add(f.mul(f.mul(x,x),x),f.mul(a,x)),b);auto it=square_roots.find(rhs);if(it==square_roots.end())continue;N+=it->second.size();
   if(ext&&x<p)for(U y:it->second){P projected=affine(f,jm(f,a,h,P{x,y,false}));if(!projected.inf)projected_base.insert(projected);}
  }
  need(N==r*h,"source point count/subgroup mismatch");if(row.contains("order"))need(num(row,"order")==N,"source order mismatch");
  if(ext){need(row.contains("nonresidue")&&num(row,"nonresidue")==f.d,"field representation mismatch");need(h%2==0,"missing even cofactor");P torsion=point(row["torsion"]);check_point(torsion);need(!jm(f,a,2,torsion).z&&!jm(f,a,h,torsion).z,"torsion assertion mismatch");}
 }
 void check_point(P pnt)const{need(!pnt.inf&&pnt.x<f.q&&pnt.y<f.q&&f.mul(pnt.y,pnt.y)==f.add(f.add(f.mul(f.mul(pnt.x,pnt.x),pnt.x),f.mul(a,pnt.x)),b),"off-curve point");}
 void consistent(const Json& row)const{need(num(row,"item")==item&&num(row,"p")==p&&num(row,"a")==a&&num(row,"b")==b&&num(row,"r")==r,"inconsistent curve metadata");if(item==14)need(num(row,"B")==B,"inconsistent factor-base bound");if(row.contains("generator"))need(generator&&point(row["generator"])==*generator,"inconsistent generator");}
};
using Key=std::pair<U,U>;
struct Record {U item;P target;bool found;bool planted;};
struct Workload {std::map<Key,Record> records;std::map<U,std::unique_ptr<Meta>> curves;U lines=0,unavailable=0;std::string digest;};
static Key key(const Json& row){U item=num(row,"item");need(item==1||item==14,"unknown item");if(item==1){need(!row.contains("target_id"),"unexpected Fp2 target ID");return {num(row,"curve_id"),0};}return {num(row,"curve_id"),num(row,"target_id")};}
static Workload workload(const std::filesystem::path& dir,const std::string& expected,bool strict,const Workload* original=nullptr){
 Workload out;out.digest=hash_file(dir/"results.jsonl");need(out.digest==expected,"source results SHA256 mismatch");std::ifstream in(dir/"results.jsonl");need(bool(in),"source results open");std::string line;
 while(std::getline(in,line)){need(!line.empty(),"empty result line");++out.lines;Json row=parse(line);if(row.contains("status")){need(!strict&&row["status"]=="empty_base_no_planted_control","unsupported result status");need(num(row,"item")==14&&num(row,"target_id")==32,"unavailable control mismatch");++out.unavailable;continue;}
  Key id=key(row);need(!out.records.count(id),"duplicate source target key");U item=num(row,"item");need(!strict||item==14,"strict arm contains other item");auto found=out.curves.find(id.first);
  if(found==out.curves.end())found=out.curves.emplace(id.first,std::make_unique<Meta>(row)).first;else found->second->consistent(row);Meta& curve=*found->second;
  P target=point(row["target"]);curve.check_point(target);need(!jm(curve.f,curve.a,curve.r,target).z,"source target outside subgroup");bool planted=item==14?flag(row,"planted"):false;
  if(item==14){need((id.second==32)==planted&&id.second<=32,"source target law mismatch");need(flag(row,"reference_agreement"),"source oracle disagreement");}
  if(strict){need(original,"strict source requires original workload");auto prior=original->records.find(id);need(prior!=original->records.end(),"strict key absent from original workload");auto old=original->curves.find(id.first);need(old!=original->curves.end(),"strict curve absent from original");old->second->consistent(row);curve.generator=old->second->generator;if(!planted)need(prior->second.target==target,"strict natural target differs from original");}
  const char* answer=item==1||(!strict)?"known_answer_k":"known_k";
  if(row.contains(answer)&&!row[answer].is_null()&&curve.generator){U k=num(row,answer);need(k>0&&k<curve.r&&affine(curve.f,jm(curve.f,curve.a,k,*curve.generator))==target,"source known-answer target mismatch");}
  out.records.emplace(id,Record{item,target,flag(row,item==1?"two_sum_found":"found"),planted});
 }
 need(in.eof(),"source result read failure");return out;
}
int main(int argc,char**argv){try{
 need(argc==4||argc==6,"usage: verify_v2 ordered DIR RESULTS_SHA | strict DIR RESULTS_SHA ORIGINAL_DIR ORIGINAL_RESULTS_SHA");std::string mode=argv[1];need(mode=="ordered"||mode=="strict","unknown mode");bool strict=mode=="strict";need(strict?(argc==6):(argc==4),"mode argument mismatch");
 std::optional<Workload> original;if(strict)original=workload(argv[4],argv[5],false);Workload data=workload(argv[2],argv[3],strict,original?&*original:nullptr);
 U expected=0;for(const auto& [id,row]:data.records)expected+=row.found;std::set<Key> seen;U fp2=0,prime4=0;std::ifstream cert(std::filesystem::path(argv[2])/"certificates.jsonl");need(bool(cert),"certificate input open");std::string line;
 while(std::getline(cert,line)){need(!line.empty(),"empty certificate line");Json certificate=parse(line);Key id=key(certificate);auto row=data.records.find(id);need(row!=data.records.end(),"unexpected certificate target key");need(row->second.found,"certificate for a source miss");need(seen.insert(id).second,"duplicate certificate target key");need(num(certificate,"item")==row->second.item,"certificate item mismatch");P target=point(certificate["target"]);need(target==row->second.target,"certificate target differs from frozen source target");
  auto& meta=*data.curves.at(id.first);need(certificate.contains("points")&&certificate["points"].is_array(),"missing point list");const auto& points=certificate["points"];need(points.size()==(meta.ext?2:4),"certificate arity");J total;std::set<U> distinct_x;
  for(const auto& encoded:points){P pnt=point(encoded);meta.check_point(pnt);need(!jm(meta.f,meta.a,meta.r,pnt).z,"certificate point outside subgroup");if(meta.ext)need(meta.projected_base.count(pnt),"point outside projected Fp2 base");else need(pnt.x<meta.B,"factor-base bound");distinct_x.insert(pnt.x);total=ja(meta.f,meta.a,total,{pnt.x,pnt.y,1});}
  if(strict){need(distinct_x.size()==4,"strict abscissae are not distinct");if(certificate.contains("distinct_abscissae"))need(flag(certificate,"distinct_abscissae"),"false strictness label");}
  need(affine(meta.f,total)==target,"Jacobian certificate sum mismatch");meta.ext?++fp2:++prime4;
 }
 need(cert.eof(),"certificate input read failure");need(seen.size()==expected,"missing certificates for source hits");for(const auto& [id,row]:data.records)need(!row.found||seen.count(id),"missing source target certificate");
 Json report={{"schema","targets-prime-verification/v2"},{"status","verified"},{"mode",mode},{"source_results_sha256",data.digest},{"certificate_sha256",hash_file(std::filesystem::path(argv[2])/"certificates.jsonl")},{"source_rows",data.lines},{"actual_target_rows",data.records.size()},{"unavailable_control_rows",data.unavailable},{"source_hits",expected},{"verified_certificates",seen.size()},{"fp2_certificates",fp2},{"prime4_certificates",prime4},{"strict_distinctness_enforced",strict},{"exact_target_and_coverage_binding",true},{"group_law","Jacobian formulas; finite-field arithmetic shared"},{"scope","additive frozen-record verification; no scientific trials"}};
 if(original)report["original_reference_results_sha256"]=original->digest;std::cout<<report.dump()<<"\n";return 0;
}catch(const std::exception& error){std::cerr<<Json({{"schema","targets-prime-verification/v2"},{"status","rejected"},{"reason",error.what()}}).dump()<<"\n";return 1;}}
