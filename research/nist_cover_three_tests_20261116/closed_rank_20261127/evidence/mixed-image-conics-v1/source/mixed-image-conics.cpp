#define main frozen_expanded_conic_norm_entrypoint
#include "norms.cpp"
#undef main
#include "conic_arithmetic.hpp"
#include <cstdint>

using ECSparse=norm_test::Sparse;
struct ECPencil {std::array<int,2> line_pair;Anchors anchors;};
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

int main(int argc,char** argv) {
try {
    require(argc>=11&&argc<=13,"usage: mixed-image-conics QUARTIC BACKMAP BASELINE TANGENTS OLD_SCAN EXPANDED BALANCED X_FIBRE_RESULT plan SCHEDULE | batch PLAN START COUNT | combine PLAN BATCH_LIST");
    const auto previous=read_json(argv[7]),next_previous=read_json(argv[8]);
    const std::string mode=argv[9];
    require((mode=="plan"&&argc==11)||(mode=="batch"&&argc==13)||(mode=="combine"&&argc==12),"explicit mixed-image stage");
    require(previous.at("controls_pass").scalar=="true"&&previous.at("balanced_conic_pattern_excluded").scalar=="true"&&
            previous.at("closed_rank").integer()==6,"frozen balanced-image exclusion");
    require(next_previous.at("controls_pass").scalar=="true"&&next_previous.at("supported_conics").integer()==0&&
            next_previous.at("closed_rank").integer()==6&&next_previous.at("all_x_fibre_pencils").integer()==625,
            "frozen completed x-fibre family");
    const auto model=read_json(argv[1]),back=read_json(argv[2]),baseline=read_json(argv[3]),
               tangents=read_json(argv[4]),scan=read_json(argv[5]),expanded=read_json(argv[6]);
    ss_private(baseline,tangents);
    require(expanded.at("controls_pass").scalar=="true"&&expanded.at("closed_rank").integer()==6&&
            expanded.at("expanded_curve_pool").integer()==1434&&expanded.at("selected_sections").integer()==128,
            "successful frozen expansion dependency");
    const auto curve=read_bp(model.at("terms"),true);
    std::array<BP,4> map;size_t position=0;
    for(const auto* name:{"X_numerator","X_denominator","Y_numerator","Y_denominator"})map[position++]=read_bp(back.at(name));
    require(global_identity(curve,map),"actual expanded-conic map identity");
    std::vector<F> leading(5);
    for(const auto& [e,a]:curve.t)if(e[0]+e[1]==4)leading[size_t(e[1])]=a;
    const Poly infinity(std::move(leading)),z{F(0),F(1)};
    require(gcd(infinity,wide_power(z,prime(),infinity)-z).degree()==0,"no omitted rational source infinity");
    std::vector<::Point> base;
    std::vector<audit::Point> gmp_base,original_base;
    std::set<std::string> column_x;
    for(const auto& q:tangents.at("projected_base").arr) {
        const auto p=point(q);valid(p);
        require(!p.zero&&BN_cmp(p.y.p.at(0).n.get(),(-p.y.p.at(0)).n.get())<=0&&
                column_x.insert(decimal(p.x.p.at(0))).second,"original429canonical image columns");
        original_base.push_back(ec_gmp(p));
        if(base.size()<225){base.push_back(p);gmp_base.push_back(ec_gmp(p));}
    }
    require(original_base.size()==429,"original full image width");
    for(size_t i=0;i<225;++i)require(base[i]==point(baseline.at("projected_base").arr.at(i)),"old225prefix unchanged");
    require(expanded.at("selected_images").arr.size()==256&&expanded.at("fibres").arr.size()==256,
            "all256new image/fibre records");
    for(const auto& q:expanded.at("selected_images").arr) {
        const auto p=point(q);valid(p);
        require(!p.zero&&BN_cmp(p.y.p.at(0).n.get(),(-p.y.p.at(0)).n.get())<=0&&
                column_x.insert(decimal(p.x.p.at(0))).second,"new canonical auxiliary image column");
        base.push_back(p);gmp_base.push_back(ec_gmp(p));
    }
    require(base.size()==481,"expanded geometric image width");
    std::vector<CPoint> points;
    std::map<std::string,int> coordinates;
    auto append=[&](const Json& q,int index,int sign) {
        const auto x=coefficient(q.at("x")),y=coefficient(q.at("y"));
        const auto actual=fs_image(curve,map,x,y).value;
        require(actual==point(q.at("image"))&&coordinates.emplace(sp_key(x,y),int(points.size())).second,
                "fresh distinct expanded conic-pool image");
        if(index<0)require(index==-1&&sign==0&&actual.zero,"identity pool label");
        else {
            require(size_t(index)<base.size()&&(sign==1||sign==-1),"signed pool label bounds");
            auto expected=base.at(size_t(index));if(sign<0)expected.y=-expected.y;
            require(actual==expected,"pool exact canonical image/sign");
        }
        const auto fx=fs_eval(curve.partial(0),x,y),fy=fs_eval(curve.partial(1),x,y);
        require(fx!=F(0)||fy!=F(0),"smooth expanded pool point");
        points.push_back({{x,y,actual,index,sign},{F(1),x,y,x*x,x*y,y*y},
                          {F(0),fy,-fx,F(2)*x*fy,y*fy-x*fx,-F(2)*y*fx}});
    };
    for(const auto& q:scan.at("points").arr) {
        require(q.at("index").integer()<225,"old scan uses only original image domain");
        append(q,q.at("index").integer(),q.at("sign").integer());
    }
    require(points.size()==652,"unchanged old pool prefix");
    for(size_t i=0;i<256;++i) {
        const auto& fibre=expanded.at("fibres").arr.at(i);
        require(fibre.at("target").integer()==int(i),"ordered frozen fibre inventory");
        for(const auto& q:fibre.at("points").arr) {
            const auto claimed=point(q.at("image"));
            require(!claimed.zero&&claimed.x==base[225+i].x,"exact new fibre image x");
            append(q,int(225+i),claimed.y==base[225+i].y?1:-1);
        }
    }
    require(points.size()==1434&&points.size()<=3000,"complete bounded actual expanded pool");
    for(size_t i=0;i<226;++i) {
        const auto& q=baseline.at("pool").at("points").arr.at(i);
        require(decimal(points[i].value.x)==q.at("x").scalar&&decimal(points[i].value.y)==q.at("y").scalar&&
                points[i].value.image==point(baseline.at("images").arr.at(i).at("point")),"original226bindings");
    }
    std::vector<Anchors> known;
    std::vector<BP> lines;
    for(const auto& fibre:baseline.at("pool").at("split_fibers").arr) {
        Anchors ids{};
        require(fibre.at("point_indices").arr.size()==4,"original split degree4");
        for(size_t i=0;i<4;++i)ids[i]=fibre.at("point_indices").arr[i].integer();
        const auto& a=points.at(size_t(ids[0])).value;
        const auto& b=points.at(size_t(ids[1])).value;
        BP line;line.term({0,0},b.y*a.x-b.x*a.y);line.term({1,0},a.y-b.y);line.term({0,1},b.x-a.x);
        for(int id:ids)require(fs_eval(line,points.at(size_t(id)).value.x,points.at(size_t(id)).value.y)==F(0),
                               "original split line incidence");
        known.push_back(ids);lines.push_back(std::move(line));
    }
    require(known.size()==7,"original seven complete sections");
    SCRow reference(481);::Point kappa;
    for(int id:known[0]) {
        const auto& p=points.at(size_t(id)).value;kappa=add(kappa,p.image);
        if(p.index>=0)reference[size_t(p.index)]+=p.sign;
    }
    audit::SSL ssl;
    auto check_row=[&](const SCRow& row) {
        require(norm_test::row_sum(row,gmp_base).zero&&ec_ssl_zero(ec_sparse(row),gmp_base,ssl),
                "independent GMP/named conic or seed row sum");
    };
    std::vector<SCRow> line_rows;
    for(const auto& ids:known) {
        SCRow row(481);
        for(int id:ids){const auto& p=points.at(size_t(id)).value;if(p.index>=0)row[size_t(p.index)]+=p.sign;}
        for(size_t i=0;i<row.size();++i)row[i]-=reference[i];
        check_row(row);line_rows.push_back(std::move(row));
    }
    auto matrix=sc_read_matrix(tangents.at("matrix"));
    for(auto& row:matrix) {
        require(norm_test::row_sum(row,original_base).zero&&ec_ssl_zero(ec_sparse(row),original_base,ssl),
                "original108row sums in both implementations");
        row.resize(685);
    }
    std::set<ECSparse> seed_rows,allowed_rows;
    allowed_rows.insert(ECSparse{});
    for(const auto& row:baseline.at("rows").arr) {
        const auto sparse=ec_read_sparse(row,225);
        allowed_rows.insert(sparse);allowed_rows.insert(norm_test::opposite(sparse));
    }
    for(const auto& saved:expanded.at("seed_sections").arr) {
        const int i=saved.at("i").integer(),j=saved.at("j").integer();
        require(i>=0&&j>=0&&i<652&&j<652,"saved seed origins in old pool");
        const bool tangent=saved.at("tangent").scalar=="true";
        require(tangent?i==j:i<j,"saved seed secant/tangent order");
        const auto& a=points[size_t(i)].value;const auto& b=points[size_t(j)].value;
        const auto dx=coefficient(saved.at("direction").arr[0]),dy=coefficient(saved.at("direction").arr[1]);
        require(tangent?(dx==fs_eval(curve.partial(1),a.x,a.y)&&dy==-fs_eval(curve.partial(0),a.x,a.y)):
                        (dx==b.x-a.x&&dy==b.y-a.y),"saved seed direction binding");
        const auto restriction=ss_restrict(curve,a.x,a.y,dx,dy);
        require(restriction==read_poly(saved.at("restriction")),"actual saved seed restriction");
        const auto roots=ss_quadratic(restriction.exact(tangent?Poly{F(0),F(0),F(1)}:Poly{F(0),F(-1),F(1)}));
        require(roots.size()==2,"saved split residual factor product");
        std::array<std::string,2> computed{sp_key(a.x+dx*roots[0],a.y+dy*roots[0]),sp_key(a.x+dx*roots[1],a.y+dy*roots[1])},recorded;
        std::array<int,4> ids{i,j,0,0};
        for(size_t k=0;k<2;++k) {
            const auto& q=saved.at("residual_points").arr.at(k);
            recorded[k]=q.at("x").scalar+":"+q.at("y").scalar;
            ids[2+k]=coordinates.at(recorded[k]);
            require(points[size_t(ids[2+k])].value.image==point(q.at("image")),"saved residual exact image");
        }
        std::sort(computed.begin(),computed.end());std::sort(recorded.begin(),recorded.end());
        require(computed==recorded,"saved residual geometry and factors agree");
        SCRow row(481);
        for(int id:ids){const auto& p=points[size_t(id)].value;if(p.index>=0)row[size_t(p.index)]+=p.sign;}
        for(size_t k=0;k<row.size();++k)row[k]-=reference[k];
        const auto sparse=ec_sparse(row);
        require(sparse==ec_read_sparse(saved.at("row"),481)&&seed_rows.insert(sparse).second,"saved seed row exact binding");
        check_row(row);matrix.push_back(ec_embed(row));allowed_rows.insert(sparse);
        require(!tangent,"declared mixed-image family uses split secant seeds");
        std::set<int> distinct(ids.begin(),ids.end());require(distinct.size()==4,"four distinct known seed support points");
        BP line;line.term({0,0},dy*a.x-dx*a.y);line.term({1,0},-dy);line.term({0,1},dx);
        for(int id:ids)require(fs_eval(line,points[size_t(id)].value.x,points[size_t(id)].value.y)==F(0),"known seed line incidence");
        known.push_back(ids);lines.push_back(std::move(line));line_rows.push_back(row);
    }
    require(seed_rows.size()==128&&matrix.size()==236,"all128actual seed rows");
    std::set<ECSparse> norm_rows;
    for(const auto& row:expanded.at("norm_rows").arr) {
        const auto sparse=ec_read_sparse(row.at("row"),481);
        require(allowed_rows.count(sparse)&&norm_rows.insert(sparse).second,"saved norm row belongs to seed/baseline span");
    }
    require(norm_rows.size()==135,"all saved necessary norm rows bound");
    for(const auto& row:seed_rows)require(norm_rows.count(row),"all seeds in saved necessary norm inventory");
    require(sc_rank(matrix)==236&&ec_aux_rank(matrix)==230,"starting full236aux230closed6");
    auto changed_seed=norm_test::dense(*seed_rows.begin(),481);++changed_seed[0];
    require(!norm_test::row_sum(changed_seed,gmp_base).zero&&!ec_ssl_zero(ec_sparse(changed_seed),gmp_base,ssl),
            "changed seed coefficient rejects in both implementations");
    auto uncontaminated=[&](const SCMat& full) {
        for(size_t i=108;i<full.size();++i)for(size_t j=225;j<429;++j)
            require(sc_reduce(full[i][j])==0,"new row cannot touch peeled original private columns");
    };
    uncontaminated(matrix);
    auto contaminated=matrix;contaminated[108][225]=1;
    fs_reject([&]{uncontaminated(contaminated);});
    SCMat core;
    for(size_t i=0;i<matrix.size();++i) {
        if(i>=6&&i<108)continue;
        SCRow row;
        row.insert(row.end(),matrix[i].begin(),matrix[i].begin()+225);
        row.insert(row.end(),matrix[i].begin()+429,matrix[i].end());
        core.push_back(std::move(row));
    }
    require(core.size()==134&&core[0].size()==481&&sc_rank(core)==134&&ec_aux_rank(core)==128,
            "exact private-row elimination gives134full128auxiliary6closed");
    matrix=std::move(core);
    require(known.size()==135&&lines.size()==135&&line_rows.size()==135,"all135known split lines");
    MIBasis initial_basis(matrix);
    std::vector<SCRow> line_representations;
    std::set<std::string> distinct_lines;
    for(size_t i=0;i<lines.size();++i) {
        require(distinct_lines.insert(ckey(mi_coefficients(lines[i]))).second,"distinct known split lines");
        const auto representation=initial_basis.representation(line_rows[i]);
        require(representation.first,"every known line row belongs to initial basis");
        line_representations.push_back(representation.second);
    }
    const auto represented=initial_basis.representation(line_rows[1]);
    require(represented.first&&!ec_sparse(line_rows[1]).empty(),"nonzero known row representation control");
    auto bad_coefficients=represented.second;bad_coefficients[0]=sc_reduce(bad_coefficients[0]+1);
    require(!initial_basis.verifies(line_rows[1],bad_coefficients),"changed representation coefficient rejects");
    size_t free_old=0;
    while(free_old<225&&std::find(initial_basis.pivots.begin(),initial_basis.pivots.end(),free_old)!=initial_basis.pivots.end())++free_old;
    require(free_old<225,"synthetic old-column direction outside current span");
    auto numerical_fixture=initial_basis;SCRow unit(481);unit[free_old]=1;
    numerical_fixture.append(unit,"synthetic-old-unit");
    require(sc_rank(numerical_fixture.source)==135&&ec_aux_rank(numerical_fixture.source)==128,
            "separate numerical positive control has closed rank7");
    std::map<std::string,std::array<int,2>> known_products;
    for(size_t i=0;i<lines.size();++i)for(size_t j=i;j<lines.size();++j) {
        const auto key=ckey(mi_coefficients(lines[i]*lines[j]));
        require(known_products.emplace(key,std::array<int,2>{int(i),int(j)}).second,"unique normalized line-product reference");
    }
    require(known_products.size()==lines.size()*(lines.size()+1)/2,"complete known product dictionary");
    ECDigest product_digest;size_t product_number=0;
    for(const auto& [key,pair]:known_products)product_digest.add(product_number++,key,{pair[0],pair[1]});
    const auto product_sha=product_digest.finish();
    std::vector<ECPencil> pencils;
    std::set<Anchors> distinct_pencils;
    auto admit=[&](std::array<int,2> pair,Anchors anchors) {
        std::sort(anchors.begin(),anchors.end());
        require(std::adjacent_find(anchors.begin(),anchors.end())==anchors.end()&&cgeneral(anchors,points),"mixed-image general-position quadruple");
        require(distinct_pencils.insert(anchors).second,"distinct mixed-image pencil");
        std::set<int> image_labels;::Point sum;size_t new_points=0;
        for(int id:anchors) {
            const auto& p=points.at(size_t(id)).value;
            image_labels.insert(p.index);sum=add(sum,p.image);if(id>=652)++new_points;
        }
        require(image_labels.size()>1&&new_points>=2&&!sum.zero,"new mixed-image nonbalanced pencil outside completed families");
        pencils.push_back({pair,anchors});
    };
    for(size_t s=0;s<128;++s) {
        const size_t old_line=s%7,new_line=7+s;
        auto old_ids=known[old_line];std::sort(old_ids.begin(),old_ids.end());
        const std::array<int,2> residual{known[new_line][2],known[new_line][3]};
        require(residual[0]>=652&&residual[1]>=652,"new residual anchors");
        std::optional<Anchors> first;
        for(size_t i=0;i<4&&!first;++i)for(size_t j=i+1;j<4&&!first;++j) {
            const Anchors a{old_ids[i],old_ids[j],residual[0],residual[1]};
            if(cgeneral(a,points))first=a;
        }
        require(bool(first),"general old/new split-line pencil exists");
        admit({int(old_line),int(new_line)},*first);
    }
    for(size_t s=0;s<64;++s) {
        const size_t i=7+s,j=7+s+64;
        admit({int(i),int(j)},{known[i][2],known[i][3],known[j][2],known[j][3]});
    }
    require(pencils.size()==192,"complete stratified192pencil plan");
    const size_t expected_new_pencils=192;
    auto omitted_pencils=pencils;omitted_pencils.pop_back();
    fs_reject([&]{require(omitted_pencils.size()==expected_new_pencils,"omitted pencil accounting");});
    ECDigest plan_digest;
    for(size_t i=0;i<pencils.size();++i) {
        const auto& p=pencils[i];
        plan_digest.add(i,std::to_string(p.line_pair[0])+","+std::to_string(p.line_pair[1])+":",std::vector<int>(p.anchors.begin(),p.anchors.end()));
    }
    const auto plan_sha=plan_digest.finish();
    auto emit_pencils=[&]() {
        std::cout<<'[';
        for(size_t i=0;i<pencils.size();++i) {
            if(i)std::cout<<',';
            std::cout<<"{\"line_pair\":["<<pencils[i].line_pair[0]<<','<<pencils[i].line_pair[1]<<"],\"anchors\":[";
            for(size_t j=0;j<4;++j)std::cout<<(j?",":"")<<pencils[i].anchors[j];
            std::cout<<"]}";
        }
        std::cout<<']';
    };
    if(mode=="plan") {
        std::ofstream schedule(argv[10]);require(bool(schedule),"native schedule output");
        for(size_t first=0;first<pencils.size();first+=128)schedule<<first<<' '<<std::min(size_t(128),pencils.size()-first)<<'\n';
        schedule.close();require(bool(schedule),"complete native schedule write");
        std::cout<<"{\"scope\":\"native-stratified-mixed-image-conic-plan\",\"pool_size\":1434,\"old_columns\":225,"
                 <<"\"known_lines\":135,\"known_product_references\":"<<known_products.size()
                 <<",\"known_product_reference_sha256\":\""<<product_sha<<"\",\"old_new_pencils\":128,\"new_new_pencils\":64,"
                 <<"\"mixed_image_pencils\":"<<pencils.size()
                 <<",\"plan_sha256\":\""<<plan_sha<<"\",\"core_full_rank\":134,\"core_auxiliary_rank\":128,"
                 <<"\"closed_rank\":6,\"private_rows_removed\":102,\"private_columns_removed\":204,\"setup_controls_pass\":true,\"pencils\":";
        emit_pencils();std::cout<<",\"line_rows\":[";
        for(size_t i=0;i<lines.size();++i) {
            if(i)std::cout<<',';
            std::cout<<"{\"line_id\":"<<i<<",\"support\":[";
            for(size_t j=0;j<4;++j)std::cout<<(j?",":"")<<known[i][j];
            std::cout<<"],\"row\":";ec_emit_sparse(line_rows[i]);
            std::cout<<",\"initial_basis_certificate\":[";bool comma=false;
            for(size_t j=0;j<line_representations[i].size();++j)if(line_representations[i][j]!=0) {
                if(comma)std::cout<<',';
                comma=true;
                std::cout<<'['<<j<<",\""<<line_representations[i][j].get_str()<<"\"]";
            }
            std::cout<<"]}";
        }
        std::cout<<"]}\n";return 0;
    }
    const auto frozen_plan=read_json(argv[10]);
    auto bind_plan=[&](const Json& plan) {
        require(plan.at("plan_sha256").scalar==plan_sha&&plan.at("pencils").arr.size()==pencils.size()&&
                plan.at("known_product_reference_sha256").scalar==product_sha,"exact frozen mixed-image plan and product reference binding");
        for(size_t i=0;i<pencils.size();++i) {
            const auto& q=plan.at("pencils").arr[i];
            for(size_t j=0;j<2;++j)require(q.at("line_pair").arr.at(j).integer()==pencils[i].line_pair[j],"exact frozen plan parent lines");
            for(size_t j=0;j<4;++j)require(q.at("anchors").arr.at(j).integer()==pencils[i].anchors[j],"exact frozen plan anchor tuple");
        }
        require(plan.at("line_rows").arr.size()==135,"all planned parent-line rows");
        for(size_t i=0;i<135;++i) {
            const auto& line=plan.at("line_rows").arr[i];
            require(line.at("line_id").integer()==int(i)&&ec_read_sparse(line.at("row"),481)==ec_sparse(line_rows[i]),"exact planned parent row");
            for(size_t j=0;j<4;++j)require(line.at("support").arr.at(j).integer()==known[i][j],"exact planned parent support");
            SCRow coefficients(134);std::set<int> indices;
            for(const auto& term:line.at("initial_basis_certificate").arr) {
                const int j=term.arr.at(0).integer();
                require(j>=0&&j<134&&term.arr.at(1).scalar.size()<=60&&indices.insert(j).second,"bounded parent basis certificate encoding");
                const SCBig value(term.arr.at(1).scalar);
                require(value>0&&value<sc_n,"canonical parent basis coefficient");coefficients[size_t(j)]=value;
            }
            require(initial_basis.verifies(line_rows[i],coefficients),"planned parent initial-basis certificate reconstructs");
        }
    };
    bind_plan(frozen_plan);
    auto changed_plan=frozen_plan;
    changed_plan.obj["pencils"].arr[0].obj["anchors"].arr[0].scalar="-1";
    fs_reject([&]{bind_plan(changed_plan);});
    std::map<int,Jet> jets;precision=9;
    auto multiplicity=[&](const CV& q,int index) {
        const auto& p=points.at(size_t(index));
        require(cdot(q,p.monomial)==F(0),"expanded conic local incidence");
        if(cdot(q,p.derivative)!=F(0))return 1;
        auto at=jets.find(index);
        if(at==jets.end())at=jets.emplace(index,lift(homogenize(curve),
            {R{K(p.value.x)},R{K(p.value.y)},R{K(1)}},2)).first;
        const int order_value=order(evaluate(homogenize(cbp(q),2),at->second.xyz));
        require(order_value>=2&&order_value<=8,"exact expanded conic tangent order");
        return order_value;
    };
    auto section=[&](const CV& q,const std::vector<int>& ids)->std::optional<CSection> {
        int degree=0;std::map<int,int> support;SCRow row(481);::Point sum;
        for(int index:ids) {
            const int m=multiplicity(q,index);
            require(support.emplace(index,m).second,"distinct complete conic support");
            degree+=m;
            const auto& p=points.at(size_t(index)).value;
            for(int n=0;n<m;++n)sum=add(sum,p.image);
            if(p.index>=0)row[size_t(p.index)]+=m*p.sign;
        }
        require(degree<=8,"proper conic/quartic intersection degree");
        if(degree!=8)return {};
        auto negative=kappa;if(!negative.zero)negative.y=-negative.y;
        require(add(add(sum,negative),negative).zero,"actual degree-eight image row");
        for(size_t i=0;i<row.size();++i)row[i]-=2*reference[i];
        check_row(row);
        return CSection{q,std::move(support),std::move(row)};
    };
    ECDigest digest;
    size_t candidate_occurrences=0;
    auto collect=[&](const Anchors& anchors,bool actual,size_t pindex) {
        require(cgeneral(anchors,points),"selected conic anchors in general position");
        const auto basis=cbasis(anchors,points);
        std::map<std::string,std::pair<CV,std::vector<int>>> candidates;
        for(size_t i=0;i<points.size();++i) {
            if(std::find(anchors.begin(),anchors.end(),int(i))!=anchors.end())continue;
            const auto q=cthrough(basis,points[i].monomial);
            auto [at,inserted]=candidates.emplace(ckey(q),std::make_pair(q,std::vector<int>{}));
            (void)inserted;at->second.second.push_back(int(i));
        }
        for(int index:anchors) {
            const auto q=cthrough(basis,points[size_t(index)].derivative);
            candidates.emplace(ckey(q),std::make_pair(q,std::vector<int>{}));
        }
        if(actual)candidate_occurrences+=candidates.size();
        std::map<std::string,CSection> supported;
        for(const auto& [key,value]:candidates) {
            std::vector<int> ids(anchors.begin(),anchors.end());
            ids.insert(ids.end(),value.second.begin(),value.second.end());
            require(ids.size()<=8,"complete distinct conic intersection bound");
            if(actual)digest.add(pindex,key,ids);
            bool tangent=false;
            for(int index:ids) {
                require(cdot(value.first,points[size_t(index)].monomial)==F(0),"complete parameter bucket incidence");
                if(cdot(value.first,points[size_t(index)].derivative)==F(0))tangent=true;
            }
            if(ids.size()<8&&!tangent)continue;
            const auto accepted=section(value.first,ids);
            if(accepted)supported.emplace(key,*accepted);
        }
        return supported;
    };
    auto coefficients=[&](const BP& q){return mi_coefficients(q);};
    const Anchors control_anchors{known[0][0],known[0][1],known[1][0],known[1][1]};
    const auto control_key=ckey(coefficients(lines[0]*lines[1]));
    const auto control_pencil=collect(control_anchors,false,0);
    require(control_pencil.count(control_key)==1,"nonempty same-collector known line-product fixture");
    require(sc_rank(SCMat{control_pencil.at(control_key).row})==1,"control pencil recovers nonzero relation");
    auto dropped_control=control_pencil;dropped_control.erase(control_key);
    fs_reject([&]{require(dropped_control.count(control_key)==1,"missing known control conic");});
    SCMat control_rows;
    for(size_t i=0;i<7;++i)for(size_t j=i+1;j<7;++j) {
        const auto q=coefficients(lines[i]*lines[j]);
        std::vector<int> incident;
        for(size_t k=0;k<points.size();++k)if(cdot(q,points[k].monomial)==F(0))incident.push_back(int(k));
        const auto accepted=section(q,incident);
        require(bool(accepted),"all21direct known line-product controls");
        control_rows.push_back(accepted->row);
    }
    require(control_rows.size()==21&&sc_rank(control_rows)==6,"direct control rank6");
    const auto repeated=coefficients(lines[0]*lines[0]);
    std::vector<int> reference_ids(known[0].begin(),known[0].end());
    const auto square=section(repeated,reference_ids);
    require(bool(square)&&square->support.size()==4,"repeated reference conic control");
    for(auto [index,m]:square->support){(void)index;require(m==2,"repeated reference multiplicity2");}
    require(ec_sparse(square->row).empty(),"repeated reference zero row");
    auto changed=repeated;changed[0]=changed[0]+F(1);
    fs_reject([&]{(void)section(changed,reference_ids);});
    reference_ids.pop_back();require(!section(repeated,reference_ids),"missing support remains incomplete");

    auto replay=[&](const CV& q,const std::map<int,int>& claimed_support,const ECSparse& claimed_row,
                    const Anchors& origin) {
        require(q==cnormalize(q),"canonical replay conic coefficients");
        for(int id:origin)require(cdot(q,points.at(size_t(id)).monomial)==F(0),"replay conic belongs to planned origin pencil");
        std::vector<int> incident;
        for(size_t i=0;i<points.size();++i)if(cdot(q,points[i].monomial)==F(0))incident.push_back(int(i));
        const auto accepted=section(q,incident);
        require(bool(accepted)&&accepted->support==claimed_support&&ec_sparse(accepted->row)==claimed_row,
                "replayed complete support,multiplicities and image row");
        return *accepted;
    };
    const auto& control_value=control_pencil.at(control_key);
    (void)replay(control_value.coefficients,control_value.support,ec_sparse(control_value.row),control_anchors);
    auto bad_support=control_value.support;++bad_support.begin()->second;
    fs_reject([&]{(void)replay(control_value.coefficients,bad_support,ec_sparse(control_value.row),control_anchors);});
    auto bad_row=ec_sparse(control_value.row);++bad_row[0];
    fs_reject([&]{(void)replay(control_value.coefficients,control_value.support,bad_row,control_anchors);});
    auto product_row=[&](const std::array<int,2>& pair) {
        require(pair[0]>=0&&pair[1]>=pair[0]&&pair[1]<135,"known product parent indices");
        auto row=line_rows[size_t(pair[0])];
        for(size_t i=0;i<row.size();++i)row[i]+=line_rows[size_t(pair[1])][i];
        return row;
    };
    auto product_certificate=[&](const CSection& value,const std::array<int,2>& pair) {
        require(pair[0]>=0&&pair[1]>=pair[0]&&pair[1]<135,"bounded product parent indices");
        require(value.coefficients==coefficients(lines[size_t(pair[0])]*lines[size_t(pair[1])])&&
                value.row==product_row(pair),"exact known-product polynomial and parent-row sum certificate");
    };
    product_certificate(control_value,{0,1});
    fs_reject([&]{product_certificate(control_value,{0,2});});
    std::map<std::string,CSection> sections;
    std::map<std::string,size_t> origins;
    size_t first=0,last=pencils.size(),batch_count=0;
    std::string bucket_sha;
    if(mode=="batch") {
        const int start=std::stoi(argv[11]),count=std::stoi(argv[12]);
        require(start>=0&&count>0&&count<=128&&size_t(start)<pencils.size()&&start%128==0&&
                size_t(count)==std::min(size_t(128),pencils.size()-size_t(start)),"exact admitted native batch range");
        first=size_t(start);last=first+size_t(count);batch_count=1;
        for(size_t i=first;i<last;++i) {
            const auto found=collect(pencils[i].anchors,true,i);
            const auto& pair=pencils[i].line_pair;
            const auto expected=ckey(coefficients(lines[size_t(pair[0])]*lines[size_t(pair[1])]));
            require(found.count(expected)==1,"every actual pencil recovers its nominated product conic");
            for(const auto& [key,value]:found)if(sections.emplace(key,value).second)origins.emplace(key,i);
            require(sections.size()<=4096,"supported conic cap;incomplete if exceeded");
            if((i-first)%32==0)std::cerr<<"mixed-image pencil "<<i<<" accepted "<<sections.size()<<'\n';
        }
        bucket_sha=digest.finish();
    }else {
        require(mode=="combine","explicit final combine mode");
        std::ifstream list(argv[11]);require(bool(list),"native batch-list input");
        std::string path;
        std::vector<std::array<size_t,2>> ranges;
        ECDigest receipt_digest;
        while(std::getline(list,path)) {
            require(!path.empty(),"nonempty batch path");
            const auto batch=read_json(path);
            require(batch.at("stage").scalar=="batch"&&batch.at("controls_pass").scalar=="true"&&
                    batch.at("plan_sha256").scalar==plan_sha,"batch standing and plan binding");
            const int start=batch.at("batch_start").integer(),end=batch.at("batch_end").integer();
            require(start>=0&&end>start&&size_t(end)<=pencils.size(),"batch range bounds");
            ranges.push_back({size_t(start),size_t(end)});
            const int candidates=batch.at("candidate_occurrences").integer();
            require(candidates>=0,"nonnegative native candidate count");
            candidate_occurrences+=size_t(candidates);
            receipt_digest.add(size_t(start),batch.at("candidate_bucket_stream_sha256").scalar,
                               {end,candidates,int(batch.at("sections").arr.size())});
            std::map<std::string,CSection> batch_sections;
            for(const auto& record:batch.at("sections").arr) {
                const int origin=record.at("origin_pencil").integer();
                require(origin>=start&&origin<end,"accepted conic origin in declared batch");
                CV q{};require(record.at("coefficients").arr.size()==6,"six replay conic coefficients");
                for(size_t i=0;i<6;++i)q[i]=coefficient(record.at("coefficients").arr[i]);
                std::map<int,int> support;
                for(const auto& term:record.at("support").arr) {
                    const int id=term.arr.at(0).integer(),m=term.arr.at(1).integer();
                    require(id>=0&&size_t(id)<points.size()&&m>=1&&m<=8&&support.emplace(id,m).second,
                            "valid replay support encoding");
                }
                const auto accepted=replay(q,support,ec_read_sparse(record.at("row"),481),pencils[size_t(origin)].anchors);
                const auto key=ckey(q);
                require(batch_sections.emplace(key,accepted).second,"unique conic within batch output");
                if(sections.emplace(key,accepted).second)origins.emplace(key,size_t(origin));
            }
            for(size_t i=size_t(start);i<size_t(end);++i) {
                const auto& pair=pencils[i].line_pair;
                require(batch_sections.count(ckey(coefficients(lines[size_t(pair[0])]*lines[size_t(pair[1])]))),"final combine recovers every nominated batch product");
            }
            SCMat batch_basis_rows=matrix;
            std::vector<std::string> batch_labels=initial_basis.labels;
            std::set<std::string> independent_labels;
            for(const auto& entry:batch.at("basis_conic_keys").arr) {
                const std::string label=entry.scalar;
                require(label.rfind("q:",0)==0&&independent_labels.insert(label).second,"canonical distinct batch basis label");
                const auto key=label.substr(2);
                require(batch_sections.count(key)&&!known_products.count(key),"only nonproduct conics may enlarge basis");
                batch_basis_rows.push_back(batch_sections.at(key).row);batch_labels.push_back(label);
            }
            require(batch_basis_rows.size()<=256,"batch certified basis cap");
            MIBasis batch_basis(std::move(batch_basis_rows));batch_basis.labels=batch_labels;
            std::map<std::string,size_t> batch_indices;
            for(size_t i=0;i<batch_labels.size();++i)require(batch_indices.emplace(batch_labels[i],i).second,"unique complete batch basis labels");
            for(const auto& record:batch.at("sections").arr) {
                CV q{};for(size_t i=0;i<6;++i)q[i]=coefficient(record.at("coefficients").arr[i]);
                const auto key=ckey(q);const auto& value=batch_sections.at(key);
                const auto product=known_products.find(key);
                if(product!=known_products.end()) {
                    require(record.at("classification").scalar=="known-product","batch product classification");
                    std::array<int,2> pair{};
                    for(size_t i=0;i<2;++i)pair[i]=record.at("known_product_lines").arr.at(i).integer();
                    require(pair==product->second,"exact batch product parents");product_certificate(value,pair);
                }else if(independent_labels.count("q:"+key)) {
                    require(record.at("classification").scalar=="additional-independent","batch independent-row classification");
                }else {
                    require(record.at("classification").scalar=="additional-dependent","batch dependent-row classification");
                    SCRow coefficients(batch_labels.size());std::set<std::string> seen;
                    for(const auto& term:record.at("dependence_certificate").arr) {
                        const std::string label=term.arr.at(0).scalar;
                        require(batch_indices.count(label)&&seen.insert(label).second&&term.arr.at(1).scalar.size()<=60,"batch dependency source encoding");
                        const SCBig value_coefficient(term.arr.at(1).scalar);
                        require(value_coefficient>0&&value_coefficient<sc_n,"canonical dependency coefficient");
                        coefficients[batch_indices.at(label)]=value_coefficient;
                    }
                    require(batch_basis.verifies(value.row,coefficients),"batch dependency certificate reconstructs exact row");
                }
            }
            require(sections.size()<=4096,"combined supported conic cap;incomplete if exceeded");
        }
        require(list.eof(),"complete batch-list read");
        auto coverage=[&](const std::vector<std::array<size_t,2>>& supplied) {
            size_t next=0;
            for(const auto& range:supplied) {
                require(range[0]==next&&next<pencils.size()&&range[1]==std::min(next+128,pencils.size()),
                        "ordered native batch coverage without gap,overlap or duplicate");
                next=range[1];
            }
            require(next==pencils.size(),"complete native final batch coverage");
        };
        coverage(ranges);batch_count=ranges.size();
        auto omitted=ranges;omitted.pop_back();fs_reject([&]{coverage(omitted);});
        auto duplicate=ranges;duplicate.insert(duplicate.begin(),ranges[0]);fs_reject([&]{coverage(duplicate);});
        bucket_sha=receipt_digest.finish();
    }
    struct Classification {std::string kind;std::array<int,2> product{-1,-1};std::vector<std::pair<std::string,SCBig>> certificate;};
    std::map<std::string,Classification> classifications;
    MIBasis final_basis(matrix);
    size_t product_count=0,additional_count=0,dependent_additional=0;
    for(const auto& [key,value]:sections) {
        const auto product=known_products.find(key);
        if(product!=known_products.end()) {
            product_certificate(value,product->second);++product_count;
            classifications.emplace(key,Classification{"known-product",product->second,{}});
            continue;
        }
        ++additional_count;
        const auto representation=final_basis.representation(value.row);
        if(representation.first) {
            ++dependent_additional;Classification classification{"additional-dependent",{-1,-1},{}};
            for(size_t i=0;i<representation.second.size();++i)if(representation.second[i]!=0)
                classification.certificate.emplace_back(final_basis.labels[i],representation.second[i]);
            classifications.emplace(key,std::move(classification));
        }else {
            final_basis.append(value.row,"q:"+key);
            classifications.emplace(key,Classification{"additional-independent",{-1,-1},{}});
        }
    }
    matrix=final_basis.source;
    require(matrix.size()<=256&&matrix[0].size()<=1024,"unchanged independent dense-basis caps");
    const size_t full=sc_rank(matrix),auxiliary=ec_aux_rank(matrix);
    require(full>=auxiliary+6,"old closed baseline retained after all auxiliary elimination");
    require(full==matrix.size(),"stored final basis agrees with separate rank helper");
    std::cout<<"{\"scope\":\"stratified-mixed-image-split-line-conic-pencils\",\"stage\":\""<<mode
             <<"\",\"pool_size\":1434,\"old_columns\":225,\"original_auxiliary_columns\":204,\"new_auxiliary_columns\":256,"
             <<"\"plan_sha256\":\""<<plan_sha<<"\",\"mixed_image_pencils\":"<<pencils.size()
             <<",\"known_product_references\":"<<known_products.size()<<",\"known_product_reference_sha256\":\""<<product_sha<<'"'
             <<",\"batch_start\":"<<first<<",\"batch_end\":"<<last<<",\"batch_count\":"<<batch_count
             <<",\"candidate_occurrences\":"<<candidate_occurrences<<",\"candidate_bucket_stream_sha256\":\""<<bucket_sha
             <<"\",\"supported_conics\":"<<sections.size()<<",\"known_product_conics\":"<<product_count
             <<",\"additional_conics\":"<<additional_count<<",\"dependent_additional_conics\":"<<dependent_additional
             <<",\"independent_additional_rows\":"<<matrix.size()-134<<",\"core_rows\":"<<matrix.size()<<",\"core_columns\":481,"
             <<"\"core_full_rank\":"<<full<<",\"core_auxiliary_rank\":"<<auxiliary
             <<",\"private_rows_removed\":102,\"private_columns_removed\":204,\"restored_ranks_are_derived\":true,"
             <<"\"full_rank\":"<<full+102<<",\"auxiliary_rank\":"<<auxiliary+102<<",\"closed_rank\":"<<full-auxiliary
             <<",\"candidate_gain\":"<<full-auxiliary-6<<",\"controls_pass\":true,\"control_pencil_supported_conics\":"<<control_pencil.size()
             <<",\"control_rank\":6,\"numerical_positive_control_closed_rank\":7,\"synthetic_rows_in_actual_matrix\":0,"
             <<"\"positive_gain_established\":false,\"basis_conic_keys\":[";
    for(size_t i=134;i<final_basis.labels.size();++i)std::cout<<(i==134?"":",")<<'"'<<final_basis.labels[i]<<'"';
    std::cout<<"],\"sections\":[";
    bool comma=false;
    for(const auto& [key,value]:sections) {
        (void)key;
        if(comma)std::cout<<',';
        comma=true;
        std::cout<<"{\"origin_pencil\":"<<origins.at(key)<<",\"coefficients\":[";
        for(size_t j=0;j<6;++j)std::cout<<(j?",":"")<<'"'<<decimal(value.coefficients[j])<<'"';
        std::cout<<"],\"support\":[";bool sep=false;
        for(auto [index,m]:value.support) {
            if(sep)std::cout<<',';
            sep=true;
            std::cout<<'['<<index<<','<<m<<']';
        }
        std::cout<<"],\"row\":";ec_emit_sparse(value.row);
        const auto& classification=classifications.at(key);
        std::cout<<",\"classification\":\""<<classification.kind<<'"';
        if(classification.kind=="known-product")std::cout<<",\"known_product_lines\":["<<classification.product[0]<<','<<classification.product[1]<<']';
        else {
            std::cout<<",\"dependence_certificate\":[";
            for(size_t i=0;i<classification.certificate.size();++i) {
                if(i)std::cout<<',';
                std::cout<<"[\""<<classification.certificate[i].first<<"\",\""<<classification.certificate[i].second.get_str()<<"\"]";
            }
            std::cout<<']';
        }
        std::cout<<'}';
    }
    std::cout<<"],\"standing\":\"producer-unverified;native core computed,restored counts add102private ranks;positive needs KM and separate replay\"}\n";
    return 0;
}catch(const std::exception& e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}
}
