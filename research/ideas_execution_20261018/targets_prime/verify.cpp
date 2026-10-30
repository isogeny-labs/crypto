// Replay certificates in independent Jacobian coordinates. The experiment uses
// affine addition; this verifier never calls Curve::add or Curve::mul.
#define main experiment_main_unused
#include "diagnostic.cpp"
#undef main
#include <regex>

U number(const std::string&s,const std::string&key){auto i=s.find("\""+key+"\":");if(i==std::string::npos)throw std::runtime_error("missing key: "+key);i+=key.size()+3;return std::stoull(s.substr(i));}
struct J {U x=0,y=1,z=0;};
J jd(const F&f,U a,J p){if(!p.z||!p.y)return {};U A=f.mul(p.x,p.x),B=f.mul(p.y,p.y),C=f.mul(B,B),D=f.mul(2,f.sub(f.sub(f.mul(f.add(p.x,B),f.add(p.x,B)),A),C)),zz=f.mul(p.z,p.z),E=f.add(f.mul(3,A),f.mul(a,f.mul(zz,zz))),FF=f.mul(E,E);U x=f.sub(FF,f.mul(2,D));return {x,f.sub(f.mul(E,f.sub(D,x)),f.mul(8,C)),f.mul(2,f.mul(p.y,p.z))};}
J ja(const F&f,U a,J p,J q){if(!p.z)return q;if(!q.z)return p;U z1=f.mul(p.z,p.z),z2=f.mul(q.z,q.z),u1=f.mul(p.x,z2),u2=f.mul(q.x,z1),s1=f.mul(p.y,f.mul(q.z,z2)),s2=f.mul(q.y,f.mul(p.z,z1));if(u1==u2)return s1==s2?jd(f,a,p):J{};U h=f.sub(u2,u1),i=f.mul(f.mul(2,h),f.mul(2,h)),j=f.mul(h,i),r=f.mul(2,f.sub(s2,s1)),v=f.mul(u1,i),x=f.sub(f.sub(f.mul(r,r),j),f.mul(2,v));return {x,f.sub(f.mul(r,f.sub(v,x)),f.mul(2,f.mul(s1,j))),f.mul(f.sub(f.sub(f.mul(f.add(p.z,q.z),f.add(p.z,q.z)),z1),z2),h)};}
P affine(const F&f,J p){if(!p.z)return {};U zi=f.inv(p.z),z2=f.mul(zi,zi);return {f.mul(p.x,z2),f.mul(p.y,f.mul(z2,zi)),false};}
int main(int argc,char**argv){try{if(argc!=2)throw std::runtime_error("usage: verify RUN_DIRECTORY");std::filesystem::path dir=argv[1];std::ifstream r(dir/"results.jsonl"),c(dir/"certificates.jsonl");if(!r||!c)throw std::runtime_error("missing input");struct C{U p,a,b,B;bool ext;};std::map<U,C>curves;std::string line;while(std::getline(r,line)){if(line.find("\"status\"")!=std::string::npos)continue;U item=number(line,"item");curves.insert_or_assign(number(line,"curve_id"),C{number(line,"p"),number(line,"a"),number(line,"b"),item==14?number(line,"B"):0,item==1});}
std::regex pair("\\[([0-9]+),([0-9]+)\\]");U count=0,m4=0,fp2=0;while(std::getline(c,line)){auto z=curves.at(number(line,"curve_id"));F f(z.p,z.ext);std::vector<P>p;for(std::sregex_iterator it(line.begin(),line.end(),pair),end;it!=end;++it)p.push_back({std::stoull((*it)[1]),std::stoull((*it)[2]),false});U arity=z.ext?2:4;if(p.size()!=arity+1)throw std::runtime_error("certificate arity");J acc;for(size_t i=0;i<p.size();++i){P q=p[i];if(q.x>=f.q||q.y>=f.q||f.mul(q.y,q.y)!=f.add(f.add(f.mul(f.mul(q.x,q.x),q.x),f.mul(z.a,q.x)),z.b))throw std::runtime_error("off-curve point");if(i){if(z.B&&q.x>=z.B)throw std::runtime_error("factor-base bound");acc=ja(f,z.a,acc,{q.x,q.y,1});}}if(affine(f,acc)!=p[0])throw std::runtime_error("Jacobian certificate sum mismatch");++count;if(z.ext)++fp2;else++m4;}
std::cout<<"PASS independent Jacobian replay: "<<count<<" certificates (Fp2="<<fp2<<", prime m4="<<m4<<")\n";return 0;}catch(std::exception&e){std::cerr<<"FAIL: "<<e.what()<<'\n';return 1;}}
