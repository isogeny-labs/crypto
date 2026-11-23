// Coordinate-input recognition in an explicit, small CM Weber field.
// Dense field/lattice dimensions grow with r; no generic efficiency claim.
#include "../native_base.hpp"
#include "lattice.hpp"
#include <fstream>
#include <numeric>
using weber::E;
using weber::P;
namespace w=weber;

Z lcm_z(const Z& a,const Z& b){Z z;mpz_lcm(z.get_mpz_t(),a.get_mpz_t(),b.get_mpz_t());return z;}
unsigned valuation(Z a,const Z& q){
    check(a!=0,"valuation of zero");unsigned v=0;
    while(a%q==0){a/=q;++v;}return v;
}
Z omega_lift(Z a,const Z& q,unsigned n){
    check(n>0,"zero precision");unsigned digits=1;
    while(digits<n){digits=std::min(2*digits,n);Z m=power(q,digits);
        a=mod(a-(a*a+a+1)*inv(2*a+1,m),m);}
    return a;
}
Z project(const E& a,const Z& q,const Z& zeta,unsigned n){
    Z denominator=lcm_z(a.a.get_den(),a.b.get_den());unsigned v=valuation(denominator,q);
    Z modulus=power(q,n+v),z=omega_lift(zeta,q,n+v);
    Z numerator=mod(a.a.get_num()*(denominator/a.a.get_den())+
                    a.b.get_num()*(denominator/a.b.get_den())*z,modulus);
    Z qv=power(q,v);check(numerator%qv==0,"coefficient has a local pole");
    return mod((numerator/qv)*inv(denominator/qv,power(q,n)),power(q,n));
}
using MP=std::vector<Z>;
void mtrim(MP& a){while(!a.empty()&&a.back()==0)a.pop_back();}
MP msub(MP a,const MP& b,const Z& q){
    a.resize(std::max(a.size(),b.size()));
    for(size_t i=0;i<b.size();++i)a[i]=mod(a[i]-b[i],q);
    mtrim(a);return a;
}
MP mrem(MP a,const MP& c,const Z& q){
    check(!c.empty(),"zero modular polynomial");Z lead=inv(c.back(),q);
    while(a.size()>=c.size()){
        size_t k=a.size()-c.size();Z v=mod(a.back()*lead,q);
        for(size_t j=0;j<c.size();++j)a[k+j]=mod(a[k+j]-v*c[j],q);
        mtrim(a);
    }
    return a;
}
MP mmul(const MP& a,const MP& b,const MP& c,const Z& q){
    if(a.empty()||b.empty())return {};
    MP z(a.size()+b.size()-1);
    for(size_t i=0;i<a.size();++i)for(size_t j=0;j<b.size();++j)z[i+j]=mod(z[i+j]+a[i]*b[j],q);
    mtrim(z);return mrem(z,c,q);
}
MP mpower(MP a,Z n,const MP& c,const Z& q){
    MP out{1};while(n>0){if(mpz_odd_p(n.get_mpz_t()))out=mmul(out,a,c,q);n>>=1;if(n>0)a=mmul(a,a,c,q);}
    return out;
}
MP mgcd(MP a,MP b,const Z& q){
    while(!b.empty()){MP t=mrem(a,b,q);a=std::move(b);b=std::move(t);}
    if(!a.empty()){Z s=inv(a.back(),q);for(Z& v:a)v=mod(v*s,q);}return a;
}
Z meval(const MP& c,const Z& x,const Z& q){Z v=0;for(auto i=c.rbegin();i!=c.rend();++i)v=mod(v*x+*i,q);return v;}
MP projection(const P& c,const Z& q,const Z& z,unsigned n){
    MP out;for(const E& a:c)out.push_back(project(a,q,z,n));mtrim(out);return out;
}
bool irreducible(const MP& c,unsigned q){
    check(c.size()>1,"constant irreducibility input");unsigned d=c.size()-1;
    std::vector<unsigned> prime_divisors;unsigned remaining=d;
    for(unsigned t=2;t<=remaining;++t)if(remaining%t==0){prime_divisors.push_back(t);while(remaining%t==0)remaining/=t;}
    MP x{0,1},value=x;
    for(unsigned j=1;j<=d;++j){
        value=mpower(value,Z(q),c,Z(q));
        for(unsigned t:prime_divisors)if(j==d/t&&mgcd(c,msub(value,x,Z(q)),Z(q)).size()!=1)return false;
    }
    return value==x;
}
#include "modular_kernel.hpp"
struct WeberModel {
    unsigned u=0,c0=0,aux=0;Z p,r,B,Bp,Br,zp,zr,z0,auxz;
    Point generator,target;P kernel;
};
void save_model(const WeberModel& m,const std::string& path){
    check(!std::ifstream(path).good(),"refusing model overwrite");std::ofstream f(path);
    f<<"WEBER-CM-1\n"<<m.u<<' '<<m.c0<<' '<<m.aux<<'\n'
     <<m.p<<' '<<m.r<<' '<<m.B<<' '<<m.Bp<<' '<<m.Br<<'\n'
     <<m.zp<<' '<<m.zr<<' '<<m.z0<<' '<<m.auxz<<'\n'
     <<m.generator.x<<' '<<m.generator.y<<' '<<m.target.x<<' '<<m.target.y<<'\n'
     <<m.kernel.size()<<'\n';for(const E& a:m.kernel)f<<a<<'\n';
    check(f.good(),"model write failed");
}
WeberModel load_model(const std::string& path){
    WeberModel m;std::ifstream f(path);std::string tag;size_t n;
    f>>tag>>m.u>>m.c0>>m.aux>>m.p>>m.r>>m.B>>m.Bp>>m.Br>>m.zp>>m.zr>>m.z0>>m.auxz
     >>m.generator.x>>m.generator.y>>m.target.x>>m.target.y>>n;
    check(tag=="WEBER-CM-1"&&n>1&&n<=22,"bad model header");m.kernel.resize(n);
    for(E& a:m.kernel){f>>a;a.a.canonicalize();a.b.canonicalize();}
    check(f.good(),"truncated model");m.generator.infinity=false;m.target.infinity=false;return m;
}
void emit_poly(const P& c){
    std::cout<<'[';for(size_t i=0;i<c.size();++i){if(i)std::cout<<',';std::cout<<"[\""<<c[i].a<<"\",\""<<c[i].b<<"\"]";}std::cout<<']';
}
CurveChoice counted_curve(const Z& q,const Z& r,unsigned h,const std::string& label){
    Z generator=sixth_class_generator(q),B=1;
    for(unsigned twist=0;twist<6;++twist,B=mod(B*generator,q)){
        if(count_small(B,q.get_ui())!=Z(h)*r)continue;
        Group group({0,0,0,0,B},q);
        for(unsigned j=0;j<16;++j){Point p=times(group,hash_point(q,B,label+"/"+std::to_string(j)),Z(h));
            if(!p.infinity){check(times(group,p,r).infinity,"fixture order");return {B,p,twist};}}
    }
    throw std::runtime_error("no counted fixture twist");
}
Z weber_value(const Point& p,const Z& B,const Z& q){
    check(!p.infinity,"identity has no finite Weber input");return mod(p.x*p.x*p.x*inv(B,q),q);
}
WeberModel setup(unsigned u,const std::string& output){
    check(std::set<unsigned>{3,5,7}.count(u)==1,"undeclared setup");
    WeberModel m;m.u=u;m.r=3*Z(u)*(u-1)+1;check(small_prime(m.r.get_ui()),"nonprime subgroup");
    for(m.c0=2;m.c0<=32;++m.c0){m.p=Z(m.c0*m.c0)*m.r+m.c0+1;if(small_prime(m.p.get_ui()))break;}
    check(m.c0<=32,"no admitted source prime");
    std::cout<<"{\"kind\":\"setup_stage\",\"u\":"<<u<<",\"stage\":\"symbolic_kernel\"}\n";
    auto kernel=modular_kernel(u);m.kernel=kernel.c;
    for(unsigned ell=7;ell<=257&&m.aux==0;++ell){
        if(ell%3!=1||!small_prime(ell)||Z(ell)==m.r)continue;
        Z z=powmod(sixth_class_generator(Z(ell)),Z((ell-1)/3),Z(ell));
        try{MP c=projection(m.kernel,Z(ell),z,1);if(c.size()==m.kernel.size()&&irreducible(c,ell)){m.aux=ell;m.auxz=z;}}
        catch(const std::exception&){/* bad coefficient prime; protocol scans the next */}
    }
    check(m.aux!=0,"no auxiliary irreducibility certificate");
    CurveChoice source=counted_curve(m.p,m.r,m.c0*m.c0,"round04/u"+std::to_string(u)+"/source/generator");
    CurveChoice target=counted_curve(m.r,m.r,1,"round04/u"+std::to_string(u)+"/target/generator");
    m.Bp=source.B;m.Br=target.B;m.generator=source.generator;m.target=target.generator;
    if(u==3){m.generator={false,42,31};m.target={false,4,1};}
    m.B=m.Bp+m.p*mod((m.Br-m.Bp)*inv(m.p,m.r),m.r);
    Z b=2*Z(u)-1;m.zp=mod((1+Z(m.c0)*(1-Z(u)))*inv(Z(m.c0)*b,m.p),m.p);
    m.zr=mod(-Z(u)*inv(b,m.r),m.r);m.z0=mod((1-Z(u))*inv(b,m.r),m.r);
    for(const auto& [q,z,B,point]:std::vector<std::tuple<Z,Z,Z,Point>>{
            {m.p,m.zp,m.Bp,m.generator},{m.r,m.zr,m.Br,m.target}}){
        check(mod(z*z+z+1,q)==0,"bad cube-root embedding");MP c=projection(m.kernel,q,z,1);
        Z value=weber_value(point,B,q);check(meval(c,value,q)==0,"generator outside fixed kernel");
        check(!external_times(q,0,B,point,1).infinity&&external_times(q,0,B,point,m.r).infinity,"OpenSSL fixture order");
    }
    save_model(m,output);
    std::cout<<"{\"kind\":\"setup\",\"u\":"<<u<<",\"p\":\""<<m.p<<"\",\"r\":\""<<m.r
             <<"\",\"cofactor\":"<<m.c0*m.c0<<",\"B\":\""<<m.B<<"\",\"weber_degree\":"<<m.kernel.size()-1
             <<",\"lattice_dimension\":"<<2*(m.kernel.size()-1)+1<<",\"auxiliary_prime\":"<<m.aux
             <<",\"auxiliary_omega\":\""<<m.auxz<<"\",\"kernel\":";emit_poly(m.kernel);
    std::cout<<",\"x_numerator\":";emit_poly(kernel.nx);std::cout<<",\"x_quotient\":";emit_poly(kernel.qx);
    std::cout<<",\"y_numerator\":";emit_poly(kernel.ny);std::cout<<",\"y_quotient\":";emit_poly(kernel.qy);
    std::cout<<"}\n";return m;
}
Z hensel_root(const P& c,const Z& q,const Z& zeta,Z a,unsigned n){
    check(n>0,"zero root precision");unsigned digits=1;
    while(digits<n){digits=std::min(2*digits,n);Z modulus=power(q,digits);MP f=projection(c,q,zeta,digits);
        MP df(f.size()-1);for(size_t i=1;i<f.size();++i)df[i-1]=mod(Z(i)*f[i],modulus);
        a=mod(a-meval(f,a,modulus)*inv(meval(df,a,modulus),modulus),modulus);
        check(meval(f,a,modulus)==0,"C-root Newton step failed");}
    check(meval(projection(c,q,zeta,n),a,power(q,n))==0,"bad root residue");return a;
}
Z specialize(const P& beta,const WeberModel& m,bool target,unsigned n=1){
    Z q=target?m.r:m.p,z=target?m.zr:m.zp,B=target?m.Br:m.Bp;
    Point generator=target?m.target:m.generator;
    Z denominator=1;for(const E& a:beta){denominator=lcm_z(denominator,a.a.get_den());denominator=lcm_z(denominator,a.b.get_den());}
    unsigned v=valuation(denominator,q),digits=n+v;Z modulus=power(q,digits);
    Z omega=omega_lift(z,q,digits),root=hensel_root(m.kernel,q,z,weber_value(generator,B,q),digits),value=0;
    for(auto i=beta.rbegin();i!=beta.rend();++i){
        Z numerator=i->a.get_num()*(denominator/i->a.get_den())+
                    i->b.get_num()*(denominator/i->b.get_den())*omega;
        value=mod(value*root+numerator,modulus);
    }
    Z qv=power(q,v);check(value%qv==0,"candidate has a target-place pole");
    return mod((value/qv)*inv(denominator/qv,power(q,n)),power(q,n));
}
bool modular_candidate(const std::vector<Z>& vector,const WeberModel& m){
    unsigned d=m.kernel.size()-1;Z q=m.aux,z=m.auxz;MP c=projection(m.kernel,q,z,1),a(d);
    for(unsigned j=0;j<d;++j)a[j]=mod(vector[2*j]+vector[2*j+1]*z,q);
    mtrim(a);Z denominator=mod(vector.back(),q),dpower=1;MP out{1};
    for(unsigned j=d;j>0;--j){out=mmul(out,a,c,q);dpower=mod(dpower*denominator,q);
        if(out.empty())out.resize(1);
        out[0]=mod(out[0]+c[j-1]*dpower,q);mtrim(out);}
    return out.empty();
}
struct Recognized {bool found=false;P beta;Z image;unsigned candidates=0,screened=0,exact=0;w::Reduced lattice;};
Recognized recognize(const WeberModel& m,const Point& input,unsigned n){
    Group source({0,0,0,0,m.Bp},m.p);
    check(equation(source.a,input,m.p)==0&&!input.infinity&&times(source,input,m.r).infinity,"bad public subgroup input");
    unsigned d=m.kernel.size()-1,da=(d-1)/2,db=d-1-da,split=2*(da+1),dimension=2*d+2;
    Z modulus=power(m.p,n);
    Z omega=omega_lift(m.zp,m.p,n),root=hensel_root(m.kernel,m.p,m.zp,weber_value(m.generator,m.Bp,m.p),n);
    Z input_root=hensel_root(m.kernel,m.p,m.zp,weber_value(input,m.Bp,m.p),n);
    std::vector<Z> basis(dimension);Z monomial=1;
    for(unsigned j=0;j<=std::max(da,db);++j){
        if(j<=da){basis[2*j]=monomial;basis[2*j+1]=mod(monomial*omega,modulus);}
        if(j<=db){basis[split+2*j]=mod(-input_root*monomial,modulus);
            basis[split+2*j+1]=mod(-input_root*monomial*omega,modulus);}
        monomial=mod(monomial*root,modulus);
    }
    w::Matrix lattice(dimension,std::vector<Z>(dimension));lattice[0][0]=modulus;
    for(unsigned i=1;i<dimension;++i){lattice[i][0]=-basis[i];lattice[i][i]=1;}
    Recognized out;out.lattice=w::lll(lattice);
    for(const auto& original:out.lattice.basis){
        Z congruence=0;
        for(unsigned i=0;i<dimension;++i)congruence+=original[i]*basis[i];
        check(congruence%modulus==0,"LLL row not in input congruence lattice");
        Z divisor=0;for(const Z& a:original)divisor=gcd_z(divisor,a);
        check(divisor!=0,"zero LLL basis row");std::vector<Z> v=original;for(Z& a:v)a/=divisor;
        P numerator(da+1),denominator(db+1);
        for(unsigned j=0;j<=da;++j)numerator[j]=E(w::Q(v[2*j]),w::Q(v[2*j+1]));
        for(unsigned j=0;j<=db;++j)denominator[j]=E(w::Q(v[split+2*j]),w::Q(v[split+2*j+1]));
        w::trim(numerator);w::trim(denominator);if(denominator.empty())continue;
        Z denominator_value=meval(projection(denominator,m.p,m.zp,1),mod(root,m.p),m.p);
        if(denominator_value==0)continue;
        ++out.candidates;Z aux=m.aux;MP c=projection(m.kernel,aux,m.auxz,1),
            a=projection(numerator,aux,m.auxz,1),b=projection(denominator,aux,m.auxz,1),value{1},bpower{1};
        for(unsigned j=d;j>0;--j){
            value=mmul(value,a,c,aux);bpower=mmul(bpower,b,c,aux);
            MP term=bpower;for(Z& z:term)z=mod(z*c[j-1],aux);
            value.resize(std::max(value.size(),term.size()));
            for(size_t k=0;k<term.size();++k)value[k]=mod(value[k]+term[k],aux);
            mtrim(value);
        }
        if(b.empty()||!value.empty()){++out.screened;continue;}
        P beta=w::multiply_mod(numerator,w::inverse_mod(denominator,m.kernel),m.kernel);
        ++out.exact;if(!w::evaluate_mod(m.kernel,beta,m.kernel).empty())continue;
        check(specialize(beta,m,false,n)==input_root,"global root does not match fixed source place");
        Z image=specialize(beta,m,true);
        if(out.found)check(beta==out.beta,"two different certified input roots");
        else{out.found=true;out.beta=beta;out.image=image;}
    }
    return out;
}
Z cube_root(const Z& a,const Z& q){
    check(a!=0&&powmod(a,(q-1)/3,q)==1,"not a nonzero cube");Z t=q-1,N=1;unsigned s=0;
    while(t%3==0){t/=3;N*=3;++s;}
    Z g=2;while(powmod(g,(q-1)/3,q)==1)++g;
    Z z=powmod(g,t,q),order3=powmod(z,N/3,q),c=powmod(a,t,q),digits=0,place=1;
    for(unsigned j=0;j<s;++j){Z residue=powmod(mod(c*inv(powmod(z,digits,q),q),q),N/(3*place),q),candidate=1;unsigned digit=0;
        while(digit<3&&candidate!=residue){candidate=mod(candidate*order3,q);++digit;}
        check(digit<3,"ternary logarithm digit missing");digits+=Z(digit)*place;place*=3;
    }
    check(digits%3==0,"cube has nondivisible ternary logarithm");
    Z e=t==1?Z(1):inv(3,t),carry=(3*e-1)/t,correction=mod(-(digits/3)*carry,N/3);
    Z root=mod(powmod(a,e,q)*powmod(z,correction,q),q);check(mod(root*root*root,q)==a,"cube root failed");return root;
}
Z decode(const WeberModel& m,const Point& input,const Z& image,unsigned& comparisons){
    Z x=cube_root(mod(m.Br*image,m.r),m.r),y;check(square_root(x*x*x+m.Br,m.r,y),"target ordinate absent");
    Point candidate{false,x,y};unsigned tau=1;Z lambda;
    for(;tau<=4;++tau){lambda=smart_lambda(m.target,m.r,m.B,tau);if(lambda!=0)break;}
    check(tau<=4,"degenerate Smart generator");Z k=mod(smart_lambda(candidate,m.r,m.B,tau)*inv(lambda,m.r),m.r);
    check(equal(external_times(m.r,0,m.Br,m.target,k),candidate),"target scalar replay failed");
    Z unit=1;
    for(unsigned j=0;j<3;++j,unit=mod(unit*m.z0,m.r))for(int sign:{1,-1}){
        Z scalar=mod(Z(sign)*unit*k,m.r);++comparisons;
        if(equal(external_times(m.p,0,m.Bp,m.generator,scalar),input)){
            check(!equal(external_times(m.p,0,m.Bp,m.generator,mod(scalar+1,m.r)),input),"perturbed scalar accepted");
            check(weber_value(external_times(m.r,0,m.Br,m.target,scalar),m.Br,m.r)==image,"target image relation failed");
            return scalar;
        }
    }
    throw std::runtime_error("no CM-unit candidate matches the source");
}
Point public_input(const WeberModel& m,unsigned index){
    Group g({0,0,0,0,m.Bp},m.p);
    for(unsigned attempt=0;attempt<16;++attempt){Point q=times(g,hash_point(m.p,m.Bp,"prime-field-transfer/round-04/u"+
        std::to_string(m.u)+"/source/public/"+std::to_string(index)+"/attempt/"+std::to_string(attempt)),Z(m.c0*m.c0));if(!q.infinity)return q;}
    throw std::runtime_error("public admission exhausted");
}
void controls(const WeberModel& m,const Point& input){
    Z source=weber_value(input,m.Bp,m.p),ep=m.r*inv(m.r,m.p),er=1-ep;
    P chimera{E(w::Q(ep*source)),E(w::Q(er))};
    check(specialize(chimera,m,false)==source&&specialize(chimera,m,true)==weber_value(m.target,m.Br,m.r),"chimera local gates");
    check(!w::evaluate_mod(m.kernel,chimera,m.kernel).empty(),"chimera accepted globally");
    w::F variable(P{E(0),E(1)},P{E(1)});
    w::F doubled=variable*w::power(variable-w::F(8),3)/(w::F(64)*w::power(variable+w::F(1),3));
    P wrong_root=w::multiply_mod(doubled.n,w::inverse_mod(doubled.d,m.kernel),m.kernel);
    check(w::evaluate_mod(m.kernel,wrong_root,m.kernel).empty(),"control is not a genuine global root");
    check(specialize(wrong_root,m,false)!=weber_value(m.generator,m.Bp,m.p),"wrong root accepted at reference source place");
    check(meval(projection(m.kernel,m.r,m.zr,1),specialize(wrong_root,m,true),m.r)==0,"wrong-root target gate");
    Z wrong_z=mod(m.zr*m.zr,m.r);bool wrong_accepted=false,wrong_pole=false;
    try{wrong_accepted=meval(projection(m.kernel,m.r,wrong_z,1),weber_value(m.target,m.Br,m.r),m.r)==0;}
    catch(const std::runtime_error& e){if(std::string(e.what())!="coefficient has a local pole")throw;wrong_pole=true;}
    check(!wrong_accepted,"wrong CM orientation accepted");
    std::cout<<"{\"kind\":\"controls\",\"u\":"<<m.u<<",\"chimera_local_gates\":true,\"chimera_global_rejected\":true,\"wrong_global_root_rejected_at_source\":true,\"wrong_global_root_input\":\"generator\",\"wrong_orientation_rejected\":true,\"wrong_orientation_pole\":"<<(wrong_pole?"true":"false")<<"}\n";
}
void qualification(const WeberModel& m,int index){
    std::vector<std::pair<Point,unsigned>> inputs;
    if(index<0){check(m.u==3,"only the order19 auditor enumerates source multiples");Group g({0,0,0,0,m.Bp},m.p);
        for(unsigned k=1;k<19;++k)inputs.push_back({times(g,m.generator,Z(k)),k});}
    else{check(index<4,"undeclared public input");inputs.push_back({public_input(m,static_cast<unsigned>(index)),0});}
    controls(m,inputs.front().first);
    std::vector<unsigned> precisions=index<0?std::vector<unsigned>{1,2,4,8,16,32}:std::vector<unsigned>{4,8,16,32};
    for(unsigned n:precisions){unsigned accepted=0;
        for(size_t j=0;j<inputs.size();++j){const Point& q=inputs[j].first;
            std::cout<<"{\"kind\":\"recognition_started\",\"u\":"<<m.u<<",\"input\":"<<j<<",\"precision\":"<<n<<"}\n";
            Recognized result=recognize(m,q,n);
            std::cout<<"{\"kind\":\"recognition\",\"u\":"<<m.u<<",\"input\":"<<j<<",\"precision\":"<<n
                     <<",\"source_x\":\""<<q.x<<"\",\"source_y\":\""<<q.y<<"\",\"accepted\":"<<(result.found?"true":"false")
                     <<",\"lll_dimension\":"<<result.lattice.basis.size()<<",\"lll_steps\":"<<result.lattice.steps
                     <<",\"basis_change_checked\":true,\"candidates\":"<<result.candidates<<",\"screen_rejected\":"<<result.screened
                     <<",\"exact_candidates\":"<<result.exact;
            if(result.found){unsigned comparisons=0;Z k=decode(m,q,result.image,comparisons);
                if(inputs[j].second)check(k==inputs[j].second,"auditor scalar mismatch");
                P mutation=result.beta+P{E(1)};check(!w::evaluate_mod(m.kernel,mutation,m.kernel).empty(),"mutated global root accepted");
                std::cout<<",\"target_weber\":\""<<result.image<<"\",\"scalar\":\""<<k<<"\",\"source_comparisons\":"<<comparisons<<",\"beta\":";
                emit_poly(result.beta);++accepted;
            }
            std::cout<<"}\n";
        }
        std::cout<<"{\"kind\":\"precision_summary\",\"u\":"<<m.u<<",\"precision\":"<<n<<",\"inputs\":"<<inputs.size()<<",\"accepted\":"<<accepted<<"}\n";
    }
}
int main(int argc,char** argv){
    std::cout.setf(std::ios::unitbuf);
    try{
        check(argc==4,"usage: recognizer setup U MODEL | qualify MODEL INDEX(-1=order19 auditor)");
        if(std::string(argv[1])=="setup")setup(std::stoul(argv[2]),argv[3]);
        else if(std::string(argv[1])=="qualify")qualification(load_model(argv[2]),std::stoi(argv[3]));
        else throw std::runtime_error("unknown command");
        return 0;
    }catch(const std::exception& e){std::cerr<<e.what()<<'\n';return 1;}
}
