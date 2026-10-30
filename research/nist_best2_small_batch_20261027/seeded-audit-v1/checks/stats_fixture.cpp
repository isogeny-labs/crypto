#include <iostream>
int main(){for(int seed=1;seed<=8;seed++)for(int arm=0;arm<2;arm++)std::cout<<"{\"schema\":\"nist.seeded_dp/v1\",\"curve\":\"P-256\",\"W\":16,\"iters\":100,\"dp_bits\":8,\"seed_id\":"<<seed<<",\"best2\":"<<arm<<",\"walk_seed\":"<<seed<<",\"table_seed\":7,\"emitted_dp\":"<<10+arm<<",\"advanced\":1500,\"attempts\":1600,\"cycle_detect\":30,\"seek\":70,\"inf\":0}\n";}
