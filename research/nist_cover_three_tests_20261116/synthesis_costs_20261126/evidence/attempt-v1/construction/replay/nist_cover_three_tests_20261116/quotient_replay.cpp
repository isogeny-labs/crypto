// Exact quotient-group image representation. This is not a plane-quartic model.
#include "kernel_arithmetic.hpp"
#include "module_arithmetic.hpp"
#include <memory>
#include <unordered_map>
#include <openssl/sha.h>
struct Orbit {G representative;std::string key;};
using Q=std::shared_ptr<const Orbit>;
struct Quotient {
    std::vector<G> kernel;
    std::unordered_map<std::string,Q> cache;
    Quotient(const std::vector<G>&basis,const Basis&coordinates,const M&a,const M&form){
        require(coordinates.size()==3&&rank(coordinates)==3,"quotient kernel rank");
        require(stable(coordinates,a)&&isotropic(coordinates,form),"quotient kernel stability/isotropy");
        std::vector<G> generators;for(auto row:coordinates)generators.push_back(combination(basis,std::vector<int>(row.begin(),row.end())));
        kernel={G{}};for(auto&g:generators){require(g.times(3)==G{},"quotient kernel killed by3");auto old=kernel;for(auto&k:old){kernel.push_back(k.add(g));kernel.push_back(k.add(g.neg()));}}
        std::set<std::string> keys;for(auto&k:kernel)keys.insert(gkey(k));require(keys.size()==27,"actual quotient kernel cardinality");
        for(auto&k:kernel){require(keys.count(gkey(gfrob(k))),"actual kernel Frobenius stability");for(auto&g:generators)require(keys.count(gkey(k.add(g))),"actual kernel generator closure");}
    }
    Q image(const G&d){d.check();auto key=gkey(d);auto found=cache.find(key);if(found!=cache.end())return found->second;
        std::vector<G> orbit;std::vector<std::string> keys;size_t least=0;
        for(auto&k:kernel){orbit.push_back(d.add(k));keys.push_back(gkey(orbit.back()));if(keys.back()<keys[least])least=keys.size()-1;}
        auto q=std::make_shared<Orbit>(Orbit{orbit[least],keys[least]});
        for(auto&s:keys){auto [it,inserted]=cache.emplace(s,q);require(inserted||it->second->key==q->key,"inconsistent coset cache");}return q;
    }
    bool equal(const Q&a,const Q&b)const{return a->key==b->key;}
    Q add(const Q&a,const Q&b){return image(a->representative.add(b->representative));}
    Q negate(const Q&a){return image(a->representative.neg());}
    Q multiply(const Q&a,const mpz_class&n){return image(a->representative.times(n));}
    G dual(const Q&a)const{return a->representative.times(3);}
    Q Fmap(const EC&p){return image(pull(p));}
    EC Tmap(const Q&a)const{return norm(dual(a));}
};
void initialize_cover(const Json&j){prime=mpz_class(j.at("source_model").at("p").scalar);modulus=base_poly(j.at("finite_field_modulus_coefficients"));F b(Poly<Z>{Z(mpz_class(j.at("source_model").at("b").scalar))});dd=F(7);cc=F(1024);P t{F(0),F(1)},one{F(1)},tt=t*t,tm=tt-one,td=tt+P{dd};P h=tm*(td*td*td-(td*tm*tm).scale(F(3))+(tm*tm*tm).scale(b));require(h.degree()==8&&h.eval(F(1))==F(0),"cover input");
    std::vector<F> q(8);for(int i=0;i<=8;++i){long choose=1;F cp(1);for(int k=1;k<=i;++k){choose=choose*(i-k+1)/k;cp=cp*cc;q[8-k]=q[8-k]+h.at(i)*F(choose)/cp;}}curve=P(q);require(curve.degree()==7&&curve.c.back()==F(1),"cover normalization");}
