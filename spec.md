This is the spec for a 16-bit CPU that im working on called Ember.

The default byte order of the CPU is Little Endian, including in instructions.

# Instructions

Instructions are 32 bits. Last 16 are typically an address or immediate value.
First 4 are an opcode. The rest are decided by what that opcode is.

- `0b0000`: Operation
- `0b0001`: Comparision
- `0b0010`: Load
- `0b0011`: Store
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

## Comparison

Works the same as [Operation](#operation), but while what is referred to as the
Destination register in that instruction is just the A input and not where the
output is placed in. The output is treated as always being put in register zero,
which means it is voided.

## Load

First 2 bytes: `0010 DD OO AA E M SSS x`

`D` is the destination, where the data is put when its been retrieved.

`O` is the offset register. The value is shifted left by `S` and then added to
the base address.

`A` is the addressing mode, and therefore decides how the last 2 bytes are used.

- `00`: Immediate. The last 2 bytes are the direct value to be loaded into the
  register. Offset register is ignored.
- `01`: Direct address. The last 2 bytes is the base address.
- `10`: Register. The last 2 bits indicates a register where the data to be
  loaded from is stored. Rest of the last 2 bytes and offset register are
  ignored.
- `11`: Address register. The last 2 bits indicates a register where the base
  address is stored. Rest of the last 2 bytes are ignored.

`E` decides endianness. `0` means little endian, `1` means big endian. Ignored
for an 8-bit load or when loading from register or immediate.

`M` indicates whether the processor should load 8 or 16 bits. `1` means 8-bit
mode, `0` means 16-bit mode.

## Store

First 2 bytes: `0011 DD OO A x E M SSS x`

`D` is the source register, where the data is coming from.

`O` is the offset register. Shifted to the left by S and added to the base
address to get the full address.

`A` is the addressing mode. It decides how the last 2 bytes are used.

- `0`: Direct address. The last 2 bytes are the base address.
- `1`: Address register. The last 2 bits indicate what register holds the base
  address. The rest of the last 2 bytes are ignored.

`E` decides Endianness. `1` means Big Endian, `0` means Little Endian.

`M` decides whether its an 8- or 16-bit store. `0` means 16-bit, `1` means
8-bit.

## Jump

## Stack Read

Instruction bytes: `0110 DD OR SOR SSS PP OOOOOOOO OOOOOOOO`

`D` is the destination register, where the data read is put.

`OR` is the offset register. It is shifted left by `SOR` and added to the base
offset to create the complete offset.

`P` is the amount to change the stack pointer by. It is shifted left by `S` and
the stack pointer is increased by that amount after the data is retreived.

`O` is the base offset. When added to the shifted `OR` (the complete offset), it
is added to the `SP` to create the address to read data from.

## Stack Write

Instruction bytes: `0111 DD OR SOR SSS PP OOOOOOOO OOOOOOOO`

`D` is the data/source register, where the data to write is read from.

`OR` is the offset register. It is shifted left by `SOR` and added to the base
offset to create the complete offset.

`P` is the amount to change the stack pointer by. It is shifted left by `S` and
the stack pointer is decreased by that amount after the data is written.

`O` is the base offset. When added to the shifted `OR` (the complete offset), it
is added to the `SP` to create the address to write data to.

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
- `0x002`: Move stack pointer. Adds the last 2 bytes of the instruction to the
  stack pointer.

## Notes for the instruction definitions

Any `x` is preferrably ignored by emulator/implementation, and preferrably set
to 0 in program.

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

Flags are set by the Comparison or Operation instruction, or when otherwise
noted. Any instruction that does not say it sets flags does not do so
automatically just because it happens to use the ALU.

# ALU

The base ALU is simply an adder, but then you add these control bits to it:

In order MSB to LSB:

- Invert A: bitwise not applied to input A before main operation
- Invert B: same but for input B
- Flood Carry: The carry out for each bit (including the carry out of the whole
  ALU, meaning that if this is set the carry flag will always be set if the
  instruction updates flags) is set to 1, but not the initial carry in for the
  whole adder
- Carry In: Initial carry in is set to 1
- Or mode: The XOR operation part of the addition is changed to an OR, and the
  carry calculation for each bit is changed to A & B & CARRY instead of minimum
  2 out of A, B and CARRY, which is the default.

# Shifting

Just like the ALU, the shift system has 5 control bits and an A and B input. The
A input is the one that is shifted, and the B input is the amount to shift by.
Shift direction is to the left unless otherwise specified.

When a shift operation is performed, the last bit shifted out ends up in the
carry flag, and the others are set like normal based on the end result of the
shift.

The first bit in the control bits decides if it is a rotate or shift operation.
`1` means rotate, `0` means shift. If it is a rotation, when a bit is shifted
out on a side it immediately goes to the other, all throughout the operation.
The rest of the control bits depend on whether it is a rotation or shift.

Shifting:

- Direction. `0` means a right shift and `1` means left shift.
- Type. `0` means regular shift, `1` means arithmetic shift.
- The rest are ignored

Rotating:

- Whether to rotate through carry. Rotates through carry instead of like usual
  if `1`.
- Wrap Mode. `0` means normal rotate, `1` means complement on wrap (XOR with 1
  during wrap-around).
- The rest are ignored

# Errors

How errors are handled depends on the specific implementation/emulator, but
should always either result in a noop or halt functionally for the processor,
but can involve extra things (such as an emulator logging what went wrong).
