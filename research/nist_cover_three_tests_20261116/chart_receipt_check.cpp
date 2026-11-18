// Check the nonzero half of symmetry from the recorded chart values.
#include "point_arithmetic.hpp"
#include <map>
int main(int argc,char**argv){try{
    require(argc==4||argc==5,"usage: chart_receipt_check prym.json Weil3.json chart.json [--canary]");
    init_field(read_json(argv[1]));auto f=read_json(argv[2]),chart=read_json(argv[3]);
    auto& rows=f.at("functions").arr;auto& values=chart.at("collision_values").arr;
    require(rows.size()==27&&values.size()==52,"chart receipt sizes");
    std::map<std::string,int> index;
    for(int i=1;i<27;++i){auto u=polynomial(rows[i].at("P_U")),v=polynomial(rows[i].at("P_V"));index.emplace(u.key()+"|"+v.key(),i);}
    for(int i=1;i<27;++i){
        auto u=polynomial(rows[i].at("P_U")),v=polynomial(rows[i].at("P_V"));int j=index.at(u.key()+"|"+((-v)%u).key());
        F x=field(values[2*(i-1)]),y=field(values[2*(j-1)]);
        if(argc==5&&i==1)x=x+F(1);
        require(x!=F(0)&&y!=F(0),"nonzero matching-index chart value");
        require(field(rows[i].at("alpha"))*x==field(rows[j].at("alpha"))*y,"nonzero collision-chart symmetry");
    }
    std::cout<<"PASS26 nonzero normalized collision-chart symmetry equations from public receipts\n";
}catch(const std::exception&e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
