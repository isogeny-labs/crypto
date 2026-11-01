#include <pari/pari.h>
int main(int argc,char **argv){
 if(argc!=2)return 2;
 pari_init(256000000,2000000);
 gp_read_file(argv[1]);pari_close();return 0;
}
