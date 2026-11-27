#include "target_weil_arithmetic.hpp"
#include "first_order.hpp"

TP read_tp(const Json& j){std::vector<T24> c;for(auto& a:j.arr)c.push_back(read_t24(a));return TP(c);}
FiniteDivisor read_finite(const Json& j){return {read_tp(j.at("U")),read_tp(j.at("V"))};}
Matrix3<T24> read_matrix3(const Json& j){require(j.arr.size()==3,"matrix3 shape");Matrix3<T24> out{};for(int i=0;i<3;++i)out[i]=read_line(j.arr[i]);return out;}
std::vector<T24> source_normalization(const Level2Input& input,const QuotientSection& section,int& checks){
    auto generic=generic_inputs(section.source_kernel);GT24 z(generic[0]),heldout(generic[1]);
    std::vector<GT24> shifted;for(auto& s:input.classes)shifted.push_back(z.add(s));
    std::vector<T24> alpha(64,T24(1)),raw;for(auto& s:input.classes){auto v=eta2(s,z);require(v!=T24(0),"differential source normalization nonzero");raw.push_back(v);}
    for(int bit=0;bit<6;++bit){int b=1<<bit;alpha[b]=sqrt24(T24(1)/(raw[b]*eta2(input.classes[b],shifted[b])));
        for(int mask=1;mask<b;++mask){T24 gamma=T24(d2(mask,b))*raw[mask^b]/(raw[mask]*eta2(input.classes[b],shifted[mask]));alpha[mask^b]=alpha[mask]*alpha[b]/gamma;}
    }
    std::vector<T24> test;for(auto& s:input.classes)test.push_back(eta2(s,heldout));
    for(int p=0;p<64;++p){auto translated=heldout.add(input.classes[p]);for(int bit=0;bit<6;++bit){int q=1<<bit;require(alpha[p]*test[p]*alpha[q]*eta2(input.classes[q],translated)==T24(d2(p,q))*alpha[p^q]*test[p^q],"differential source heldout cocycle");++checks;}}
    return alpha;
}
bool regular_theta(const GT24& z,const QuotientSection& section){
    if(z.u.degree()!=3)return false;
    for(int i=1;i<27;++i)if(determinant3(multiplication3(z.u,embed_polynomial<T24>(section.functions[i].support.u)))==T24(0))return false;
    return true;
}
FO first_theta(const FOG& z,const QuotientSection& section){
    require(z.u.degree()==3,"first-order theta degree3");FO value(1);
    for(int i=1;i<27;++i){const auto& f=section.functions[i];auto u=first_constant(embed_polynomial<T24>(f.support.u)),v=first_constant(embed_polynomial<T24>(f.support.v));
        auto a=first_constant(embed_polynomial<T24>(f.a)),b=first_constant(embed_polynomial<T24>(f.b));auto m=multiplication3(z.u,u);FO den=determinant3(m);require(den.value!=T24(0),"first-order theta regular denominator");
        auto w=(z.v+v)%z.u;for(int j=0;j<3;++j)m[j][2]=w.at(j);FO c=determinant3(m),norm=determinant3(multiplication3(z.u,a+b*z.v));
        value=value+FO(T24(section.alpha[i]))*c*c*c*norm/(den*den*den);
    }return value;
}
struct SourceDifferentials {GT24 base;int representative=0,excluded=0,checks=0;std::array<Line,4> gradients{};std::array<Matrix3<T24>,4> forms{};};
SourceDifferentials source_differentials(const Level2Input& input,QuotientSection& section,const std::vector<T24>& alpha){
    SourceDifferentials out;std::array<int,4> labels{63,38,41,13};
    for(int j=1;j<27;++j){auto base=input.classes[input.delta].add(section.kernel[j]);bool usable=regular_theta(base,section);
        for(int s:labels)if(usable&&!regular_theta(base.add(input.classes[source_code(s)]),section))usable=false;
        if(usable){for(int s:labels)if(determinant3(multiplication3(base.u,input.classes[source_code(s)].u))==T24(0)){usable=false;break;}}
        if(usable&&section.theta(base)!=T24(0)){out.base=base;out.representative=j;break;}
        ++out.excluded;std::cerr<<"source differential excludes kernel representative "<<j<<'\n';
    }
    require(out.representative>0,"regular differential representative bound");
    auto f=GT24::f,fp=polynomial_partial(f);FOG::f=first_constant(f);TP x{T24(0),T24(1)};Matrix3<T24> velocity{};std::array<Line,4> directions{};
    for(int flow=0;flow<3;++flow){TP h=tpower(x,flow),dx=(out.base.v*h).scale(T24(2))%out.base.u,dy=(fp*h)%out.base.u;
        auto arc=moving_cubic({out.base.u,out.base.v},dx,dy);arc.check();
        for(int j=0;j<3;++j)velocity[j][flow]=cubic_trace(out.base.u,tpower(x,j+flow));
        for(int i=0;i<4;++i){auto shifted=arc.add(first_constant(input.classes[source_code(labels[i])]));shifted.check();auto value=first_theta(shifted,section);require(value.value==T24(0),"source odd theta zero at differential origin");directions[i][flow]=value.derivative;++out.checks;}
        std::cerr<<"source differential flow "<<flow<<" passed\n";
    }
    T24 theta0=section.theta(out.base);for(int i=0;i<4;++i){int code=source_code(labels[i]);out.gradients[i]=abel_gradient(velocity,directions[i]);
        T24 eta=alpha[code]*eta2(input.classes[code],out.base);out.forms[i]=outer3(out.gradients[i],eta*eta*eta/(theta0*theta0));}
    return out;
}
struct TargetDifferentials {std::array<Line,4> gradients{};std::array<Matrix3<T24>,4> forms{};int checks=0;};
TargetDifferentials target_differentials(TargetWeil& target,const FiniteDivisor& e0,const std::vector<T24>& scales){
    TargetDifferentials out;std::array<int,4> labels{63,38,41,13};TP x{T24(0),T24(1)},one{T24(1)};
    auto fx=target.f.partial(0),fy=target.f.partial(1);TP vx=target.equation(x,e0.v,e0.u);require(vx.degree()<0,"target origin curve");
    auto eval=[&](const Form& f){TP value;for(auto& [e,c]:f.c)value=(value+(tpower(x,e[0])*tpower(e0.v,e[1])).scale(c))%e0.u;return value;};
    TP Fx=eval(fx),Fy=eval(fy);std::array<TP,3> h{one,x,e0.v},canonical{x,e0.v,one};Matrix3<T24> velocity{};std::array<Line,4> directions{};
    for(int flow=0;flow<3;++flow){auto arc=moving_cubic(e0,(Fy*h[flow])%e0.u,(-Fx*h[flow])%e0.u);require(first_equation(target.f,arc).degree()<0,"target first-order curve equation");
        for(int j=0;j<3;++j)velocity[j][flow]=cubic_trace(e0.u,canonical[j]*h[flow]);
        for(int i=0;i<4;++i){auto basis=target.weil_basis(labels[i]^52);Matrix3<FO> m{};for(int j=0;j<3;++j){auto value=conic_value(basis[j],FOP{FO(0),FO(1)},arc.v)%arc.u;for(int k=0;k<3;++k)m[k][j]=value.at(k);}
            auto value=determinant3(m);require(value.value==T24(0),"target odd determinant zero at origin");directions[i][flow]=value.derivative;++out.checks;}
    }
    for(int i=0;i<4;++i){out.gradients[i]=abel_gradient(velocity,directions[i]);const auto& l=target.lines.at(labels[i]^52);TP ell=TP{l[2]}+x.scale(l[0])+e0.v.scale(l[1]);
        T24 norm=determinant3(multiplication3(e0.u,ell)),den=e0.v.at(2)*e0.v.at(2)*norm;require(den!=T24(0),"target differential Weil unit");out.forms[i]=outer3(out.gradients[i],scales[i]/den);}
    return out;
}
T24 common_ratio(const std::array<Matrix3<T24>,4>& a,const std::array<Matrix3<T24>,4>& b){
    T24 ratio;bool set=false;for(int n=0;n<4;++n)for(int i=0;i<3;++i)for(int j=0;j<3;++j){
        if(b[n][i][j]!=T24(0)&&!set){ratio=a[n][i][j]/b[n][i][j];set=true;}
        if(set)require(a[n][i][j]==ratio*b[n][i][j],"four-form common differential scalar");else require(a[n][i][j]==T24(0),"matching quadratic form zero");
    }require(set&&ratio!=T24(0),"nonzero common differential scalar");return ratio;
}
Z base_value(const T24& c){require(c.a.degree()<=0&&c.a.at(0).a.degree()<=0,"required original-field scalar");return c.a.at(0).a.at(0);}
int symbol(const T24& a){auto z=base_value(a);mpz_class result;mpz_powm(result.get_mpz_t(),z.x.get_mpz_t(),mpz_class((prime-1)/2).get_mpz_t(),prime.get_mpz_t());require(result==1||result==prime-1,"nonzero scalar quadratic character");return result==1?1:-1;}

