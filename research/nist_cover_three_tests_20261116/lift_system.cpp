#include "target_kummer_fit_arithmetic.hpp"
using LiftExponent=std::array<int,7>;
struct LiftPoly {
    std::map<LiftExponent,T24> terms;
    LiftPoly()=default;
    explicit LiftPoly(const T24& c){if(c!=T24(0))terms.emplace(LiftExponent{},c);}
    explicit LiftPoly(long n):LiftPoly(T24(n)){}
    void add(const LiftExponent& e,const T24& c){if(c==T24(0))return;auto at=terms.find(e);if(at==terms.end())terms.emplace(e,c);else{at->second=at->second+c;if(at->second==T24(0))terms.erase(at);}}
    static LiftPoly variable(int i){LiftPoly p;LiftExponent e{};e.at(size_t(i))=1;p.add(e,T24(1));return p;}
    LiftPoly scale(const T24& c)const{LiftPoly p;if(c!=T24(0))for(auto& [e,v]:terms)p.add(e,v*c);return p;}
    LiftPoly operator+(const LiftPoly& b)const{LiftPoly p=*this;for(auto& [e,c]:b.terms)p.add(e,c);return p;}
    LiftPoly operator-()const{return scale(T24(-1));}
    LiftPoly operator-(const LiftPoly& b)const{return *this+(-b);}
    LiftPoly operator*(const LiftPoly& b)const{LiftPoly p;for(auto& [e,c]:terms)for(auto& [f,d]:b.terms){LiftExponent g{};for(int i=0;i<7;++i)g[i]=e[i]+f[i];p.add(g,c*d);}return p;}
    LiftPoly power(int n)const{LiftPoly r(1);for(int i=0;i<n;++i)r=r*(*this);return r;}
    T24 eval(const std::array<T24,7>& x)const{std::array<std::array<T24,16>,7> powers{};for(int i=0;i<7;++i){powers[i][0]=T24(1);for(int j=1;j<16;++j)powers[i][j]=powers[i][j-1]*x[i];}T24 out;
        for(auto& [e,c]:terms){T24 v=c;for(int i=0;i<7;++i){require(e[i]>=0&&e[i]<16,"lift evaluation exponent bound");if(e[i])v=v*powers[i][e[i]];}out=out+v;}return out;}
};
void emit_lift_poly(const LiftPoly& p){std::cout<<'[';bool first=true;for(auto& [e,c]:p.terms){if(!first)std::cout<<',';first=false;std::cout<<"{\"exponents\":[";for(int i=0;i<7;++i)std::cout<<(i?",":"")<<e[i];std::cout<<"],\"coefficient\":";emit_t24(c);std::cout<<'}';}std::cout<<']';}
struct LiftSystem {
    std::array<LiftPoly,8> h;
    std::array<LiftPoly,3> curve;
    LiftPoly omega;
    LiftSystem(const Json& good,const TargetWeil& target){
        std::array<std::array<LiftPoly,8>,3> basis{};std::array<LiftPoly,3> x,y;T24 slope=target.infinity2[1];
        for(int i=0;i<3;++i){x[i]=LiftPoly::variable(2*i);y[i]=LiftPoly::variable(2*i+1);auto ell=y[i]-x[i].scale(slope);basis[i]={LiftPoly(1),x[i],x[i]*x[i],y[i],y[i]*y[i],x[i]*y[i],x[i]*ell*ell,y[i]*ell*ell};
            for(auto& [e,c]:target.f.c)curve[i]=curve[i]+(x[i].power(e[0])*y[i].power(e[1])).scale(c);
        }
        omega=x[0]*(y[1]-y[2])-y[0]*(x[1]-x[2])+x[1]*y[2]-y[1]*x[2];
        std::vector<LiftPoly> symmetric;for(auto& row:good.at("symmetric_triples").arr){Triple t{};for(int i=0;i<3;++i)t[i]=row.arr[i].integer();LiftPoly p;do{p=p+basis[0].at(size_t(t[0]))*basis[1].at(size_t(t[1]))*basis[2].at(size_t(t[2]));}while(std::next_permutation(t.begin(),t.end()));symmetric.push_back(p);}
        require(symmetric.size()==120&&good.at("function_coefficients").arr.size()==8,"lift pinned good-function shape");
        for(int i=0;i<8;++i){const auto& row=good.at("function_coefficients").arr[i];require(row.arr.size()==120,"lift pinned coefficient width");for(int j=0;j<120;++j)h[i]=h[i]+symmetric[j].scale(read_t24(row.arr[j]));require(!h[i].terms.empty(),"nonzero expanded good function");}
    }
    static int normalize(Values8& image){int pivot=0;while(pivot<8&&image[pivot]==T24(0))++pivot;require(pivot<8,"nonzero lift image");T24 inv=image[pivot].inv();for(auto& c:image)c=c*inv;return pivot;}
    std::vector<LiftPoly> equations(Values8 image)const{int pivot=normalize(image);std::vector<LiftPoly> out(curve.begin(),curve.end());for(int i=0;i<8;++i)if(i!=pivot)out.push_back(h[i]-h[pivot].scale(image[i]));out.push_back(LiftPoly::variable(6)*h[pivot]*omega-LiftPoly(1));require(out.size()==11,"eleven finite inverse equations");return out;}
};
void emit_lift_list(const std::vector<LiftPoly>& p){std::cout<<'[';for(size_t i=0;i<p.size();++i){if(i)std::cout<<',';emit_lift_poly(p[i]);}std::cout<<']';}
int main(int argc,char** argv){try{
    require(argc==5||argc==7,"usage: lift_system kernel3-study good.json match.json source.json [--emit-system 0|1]");std::string root=argv[1];initialize_cover(read_json(root+"/inputs/p192-d7-prym.json"));Level2Input input;TargetWeil target(root);
    auto good=read_json(argv[2]),match=read_json(argv[3]),source=read_json(argv[4]);require(source.at("kernel").integer()==3&&match.at("fit_rank").integer()==8,"qualified kernel3 inputs");LiftSystem system(good,target);std::cerr<<"eight good polynomials expanded\n";
    auto descent=read_json(root+"/evidence/descent-v1/result.json"),receipt=read_json(root+"/evidence/destination-relations-v2/result.json");auto points=ek_input_points(target,descent,receipt);require(match.at("heldout_supports").arr.size()==16,"sixteen pinned inverse controls");int checks=0,negative=0;
    for(int n=0;n<16;++n){auto& row=match.at("heldout_supports").arr[n];std::array<T24,7> values{};for(int i=0;i<3;++i){int index=row.arr[i].integer();values[2*i]=points.at(size_t(index)).point[0];values[2*i+1]=points.at(size_t(index)).point[1];}T24 w=system.omega.eval(values);require(w!=T24(0),"noncollinear inverse control");Values8 image{};
        for(int i=0;i<8;++i){image[i]=read_t24(match.at("heldout_H").arr[n].arr[i]);require(system.h[i].eval(values)==image[i]*w*w,"expanded versus retained good-function value");++checks;}
        int pivot=LiftSystem::normalize(image);values[6]=(system.h[pivot].eval(values)*w).inv();auto equations=system.equations(image);for(auto& p:equations){require(p.eval(values)==T24(0),"known destination solves all finite inverse equations");++checks;}
        auto bad=image;bad[(pivot+1)%8]=bad[(pivot+1)%8]+T24(1);bool rejected=false;for(auto& p:system.equations(bad))if(p.eval(values)!=T24(0))rejected=true;require(rejected,"perturbed inverse image rejection");++negative;
    }
    std::array<T24,7> zero{};require(system.omega.eval(zero)==T24(0),"collinear origin determinant");Values8 control{};control[0]=T24(1);auto saturated=system.equations(control).back();require(saturated.eval(zero)==T24(-1),"collinear saturation at parameter zero");zero[6]=T24(1);require(saturated.eval(zero)==T24(-1),"collinear saturation at parameter one");
    require((system.h[0]*system.omega).eval(zero)==T24(0),"symbolic linear saturation coefficient at collinear substitution");std::cerr<<"sixteen inverse controls and saturation checks passed\n";
    TSpace matrix;for(auto& row:match.at("good_from_kummer").arr){TVector r;for(auto& c:row.arr)r.push_back(read_t24(c));require(r.size()==8,"lift matrix width");matrix.push_back(r);}require(matrix.size()==8,"lift matrix height");std::array<Values8,2> images{},kummers{};std::array<int,2> pivots{};
    for(int n=0;n<2;++n){const auto& row=source.at("images").arr.at(size_t(n?3:1));TVector k;for(int i=0;i<8;++i){kummers[n][i]=read_t24(row.at("coordinates").arr[i]);k.push_back(kummers[n][i]);}for(int i=0;i<8;++i)images[n][i]=good_dot(matrix[i],k);pivots[n]=LiftSystem::normalize(images[n]);auto equations=system.equations(images[n]);require(!equations.back().terms.empty(),"nonzero source saturation equation");}
    if(argc==7){require(std::string(argv[5])=="--emit-system","finite inverse output selector");int selected=std::stoi(argv[6]);require(selected==0||selected==1,"public image selector0=G,1=Q");std::cout<<"{\"variables\":[\"x1\",\"y1\",\"x2\",\"y2\",\"x3\",\"y3\",\"t\"],\"public_fixture\":"<<selected<<",\"equations\":";emit_lift_list(system.equations(images[selected]));std::cout<<"}\n";return 0;}
    std::cout<<"{\"scope\":\"P192-kernel3-finite-inverse-equations\",\"kernel\":3,\"standing\":\"producer-unverified\",\"heldout_triples\":16,\"positive_equation_and_value_checks\":"<<checks<<",\"perturbed_image_rejections\":"<<negative<<",\"saturation_checks\":3,\"equations_per_image\":11,\"variable_count\":7,\"roots_computed\":false,\"shared_h\":[";for(int i=0;i<8;++i){if(i)std::cout<<',';emit_lift_poly(system.h[i]);}std::cout<<"],\"omega\":";emit_lift_poly(system.omega);std::cout<<",\"curve_equations\":";emit_lift_list(std::vector<LiftPoly>(system.curve.begin(),system.curve.end()));std::cout<<",\"source_images\":[";
    for(int n=0;n<2;++n){if(n)std::cout<<',';std::cout<<"{\"fixture\":\""<<(n?"Q":"G")<<"\",\"source_receipt_index\":"<<(n?3:1)<<",\"pivot\":"<<pivots[n]<<",\"K\":";emit_ek_values(kummers[n]);std::cout<<",\"normalized_H\":";emit_ek_values(images[n]);std::cout<<'}';}std::cout<<"]}\n";
}catch(const std::exception& e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
