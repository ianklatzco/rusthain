# the goal is to run main4 with some x86 assembly code
# first, run the rust binary without any input:
# cargo run --bin main4

# copy this assembly code into a file named "asm.asm" !
format binary
use64
    xor    eax,eax
	add		eax,69
	ret

# then:
# ubuntu: sudo apt install fasm -y
# mac:    uhhhhhhhhh apple silicon cant do W&X pages

# then: 
# fasm asm.asm

# then:
# cat asm.bin | cargo run --bin main4

# and you'll have handwritten your first piece of assembly code,
# then executed it on your laptop ^^


