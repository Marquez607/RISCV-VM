#include <stdint.h>

char dummy[] = "dummy";
char marquez_arr[] = "Marquez";

#define PORTA_IN 0x70000000
#define PORTB_OUT 0x70000002

void putchar(char c){
    uint8_t *out = (uint8_t*)PORTB_OUT;
    *out = c;
}

int main()
{
  char *ptr = marquez_arr;
  while(*ptr){
    putchar(*ptr);
    ptr++; 
  }
  putchar('\n');
  
  //dummy section
  ptr = dummy;
  while(*ptr){
    putchar(*ptr);
    ptr++; 
  }
  putchar('\n');
  
  while(1);
}