int main(int argc,char** argv){try{
    require(argc==2,"usage: differential_twist study-directory");std::string root=argv[1];initialize_cover(read_json(root+"/inputs/p192-d7-prym.json"));Level2Input input;QuotientSection section(root);TargetWeil target(root);
    auto receipt=read_json(root+"/evidence/target-weil-v3/result.json"),descent=read_json(root+"/evidence/descent-v1/result.json");auto e0=read_finite(receipt.at("canonical_origin_divisor"));target.check(e0,3);
    std::vector<T24> scales;for(auto& a:receipt.at("scales").arr)scales.push_back(read_t24(a));require(scales.size()==4,"four pinned target normalizations");
    int normalization_checks=0;auto alpha=source_normalization(input,section,normalization_checks);std::cerr<<"source normalization passed\n";
    auto a=source_differentials(input,section,alpha);auto c=target_differentials(target,e0,scales);std::cerr<<"source and target derivative forms passed\n";
    auto B=read_matrix3(descent.at("change_of_coordinates"));
    auto transformed=transform_form(target.f,B);T24 lambda=transformed.c.begin()->second;auto differential=scaled3(B,determinant3(B)/lambda);
    std::array<Matrix3<T24>,4> qc{};for(int i=0;i<4;++i)qc[i]=multiply3(transpose3(differential),multiply3(c.forms[i],differential));
    std::array<int,4> labels{63,38,41,13};std::array<Line,4> lc{};for(int i=0;i<4;++i)lc[i]=row_times(target.lines.at(labels[i]),B);
    std::array<Line,3> fc{lc[0],lc[1],lc[2]},fa{a.gradients[0],a.gradients[1],a.gradients[2]};auto target_frame=frame_coordinates(lc[3],fc),source_frame=frame_coordinates(a.gradients[3],fa);Matrix3<T24> LC=fc,M{};
    for(int j=0;j<3;++j){Line rhs;for(int i=0;i<3;++i){require(target_frame[i]!=T24(0)&&source_frame[i]!=T24(0),"four-gradient frame");rhs[i]=source_frame[i]/target_frame[i]*fa[i][j];}auto column=solve3(LC,rhs);for(int i=0;i<3;++i)M[i][j]=column[i];}
    T24 first;for(auto& row:M)for(auto& v:row)if(first==T24(0)&&v!=T24(0))first=v;require(first!=T24(0)&&determinant3(M)!=T24(0),"nonsingular projective differential matrix");M=scaled3(M,first.inv());
    for(auto& row:M)for(auto& v:row)base_value(v);
    std::array<Matrix3<T24>,4> pulled{};for(int i=0;i<4;++i)pulled[i]=multiply3(transpose3(M),multiply3(qc[i],M));
    auto s=common_ratio(a.forms,pulled);base_value(s);int character=symbol(s);
    require(common_ratio(pulled,pulled)==T24(1),"identical-model scalar control");T24 nonsquare(2);for(int j=2;j<64;++j){nonsquare=T24(j);if(symbol(nonsquare)==-1)break;}require(symbol(nonsquare)==-1,"fixed nonsquare control bound");
    auto twisted=pulled;for(auto& form:twisted)form=scaled3(form,nonsquare);require(symbol(common_ratio(twisted,pulled))==-1,"synthetic differential twist discriminator");
    rejects([&]{auto changed=a.forms;changed[0][0][0]=changed[0][0][0]+T24(1);common_ratio(changed,pulled);},"changed differential quadratic form");
    rejects([&]{FO nilpotent(T24(0),T24(1));nilpotent.inv();},"nilpotent first-order denominator");
    std::cout<<"{\"scope\":\"P192-kernel1-conditional-differential-twist\",\"source_normalization_checks\":"<<normalization_checks<<",\"source_representative\":"<<a.representative<<",\"excluded_representatives\":"<<a.excluded<<",\"source_derivative_checks\":"<<a.checks<<",\"target_derivative_checks\":"<<c.checks<<",\"common_quadratic_forms\":4,\"scalar_quadratic_character\":"<<character<<",\"conditional_on_marked_ppav_normalization\":true,\"divisor_map_constructed\":false,\"standing\":\"producer-unverified\",\"source_base_divisor\":";emit_finite({a.base.u,a.base.v});std::cout<<",\"projective_differential\":";emit_matrix(M);std::cout<<",\"target_differential_change\":";emit_matrix(differential);std::cout<<",\"common_scalar\":";emit_t24(s);
    std::cout<<",\"source_forms\":[";for(int i=0;i<4;++i){if(i)std::cout<<',';emit_matrix(a.forms[i]);}std::cout<<"],\"target_base_forms\":[";for(int i=0;i<4;++i){if(i)std::cout<<',';emit_matrix(qc[i]);}std::cout<<"]}\n";
}catch(const std::exception& e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
