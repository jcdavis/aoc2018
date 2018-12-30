#include <stdio.h>

#ifdef PRINT
#define DEBUG(line) printf("%d: %lld %lld %lld (3) %lld %lld\n", line, r0, r1, r2, r4, r5);
#else
#define DEBUG(line)
#endif

int main(int argc, char** argv) {
  long long r0=1,r1=0,r2=0,r4=0,r5=0;
L0: DEBUG(0) goto L17; //addi 3 16 3
L1: DEBUG(1) r5=1; //seti 1 9 5
L2: DEBUG(2) r4=1; //seti 1 1 4
L3: DEBUG(3) r2=r4*r5; //mulr 5 4 2
L4: DEBUG(4) r2= r1==r2?1:0; //eqrr 2 1 2
L5: DEBUG(5) if (r2 == 1) { goto L7;} else { goto L6;} //addr 2 3 3
L6: DEBUG(6) goto L8; //adddi 3 1 3
L7: DEBUG(7) r0 = r0+r5; //addr 5 0 0
L8: DEBUG(8) r4 = r4+1; //addi 4 1 4
L9: DEBUG(7) r2 = r4 > r1 ? 1 : 0; // gtrr 4 1 2
L10: DEBUG(10) if (r2 == 1) { goto L12;} else { goto L11;} //addr 3 2 3
L11: DEBUG(11) goto L3; //seti 2 3 3
L12: DEBUG(12) r5 = r5+1; //addi 5 1 5
L13: DEBUG(13) r2 = r5>r1?1:0; //gtrr 5 1 2
L14: DEBUG(14) if (r2 == 1) {goto L16;} else {goto L15;} //addr 2 3 3
L15: DEBUG(15) goto L2; //seti 1 4 3
L16: DEBUG(16) goto end; //mulr 3 3 3
L17: DEBUG(17) r1 = r1 + 2; //addi 1 2 1
L18: DEBUG(18) r1= r1*r1; //mulr 1 1 1
L19: DEBUG(19) r1 = r1*19; // mulr 3 1 1
L20: DEBUG(20) r1 = r1*11; // muli 1 11 1
L21: DEBUG(21) r2 = r2 + 2; // addi 2 2 2
L22: DEBUG(22) r2 = r2*22; //mulr 2 3 2
L23: DEBUG(23) r2 = r2+20; //addi 2 20 2
L24: DEBUG(24) r1 = r1+r2; //addr 1 2 1
L25: DEBUG(25) printf("%lld\n", r0); if (r0 == 1) {goto L27;} else {goto L26;} //addr 3 0 3
L26: DEBUG(26) goto L1; //seti 0 4 3
L27: DEBUG(27) r2 = 27; //setr 3 9 2
L28: DEBUG(28) r2 = r2*28; //mulr 2 3 2
L29: DEBUG(29) r2 = r2+29; //addr 3 2 2
L30: DEBUG(30) r2 = r2*30; //mulr 3 2 2
L31: DEBUG(31) r2 = r2*14; //muli 2 14 2
L32: DEBUG(32) r2 = r2*32; //mulr 2 3 2
L33: DEBUG(33) r1 = r1+r2; //addr 1 2 1
L34: DEBUG(34) r0 = 0; //seti 0 6 0
L35: DEBUG(35) goto L1; //seti 0 0 3
 end: printf("%lld\n", r0);
}
