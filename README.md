# RISCV_VM
## Reference Material 
### Making a baremetal RISC-V executable 
https://wiki.osdev.org/RISC-V_Bare_Bones

### RISCV ISA 
ISA: https://msyksphinz-self.github.io/riscv-isadoc/html/rvi.html \
Other Specs/Docs: https://riscv.org/technical/specifications/

## Development Environment
### Rust 
The system we write will be in rust 
Follow rust book for installation process: https://doc.rust-lang.org/book/

### OS -> Linux
makefiles will be running linux commands
### RiscV compiler 
Install x86_64-gcc-10.1.0-nolibc-riscv32-linux.tar.gz from this link:
https://mirrors.edge.kernel.org/pub/tools/crosstool/files/bin/x86_64/10.1.0/

store the .gz file anywhere you want \

decompressing it \
***NOTE***: the ```-C``` flag will place the decompressed folder at the designated path. You can omit this flag and leave the decompressed files wherever they were downloaded
```
cd your/download/folder
sudo tar -xzf TOOLCHAIN.tar.gz -C /where/you/want/it
```

Temporarily adding the compiler to your shell environment \
***NOTE***: you will need to run this command every time you open a terminal. You can also just add this to your ```.bashrc``` file which will auto add the compiler to your path. 
```
export PATH=$PATH:/where/you/want/it
```

### SiFive elf2hex
You'll also need this downloaded and compiled on your system somewhere, it's used for generating the binary or hex text files used in the tool.

Link: https://github.com/sifive/elf2hex


This should also be added to your $PATH variable after downloading 
```
export PATH=$PATH:/where/you/put/elf2hex/
```

## Folder/Files

### riscv_code 
This contains the makefile and C programs we'll be testing on the "VM"

### app
Rust app
