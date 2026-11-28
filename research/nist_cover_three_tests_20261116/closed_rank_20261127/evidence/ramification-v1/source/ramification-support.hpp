#pragma once

namespace ramification {
using AP=audit::Point;
BP bp_power(BP a,int n){BP result=constant(F(1));for(int i=0;i<n;++i)result=result*a;return result;}
BP substitute(const HP& f,const std::array<BP,3>& xyz) {
    std::array<std::vector<BP>,3> powers;
    for(size_t axis=0;axis<3;++axis){powers[axis].push_back(constant(F(1)));for(int n=1;n<=f.degree;++n)powers[axis].push_back(powers[axis].back()*xyz[axis]);}
    BP result;
    for(const auto& [e,c]:f.t)result=result+(powers[0][size_t(e[0])]*powers[1][size_t(e[1])]*powers[2][size_t(e[2])]).scale(c);
    return result;
}
std::array<F,3> linear_coefficients(const BP& line) {
    require(line.degree()>=0&&line.degree()<=1,"nonzero homogeneous linear section in affine coordinates");
    std::array<F,3> result{};const std::array<Exponent,3> e{{{1,0},{0,1},{0,0}}};
    for(size_t i=0;i<3;++i){const auto at=line.t.find(e[i]);if(at!=line.t.end())result[i]=at->second;}
    return result;
}
struct Axis {
    FM polar,reflection;
    std::vector<F> center;
    size_t rank=0;
    bool complete=false,center_off_axis=false,quartic_invariant=false,map_compatible=false;
};
std::array<BP,3> matrix_coordinates(const FM& matrix) {
    require(matrix.size()==3&&matrix[0].size()==3,"projective reflection matrix shape");
    std::array<BP,3> result;
    for(size_t i=0;i<3;++i){result[i].term({1,0},matrix[i][0]);result[i].term({0,1},matrix[i][1]);result[i].term({0,0},matrix[i][2]);}
    return result;
}
bool invariant(const BP& curve,const FM& reflection){return substitute(homogenize(curve,4),matrix_coordinates(reflection)).t==curve.t;}
Axis axis_test(const BP& curve,const BP& line,const std::array<BP,4>* map=nullptr) {
    const auto ell=linear_coefficients(line);size_t pivot=0;while(pivot<3&&ell[pivot]==F(0))++pivot;
    require(pivot<3,"nonzero axis");std::array<size_t,2> free{};size_t count=0;for(size_t i=0;i<3;++i)if(i!=pivot)free[count++]=i;
    std::array<BP,3> coordinates;coordinates[free[0]].term({1,0},F(1));coordinates[free[1]].term({0,1},F(1));
    coordinates[pivot]=coordinates[free[0]].scale(-ell[free[0]]/ell[pivot])+coordinates[free[1]].scale(-ell[free[1]]/ell[pivot]);
    Axis out;out.polar=fs_zero(4,3);const auto homogeneous=homogenize(curve,4);
    for(size_t axis=0;axis<3;++axis) {
        const auto restricted=substitute(homogeneous.partial(int(axis)),coordinates);
        require(restricted.degree()<=3,"restricted polar degree");
        for(size_t k=0;k<4;++k){const auto at=restricted.t.find(Exponent{int(k),3-int(k)});if(at!=restricted.t.end())out.polar[k][axis]=at->second;}
    }
    const auto reduced=fs_rref(out.polar);out.rank=reduced.pivots.size();
    if(out.rank==3){out.complete=true;return out;}
    if(out.rank!=2)return out;
    size_t variable=0;while(std::find(reduced.pivots.begin(),reduced.pivots.end(),variable)!=reduced.pivots.end())++variable;
    out.center.assign(3,F(0));out.center[variable]=F(1);
    for(size_t i=0;i<2;++i)out.center[reduced.pivots[i]]=-reduced.a[i][variable];
    for(const auto& row:out.polar){F value;for(size_t j=0;j<3;++j)value=value+row[j]*out.center[j];require(value==F(0),"polar center nullspace check");}
    F normalizer;for(size_t i=0;i<3;++i)normalizer=normalizer+ell[i]*out.center[i];out.complete=true;
    if(normalizer==F(0))return out;
    out.center_off_axis=true;for(auto& value:out.center)value=value/normalizer;
    out.reflection=fs_eye(3);
    for(size_t i=0;i<3;++i)for(size_t j=0;j<3;++j)out.reflection[i][j]=out.reflection[i][j]-F(2)*out.center[i]*ell[j];
    require(fs_mul(out.reflection,out.reflection)==fs_eye(3),"reflection squares to identity");
    out.quartic_invariant=invariant(curve,out.reflection);if(!out.quartic_invariant||map==nullptr)return out;
    out.map_compatible=true;const auto xyz=matrix_coordinates(out.reflection);
    for(size_t i=0;i<4;i+=2) {
        const int d=std::max((*map)[i].degree(),(*map)[i+1].degree());
        const auto a=substitute(homogenize((*map)[i],d),xyz),b=substitute(homogenize((*map)[i+1],d),xyz);
        if(!(a*(*map)[i+1]-(*map)[i]*b).reduced(curve).t.empty())out.map_compatible=false;
    }
    return out;
}
struct AxisControls {size_t positive_rank=0,sheared_rank=0,negative_rank=0;};
AxisControls axis_controls() {
    BP x,y;x.term({1,0},F(1));y.term({0,1},F(1));const auto z=constant(F(1));
    const auto fermat=bp_power(x,4)+bp_power(y,4)+z;
    const auto first=axis_test(fermat,z);
    auto diagonal=fs_eye(3);diagonal[2][2]=F(-1);
    require(first.complete&&first.rank==2&&first.center_off_axis&&first.quartic_invariant&&
            first.reflection==diagonal,"Fermat infinity-axis reflection control");
    const FM change{{F(1),F(0),F(2)},{F(0),F(1),F(3)},{F(1),F(1),F(1)}};
    const auto transformed=substitute(homogenize(fermat,4),matrix_coordinates(change));
    const auto hidden=axis_test(transformed,x+y+z);
    const auto expected=fs_mul(fs_mul(fs_inverse(change),diagonal),change);
    require(hidden.complete&&hidden.rank==2&&hidden.center_off_axis&&hidden.quartic_invariant&&
            hidden.reflection==expected,"non-coordinate conjugated reflection control");
    const auto negative=axis_test(fermat+x,z);
    require(negative.complete&&negative.rank==2&&negative.center_off_axis&&!negative.quartic_invariant,
            "necessary polar condition alone rejects odd perturbation");
    auto changed=expected;changed[0][2]=changed[0][2]+F(1);
    require(!invariant(transformed,changed),"mutated reflection rejected");
    return {first.rank,hidden.rank,negative.rank};
}
Point frobenius(const Point& point){return point.zero?Point{}:Point(point.x.frobenius(),point.y.frobenius());}
struct Local {
    Point image;
    Value x,y;
    int line_order=0,index=0,solved=0,free=0;
};
Local local_map(const BP& curve,const std::array<BP,4>& map,const BP& line,const K& x,const K& y) {
    precision=45;const auto jet=lift(homogenize(curve,4),{R{x},R{y},R{K(1)}},2);
    require(jet.xyz[2]==R{K(1)},"affine local chart fixed");
    const auto a=evaluate(homogenize(map[0]),jet.xyz),b=evaluate(homogenize(map[1]),jet.xyz),
               c=evaluate(homogenize(map[2]),jet.xyz),d=evaluate(homogenize(map[3]),jet.xyz);
    const int oa=order(a),ob=order(b),oc=order(c),od=order(d);
    Local result;result.x={oa-ob,a.at(oa)/b.at(ob),oa,ob};result.y={oc-od,c.at(oc)/d.at(od),oc,od};
    result.image=endpoint(result.x,result.y);result.line_order=order(evaluate(homogenize(line,1),jet.xyz));
    result.solved=jet.solved;result.free=jet.free;
    if(result.image.zero)result.index=result.x.valuation-result.y.valuation;
    else if(result.image.y==K(0)) {
        require(K(3)*result.image.x*result.image.x-K(3)!=K(0),"smooth elliptic two-torsion parameter");result.index=result.y.valuation;
    } else result.index=order(a-b.scale(result.image.x))-ob;
    require(result.index>0&&result.index<=6&&result.index==result.line_order+1,"actual tame local ramification index");
    return result;
}
struct Branch {
    Poly factor,x,y;
    int multiplicity=0,degree=0,image_degree=0;
    Local local;
    AP trace;
    int trace_column=-1,trace_sign=0;
    bool conjugate_checked=false;
};
struct Exceptional {int pool_id=0;F x,y;Local local;};
void emit_extension_point(const Point& point) {
    std::cout<<"{\"infinity\":"<<(point.zero?"true":"false");
    if(!point.zero){std::cout<<",\"x_coefficients\":";emit(point.x.p);std::cout<<",\"y_coefficients\":";emit(point.y.p);}
    std::cout<<'}';
}
void emit_ap(const AP& point) {
    std::cout<<"{\"infinity\":"<<(point.zero?"true":"false");
    if(!point.zero)std::cout<<",\"x\":\""<<point.x<<"\",\"y\":\""<<point.y<<'"';
    std::cout<<'}';
}
void emit_matrix(const FM& matrix) {
    std::cout<<'[';for(size_t i=0;i<matrix.size();++i){if(i)std::cout<<',';std::cout<<'[';for(size_t j=0;j<matrix[i].size();++j)std::cout<<(j?",":"")<<'"'<<decimal(matrix[i][j])<<'"';std::cout<<']';}std::cout<<']';
}
void emit_local(const Local& local) {
    std::cout<<"{\"line_order\":"<<local.line_order<<",\"ramification_index\":"<<local.index<<",\"solved_axis\":"<<local.solved<<",\"free_axis\":"<<local.free
             <<",\"X_valuation\":"<<local.x.valuation<<",\"Y_valuation\":"<<local.y.valuation<<",\"X_numerator_order\":"<<local.x.numerator_order
             <<",\"X_denominator_order\":"<<local.x.denominator_order<<",\"Y_numerator_order\":"<<local.y.numerator_order<<",\"Y_denominator_order\":"<<local.y.denominator_order<<",\"X_leading\":";
    emit(local.x.leading.p);std::cout<<",\"Y_leading\":";emit(local.y.leading.p);std::cout<<",\"image\":";emit_extension_point(local.image);std::cout<<'}';
}
}
