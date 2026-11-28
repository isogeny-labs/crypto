#pragma once
// Checked arithmetic/basis prefix reused without the conic scan.
#define main frozen_expanded_conic_norm_entrypoint
#include "norms.cpp"
#undef main
#include "conic_arithmetic.hpp"
#include <cstdint>

using ECSparse=norm_test::Sparse;
using JAnchors=std::array<int,2>;
struct ECPencil {JAnchors anchors;};
struct ECDigest {
    std::unique_ptr<EVP_MD_CTX,decltype(&EVP_MD_CTX_free)> state{EVP_MD_CTX_new(),EVP_MD_CTX_free};
    ECDigest(){require(bool(state)&&EVP_DigestInit_ex(state.get(),EVP_sha256(),nullptr)==1,"conic digest initialize");}
    void add(size_t pencil,const std::string& key,const std::vector<int>& ids) {
        std::string payload=std::to_string(pencil)+":"+key;
        for(int id:ids)payload+=std::to_string(id)+",";
        require(payload.size()<65536,"bounded candidate digest frame");
        const auto n=std::uint32_t(payload.size());
        std::array<unsigned char,4> frame{};
        for(size_t i=0;i<4;++i)frame[i]=static_cast<unsigned char>((n>>(8*i))&255U);
        require(EVP_DigestUpdate(state.get(),frame.data(),frame.size())==1&&
                EVP_DigestUpdate(state.get(),payload.data(),payload.size())==1,"conic digest update");
    }
    std::string finish() {
        std::array<unsigned char,32> bytes{};unsigned int n=0;
        require(EVP_DigestFinal_ex(state.get(),bytes.data(),&n)==1&&n==32,"conic digest final");
        std::ostringstream out;
        for(auto b:bytes)out<<std::hex<<std::setw(2)<<std::setfill('0')<<unsigned(b);
        return out.str();
    }
};
audit::Point ec_gmp(const ::Point& p) {
    if(p.zero)return {};
    return {false,audit::Big(decimal(p.x.p.at(0))),audit::Big(decimal(p.y.p.at(0)))};
}
ECSparse ec_sparse(const SCRow& row) {
    ECSparse out;
    for(size_t i=0;i<row.size();++i) {
        SCBig value=sc_reduce(row[i]);if(value>sc_n/2)value-=sc_n;
        require(value>=-16&&value<=16,"small conic source coefficient");
        if(value!=0)out[int(i)]=int(value.get_si());
    }
    return out;
}
ECSparse ec_read_sparse(const Json& row,size_t width) {
    ECSparse out;
    for(const auto& pair:row.arr) {
        const int column=pair.arr.at(0).integer(),coefficient=pair.arr.at(1).integer();
        require(column>=0&&size_t(column)<width&&coefficient&&coefficient>=-16&&coefficient<=16&&
                out.emplace(column,coefficient).second,"canonical saved sparse row");
    }
    return out;
}
bool ec_ssl_zero(const ECSparse& row,const std::vector<audit::Point>& base,const audit::SSL& ssl) {
    auto sum=ssl.make({}),term=ssl.make({});
    for(auto [column,coefficient]:row) {
        auto p=ssl.make(base.at(size_t(column)));
        auto scalar=audit::bn(audit::Big(std::abs(coefficient)));
        require(EC_POINT_mul(ssl.group.get(),term.get(),nullptr,p.get(),scalar.get(),ssl.context.get())==1,
                "conic named coefficient multiplication");
        if(coefficient<0)require(EC_POINT_invert(ssl.group.get(),term.get(),ssl.context.get())==1,"conic named negation");
        require(EC_POINT_add(ssl.group.get(),sum.get(),sum.get(),term.get(),ssl.context.get())==1,"conic named row sum");
    }
    return EC_POINT_is_at_infinity(ssl.group.get(),sum.get())==1;
}
SCRow ec_embed(const SCRow& row) {
    require(row.size()==481,"expanded geometric row width");
    SCRow result(685);
    for(size_t i=0;i<row.size();++i)result[i<225?i:i+204]=row[i];
    return result;
}
size_t ec_aux_rank(const SCMat& matrix) {
    SCMat auxiliary;
    for(const auto& row:matrix)auxiliary.emplace_back(row.begin()+225,row.end());
    return sc_rank(auxiliary);
}
void ec_emit_sparse(const SCRow& row) {
    std::cout<<'[';bool comma=false;
    for(auto [column,coefficient]:ec_sparse(row)) {
        if(comma)std::cout<<',';
        comma=true;
        std::cout<<'['<<column<<','<<coefficient<<']';
    }
    std::cout<<']';
}
CV mi_coefficients(const BP& q) {
    CV out{};const std::array<Exponent,6> powers{{{0,0},{1,0},{0,1},{2,0},{1,1},{0,2}}};
    for(size_t i=0;i<6;++i){const auto at=q.t.find(powers[i]);out[i]=at==q.t.end()?F(0):at->second;}
    return cnormalize(out);
}
struct MIBasis {
    SCMat source,reduced,provenance;
    std::vector<size_t> pivots;
    std::vector<std::string> labels;
    explicit MIBasis(SCMat rows):source(std::move(rows)) {
        for(size_t i=0;i<source.size();++i)labels.push_back("base:"+std::to_string(i));
        rebuild();
    }
    void rebuild() {
        require(!source.empty()&&source.size()<=256&&source[0].size()<=1024,"native independent-basis caps");
        reduced=source;provenance.assign(source.size(),SCRow(source.size()));pivots.clear();
        for(size_t i=0;i<source.size();++i) {
            require(source[i].size()==source[0].size(),"basis rectangle");
            provenance[i][i]=1;
            for(auto& value:reduced[i])value=sc_reduce(value);
        }
        size_t rank=0;
        for(size_t column=0;column<source[0].size()&&rank<source.size();++column) {
            size_t pivot=rank;while(pivot<source.size()&&reduced[pivot][column]==0)++pivot;
            if(pivot==source.size())continue;
            std::swap(reduced[pivot],reduced[rank]);std::swap(provenance[pivot],provenance[rank]);
            SCBig inverse;
            require(mpz_invert(inverse.get_mpz_t(),reduced[rank][column].get_mpz_t(),sc_n.get_mpz_t())==1,"basis inverse");
            for(auto& value:reduced[rank])value=sc_reduce(value*inverse);
            for(auto& value:provenance[rank])value=sc_reduce(value*inverse);
            for(size_t i=0;i<source.size();++i)if(i!=rank&&reduced[i][column]!=0) {
                const SCBig coefficient=reduced[i][column];
                for(size_t j=0;j<source[0].size();++j)reduced[i][j]=sc_reduce(reduced[i][j]-coefficient*reduced[rank][j]);
                for(size_t j=0;j<source.size();++j)provenance[i][j]=sc_reduce(provenance[i][j]-coefficient*provenance[rank][j]);
            }
            pivots.push_back(column);++rank;
        }
        require(rank==source.size(),"stored numerical basis stays independent");
    }
    bool verifies(const SCRow& row,const SCRow& coefficients)const {
        if(row.size()!=source[0].size()||coefficients.size()!=source.size())return false;
        SCRow reconstructed(row.size());
        for(size_t i=0;i<source.size();++i)if(coefficients[i]!=0)
            for(size_t j=0;j<row.size();++j)if(source[i][j]!=0)reconstructed[j]+=coefficients[i]*source[i][j];
        for(size_t j=0;j<row.size();++j)if(sc_reduce(reconstructed[j]-row[j])!=0)return false;
        return true;
    }
    std::pair<bool,SCRow> representation(const SCRow& row)const {
        require(row.size()==source[0].size(),"representation width");
        SCRow work=row,coefficients(source.size());
        for(auto& value:work)value=sc_reduce(value);
        for(size_t i=0;i<pivots.size();++i) {
            const SCBig value=work[pivots[i]];
            if(value==0)continue;
            for(size_t j=0;j<work.size();++j)work[j]=sc_reduce(work[j]-value*reduced[i][j]);
            for(size_t j=0;j<source.size();++j)coefficients[j]=sc_reduce(coefficients[j]+value*provenance[i][j]);
        }
        const bool dependent=std::all_of(work.begin(),work.end(),[](const SCBig& value){return value==0;});
        if(dependent)require(verifies(row,coefficients),"exact row-space reconstruction");
        return {dependent,std::move(coefficients)};
    }
    void append(const SCRow& row,const std::string& label) {
        require(source.size()<256&&!representation(row).first,"independent append within existing cap");
        source.push_back(row);labels.push_back(label);rebuild();
    }
};
