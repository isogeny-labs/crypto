#include "quotient_theta2_arithmetic.hpp"
T24 read_t24(const Json& row){require(row.arr.size()==3,"tower value shape");std::vector<F> a;for(auto& r:row.arr)a.push_back(field(r));return T24(P(a));}
using SKValues=std::array<T24,8>;
int sk_sign(int a,int b){return __builtin_popcount(unsigned(a&b))%2?-1:1;}
int sk_code(int label){return ((label&7)<<3)|(label>>3);}
bool sk_equal(const SKValues& a,const SKValues& b){int p=0;while(p<8&&b[p]==T24(0))++p;require(p<8,"nonzero source Kummer vector");if(a[p]==T24(0))return false;for(int j=0;j<8;++j)if(a[j]*b[p]!=b[j]*a[p])return false;return true;}
void sk_emit(const SKValues& a){std::cout<<'[';for(int i=0;i<8;++i){if(i)std::cout<<',';emit_t24(a[i]);}std::cout<<']';}
std::vector<T24> sk_normalization(const Level2Input& input,const QuotientSection& section,int& checks){
    auto generic=generic_inputs(section.source_kernel);GT24 z(generic[0]),heldout(generic[1]);std::vector<GT24> shifted;for(auto& s:input.classes)shifted.push_back(z.add(s));
    std::vector<T24> alpha(64,T24(1)),raw;for(auto& s:input.classes){auto v=eta2(s,z);require(v!=T24(0),"source normalization nonzero");raw.push_back(v);}
    for(int bit=0;bit<6;++bit){int b=1<<bit;alpha[b]=sqrt24(T24(1)/(raw[b]*eta2(input.classes[b],shifted[b])));
        for(int mask=1;mask<b;++mask){T24 gamma=T24(d2(mask,b))*raw[mask^b]/(raw[mask]*eta2(input.classes[b],shifted[mask]));alpha[mask^b]=alpha[mask]*alpha[b]/gamma;}
    }
    std::vector<T24> test;for(auto& s:input.classes)test.push_back(eta2(s,heldout));
    for(int p=0;p<64;++p){auto translated=heldout.add(input.classes[p]);for(int bit=0;bit<6;++bit){int q=1<<bit;require(alpha[p]*test[p]*alpha[q]*eta2(input.classes[q],translated)==T24(d2(p,q))*alpha[p^q]*test[p^q],"source heldout normalization cocycle");++checks;}}
    return alpha;
}
struct SKImage {SKValues functions{},coordinates{};std::vector<int> representatives,rejected;GT24 source;};
struct SourceKummer {
    Level2Input& input;QuotientSection& section;std::vector<T24> alpha,theta;SKValues nulls{};int normalization_checks=0;
    SourceKummer(Level2Input& i,QuotientSection& s,const std::string& root):input(i),section(s){
        alpha=sk_normalization(i,s,normalization_checks);require(normalization_checks==384,"all source normalization controls");auto r=read_json(root+"/evidence/quotient-theta2-v1/result.json");for(auto& c:r.at("theta_squares").arr)theta.push_back(read_t24(c));require(theta.size()==64,"source theta constants");
        for(int a=0;a<8;++a){for(int b=0;b<8;++b)nulls[a]=nulls[a]+T24(sk_sign(a,b))*theta[8*a+b];require(nulls[a]!=T24(0),"source nonzero theta-null chart");}
        for(int a=0;a<8;++a){T24 sum;for(int b=0;b<8;++b)sum=sum+T24(sk_sign(a,b))*theta[b];require(nulls[0]*sum==nulls[a]*nulls[a],"source theta-null origin identity");}
    }
    bool regular(const GT24& z)const{
        if(z.u.degree()!=3)return false;
        for(int i=1;i<27;++i)if(determinant3(multiplication3(z.u,embed_polynomial<T24>(section.functions[i].support.u)))==T24(0))return false;
        return true;
    }
    SKImage evaluate(const GT24& d){
        d.check();SKImage out;out.source=d;
        for(int j=1;j<27&&out.representatives.size()<2;++j){auto base=d.add(input.classes[input.delta]).add(section.kernel[j]);std::array<GT24,8> points;bool usable=true;
            for(int s=0;s<8;++s){points[s]=base.add(input.classes[sk_code(s)]);if(!regular(points[s])){usable=false;break;}}
            if(!usable){out.rejected.push_back(j);continue;}T24 theta0=section.theta(base);if(theta0==T24(0)){out.rejected.push_back(j);continue;}
            SKValues values{};for(int s=0;s<8;++s){int code=sk_code(s);T24 eta=alpha[code]*eta2(input.classes[code],base),ratio=section.theta(points[s])/theta0;values[s]=eta*eta*eta*ratio*ratio;}
            require(values[0]==T24(1),"source zero-characteristic value");if(out.representatives.empty())out.functions=values;else require(values==out.functions,"source image independent of kernel representative");out.representatives.push_back(j);
        }
        require(out.representatives.size()==2,"two regular source image representatives within fixed bound");
        for(int a=0;a<8;++a){for(int b=0;b<8;++b)out.coordinates[a]=out.coordinates[a]+T24(sk_sign(a,b))*out.functions[b];out.coordinates[a]=out.coordinates[a]/nulls[a];}return out;
    }
};
void sk_emit_image(const SKImage& i){std::cout<<"{\"source_U\":";emit_t24_polynomial(i.source.u);std::cout<<",\"source_V\":";emit_t24_polynomial(i.source.v);std::cout<<",\"representatives\":[";for(size_t j=0;j<i.representatives.size();++j)std::cout<<(j?",":"")<<i.representatives[j];std::cout<<"],\"rejected_charts\":[";for(size_t j=0;j<i.rejected.size();++j)std::cout<<(j?",":"")<<i.rejected[j];std::cout<<"],\"functions\":";sk_emit(i.functions);std::cout<<",\"coordinates\":";sk_emit(i.coordinates);std::cout<<'}';}
int main(int argc,char** argv){try{
    require(argc==2,"usage: source_kummer kernel3-study");std::string root=argv[1];initialize_cover(read_json(root+"/inputs/p192-d7-prym.json"));Level2Input input;QuotientSection section(root);SourceKummer evaluator(input,section,root);std::cerr<<"source normalization384 and theta-null identities passed\n";
    EC g(F(Poly<Z>{Z(mpz_class("188DA80EB03090F67CBF20EB43A18800F4FF0AFD82FF1012",16))}),F(Poly<Z>{Z(mpz_class("07192B95FFC8DA78631011ED6B24CDD573F977A11E794811",16))}));auto public_q=times(g,37);auto pg=pull(g),pq=pull(public_q);require(pq==pg.times(37),"public source scalar fixture consistency");
    std::vector<GT24> fixtures{GT24{},GT24(pg),GT24(pg).neg(),GT24(pq),GT24(pg).add(section.kernel[1])};std::vector<SKImage> images;
    for(size_t i=0;i<fixtures.size();++i){images.push_back(evaluator.evaluate(fixtures[i]));std::cerr<<"source Kummer fixture "<<i<<" passed two representatives\n";}
    require(sk_equal(images[0].coordinates,evaluator.nulls),"source origin Kummer coordinates");for(int s=0;s<8;++s)require(images[0].functions[s]==evaluator.theta[s],"source origin normalized Weil constants");
    require(sk_equal(images[1].coordinates,images[2].coordinates),"source D and minus-D Kummer equality");require(images[1].functions==images[4].functions&&sk_equal(images[1].coordinates,images[4].coordinates),"source actual nonzero kernel invariance");
    int pivot=0;while(pivot<8&&images[1].coordinates[pivot]==T24(0))++pivot;require(pivot<8,"nonzero source control vector");int changed=(pivot+1)%8;
    rejects([&]{auto bad=images[1].coordinates;bad[changed]=bad[changed]+T24(1);require(sk_equal(bad,images[1].coordinates),"altered source Kummer coordinate");},"altered source Kummer coordinate");
    std::cout<<"{\"scope\":\"P192-kernel3-source-Kummer-images\",\"kernel\":3,\"normalization_checks\":"<<evaluator.normalization_checks<<",\"fixture_count\":5,\"representatives_per_fixture\":2,\"origin_checks\":9,\"negation_checks\":1,\"actual_kernel_invariance_checks\":1,\"corruption_controls\":1,\"divisor_map_constructed\":false,\"standing\":\"producer-unverified\",\"theta_null\":";sk_emit(evaluator.nulls);std::cout<<",\"images\":[";for(size_t i=0;i<images.size();++i){if(i)std::cout<<',';sk_emit_image(images[i]);}std::cout<<"]}\n";
}catch(const std::exception& e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
