#pragma once
// Exact capped independent basis reused from the checked conic collector.
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

