This is the spec for a 16-bit CPU/ISA that im working on called Ember.

The default byte order of the CPU is Little Endian. Endianness mostly does apply to 8 bit values if in memory, including in operands.

Ember is a Von Neumann architecture, meaning the program and data memory share
the same address space.

# Instructions

Instructions are 4 bytes. Last 2 are the operand, first 2 are the instruction bytes. The instruction bytes are written out as Big Endian, while the operand should by default be treated as Little Endian like anything else.
First few of the instruction bytes are the opcode, the exact number of opcode bits differ between instructions depending on how many control bits they need. Obviously, an opcode can not start with another instructions opcode, or there will be a conflict.

- `0001---- --------`: ALU Operation
- `00001--- --------`: Shift
- `00000000 01------`: Multiply
- `0010---- --------`: Load
- `00110--- --------`: Store
- `00111--- --------`: Jump
- `00000000 00000001`: Return
- `00000000 00001---`: Pull
- `00000000 000001--`: Push
- `000001-- --------`: Stack Read
- `010000-- --------`: Stack Write
- `00000001 --------`: Load Stack Pointer
- `00000000 10------`: Move Stack Pointer
- `00000000 00000010`: Initialize Stack
- `010001-- --------`: Flag Operation
- `00000000 00010---`: Load/Store Flags
- `00000010 0-------`: Store Single Flag
- `00000000 0010----`: Extend Value
- `00000000 11------`: Scale Value
- `01111111 010101--`: Debug
- `01111111 11111111`: Query Extensions
- `00000000 00000000`: NOOP
- `11111111 11111111`: Halt

All opcodes starting with a `0` as well as `11111111 11111111` are reserved for the base spec and official extensions.

Here are the more detailed definitions of each opcode:

## ALU Operation

Instruction bytes: `0001 DD SS CCCCCC M V`

`D` is the destination register where the result is put.

`S` is the "source mode", which decides exactly how its decided where the B
input comes from, and how the operand is used.

- `00`: Immediate. The operand is used as the B input.
- `01`: Register. The  least significant bits of the operand decides which register
  should be the B input. This can be any register, including the zero register
  or destination register. The rest of the 2 bytes are ignored.
- `10`: Memory. The operand is treated as a memory
  address, and the value at that memory address is used as the B input.
- `11`: Stack. Reads the B input from the address you get by adding the operand to the stack pointer.

`C` is the ALU control bits

`M` is whether the ALU is using 8-bit mode. `1` means it does, `0` means it does not. In 8-bit mode, all flags are set as if the operation is using only the lower bytes, and the B input is only 8 bits, but the A input is still 16, and the output.

`V` decides whether to void the result. If set to `1`, the destination register is only the A input, and the result is treated as getting put in register 0. Used for comparisons.

The result of the operation always goes in the destination register, which is
also the A input of the operation.

## Shift

Instruction bytes: `00001 D RR SS II M V OO`

`R` is the register holding the value to be shifted, also where the result is put.

`S` is the source mode, which decides from where the amount to shift by comes.

- `00`: Immediate. The operand is the amount to shift by.
- `01`: Register. The operand is a register which holds the amount to shift by.
- `10`: Memory. The operand is a memory address pointing to the amount to shift by.
- `11`: Stack. Reads the amount to shift by from the address you get by adding the operand to the stack pointer.

Bits beyond the least significant 4 bits in the amount to shift are ignored.

`D` is the direction of the shift. `0` means left and `1` means right.

`I` is the insert mode, which decides what is inserted where something is shifted out of:

- `00`: `0` always inserted
- `01`: `1` always inserted
- `10`: Rotation. Whatever was shifted out last is always inserted.
- `11`: The bit at the end that bits are being shifted from is replicated (MSB for right, LSB for left). In the case of a right shift, this is an arithmetic shift.

`M` is whether the shift is in 8-bit mode. `1` means it does, `0` means it does not. In 8-bit mode, flags are set as if the lower byte of `R` is the only one, insertion to the right happens also at the lower byte, and insertions are determined as if the shift was happening in only the lower byte, but the upper byte is still shifted.

`V` decides whether to void the result. If set to `1`, `R` is not modified, and the result is treated as getting put in register 0.

`O` is the register the bits that are shifted out are put in. The bits that are shifted out are placed in the least significant bits of this register in the same order they were in the original register. The rest of that register is set to all `0`s.

Flags are set like this by a shift:
- Zero and Sign are set like usual based on the result
- Carry is the last value to be shifted out
- Overflow is set if a 1 was shifted out during the operation

## Multiply

Instruction Bytes: `00000000 01 RR SS HH`

Multiplies an unsigned 16-bit number with an 8-bit one.

`R` is the register holding the 16-bit number, and where the result is put.

