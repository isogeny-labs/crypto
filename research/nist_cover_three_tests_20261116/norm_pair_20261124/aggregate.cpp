#include "audit_arithmetic.hpp"
#include "separate_arithmetic.hpp"

int main(int argc,char** argv) {
    try {
        using audit::Big;
        using audit::Point;
        require(argc==4,"usage: aggregate BASELINE.json QUERIES.json OUTPUT.json");
        const auto baseline=read_json(argv[1]),queries=read_json(argv[2]);
        const auto& records=queries.at("queries").arr;
        require(records.size()==226,"all frozen aggregate queries");
        audit::SSL ssl;
        OpenSSLGroup separate;
        std::vector<Point> base,images,traces;
        std::vector<audit::Pair> old_projection,new_projection;
        std::map<std::string,int> columns;
        for(const auto& q:baseline.at("projected_base").arr) {
            const auto point=audit::read_point(q);
            require(!point.zero&&audit::on_curve(point)&&point.y<=audit::p-point.y,"canonical old point");
            require(columns.emplace(audit::key(point),int(base.size())).second,"distinct old columns");
            base.push_back(point);
        }
        require(base.size()==225,"old base width");
        for(const auto& q:baseline.at("images").arr) {
            const auto point=audit::read_point(q.at("point"));
            const int j=q.at("projected_index").integer(),s=q.at("sign").integer();
            if(j<0) require(j==-1&&s==0&&point.zero,"old identity binding");
            else require((s==1||s==-1)&&point==(s==1?base.at(size_t(j)):audit::negative(base.at(size_t(j)))),"old signed binding");
            images.push_back(point);old_projection.push_back({j,s});
        }
        require(images.size()==226,"old image count");
        Point kappa;
        Row reference(225);
        for(const auto& entry:baseline.at("pool").at("split_fibers").arr[0].at("point_indices").arr) {
            const size_t i=size_t(entry.integer());kappa=audit::plus(kappa,images.at(i));
            const auto p=old_projection[i];if(p[0]>=0) reference.at(size_t(p[0]))+=p[1];
        }
        size_t aliases=0,old_aliases=0,identities=0;
        for(size_t i=0;i<records.size();++i) {
            require(records[i].at("point_index").integer()==int(i),"ordered recorded query");
            const auto trace=audit::read_point(records[i].at("trace"));
            require(trace==audit::plus(kappa,audit::negative(audit::plus(images[i],images[i])))&&audit::on_curve(trace),"recorded aggregate arithmetic");
            auto sk=ssl.make(kappa),p=ssl.make(images[i]),twice=ssl.make({});
            require(EC_POINT_dbl(ssl.group.get(),twice.get(),p.get(),ssl.context.get())==1&&
                    EC_POINT_invert(ssl.group.get(),twice.get(),ssl.context.get())==1&&
                    EC_POINT_add(ssl.group.get(),twice.get(),sk.get(),twice.get(),ssl.context.get())==1&&
                    ssl.encode(twice.get())==audit::key(trace),"OpenSSL aggregate arithmetic");
            traces.push_back(trace);
            if(trace.zero) {++identities;new_projection.push_back({-1,0});continue;}
            int sign=1;auto canonical=trace;
            if(canonical.y>audit::p-canonical.y) {canonical=audit::negative(canonical);sign=-1;}
            auto hit=columns.find(audit::key(canonical));int column;
            if(hit!=columns.end()) {
                column=hit->second;++aliases;if(column<225) ++old_aliases;
            } else {
                column=int(base.size());columns.emplace(audit::key(canonical),column);base.push_back(canonical);
            }
            new_projection.push_back({column,sign});
        }
        require(base.size()<=451,"aggregate column cap");
        Matrix matrix;
        for(const auto& sparse:baseline.at("rows").arr) {
            Row row(base.size());
            for(const auto& entry:sparse.arr) row.at(size_t(entry.arr[0].integer()))+=Big(entry.arr[1].scalar);
            matrix.push_back(row);
        }
        require(matrix.size()==6,"baseline row count");
        for(size_t i=0;i<records.size();++i) {
            Row row(base.size());
            for(size_t j=0;j<225;++j) row[j]=-reference[j];
            const auto p=old_projection.at(i),q=new_projection.at(i);
            if(p[0]>=0) row.at(size_t(p[0]))+=2*p[1];
            if(q[0]>=0) row.at(size_t(q[0]))+=q[1];
            matrix.push_back(row);
        }
        require(matrix.size()==232,"aggregate row cap and count");
        std::vector<EP> openssl_base;
        for(const auto& p:base) openssl_base.push_back(separate.point({p.zero,p.x,p.y}));
        for(const auto& row:matrix) {
            require(audit::row_sum(row,base).zero,"aggregate GMP row sum");
            auto sum=separate.empty();
            for(size_t i=0;i<row.size();++i) if(row[i]!=0) separate.add(sum,openssl_base[i],row[i]);
            require(separate.zero(sum),"aggregate OpenSSL row sum");
        }
        Matrix new_matrix;
        for(const auto& row:matrix) new_matrix.emplace_back(row.begin()+225,row.end());
        const size_t full_rank=matrix_rank(matrix),new_rank=matrix_rank(new_matrix);
        require(full_rank==size_t(separate_rank(matrix))&&new_rank==size_t(separate_rank(new_matrix)),"both independent dense rank paths");
        require(full_rank>=new_rank+6,"baseline closure bound");
        std::vector<std::pair<size_t,size_t>> witnesses;
        std::set<size_t> private_rows;
        auto check_private=[&](const Matrix& value,size_t owner,size_t col) {
            require(owner<value.size()&&col>=225&&col<value[owner].size()&&reduced(value[owner][col])!=0,"private-column owner");
            for(size_t i=0;i<value.size();++i) if(i!=owner) require(reduced(value[i][col])==0,"private-column singleton");
        };
        for(size_t col=225;col<base.size();++col) {
            size_t owner=0,count=0;
            for(size_t i=0;i<matrix.size();++i) if(reduced(matrix[i][col])!=0) {owner=i;++count;}
            if(count==1) {check_private(matrix,owner,col);witnesses.emplace_back(owner,col);private_rows.insert(owner);}
        }
        if(private_rows.size()==226) require(full_rank-new_rank==6,"all-private rows have zero closed gain");
        int controls=0;
        if(!witnesses.empty()) {
            const auto witness=witnesses[0];
            rejects([&]{check_private(matrix,0,witness.second);});++controls;
            auto changed=matrix;changed[0][witness.second]=1;
            rejects([&]{check_private(changed,witness.first,witness.second);});++controls;
            auto bad=matrix[witness.first];++bad[witness.second];
            rejects([&]{require(audit::row_sum(bad,base).zero,"changed aggregate coefficient");});++controls;
        }
        std::ofstream out(argv[3]);require(bool(out),"fresh aggregate result");
        out << "{\"standing\":\"producer-unverified\",\"scope\":\"whole-recorded-residual-trace-columns\",\"old_columns\":225,\"added_rows\":226,\"total_columns\":" << base.size()
            << ",\"new_columns\":" << base.size()-225 << ",\"canonical_trace_aliases\":" << aliases << ",\"old_trace_aliases\":" << old_aliases
            << ",\"identity_traces\":" << identities << ",\"private_added_rows\":" << private_rows.size() << ",\"full_rank\":" << full_rank
            << ",\"new_column_rank\":" << new_rank << ",\"closed_rank\":" << full_rank-new_rank << ",\"closed_gain\":" << full_rank-new_rank-6
            << ",\"coefficient_deficiency\":" << base.size()-full_rank << ",\"both_rank_paths_match\":true,\"elliptic_rows_checked\":" << matrix.size()
            << ",\"controls_rejected\":" << controls << ",\"controls_unavailable\":" << (witnesses.empty()?3:0) << ",\"singleton_witnesses\":[";
        for(size_t i=0;i<witnesses.size();++i) {if(i) out << ',';out << '[' << witnesses[i].first << ',' << witnesses[i].second << ']';}
        out << "],\"matrix\":[";
        for(size_t i=0;i<matrix.size();++i) {if(i) out << ',';out << '[';for(size_t j=0;j<matrix[i].size();++j) out << (j?",":"") << '"' << matrix[i][j] << '"';out << ']';}
        out << "],\"new_curve_points\":0,\"fresh_target_classes_checked\":false}\n";
        require(bool(out),"complete aggregate result");
    } catch(const std::exception& error) {std::cerr << "REJECT " << error.what() << '\n';return 1;}
}
