# VML - Virtual Machine Language
### No longer being maintained, this language is a joke language and is really poorly written. Dont actually use it (not like anyone would anyway)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/axolotlc/vml)
![GitHub all releases](https://img.shields.io/github/downloads/axolotlc/vml/total)
![GitHub repo size](https://img.shields.io/github/repo-size/axolotlc/vml)
![GitHub](https://img.shields.io/github/license/axolotlc/vml)

Not an esoteric programming language, just a bizzare one.

<p align="center">
    <img src="https://github.com/AxolotlC/VML/blob/main/res/image_0.png" align="center">
</p>

---

## Contents
- [Usage](#usage)
- [Compilation](#compilation)
- [Basic Syntax](#basic-syntax)
- [Arithmetic](#arithmetic)
- [Variables](#variables)
- [Methods](#methods)
- [Buffers](#buffers)
- [Doubles](#doubles)
- [Boolean Operators](#boolean-operators)
- [Includes](#includes)
- [Miscellaneous](#miscellaneous)
- [Standard Library](#standard-library)

## Usage

Compiling a VML program is simple! Simply run `vml -c <filename>.vml`. This will create a file called `out.bin` which can then be run with the `-r` flag on `vml`. Furthermore (as stated in the Miscellaneous section), you can compile assembly to run on the virtual machine with `vml -a <file>.s`.

## Compilation

Since this is a simple cargo project (make sure you have `rustc` and `cargo` installed), follow these steps:

1) `cd` into the correct directory (the root of your VML folder)
2) Run `cargo build --release` (This builds the program)
3) Add `alias vml=/path/to/target/release/vml` to your .bashrc or .zshrc
4) Be sure to also add the standard library (`std/*.vml`) to your $PATH
4) Test the installation by running `vml` in a new terminal. If you get an error, well done! You can now begin programming!

## Basic syntax

While creating this language, I wanted to focus on simplistic syntax and it really shows! I've also put some effort into them being easy to use, as VML shares
many features with commonly used languages such as curly braced scopes. You may also notice a lack of a `for` loop, and that is because we simply don't need one! We can create a loop
which ranges from 0 to 20 by doing the following:

```
0 while dup 21 < {
    1 +
}
```

With all that out of the way, here are all of the keywords as well as their operations:


| Keyword | Stack Operands | Operation |
| ------- | -------------- | --------- |
| `if` | 1 |  Pops a value off of the stack, if the value is one, it executes the code within the curly braces otherwise, it ignores the code within the braces.
| `while` | 1 | While the value on top of the stack is one, then the code within the curly braces is executed. Otherwise, the code is skipped and the program continues.
| `method` | 0 | Declares the code within the curly braces as a function with the name specified after the `method` keyword. Eg. `method foo {...}` Methods can be called with the `$` character followed by their name. Eg. `$foo`
| `return` | 0 | Returns from the current method. If used in the `main` method, the program exits.
| `(float)` | 1 | Casts the top value on the stack as a **64-bit float**
| `(int)` | 1 | Casts the top value on the stack as a **64-bit integer**
| `pow` | 2 | Pops the top two **64-bit floating point numbers** from the stack and performs an exponentiation. The result is then pushed back onto the stack.
| `root` | 2 | Pops the top two **64-bit floating point numbers** from the stack and performs an nth-root on them. The result is then pushed back onto the stack.
| `include` | 1 | Includes a file. If the file doesn't exist, then the compiler will crash.
| `dup` | 1 | Duplicates the value on top of the stack. Eg. `2 dup` ---> `2 2`
| `swap` | 2 | Swaps the top two values on the stack around, such that `a b swap` ---> `b a`
| `rot` | 3 | Rotates the top three elements on the stack, such that ` a b c rot` ---> `c a b`
| `drop` | 1 | Drops the top value off of the stack (effectively removes one element and does nothing with it. |
| `memory` | 0 | Begins a buffer variable, which is automatically allocated an amount of memory that is requested by the programmer. For example: `memory 64 const Buffer` allocated 64 bytes to the variable `Buffer`
| `const` | 0 | Currently the only type of variable. Begins the declaration of a constant variable.
| `let` | 0 | Declares a non-memory variable which can be referenced throughout the program. Eg. `let "Hi" const hi_var`. Variables can be referenced anywhere throughout the program with just their name identifier (Eg. `hi_var print`).
| `copy` | 2 | Fills the first operand (as an entry into memory) with a string literal supplied by the second element down in the stack. Eg: `"Hello" buffer copy` ---> `buffer = "Hello"`
| `syscall` | x | Performs an internal system-call. Can take any amount of arguments. Do not use if you are inexperienced or do not understand the system-call numbering in this language (as it runs on a virtual machine, linux syscalls won't work).

## Includes

It would be a nightmare working with a file that has over 10,000 lines and absolutely no categorisation, and so I present includes! Fresh in version 1.0.3rev1! With includes, you can have multiple files for projects, making your code more concise and organized.

Here is a quick tutorial on how to use includes:

Say we had a file called `helpful_stuff.vml` which contained the method `read_user_input`. If we wanted to access this from a different file, we can simply add `include "helpful_stuff.vml"` to the top of the file. This will automatically stitch all of the dependencies and methods from `helpful_stuff.vml` into your file!

#### With great power comes great responsibility (READ THIS):
As of 1.0.3rev1, includes are experimental and may be buggy, so use them at your own risk. You may also want to avoid something known as "circular includes" where your file includes itself or another file includes it, resulting in a loop. This could duplicate some of your code, resulting in extremely difficult-to-find bugs.

## Arithmetic

As I said previously, I wanted to focus on simplicity, and therefore you might notice that `shift-left` and `shift-right` are missing! Well, thats because you can accomplish the same thing by dividing and multiplying by two. Don't believe me? Check!

Anyways, a list of operations can be found below:
| Operation | Description |
| --------- | ----------- |
| `+` | Pops the first two values off of the stack, and adds them. The result is pushed onto the stack. |
| - | Pops the first two values off of the stack, and subtracts them. The result is pushed onto the stack. Ordering: `a b -` ---> `a - b`|
| `*` | Pops the first two values off of the stack, and multiplies them. The result is pushed onto the stack. |
| `/` |  Pops the first two values off of the stack, and divides them. The result is pushed onto the stack.  Ordering: `a b /` ---> `a / b`|
| `and` |  Pops the first two values off of the stack, and divides them. The result is pushed onto the stack.  Ordering: `a b and` ---> `a and b`|
| `or` |  Pops the first two values off of the stack, and divides them. The result is pushed onto the stack.  Ordering: `a b or` ---> `a or b`|
| `not` | Pops the top value off of the stack and performs a bitwise not (negation). The result is then pushed onto the stack.
| `d+` | Pops the first two values off of the stack, and performs a floating point addition on them. The result is pushed onto the stack. |
| `d-` | Pops the first two values off of the stack, and performs a floating point subtraction on them. The result is pushed onto the stack. Ordering: `a b -` ---> `a - b`|
| `d*` | Pops the first two values off of the stack, and performs a floating point multiplication on them. The result is pushed onto the stack. |
| `d/` |  Pops the first two values off of the stack, and performs a floating point division on them. The result is pushed onto the stack.  Ordering: `a b /` ---> `a / b`|

## Variables

### About:
There are two types of constants in VML - `memory` constants and `let` bound constants. A `memory` constant is just a value which indicates a position in memory allocated to the constant. The value of a memory constant is dictated to by the constants that are preceeding it. For example, if I had constant A which required 8 bytes of memory, and another constant I had just declared which required 4 bytes of memory, that constant would be given index 8 into memory, as the first 8 bytes are allocated to the first constant (if that makes sense anyway!).

`let` bindings are slightly different. They are your run-of-the-mill constants which simply contain a value which you assign to them. They can either contain a string literal, a double, or an unsigned integer. They can also contain characters, but they are effectively syntactic sugar for unsigned integers.

### Declaration:

To declare a `memory` constant, use the following:
```
memory <size in bytes> const <variable name>
```
In order to declare a let-bound constant, use the following:
```
let <value> const <variable name>
```

### Naming convention:

Unlike many other programming languages, the naming convention is quite loose in VML, however there are some rules:

Variable names may not:

- Begin with a number
- Begin with a `$`
- Contain any keyword within their name (Eg. `method_1` would be illegal.)

## Methods

### About:

Methods are VML's version of subroutines. To call a method, simply write the method name. Methods also follow the default naming convention.

### Declaration:

A method may be declared by doing the following:

```
method <method name> {
    ...
}

// Somewhere else in the program

    ...
    <method name>
    ...
```

## Buffers

As noted in the Variables section, you can create memory variables. To give these any use whatsoever, you must read and write to them. You can do this with the following:

| Command | Operation |
| ------- | --------- |
| `!64`   | Pops the top value off of the stack and stores it as a 64 bit unsigned integer into the memory offset which is popped off secondly.
| `!32`   | Pops the top value off of the stack and stores it as a 32 bit unsigned integer into the memory offset which is popped off secondly.
| `!16`   | Pops the top value off of the stack and stores it as a 16 bit unsigned integer into the memory offset which is popped off secondly.
| `!8 `   | Pops the top value off of the stack and stores it as an 8 bit unsigned integer into the memory offset which is popped off secondly.
| `@64`   | Pops the top value off of the stack and treats it as a memory offset. It then loads a 64-bit value from that offset.
| `@32`   | Pops the top value off of the stack and treats it as a memory offset. It then loads a 32-bit value from that offset.
| `@16`   | Pops the top value off of the stack and treats it as a memory offset. It then loads a 16-bit value from that offset.
| `@8`   | Pops the top value off of the stack and treats it as a memory offset. It then loads an 8-bit value from that offset.

As an example, this is a program to store the number `69` into the memory location pointed to by `buffer`:

```
memory 1 const buffer // assign a memory offset to buffer with the size of 1 byte.

method main {
    buffer 69 !8 // write 69 as an 8-bit value to the memory offset pointed to by buffer.
}
```
> Please note: Buffers have no overflow checking, and so writing a 32-bit value to a 1-byte-large buffer will succeed with no error, and overwrite any subsequent memory beyond the allocated limit.

## Boolean Operators

A language needs boolean operators to be turing complete, so here are the choices available in VML:

|Operator | Function|
|-|-|
|`>`| `pop a, b`, `a > b: 1`, `b >= a: 0`
|`<`| `pop a, b`, `a < b: 1`, `b <= a: 0`
|`=`| `pop a, b`, `a == b: 1`, `b != a: 0`
|`!=`| `pop a, b`, `a != b: 1`, `b == a: 0`
`str=`|Pops two string literals off of the stack. If they are equal, then a 1 is pushed to the stack. Otherwise, a 0 is pushed.
`str!=`| Pops two string literals off of the stack. If they are equal, then a 0 is pushed to the stack. Otherwise, a 1 is pushed.
|`d>`| `pop a, b`, `a > b: 1`, `b >= a: 0`
|`d<`| `pop a, b`, `a < b: 1`, `b <= a: 0`
|`d=`| `pop a, b`, `a == b: 1`, `b != a: 0`
|`d!=`| `pop a, b`, `a != b: 1`, `b == a: 0`

## Doubles
Finally! We can do more complex calculations!

To declare a double (decimal value), simply stick a `.0` on any unsigned integer. For more specific values, you may use the `wholenumber.decimal` syntax. Here is an example:
```
let 3.141592653 const PI
```
Doubles require specific boolean operators in certain scenarios, usually prefixed with a `d`, so the double version of `>` becomes `d>`. If you wish to see more on boolean operators, read [this](#boolean-operators).

## Standard Library

At the moment, the standard library is very limited, with only the following functions:
- `std-printu`
- `std-printi`
- `std-printd`
- `std-printb`
- `std-printf`
- `std-printh`
- `std-input`

These do the same as the original compiler-implemented functions.

On top of this, there are also the new `Sizeof()` functions:
- `Sizeof(i64)`
- `Sizeof(i32)`
- `Sizeof(i16)`
- `Sizeof(i8)`
- `Sizeof(char)`
- `Sizeof(float)`
- `Sizeof(ptr)`

Not that they are any use because you can't use them inside of `memory` declarations yet :/

## Miscellaneous

> Please note - there are no includes in VML as of now, and so your projects can only contain one file. VML also requires an installation of rust to compile.

Also, for those of you who like your low-level assembly programming, you can assemble files with the `-a` flag which will produce a single bin file which can be run with `-r <file>.bin`. For an instruction set reference, please use `spec.txt`.

As of now the language is still incomplete, and will recieve updates in the near future. Expect more!

## License
This project is under the [MIT License](https://github.com/AxolotlC/VML/blob/main/LICENSE). Any contributions are also under the MIT License.
