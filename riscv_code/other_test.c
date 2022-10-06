
#include <stdint.h>

int a = 10;
int b = 20;
int c = 0; 
uint32_t d;

const int myconst = 0x2525;

int main()
{
    c = a + b;
    d = (uint32_t)&b;
    c = myconst + 1;
}
