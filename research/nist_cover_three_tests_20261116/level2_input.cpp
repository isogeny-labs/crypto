#include "weil3_arithmetic.hpp"
#include "geometry_field.hpp"
int main(int argc,char** argv){try{
    require(argc==2,"usage: level2_input study-directory");std::string root=argv[1];
    initialize_cover(read_json(root+"/inputs/p192-d7-prym.json"));Level2Input input;
    int checks=0;std::vector<int> permutation;
    for(auto& r:input.branch){
        auto a=r.power(prime);int index=-1;for(int i=0;i<7;++i)if(a==input.branch[i])index=i;
        require(index>=0,"branch Frobenius permutation");permutation.push_back(index);
    }
    for(int i=0;i<6;++i){
        int source_mask=input.masks[1<<i],target_mask=0;
        for(int j=0;j<7;++j)if((source_mask>>j)&1)target_mask^=1<<permutation[j];
        target_mask=canonical_subset(target_mask);
        auto it=std::find(input.masks.begin(),input.masks.end(),target_mask);require(it!=input.masks.end(),"Frobenius subset lookup");
        require(input.classes[1<<i].frob()==input.classes[size_t(it-input.masks.begin())],"actual2-torsion Frobenius");++checks;
    }
    rejects([&]{require(GT24::f.eval(input.branch[1]+T24(1))==T24(0),"corrupt branch root");},"corrupt branch root");
    rejects([&]{require(parity2(input.symplectic[0],input.symplectic[3])==0,"corrupt pairing");},"corrupt pairing");
    std::cout<<"{\"scope\":\"P192-d7-source-level2-input\",\"tower_degree_over_archived_field\":3,\"working_degree\":24,\"finite_branch_roots\":7,\"two_torsion_classes\":64,\"basis_Frobenius_checks\":"<<checks<<",\"characteristic_parity_checks\":64,\"rejected_controls\":2,\"delta_code\":"<<input.delta<<",\"quartic_constructed\":false,\"standing\":\"producer-unverified\",\"branch_roots\":[";
    for(int i=0;i<7;++i){if(i)std::cout<<',';emit_t24(input.branch[i]);}std::cout<<"],\"symplectic_subset_basis\":[";
    for(int i=0;i<6;++i)std::cout<<(i?",":"")<<input.symplectic[i];
    std::cout<<"],\"Frobenius_branch_permutation\":[";
    for(int i=0;i<7;++i)std::cout<<(i?",":"")<<permutation[i];
    std::cout<<"]}\n";
}catch(const std::exception&e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
