#include "quotient_theta2_arithmetic.hpp"
#include <fstream>
using PBVector=std::array<Z,24>;
using PBMatrix=std::array<PBVector,24>;
using PBPoly=Poly<Z>;
T24 pb_read_t24(const Json& row){require(row.arr.size()==3,"primitive tower value shape");std::vector<F> a;for(auto& r:row.arr)a.push_back(field(r));return T24(P(a));}
PBVector pb_flatten(const T24& x){PBVector out{};for(int j=0;j<3;++j)for(int i=0;i<8;++i)out[8*j+i]=x.a.at(j).a.at(i);return out;}
T24 pb_unflatten(const PBVector& x){std::vector<F> a;for(int j=0;j<3;++j){std::vector<Z> b;for(int i=0;i<8;++i)b.push_back(x[8*j+i]);a.emplace_back(PBPoly(b));}return T24(P(a));}
PBVector pb_apply(const PBMatrix& a,const PBVector& x){PBVector out{};for(int i=0;i<24;++i)for(int j=0;j<24;++j)out[i]=out[i]+a[i][j]*x[j];return out;}
struct PrimitiveBasis {
    T24 beta;
    std::array<T24,25> powers;
    PBMatrix tower_from_power{},power_from_tower{};
    PBPoly minimal;
    PrimitiveBasis():beta(T24(F(PBPoly{Z(0),Z(1)}))+T24(P{F(0),F(1)})){
        powers[0]=T24(1);for(int i=1;i<=24;++i)powers[i]=powers[i-1]*beta;
        for(int col=0;col<24;++col){auto v=pb_flatten(powers[col]);for(int row=0;row<24;++row)tower_from_power[row][col]=v[row];}
        auto work=tower_from_power;for(int i=0;i<24;++i)power_from_tower[i][i]=Z(1);
        for(int col=0;col<24;++col){int pivot=col;while(pivot<24&&work[pivot][col]==Z(0))++pivot;require(pivot<24,"fixed beta has power-basis rank24");std::swap(work[pivot],work[col]);std::swap(power_from_tower[pivot],power_from_tower[col]);
            Z inv=work[col][col].inv();for(int j=0;j<24;++j){work[col][j]=work[col][j]*inv;power_from_tower[col][j]=power_from_tower[col][j]*inv;}
            for(int i=0;i<24;++i)if(i!=col){Z c=work[i][col];if(c!=Z(0))for(int j=0;j<24;++j){work[i][j]=work[i][j]-c*work[col][j];power_from_tower[i][j]=power_from_tower[i][j]-c*power_from_tower[col][j];}}
        }
        auto coefficients=pb_apply(power_from_tower,pb_flatten(powers[24]));std::vector<Z> m;for(auto& c:coefficients)m.push_back(-c);m.emplace_back(1);minimal=PBPoly(m);
        require(minimal.degree()==24&&from_power(minimal)==T24(0),"minimal polynomial annihilates fixed beta");
    }
    PBPoly to_power(const T24& value)const{auto v=pb_apply(power_from_tower,pb_flatten(value));return PBPoly(std::vector<Z>(v.begin(),v.end()));}
    T24 from_power(const PBPoly& p)const{T24 out;for(int i=p.degree();i>=0;--i)out=out*beta+T24(F(PBPoly{p.at(i)}));return out;}
    T24 control_a()const{return powers[23]+powers[7]*T24(-17)+T24(-29);}
    T24 control_b()const{return powers[19]+powers[5]*T24(-31)+T24(-37);}
    void irreducibility()const{
        PBPoly x{Z(0),Z(1)},a=x;for(int i=1;i<=24;++i){a=powmod(a,prime,minimal);if(i==8||i==12){auto[g,s,t]=xgcd(minimal,a-x);require(g.degree()==0,"primitive polynomial Frobenius proper-divisor gcd");}}require(a==x,"primitive polynomial Frobenius closure24");
    }
    void audit(int& products,int& inverses,int& frobenius,int& roundtrips)const{
        std::vector<T24> basis;for(int i=0;i<24;++i){PBVector e{};e[i]=Z(1);basis.push_back(pb_unflatten(e));require(from_power(to_power(basis.back()))==basis.back(),"tower basis round trip");++roundtrips;}
        for(auto& a:basis)for(auto& b:basis){require(from_power((to_power(a)*to_power(b))%minimal)==a*b,"all tower basis products");++products;}
        for(auto& a:basis){auto[g,s,t]=xgcd(to_power(a),minimal);require(g.degree()==0&&from_power(s)==a.inv(),"power-basis inverse matches original field");++inverses;require(from_power(powmod(to_power(a),prime,minimal))==a.power(prime),"power-basis prime-Frobenius matches tower");++frobenius;}
        rejects([&]{auto bad=power_from_tower;bad[0][0]=bad[0][0]+Z(1);auto v=pb_apply(bad,pb_flatten(T24(1)));require(from_power(PBPoly(std::vector<Z>(v.begin(),v.end())))==T24(1),"altered primitive conversion matrix");},"altered primitive conversion matrix");
    }
};
void pb_emit_poly(std::ostream& out,const PBPoly& p){out<<'[';for(int i=0;i<=p.degree();++i)out<<(i?",":"")<<'"'<<p.at(i).x<<'"';out<<']';}
void pb_emit_matrix(const PBMatrix& a){std::cout<<'[';for(int i=0;i<24;++i){if(i)std::cout<<',';std::cout<<'[';for(int j=0;j<24;++j)std::cout<<(j?",":"")<<'"'<<a[i][j].x<<'"';std::cout<<']';}std::cout<<']';}
void pb_giac_poly(std::ostream& out,const PBPoly& p,const std::string& variable){out<<'(';bool any=false;for(int i=0;i<=p.degree();++i)if(p.at(i)!=Z(0)){if(any)out<<'+';any=true;out<<p.at(i).x;if(i)out<<'*'<<variable<<'^'<<i;}if(!any)out<<'0';out<<')';}
void pb_control_program(std::ostream& out,const PrimitiveBasis& pb){
    out<<"taskprime:="<<prime<<";\ntaskfield:=GF(taskprime,";pb_giac_poly(out,pb.minimal,"k");out<<",[k,K,g],undef);\ng:=taskfield(k);\na:=";pb_giac_poly(out,pb.to_power(pb.control_a()),"g");out<<";\nb:=";pb_giac_poly(out,pb.to_power(pb.control_b()),"g");out<<";\nprint(\"PB_TYPE\",type(g));\nprint(\"PB_REDUCTION\",g^24);\ntyped_basis:=gbasis([x+y-a-b,x*y-a*b],[x,y],plex);\nprint(\"PB_BASIS\",typed_basis);\nprint(\"PB_ROOT1\",normal(subst(typed_basis,[x,y],[a,b])));\nprint(\"PB_ROOT2\",normal(subst(typed_basis,[x,y],[b,a])));\nprint(\"PB_NONROOT\",normal(subst(typed_basis,[x,y],[0,0])));\n";
}
int main(int argc,char** argv){try{
    require(argc==5,"usage: primitive_field kernel3-study source-images.json inverse-system.json control.giac");std::string root=argv[1];initialize_cover(read_json(root+"/inputs/p192-d7-prym.json"));Level2Input input;PrimitiveBasis pb;std::cerr<<"fixed beta has power-basis rank24\n";pb.irreducibility();std::cerr<<"degree24 polynomial irreducibility passed\n";
    int products=0,inverses=0,frobenius=0,roundtrips=0;pb.audit(products,inverses,frobenius,roundtrips);auto source=read_json(argv[2]),system=read_json(argv[3]);int public_values=0;
    for(auto& image:source.at("images").arr)for(auto& c:image.at("coordinates").arr){auto value=pb_read_t24(c);require(pb.from_power(pb.to_power(value))==value,"public source coordinate round trip");++public_values;}
    for(auto& image:system.at("source_images").arr)for(auto& c:image.at("normalized_H").arr){auto value=pb_read_t24(c);require(pb.from_power(pb.to_power(value))==value,"public inverse image round trip");++public_values;}
    require(products==576&&inverses==24&&frobenius==24&&roundtrips==24&&public_values==56,"fixed primitive bridge acceptance counts");std::ofstream program(argv[4]);require(bool(program),"write Giac primitive control");pb_control_program(program,pb);program.close();require(bool(program),"complete Giac primitive control write");
    std::cout<<"{\"scope\":\"P192-kernel3-exact-Fp24-power-basis\",\"standing\":\"producer-unverified\",\"rank\":24,\"basis_roundtrips\":24,\"basis_product_checks\":576,\"basis_inverse_checks\":24,\"basis_Frobenius_checks\":24,\"public_coordinate_roundtrips\":56,\"corruption_controls\":1,\"minimal_polynomial\":";pb_emit_poly(std::cout,pb.minimal);std::cout<<",\"tower_from_power\":";pb_emit_matrix(pb.tower_from_power);std::cout<<",\"power_from_tower\":";pb_emit_matrix(pb.power_from_tower);std::cout<<"}\n";
}catch(const std::exception& e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
