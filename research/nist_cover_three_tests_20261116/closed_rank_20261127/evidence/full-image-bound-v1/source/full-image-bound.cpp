#include "full_bound_arithmetic.hpp"

namespace full_bound {
using AP=audit::Point;
using Sparse=norm_test::Sparse;
struct Witness {audit::Quad symbols;Sparse row;std::string classification;SCRow coefficients;size_t sources;};
struct Domain {int multiple;size_t pairs;std::string digest;std::vector<Witness> rows;};
void complete_pairs(const std::vector<norm_test::Record>& pairs,size_t width) {
    const size_t alphabet=2*width+1;
    require(pairs.size()==alphabet*(alphabet+1)/2,"complete full-image pair count");
    size_t offset=0;
    for(size_t i=0;i<alphabet;++i)for(size_t j=i;j<alphabet;++j)
        require(pairs.at(offset++).labels==audit::Pair{int(i),int(j)},"complete full-image pair labels");
}
Sparse shifted(const Sparse& input) {
    Sparse result;
    for(auto [column,value]:input)result[column<225?column:column+204]=value;
    return result;
}
int run(int argc,char** argv) {
    require(argc==5,"usage: full-image-bound BASELINE TANGENTS EXPANDED OLD_NORMS");
    const auto baseline=read_json(argv[1]),tangents=read_json(argv[2]),expanded=read_json(argv[3]),old_norms=read_json(argv[4]);
    require(expanded.at("controls_pass").scalar=="true"&&expanded.at("closed_rank").integer()==6&&
            expanded.at("selected_sections").integer()==128&&expanded.at("expanded_curve_pool").integer()==1434&&
            old_norms.at("controls_pass").scalar=="true"&&old_norms.at("closed_rank").integer()==6,
            "frozen successful dependencies");
    std::vector<AP> base;
    std::set<std::string> labels;
    auto add_point=[&](const Json& record) {
        const AP p=audit::read_point(record);
        require(!p.zero&&audit::on_curve(p)&&p.y<=audit::p-p.y&&labels.insert(p.x.get_str()).second,
                "canonical unique nonzero image");
        base.push_back(p);
    };
    for(const auto& record:tangents.at("projected_base").arr)add_point(record);
    require(base.size()==429&&baseline.at("projected_base").arr.size()==225,"original fixed widths");
    for(size_t i=0;i<225;++i)require(base[i]==audit::read_point(baseline.at("projected_base").arr[i]),"original225prefix");
    for(const auto& record:expanded.at("selected_images").arr)add_point(record);
    require(base.size()==685,"complete retained image union");
    audit::SSL ssl;
    auto valid_row=[&](const Sparse& row) {
        return norm_test::row_sum(norm_test::dense(row,685),base).zero&&ec_ssl_zero(row,base,ssl);
    };
    Sparse reference;AP kappa;
    for(const auto& id:baseline.at("pool").at("split_fibers").arr[0].at("point_indices").arr) {
        const auto& image=baseline.at("images").arr.at(size_t(id.integer()));
        kappa=audit::plus(kappa,audit::read_point(image.at("point")));
        const int column=image.at("projected_index").integer();
        if(column>=0)reference[column]+=image.at("sign").integer();
    }
    require(!kappa.zero&&norm_test::row_sum(norm_test::dense(reference,685),base)==kappa,"exact reference expression");
    auto matrix=sc_read_matrix(tangents.at("matrix"));
    require(matrix.size()==108,"original108rows");
    for(auto& row:matrix) {
        require(row.size()==429,"original429width");row.resize(685);
        require(valid_row(ec_sparse(row)),"original row both group engines");
    }
    require(baseline.at("rows").arr.size()==6,"six baseline rows");
    for(size_t i=0;i<6;++i) {
        const auto expected=norm_test::dense(ec_read_sparse(baseline.at("rows").arr[i],225),685);
        for(size_t j=0;j<685;++j)require(sc_reduce(matrix[i][j]-expected[j])==0,"exact baseline binding");
    }
    std::set<Sparse> seeds;
    for(const auto& record:expanded.at("seed_sections").arr) {
        const auto row=shifted(ec_read_sparse(record.at("row"),481));
        require(seeds.insert(row).second&&valid_row(row),"distinct mapped seed row in both engines");
        matrix.push_back(norm_test::dense(row,685));
    }
    require(seeds.size()==128&&matrix.size()==236&&sc_rank(matrix)==236&&ec_aux_rank(matrix)==230,
            "full active-auxiliary236/230/6basis");
    MIBasis basis(matrix);
    std::set<Sparse> imported;
    size_t imported_old=0,imported_new=0;
    for(const auto& record:old_norms.at("rows").arr) {
        SCRow row;
        for(const auto& value:record.at("row").arr)row.emplace_back(value.scalar);
        require(row.size()==429,"old norm row width");row.resize(685);
        const auto sparse=ec_sparse(row);
        require(valid_row(sparse)&&basis.representation(row).first,"old norm actual group and basis binding");
        imported.insert(sparse);++imported_old;
    }
    for(const auto& record:expanded.at("norm_rows").arr) {
        const auto row=shifted(ec_read_sparse(record.at("row"),481));
        require(valid_row(row)&&basis.representation(norm_test::dense(row,685)).first,"expanded norm actual group and basis binding");
        imported.insert(row);++imported_new;
    }
    require(imported_old==109&&imported_new==135,"complete saved kappa inventories");
    auto reconstructed=basis.representation(matrix[0]);
    require(reconstructed.first,"basis control representation");
    ++reconstructed.second[0];require(!basis.verifies(matrix[0],reconstructed.second),"changed basis coefficient rejects");
    bool numerical_positive=false;
    for(size_t column=0;column<225&&!numerical_positive;++column) {
        SCRow row(685);row[column]=1;
        if(basis.representation(row).first)continue;
        MIBasis control=basis;control.append(row,"synthetic:old-unit");
        require(sc_rank(control.source)==237&&ec_aux_rank(control.source)==230,"separate numerical closed7control");
        numerical_positive=true;
    }
    require(numerical_positive,"nonempty independent old-unit control");
    std::vector<Domain> domains;
    size_t additional_dependent=0,additional_independent=0;
    for(int multiple:{0,1,2}) {
        Sparse ref;
        if(multiple)for(auto [column,value]:reference)if(value)ref[column]=multiple*value;
        auto stream=norm_test::enumerate(base,norm_test::multiply(kappa,SCBig(multiple)));
        complete_pairs(stream.pairs,685);
        const auto found=norm_test::grouped(stream.pairs,ref),checked=norm_test::sorted(stream.pairs,ref);
        require(found==checked,"full-domain grouped/sorted rows and witnesses");
        if(multiple==1)for(const auto& row:imported)require(found.count(row),"every imported kappa row recovered");
        Domain domain{multiple,stream.pairs.size(),stream.digest,{}};
        for(const auto& [row,word]:found) {
            require(valid_row(row),"every retained row in both group engines");
            const auto dense=norm_test::dense(row,685);
            auto [dependent,coefficients]=basis.representation(dense);
            const bool known=multiple==1&&imported.count(row);
            require(!known||dependent,"known imported row stays dependent");
            const std::string classification=known?"imported":dependent?"new-dependent":"new-independent";
            const size_t sources=basis.source.size();
            domain.rows.push_back({word,row,classification,dependent?coefficients:SCRow{},sources});
            if(!known&&dependent)++additional_dependent;
            if(!dependent) {
                ++additional_independent;
                basis.append(dense,"target:"+std::to_string(multiple)+":"+std::to_string(domain.rows.size()-1));
            }
        }
        std::cerr<<"completed target "<<multiple<<" pairs "<<stream.pairs.size()<<" rows "<<found.size()<<'\n';
        domains.push_back(std::move(domain));
    }
    const auto generator=EC_GROUP_get0_generator(ssl.group.get());
    audit::BN gx(BN_new()),gy(BN_new());
    require(EC_POINT_get_affine_coordinates(ssl.group.get(),generator,gx.get(),gy.get(),ssl.context.get())==1,"public fixture generator");
    auto integer=[](const audit::BN& n) {
        char* value=BN_bn2dec(n.get());require(value,"fixture decimal");audit::Big out(value);OPENSSL_free(value);return out;
    };
    const AP g{false,integer(gx),integer(gy)};
    const std::vector<AP> fixture{g,audit::plus(g,g)};
    for(int multiple:{0,1,2}) {
        Sparse ref;if(multiple)ref[0]=4*multiple;
        auto stream=norm_test::enumerate(fixture,norm_test::multiply(g,SCBig(4*multiple)));
        complete_pairs(stream.pairs,2);
        const auto a=norm_test::grouped(stream.pairs,ref),b=norm_test::sorted(stream.pairs,ref);
        const Sparse expected=multiple?Sparse{{0,-4*multiple},{1,2*multiple}}:Sparse{{0,-2},{1,1}};
        require(a==b&&a.count(expected)&&norm_test::row_sum(norm_test::dense(expected,2),fixture).zero&&
                ec_ssl_zero(expected,fixture,ssl),"nonempty three-target fixture in both engines");
        auto changed=expected;++changed[0];
        require(!norm_test::row_sum(norm_test::dense(changed,2),fixture).zero&&!ec_ssl_zero(changed,fixture,ssl),
                "changed fixture coefficient rejects");
        auto missing=a;missing.erase(expected);
        norm_test::rejects_norm([&]{require(missing.count(expected),"missing fixture row");});
        stream.pairs.pop_back();norm_test::rejects_norm([&]{complete_pairs(stream.pairs,2);});
    }
    const size_t full=sc_rank(basis.source),aux=ec_aux_rank(basis.source);
    require(full==basis.source.size()&&full>=aux+6,"direct full-domain elimination");
    std::cout<<"{\"scope\":\"full-retained-image-cross-domain-bound\",\"old_columns\":225,\"original_auxiliary_columns\":204,"
             <<"\"new_auxiliary_columns\":256,\"total_image_columns\":685,\"active_auxiliary_columns\":460,"
             <<"\"imported_old_kappa_rows\":"<<imported_old<<",\"imported_new_kappa_rows\":"<<imported_new
             <<",\"imported_kappa_union_rows\":"<<imported.size()<<",\"new_dependent_rows\":"<<additional_dependent
             <<",\"new_independent_rows\":"<<additional_independent<<",\"matrix_rows\":"<<basis.source.size()
             <<",\"matrix_columns\":685,\"restored_ranks_are_derived\":false,\"full_rank\":"<<full
             <<",\"auxiliary_rank\":"<<aux<<",\"closed_rank\":"<<full-aux<<",\"candidate_gain\":"<<full-aux-6
             <<",\"balanced_conic_pattern_excluded\":"<<(domains[2].rows.empty()?"true":"false")
             <<",\"zero_quartets_only_formal\":"<<((domains[0].rows.size()==1&&domains[0].rows[0].row.empty())?"true":"false")
             <<",\"controls_pass\":true,\"numerical_positive_control_closed_rank\":7,\"synthetic_rows_in_actual_matrix\":0,"
             <<"\"positive_gain_established\":false,\"basis_labels\":[";
    for(size_t i=0;i<basis.labels.size();++i)std::cout<<(i?",":"")<<'"'<<basis.labels[i]<<'"';
    std::cout<<"],\"domains\":[";
    for(size_t i=0;i<domains.size();++i) {
        const auto& domain=domains[i];
        std::cout<<(i?",":"")<<"{\"target_multiple\":"<<domain.multiple<<",\"pairs\":"<<domain.pairs
                 <<",\"pair_stream_sha256\":\""<<domain.digest<<"\",\"distinct_rows_including_zero\":"<<domain.rows.size()<<",\"rows\":[";
        for(size_t j=0;j<domain.rows.size();++j) {
            const auto& witness=domain.rows[j];
            std::cout<<(j?",":"")<<"{\"symbols\":[";
            for(size_t k=0;k<4;++k)std::cout<<(k?",":"")<<witness.symbols[k];
            std::cout<<"],\"row\":";ec_emit_sparse(norm_test::dense(witness.row,685));
            std::cout<<",\"classification\":\""<<witness.classification<<"\",\"basis_source_count\":"<<witness.sources
                     <<",\"coefficients\":[";
            bool comma=false;
            for(size_t k=0;k<witness.coefficients.size();++k)if(witness.coefficients[k]!=0) {
                std::cout<<(comma?",":"")<<'['<<k<<",\""<<sc_reduce(witness.coefficients[k])<<"\"]";comma=true;
            }
            std::cout<<"]}";
        }
        std::cout<<"]}";
    }
    std::cout<<"],\"standing\":\"producer-unverified;necessary image bound;positive candidates need actual cover/KM and separate replay\"}\n";
    return 0;
}
}
int main(int argc,char** argv) {
    try{return full_bound::run(argc,argv);}
    catch(const std::exception& error){std::cerr<<"REJECT "<<error.what()<<'\n';return 1;}
}
