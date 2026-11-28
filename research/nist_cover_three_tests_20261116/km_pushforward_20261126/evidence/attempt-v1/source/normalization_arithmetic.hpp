#pragma once
#include "quartic_km_v3_arithmetic.hpp"
#include <chrono>
#include <cstdlib>
using Coefficients=std::vector<Vector>;
Coefficients reconstruct(const QuarticKM& km,const Class& input) {
    require(input.w.size()==22 && row_basis(input.w,evaluation_count).size()==22,"valid RR space dimension");
    Coefficients result;
    for (const auto& row:input.w) {
        require(row.size()==evaluation_count,"input evaluation width");Space equations;
        for (int i=0;i<evaluation_count;++i) {
            Vector eq;for (const auto& v:km.v3) eq.push_back(v[size_t(i)]);eq.push_back(-row[size_t(i)]);equations.push_back(std::move(eq));
        }
        auto kernel=nullspace(equations,int(km.v3.size()+1));require(kernel.size()==1 && kernel[0].back()!=Z(0),"class row belongs to V3");
        Vector c(kernel[0].begin(),kernel[0].end()-1);Z scale=kernel[0].back().inv();for (auto& x:c) x=x*scale;
        require(linear_combination(c,km.v3)==row,"exact numerator reconstruction");result.push_back(std::move(c));
    }
    return result;
}
int infinity_multiplicity(const QuarticKM& km,const Class& input) {
    auto coefficients=reconstruct(km,input);int degree=-1;
    for (const auto& row:coefficients) for (size_t i=0;i<row.size();++i) if (row[i]!=Z(0)) degree=std::max(degree,km.mons[i][0]+km.mons[i][1]);
    int m=9-degree;require(m>=0 && m<=3,"effective degree12 infinity multiplicity");return m;
}
Class shift(const QuarticKM& km,const Class& input,int exponent) {
    Space rows=input.w;
    for (auto& row:rows) for (size_t i=0;i<row.size();++i) {
        Z l(km.points[size_t(km.pool[i])].x-56),factor=zpow(l,std::abs(exponent));
        row[i]=exponent<0?row[i]/factor:row[i]*factor;
    }
    return {row_basis(rows,evaluation_count)};
}
struct Normalized { int removed=0;Class finite; };
Normalized normalize(const QuarticKM& km,const Class& input) {
    int m=infinity_multiplicity(km,input);Class finite=shift(km,input,m);
    require(infinity_multiplicity(km,finite)==0,"normalization removes all infinity support");return {m,finite};
}