`S` is the source mode. It decides how the operand is used to determine the B input (8-bit input)

- `00`: Immediate. The operand holds the B input.
- `01`: Register. The operand is a register which holds the B input in its low byte.
- `10`: Memory. The operand is a memory address pointing to the B input.
- `11`: Stack. Reads the B input from the address you get by adding the operand to the stack pointer.

`H` is the register in which the high output byte is put. Just like loading an 8 bit value into it, only the low byte of it is affected.

Flags are set like this by a multiplication:
- Zero is set like usual, but not including the high byte.
- Sign is set if the most significant bit of the 16-bit part of the result is a `1`.
- Overflow is set if the high output byte has ANY `1`s.
- Carry is set if the least significant bit of the high output byte is a `1`.

## Load

Instruction bytes: `0010 DD OO AA E M SS FF`

`D` is the destination register, where the data is put when its been retrieved.

`O` is the offset register. The value is shifted left by `S`, multiplied by `F`+1, and then added to
the base address. Ignored in Immediate or Register modes.

`A` is the addressing mode. It decides how the operand is used.

- `00`: Immediate. The operand the direct value to be loaded into the
  register. Offset register is ignored.
- `01`: Direct address. The operand is the base address.
- `10`: Register. The operand is a register where the data to be
  loaded from is stored. Offset is ignored.
- `11`: Register Indirect. The operand is a register where the base
  address is stored.

`E` decides endianness. `0` means little endian, `1` means big endian. When loading from a register, this swaps the bytes of the register.

`M` indicates whether the processor should load 8 or 16 bits. `1` means 8-bit
mode, `0` means 16-bit mode. In 8-bit mode, the upper byte of the register is left untouched, and the lower byte of the source is treated as the only byte.

## Store

Instruction bytes: `00110 DD A OO SS FF E M`

`D` is the source register, where the data is coming from.

`O` is the offset register. Shifted to the left by S and added to the base
address to get the full address.

`A` is the addressing mode. It decides how the operand is used.

- `0`: Direct address. The operand is the base address.
- `1`: Address register. The operand is what register holds the base
  address.

`E` decides Endianness. `1` means Big Endian, `0` means Little Endian.

`M` decides whether its an 8- or 16-bit store. `0` means 16-bit, `1` means
8-bit.

## Jump

Instruction bytes: `00111 OO SS FF CCCC R`

`O` is the offset register. Its value is shifted left by `S` and multiplied by `F`+1 to create the
offset.

`C` is the condition to jump. It indicates which flag needs to be set for the
jump to go through.

`R` indicates whether the jump is a subroutine call. If it is, the program
counter (before the jump) plus 4 (size of one instruction) is pushed to the stack.

The operand is the base address, which is added to the offset to decide what address to
jump to.

## Return

Instruction Bytes: `00000000 00000001`

Pulls one value off the stack and jumps to that address.

## Pull

Instruction bytes: `00000000 00001 DD M`

`D` is the destination register, where the data read is put.

`M` is whether this is a 16 or 8 bit pull. `0` means 16, `1` means 8.

## Push

Instruction bytes: `00000000 000001 S M`

`S` is the source mode.
- `0`: Immediate. The operand is the value to be pushed.
- `1`: Register. The operand is the register to be pushed.

`M` is whether this is a 16 or 8 bit push. `0` means 16, `1` means 8.

## Stack Read

Instruction Bytes: `000001 DD OO SS FF E M`

Reads a value from the stack with an offset, and without moving the stack pointer.

`D` is the register the data goes in.

`E` is the endianness. `0` is little endian, `1` is big endian.

`M` is whether this is a 16 or 8 bit read. `1` means 16, `8` means 8.

The offset is calculated as the operand plus the data at register `O` shifted left by `S` and multiplied by `F`+1.

## Stack Write

Instruction Bytes: `010000 DD OO SS FF E M`

Writes a value to the stack with an offset, and without moving the stack pointer.

`D` is the register the data comes from.

`E` is the endianness. `0` is little endian, `1` is big endian.

`M` is whether this is a 16 or 8 bit read. `1` means 16, `8` means 8.

The offset is calculated as the operand plus the data at register `O` shifted left by `S` and multiplied by `F`+1.

## Load Stack Pointer

Instruction Bytes: `00000001 DD OO SS FF`

Loads the address the stack pointer is pointing to plus an offset into a register.

`D` is the register the address is placed in.

The offset is calculated as the operand plus the data at register `O` shifted left by `S` and multiplied by `F`+1.

## Move Stack Pointer

Instruction Bytes: `00000000 10 MM SS N F`

Moves the stack pointer by adding some value to it.

The value at register `M` is shifted left by `S`, multiplied by `F`+1, and is negated (2s complement) if `N` is `1`. This is then added to the operand to create the amount to move by.