mpz_class public_scalar(const mpz_class&r){std::string label="nist-cover-three-tests/public-map-correctness/full-width-v1";unsigned char digest[32];SHA256(reinterpret_cast<const unsigned char*>(label.data()),label.size(),digest);mpz_class n;mpz_import(n.get_mpz_t(),32,1,1,1,0,digest);return mpz_class(n%r);}
template<class Fn> void rejects(Fn f,const std::string&label){bool rejected=false;try{f();}catch(const std::exception&){rejected=true;}require(rejected,"negative control accepted: "+label);}
int main(int argc,char**argv){try{require(argc==3,"usage: quotient_replay study-directory output-jsonl");std::string root=argv[1];auto j=read_json(root+"/inputs/p192-d7-prym.json");initialize_cover(j);auto actual=read_json(root+"/evidence/final-kernel-check/actual-cover-torsion.json");std::vector<G> basis;for(auto&v:actual.at("basis").arr){G d{polynomial(v.at("U")),polynomial(v.at("V"))};d.check();require(d.times(3)==G{}&&!(d==G{}),"actual input basis order3");basis.push_back(d);}require(basis.size()==6,"six input basis elements");
    auto modules=read_json(root+"/../nist_cover_separator_20261115/data/modules-192.json");Json module;bool found=false;for(auto&row:modules.at("modules").arr)if(row.at("candidate").integer()==139&&row.at("d").integer()==7){module=row;found=true;}require(found,"P192 d7 module");M a{},form{};block(a,module.at("E_Frobenius"),2,0);block(a,module.at("Prym_Frobenius"),4,2);block(form,module.at("E_Weil"),2,0);block(form,module.at("Prym_Weil"),4,2);
    for(int i=0;i<6;++i)require(gfrob(basis[i])==combination(basis,std::vector<int>(a[i].begin(),a[i].end())),"actual input basis Frobenius");
    std::vector<G> all{G{}};for(auto&d:basis){auto old=all;for(auto&w:old){all.push_back(w.add(d));all.push_back(w.add(d.neg()));}}std::set<std::string> all_keys;for(auto&d:all)all_keys.insert(gkey(d));require(all_keys.size()==729,"input full3 torsion independence");
    EC base(F(Poly<Z>{Z(mpz_class("188DA80EB03090F67CBF20EB43A18800F4FF0AFD82FF1012",16))}),F(Poly<Z>{Z(mpz_class("07192B95FFC8DA78631011ED6B24CDD573F977A11E794811",16))}));F b(Poly<Z>{Z(mpz_class(j.at("source_model").at("b").scalar))});auto valid=[&](const EC&p){require(p.zero||p.y*p.y==p.x*p.x*p.x-F(3)*p.x+b,"elliptic input equation");};valid(base);
    mpz_class r("6277101735386680763835789423176059013767194773182842284081"),inverse6;require(mpz_probab_prime_p(r.get_mpz_t(),40)>0&&times(base,r).zero,"source subgroup order screen");mpz_class six=6;require(mpz_invert(inverse6.get_mpz_t(),six.get_mpz_t(),r.get_mpz_t())!=0,"inverse6");auto full=public_scalar(r);std::vector<mpz_class> scalars{mpz_class(0),mpz_class(1),mpz_class(37),mpz_class(r-1),full};std::vector<EC> sources;for(auto&n:scalars){auto p=times(base,n);valid(p);sources.push_back(p);}
    std::vector<G> base_divisors;std::vector<EC> base_norms;for(int x=1;x<=128&&base_divisors.size()<4;++x){F X(x),value=curve.eval(X);Z v=scalar(value);mpz_class symbol,y;mpz_powm(symbol.get_mpz_t(),v.x.get_mpz_t(),mpz_class((prime-1)/2).get_mpz_t(),prime.get_mpz_t());if(symbol!=1)continue;mpz_powm(y.get_mpz_t(),v.x.get_mpz_t(),mpz_class((prime+1)/4).get_mpz_t(),prime.get_mpz_t());G d{P{-X,F(1)},P{F(Poly<Z>{Z(y)})}};d.check();base_norms.push_back(norm(d));base_divisors.push_back(d);}require(base_divisors.size()==4,"transported-base fixture size");
    std::ifstream input(root+"/evidence/final-kernel-check/kernels.jsonl");std::ofstream output(argv[2]);require(bool(input)&&bool(output),"kernel/result I/O");std::string line;int count=0;
    while(std::getline(input,line)){Parser parser{line};auto row=parser.value();Basis coordinates;for(auto&v:row.at("rref_basis").arr){require(v.arr.size()==6,"kernel coordinate shape");V vv{};for(int i=0;i<6;++i)vv[i]=v.arr[i].integer();coordinates.push_back(vv);}Quotient q(basis,coordinates,a,form);auto zero=q.image(G{});
        for(auto&d:q.kernel){require(q.equal(q.image(d),zero),"kernel collapse");}
        require(!q.equal(q.image(basis[0]),zero),"outside kernel preservation");
        std::map<std::string,int> fibers;for(auto&d:all)++fibers[q.image(d)->key];require(fibers.size()==27,"full3 quotient cardinality");for(auto&entry:fibers)require(entry.second==27,"full3 quotient fiber size");
        auto fp=q.Fmap(base);require(!q.equal(fp,zero)&&q.equal(q.multiply(fp,r),zero),"quotient subgroup order");int representative_tests=0;
        for(size_t i=0;i<sources.size();++i){auto f=q.Fmap(sources[i]);require(q.equal(f,q.multiply(fp,scalars[i])),"quotient scalar compatibility");require(q.equal(q.image(gfrob(f->representative)),f),"Frobenius fixed source image");G dual=q.dual(f);require(dual==pull(sources[i]).times(3),"dual/source composition");EC projected=q.Tmap(f);require(projected==times(sources[i],6),"TF equals6");require(times(projected,inverse6)==sources[i],"normalized roundtrip");require(q.equal(q.image(dual),q.multiply(f,3)),"other dual composition");
            for(auto&k:q.kernel){require(f->representative.add(k).times(3)==dual,"dual representative independence");++representative_tests;}
        }
        for(int i=0;i<3;++i)for(int k=0;k<3;++k){auto x=q.Fmap(sources[i]),y=q.Fmap(sources[k]);require(q.equal(q.add(x,y),q.Fmap(sources[i].add(sources[k]))),"quotient additivity");require(q.equal(q.add(x,q.negate(x)),zero),"quotient inverse");require(q.equal(q.add(q.add(x,y),fp),q.add(x,q.add(y,fp))),"quotient associativity fixture");}
        int relations=0;for(int i=0;i<4;++i)for(int k=0;k<4;++k){G sum=base_divisors[i].add(base_divisors[k].neg());EC old=base_norms[i].add(base_norms[k].neg());require(norm(sum)==old,"source projected-base difference");EC transported=q.Tmap(q.image(sum));require(transported==times(old,3),"transported-base scaling identity");++relations;}
        Basis bad{V{1,0,0,0,0,0},V{0,1,0,0,0,0},V{0,0,1,0,0,0}};rejects([&]{Quotient wrong(basis,bad,a,form);},"non-isotropic kernel");rejects([&]{valid(EC(base.x,base.y+F(1)));},"invalid source point");rejects([&]{require(q.Tmap(fp).add(base)==times(base,6),"tampered target");},"tampered target");
        output<<"{\"kernel\":"<<row.at("kernel").integer()<<",\"full3_classes\":729,\"quotient_classes\":27,\"fiber_size\":27,\"source_cases\":"<<sources.size()<<",\"representative_tests\":"<<representative_tests<<",\"projected_base_checks\":"<<relations<<",\"rejected_controls\":3,\"TF_equals_6\":true,\"inverse6_roundtrip\":true,\"standing\":\"producer-unverified\",\"DLP_solved\":false,\"plane_quartic_constructed\":false}\n";++count;
    }require(count==8,"all eight quotient kernels");std::cout<<"PASS eight exact coset quotient image models: full3 torsion partitions, dual well-definedness, Frobenius descent on transferred points, subgroup/order screens, TF=[6], inverse6 roundtrips and transported-base factor3 checks. No target-curve reconstruction or DLP solve.\n";
}catch(const std::exception&e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
