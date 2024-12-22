This is the spec for a 16-bit CPU that im working on called Ember.

The default byte order of the CPU is Little Endian, including in instructions.

# Instructions

Instructions are 32 bits. Last 16 are typically an address or immediate value.
First 4 are an opcode. The rest are decided by what that opcode is.

- `0b0000`: Operation
- `0b0001`: Load
- `0b0010`: Store
- `0b0100`: Jump
- `0b0110`: Stack Read
- `0b0111`: Stack Write
- `0b1111`: More specific instructions

The rest of the opcodes are reserved for future updates, except `0b1000` -
`0b1100` (exclusive), which can be used by the specific implementation/emulator.

Here are the more detailed definitions of each opcode:

## Operation

First 2 bytes: `0000 DD SS OOOOO M A/S`

`D` is the destination register where the result is put.

`S` is the "source mode", which decides exactly how its decided where the B
input comes from, and how the last 2 bytes of the instruction are used.

- `00`: Immediate. The last 2 bytes of the instruction are used as the B input.
- `01`: Register. The last 2 bits of the instruction decide which register
  should be the B input. This can be any register, including the zero register
  or destination register. The rest of the 2 bytes are ignored.
- `10`: Memory. The last 2 bytes of the instruction are treated as a memory
  address, and the value at that memory address is used as the B input.
- `11`: Memory (Big Endian): Same as `10` but uses Big Endian. In 8-bit mode
  this variant makes it take the value from 1 + the address.

`O` is what operation to perform. Either fed to the shift module or ALU
depending on `A/S`

`M` is whether the CPU is operating in 8- or 16-bit mode. `0` means 16-bit mode
and `1` means 8-bit mode.

`A/S` decides whether the operation is an ALU or Shift operation. `0` means ALU
and `1` means Shift.

The result of the operation always goes in the destination register, which is
also the A input of the operation.

## More specific intructions

First 2 bytes: `1111 OOOO OOOOOOOO`

`O` is a sort of sub-opcode. This opcode is intended for more specific stuff
that are not covered by the main set of instructions, but that does not mean the
have to be uncommon to use. `0x000` - `0x100` (exclusive) are reserved for the
base spec (including future updates), but the rest are free to be used by the
specific implementation/emulator.

- `0x000`: Halt.
- `0x001`: Initialize Stack. Sets the stack pointer to the value of the last 2
  bytes of the instruction.

# Registers

Registers are addressed by 2 bits. Register 0 always outputs 0 and does not
store anything put into it. Each of these are 16-bit registers. When an
operation is only using 8 bits, the LSBs are that byte, but when writing to it
with an 8-bit value the MSBs are overwritten with all `0`s.

The Program Counter and Stack Pointer are both full 16-bit addresses. The stack
grows downwards.

## Flags

The flags in the processor are the following:

- Carry
- Zero
- Sign (or MSB)

In that order, each one is numbered 1-3 or `0b01` - `0b11`. Flag 0 is considered
always active. Each flag has a separate variant for 8 or 16 bit operations.

# ALU

The base ALU is simply an adder, but then you add these control bits to it:

In order MSB to LSB:

- Invert A: bitwise not applied to input A before main operation
- Invert B: same but for input B
- Flood Carry: The carry out for each bit is set to 1, but not the initial carry
  in for the whole adder
- Carry In: Initial carry in is set to 1
- Or mode: The XOR operation part of the addition is changed to an OR, and the
  carry calculation for each bit is changed to A & B & CARRY instead of minimum
  2 out of A, B and CARRY, which is the default.