## Initialize Stack

Instruction bytes: `00000000 00000010`

Sets the Stack Pointer to the operand.

## Flag Operation

Instruction Bytes: `010001 OO AAAA BBBB`

Sets the User flag based on a logical operation between 2 flags, one of which may be the user flag itself. The 2 inputs are `A` and `B`.

`O` is the logical operation.
- `00`: Or
- `01`: Nor
- `10`: Xor
- `11`: Unused

Other logical operations can be formed by using the inverted versions of the operand flags. The user flag can also be directly set to a value by only using flag `0`, which is always True.

## Load/Store Flags

Instruction Bytes: `00000000 00010 S RR`

Loads (to current flags) or stores (to register) between the flags and a register, ordered with a more significant bit holding a higher flag number, so flag 0 (always active) is the least significant bit and it goes up through. Inverted versions of flags are not used. Only the less significant byte of the register is read or written to. What is written to unused flags or flag 0 does not matter.

`L` is whether this is a load. `0` means store, `1` means load.

`R` is the register.

## Store Single Flag

Instruction Bytes: `00000010 0 M RR FFFF`

Sets the register `R` to the value of flag `F`. That value is put in the least significant bit, the rest is cleared.

If `M` is set, only the low byte is cleared and the rest is left untouched.

## Extend Value

Instruction Bytes: `00000000 0010 MM RR`

Extends the 8 bit value in register `R` to be 16 bit, overwriting the registers more significant bit.

`M` is the mode, which decides what bits are used for the extension.

- `00`: `0`
- `01`: `1`
- `10`: Sign bit of the 8 bit value
- `11`: User flag

## Scale Value

Instruction Bytes: `00000000 11 RR SS FF`

`R` is a register holding the value to be scaled, also where the result is put.

The value is shifted left by `S` and multiplied by `F`+1 to create the result.

## Debug

Instruction Bytes: `01111111 010101 RR`

Simple debug output. Optional to implement. Outputs one register value (`R`) as well as one character from the operand, which should at minimum be ASCII but may be some form of extended ASCII (e.g. latin1) or in some other way compatible with it directly if an ASCII character is placed in the least significant byte (e.g. UTF8). If the register is the zero register, it is optional to output. If the character is null/0, it is also optional to output.

## Query Extensions

Instruction Bytes: `01111111 11111111`

Checks whether the processor currently has the extension specified by the operand. Extensions 0-255 are reserved for official extensions. Whether its available is put into the User flag.

## NOOP

Instruction bytes: `00000000 00000000`

Does absolutely nothing.

## Halt

Instruction bytes: `11111111 11111111`

Halts execution, can not be undone. Operand may be used for debugging purposes by the specific implementation.

# Registers

Registers are addressed by 2 bits. If addressed by a full 16 bit number, only the least significant bits are used, the rest are ignored. Register 0 always outputs 0 and does not
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
- Overflow (generalized as the XOR between the carry in and out of the MSB IN THE CASE OF ALU OPERATIONS, may mean something entirely different from its usual use in other cases such as Shift and Multiply)

In that order, each one is numbered 1-4 or `0b001` - `0b100`. Flag 0 is considered
always active. The rest of the flags are not used by the base ISA, and can do whatever depending on extensions, but unofficial extensions should prefer to use the User flag or a separate system if necessary.

Flags are addressed by 4 bits. The first bit decides whether to use the inverted version of the flag, The other 3 are the flag index.

Flags are set by the ALU Operation, Shift, or Multiply instructions, or when otherwise
noted. Any instruction that does not say it sets flags does not do so
automatically just because it happens to use the ALU or shift.

There is also the special User flag. It is flag 0b111, and is only set by any instruction that specifically says it can set it.

# ALU

The base ALU is simply an adder, but then you add these control bits to it:

In order MSB to LSB:

- Invert A: bitwise not applied to input A before main operation
- Invert B: same but for input B
- Invert Out: bitwise not applied to output
- Flood Carry: The carry out for each bit (including the carry out of the whole
  ALU, meaning that if this is set the carry flag will always be set if the
  instruction updates flags) is set to 1, but not the initial carry in for the
  whole adder
- Carry In: Initial carry in is set to 1
- Logic Mode: The XOR between the A and B inputs for each bit is swapped out for an OR. Carry input of each bit still inverts the output in the same way.

## Examples

- Addition: All control bits off.
- Subtraction: Invert B (or A if you want to do B-A), Carry In
- Bitwise NOR: Flood Carry, Carry In, Logic Mode

# Errors

Error handling isnt part of the base spec. Unless changed by an extension or just implementation tho, it should cause a halt, but a program shouldnt ever depend on this.
