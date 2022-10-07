#include <stdint.h>

char dummy[] = "dummy";
char marquez_arr[] = "Marquez";

#define PORTA = 0x7000000 //some huge address

void putchar(char c){
    uint8_t *out = (uint8_t*)PORTA;
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
