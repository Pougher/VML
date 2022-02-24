# VML - Virtual Machine Language
![GitHub release (latest by date)](https://img.shields.io/github/v/release/axolotlc/vml)
![GitHub all releases](https://img.shields.io/github/downloads/axolotlc/vml/total)
![GitHub repo size](https://img.shields.io/github/repo-size/axolotlc/vml)

Simplistic syntax, complicated programs.

<p align="center">
    <img src="https://github.com/AxolotlC/VML/blob/main/res/image_0.png" align="center">
</p>

---
## Basic syntax

While creating this language, I wanted to focus on simplistic syntax and it really shows! Only 17 keywords! I've also put some effort into them being easy to use, as VML shares
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
| `print` | 1 | Prints the value on top of the stack to the standard output. The value must be a file pointer to a **string literal**, otherwise you may encounter undefined behaviour.
| `uprint` | 1 | Prints the value on top of the stack to the standard output. The value must be an **unsigned 64-bit integer**. Otherwise, undefined behaviour may occur.
| `hprint` | 1 | Prints the value on top of the stack (formatted as a hexadecimal integer) to the standard output. The value must be an **unsigned 64-bit integer**. Otherwise, undefined behaviour may occur.
| `fprint` | 1 | Prints the value on top of the stack (formatted as a binary integer) to the standard output. The value must be an **unsigned 64-bit integer**. Otherwise, undefined behaviour may occur.
| `bprint` | 1 | Prints the **string literal** contained within a **memory buffer** to the standard output (must be null-terminated).
| `dup` | 1 | Duplicates the value on top of the stack. Eg. `2 dup` ---> `2 2`
| `swap` | 2 | Swaps the top two values on the stack around, such that `a b swap` ---> `b a`
| `rot` | 3 | Rotates the top three elements on the stack, such that ` a b c rot` ---> `c a b`
| `drop` | 1 | Drops the top value off of the stack (effectively removes one element and does nothing with it. |
| `memory` | 0 | Begins a buffer variable, which is automatically allocated an amount of memory that is requested by the programmer. For example: `memory 64 const Buffer` allocated 64 bytes to the variable `Buffer`
| `const` | 0 | Currently the only type of variable. Begins the declaration of a constant variable.
| `let` | 0 | Declares a non-memory variable which can be referenced throughout the program. Eg. `let "Hi" const hi_var`. Variables can be referenced anywhere throughout the program with just their name identifier (Eg. `hi_var print`).
| `input` | 1 | Fills a buffer supplied beforehand with a read line of input from the user. Usage: `buffer input` ---> `buffer = "I'm a line of input!"`
| `copy` | 2 | Fills the first operand (as an entry into memory) with a string literal supplied by the second element down in the stack. Eg: `"Hello" buffer copy` ---> `buffer = "Hello"`

## Arithmetic

As I said previously, I wanted to focus on simplicity, and therefore you might notice that `shift-left` and `shift-right` are missing! Well, thats because you can accomplish the same thing by dividing and multiplying by two. Don't believe me? Check!

Anyways, a list of operations can be found below:
| Operation | Description |
| --------- | ----------- |
| + | Pops the first two values off of the stack, and adds them. The result is pushed onto the stack. |
| - | Pops the first two values off of the stack, and subtracts them. The result is pushed onto the stack. Ordering: `a b -` ---> `a - b`|
| * | Pops the first two values off of the stack, and multiplies them. The result is pushed onto the stack. |
| / |  Pops the first two values off of the stack, and divides them. The result is pushed onto the stack.  Ordering: `a b /` ---> `a / b`|
| and |  Pops the first two values off of the stack, and divides them. The result is pushed onto the stack.  Ordering: `a b and` ---> `a and b`|
| or |  Pops the first two values off of the stack, and divides them. The result is pushed onto the stack.  Ordering: `a b or` ---> `a or b`|
| not | Pops the top value off of the stack and performs a bitwise not (negation). The result is then pushed onto the stack.

## Variables

### About:
There are two types of constants in VML - `memory` constants and `let` bound constants. A `memory` constant is just a value which indicates a position in memory allocated to the constant. The value of a memory constant is dictated to by the constants that are preceeding it. For example, if I had constant A which required 8 bytes of memory, and another constant I had just declared which required 4 bytes of memory, that constant would be given index 8 into memory, as the first 8 bytes are allocated to the first constant (if that makes sense anyway!).

`let` bindings are slightly different. They are your run-of-the-mill constants which simply contain a value which you assign to them. They can either contain a string literal, or an unsigned integer. They can also contain characters, but they are effectively syntactic sugar for unsigned integers.

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

