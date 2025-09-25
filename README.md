# Simple  Virtual Machine

This project implements a simple stack-based **Virtual Machine (VM)** in Rust.
It supports a custom assembly-like instruction set and executes programs written in `.msm` format.

---

## Features

* **Stack-based execution** with a fixed stack size (`1024` elements).
* **Instruction set** for arithmetic, stack manipulation, and branching.
* **Custom assembly-like language** for writing programs.
* **Label-based jumps** (`jump`, `zjump`, `nzjump`).
* **Safe execution** with stack overflow/underflow checks.
* **Program counter (`ip`)** based instruction stepping.

---

## Project Structure

```
src/
 ├── codegen.rs       # Code generation helpers
 ├── instructions.rs  # Definition of the instruction set
 ├── lexer.rs         # Lexer for parsing .msm files
 ├── parser.rs        # Parser for assembly programs
 ├── main.rs          # Entry point
 ├── virtual_m.rs     # VM implementation
```

---

## Instruction Set

| Instruction | Description                                    |
| ----------- | ---------------------------------------------- |
| `push n`    | Push integer `n` onto the stack                |
| `pop`       | Pop the top element                            |
| `dup`       | Duplicate top element                          |
| `indup n`   | Duplicate element at index `n`                 |
| `swap`      | Swap top 2 elements                            |
| `iswap n`   | Swap top with element at index `n`             |
| `add`       | Pop two, push `a + b`                          |
| `sub`       | Pop two, push `b - a`                          |
| `mul`       | Pop two, push `a * b`                          |
| `div`       | Pop two, push `b / a` (panics on div by 0)     |
| `mod`       | Pop two, push `b % a`                          |
| `cmpe`      | Compare equality (push `1` if equal, else `0`) |
| `cmpne`     | Compare not equal                              |
| `cmpg`      | Greater than                                   |
| `cmpl`      | Less than                                      |
| `cmpge`     | Greater or equal                               |
| `cmple`     | Less or equal                                  |
| `print`     | Print top of stack                             |
| `jump L`    | Unconditional jump to label `L`                |
| `zjump L`   | Jump if top == 0                               |
| `nzjump L`  | Jump if top != 0                               |
| `halt`      | Stop execution                                 |
| `nop`       | No operation                                   |

---

## Example Program

`test5.msm`

```asm
push 20
push 1
push 1
push 0

loop:
iswap 0

dup
push 0
cmpe
nzjump end

pop
iswap 0
indup 2
iswap 1

pop
dup
iswap 2
pop

indup 1
indup 2
add
swap
print

iswap 0
push 1
sub
iswap 0
jump loop

end:
```

This program demonstrates **loops, conditional jumps, and arithmetic**.

---

## Usage

### 1. Build

```bash
cargo build
```

### 2. Run a program

```bash
cargo run test5.msm
```

### 3. Output

The VM will execute the instructions and print intermediate results.

---

## Future Work

* Better error messages for invalid programs.
* Debugger/step execution.
* More instructions (memory load/store, I/O, etc).
* Optimizations in instruction decoding.

---

