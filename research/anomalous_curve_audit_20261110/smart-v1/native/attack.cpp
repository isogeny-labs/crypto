#include <gmpxx.h>
#include <cstdlib>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <stdexcept>
#include <string>
#include <vector>

// Public input only: p, a, b, Gx, Gy, Qx, Qy. No expected scalar is read.
// EFD add-1998-cmo / dbl-1998-cmo polynomials, evaluated modulo p^2.
// This is not a general p-adic/group-scheme refinement theorem.
struct PublicInput {mpz_class p,a,b,gx,gy,qx,qy;};
struct J {mpz_class x,y,z;};
struct Step {std::string op,branch;mpz_class prefix;J point;};
mpz_class mod(mpz_class n,const mpz_class& m) {n%=m;if(n<0)n+=m;return n;}
mpz_class inverse(const mpz_class& n,const mpz_class& m) {
  mpz_class r;if(!mpz_invert(r.get_mpz_t(),n.get_mpz_t(),m.get_mpz_t()))
    throw std::runtime_error("nonunit inverse argument");
  return r;
}
std::string quote(const std::string& s) {
  std::string r="\"";const char* hex="0123456789abcdef";
  for(unsigned char c:s) {
    if(c=='"'||c=='\\'){r+='\\';r+=char(c);}
    else if(c<32){r+="\\u00";r+=hex[c>>4];r+=hex[c&15];}
    else r+=char(c);
  }
  return r+'"';
}
std::string q(const mpz_class& n) {return quote(n.get_str());}
PublicInput input(const std::string& path) {
  std::ifstream in(path);PublicInput u;
  if(!(in>>u.p>>u.a>>u.b>>u.gx>>u.gy>>u.qx>>u.qy))
    throw std::runtime_error("malformed seven-integer input");
  std::string extra;if(in>>extra)throw std::runtime_error("extra input token");
  if(u.p<=3||mpz_even_p(u.p.get_mpz_t()))throw std::runtime_error("odd p>3 required");
  for(const auto* n:{&u.a,&u.b,&u.gx,&u.gy,&u.qx,&u.qy})
    if(*n<0||*n>=u.p)throw std::runtime_error("input outside canonical field range");
  return u;
}
bool onCurve(const PublicInput& u,const mpz_class& x,const mpz_class& y) {
  return mod(y*y-x*x*x-u.a*x-u.b,u.p)==0;
}
struct HenselLift {mpz_class value,quotient,correction,inverseDenominator;};
HenselLift liftY(const PublicInput& u,const mpz_class& a,const mpz_class& b,
               const mpz_class& x,const mpz_class& y) {
  const mpz_class difference=x*x*x+a*x+b-y*y;
  if(mod(difference,u.p)!=0)throw std::runtime_error("Hensel numerator not divisible by p");
  const mpz_class t=difference/u.p;
  const mpz_class inverseDenominator=inverse(mod(2*y,u.p),u.p);
  const mpz_class c=mod(t*inverseDenominator,u.p);
  const mpz_class result=y+u.p*c,N=u.p*u.p;
  if(mod(result*result-x*x*x-a*x-b,N)!=0||mod(result,u.p)!=y)
    throw std::runtime_error("Hensel lift check failed");
  return {result,t,c,inverseDenominator};
}
struct RingProgram {
  mpz_class N,a,b;
  mpz_class r(const mpz_class& n) const {return mod(n,N);}
  J dbl(const J& p) const {
    if(p.z==0)return {1,1,0};
    const mpz_class yy=r(p.y*p.y),zz=r(p.z*p.z);
    const mpz_class s=r(4*p.x*yy),m=r(3*p.x*p.x+a*zz*zz);
    const mpz_class x=r(m*m-2*s);
    return {x,r(m*(s-x)-8*yy*yy),r(2*p.y*p.z)};
  }
  J add(const J& p,const J& q,std::string& branch) const {
    if(p.z==0){branch="left_zero";return q;}
    if(q.z==0){branch="right_zero";return p;}
    const mpz_class z1z1=r(p.z*p.z),z2z2=r(q.z*q.z);
    const mpz_class u1=r(p.x*z2z2),u2=r(q.x*z1z1);
    const mpz_class s1=r(p.y*q.z*z2z2),s2=r(q.y*p.z*z1z1);
    if(u1==u2) {
      if(s1==s2){branch="double";return dbl(p);}
      branch="vertical";return {1,1,0};
    }
    branch="secant";
    const mpz_class h=r(u2-u1),rr=r(s2-s1),hh=r(h*h),hhh=r(h*hh),v=r(u1*hh);
    const mpz_class x=r(rr*rr-hhh-2*v);
    return {x,r(rr*(v-x)-s1*hhh),r(p.z*q.z*h)};
  }
  void check(const J& p,const mpz_class& characteristic) const {
    const mpz_class zz=r(p.z*p.z),z4=r(zz*zz),z6=r(z4*zz);
    if(r(p.y*p.y-p.x*p.x*p.x-a*p.x*z4-b*z6)!=0)
      throw std::runtime_error("Jacobian polynomial equation failed");
    if(mod(p.x,characteristic)==0&&mod(p.y,characteristic)==0&&mod(p.z,characteristic)==0)
      throw std::runtime_error("nonprimitive Jacobian tuple");
  }
  std::vector<Step> chain(const mpz_class& scalar,const J& base,
                          const mpz_class& characteristic) const {
    std::vector<Step> trace{{"base","base",1,base}};
    J current=base;mpz_class prefix=1;check(current,characteristic);
    const std::string bits=scalar.get_str(2);
    for(size_t i=1;i<bits.size();i++) {
      current=dbl(current);prefix*=2;check(current,characteristic);
      trace.push_back({"double","double",prefix,current});
      if(bits[i]=='1') {
        std::string branch;current=add(current,base,branch);prefix+=1;check(current,characteristic);
        trace.push_back({"add",branch,prefix,current});
      }
    }
    if(prefix!=scalar)throw std::runtime_error("binary chain prefix mismatch");
    return trace;
  }
};
void writeTrace(const std::filesystem::path& path,const std::vector<Step>& t,
                const PublicInput& u,const RingProgram& ring) {
  if(std::filesystem::exists(path))throw std::runtime_error("trace already exists");
  std::ofstream out(path);if(!out)throw std::runtime_error("trace open failed");
  out<<"{\"schema\":\"smart-jacobian-trace/v1\",\"p\":"<<q(u.p)
     <<",\"modulus\":"<<q(ring.N)<<",\"a\":"<<q(ring.a)<<",\"b\":"<<q(ring.b)
     <<",\"nodes\":[\n";
  for(size_t i=0;i<t.size();i++) {
    if(i)out<<",\n";
    const auto& s=t[i];
    out<<"{\"op\":"<<quote(s.op)<<",\"branch\":"<<quote(s.branch)
       <<",\"prefix\":"<<q(s.prefix)<<",\"X\":"<<q(s.point.x)
       <<",\"Y\":"<<q(s.point.y)<<",\"Z\":"<<q(s.point.z)<<"}";
  }
  out<<"\n]}\n";if(!out)throw std::runtime_error("trace write failed");
}
struct Phi {mpz_class inverseY,value,coefficient;};
Phi phi(const J& j,const PublicInput& u,const mpz_class& N) {
  const mpz_class invY=inverse(j.y,N),value=mod(-j.x*j.z*invY,N);
  if(mod(value,u.p)!=0)throw std::runtime_error("p multiple outside reduction kernel");
  return {invY,value,mpz_class(value/u.p)};
}
int main(int argc,char** argv) {
  try {
    if(argc!=3)throw std::runtime_error("usage: attack PUBLIC_INPUT TRACE_DIRECTORY");
    const PublicInput u=input(argv[1]);
    if(!onCurve(u,u.gx,u.gy)||!onCurve(u,u.qx,u.qy)) {
      std::cout<<"{\"status\":\"rejected\",\"reason\":\"input point off curve\"}\n";
      return 2;
    }
    if(mpz_sizeinbase(u.p.get_mpz_t(),2)>32&&!std::getenv("FROZEN_SMART_PROTOCOL_SHA256"))
      throw std::runtime_error("large fixture requires the frozen-protocol runner");
    const std::filesystem::path dir=argv[2];
    if(std::filesystem::exists(dir))throw std::runtime_error("refusing trace directory overwrite");
    std::filesystem::create_directories(dir);
    std::cout<<"{\"schema\":\"smart-known-answer/v1\",\"status\":\"processed\",\"p_bits\":"
      <<mpz_sizeinbase(u.p.get_mpz_t(),2)<<",\"lifts\":[";
    int nondegenerate=0;bool error=false;
    for(int i=0;i<4;i++) {
      if(i)std::cout<<",";
      try {
        const mpz_class N=u.p*u.p,A=u.a+i*u.p,B=u.b+(2*i+1)*u.p;
        const HenselLift gl=liftY(u,A,B,u.gx,u.gy),ql=liftY(u,A,B,u.qx,u.qy);
        const RingProgram ring{N,A,B};
        const auto gt=ring.chain(u.p,{u.gx,gl.value,1},u.p),qt=ring.chain(u.p,{u.qx,ql.value,1},u.p);
        const std::string gn="lift-"+std::to_string(i)+"-G.json",qn="lift-"+std::to_string(i)+"-Q.json";
        writeTrace(dir/gn,gt,u,ring);writeTrace(dir/qn,qt,u,ring);
        const Phi pg=phi(gt.back().point,u,N),pq=phi(qt.back().point,u,N);
        std::cout<<"{\"lift\":"<<i<<",\"a\":"<<q(A)<<",\"b\":"<<q(B)
          <<",\"G_y_lift\":"<<q(gl.value)<<",\"Q_y_lift\":"<<q(ql.value)
          <<",\"G_hensel_quotient\":"<<q(gl.quotient)<<",\"Q_hensel_quotient\":"<<q(ql.quotient)
          <<",\"G_hensel_correction\":"<<q(gl.correction)<<",\"Q_hensel_correction\":"<<q(ql.correction)
          <<",\"G_hensel_inverse\":"<<q(gl.inverseDenominator)<<",\"Q_hensel_inverse\":"<<q(ql.inverseDenominator)
          <<",\"G_inverse_Y\":"<<q(pg.inverseY)<<",\"Q_inverse_Y\":"<<q(pq.inverseY)
          <<",\"G_phi\":"<<q(pg.value)<<",\"Q_phi\":"<<q(pq.value)
          <<",\"G_coefficient\":"<<q(pg.coefficient)<<",\"Q_coefficient\":"<<q(pq.coefficient)
          <<",\"jacobian_steps\":"<<(gt.size()+qt.size()-2)
          <<",\"G_trace\":"<<quote(gn)<<",\"Q_trace\":"<<quote(qn);
        if(pg.coefficient==0)std::cout<<",\"status\":\"degenerate\",\"G_inverse_coefficient\":null,\"result\":null}";
        else {
          const mpz_class inverseCoefficient=inverse(pg.coefficient,u.p);
          const mpz_class result=mod(pq.coefficient*inverseCoefficient,u.p);
          nondegenerate++;
          std::cout<<",\"status\":\"nondegenerate\",\"G_inverse_coefficient\":"<<q(inverseCoefficient)
            <<",\"result\":"<<q(result)<<"}";
        }
      } catch(const std::exception& e) {
        error=true;std::cout<<"{\"lift\":"<<i<<",\"status\":\"error\",\"reason\":"<<quote(e.what())<<"}";
      }
    }
    std::cout<<"],\"nondegenerate\":"<<nondegenerate<<"}\n";
    return !error&&nondegenerate>0?0:3;
  } catch(const std::exception& e) {std::cerr<<e.what()<<"\n";return 64;}
}
