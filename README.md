# RUST

## Index
- [RUST](#rust)
  - [Index](#index)
  - [Fundamentals](#fundamentals)
    - [Data Types](#data-types)
    - [Variables](#variables)
    - [Functions](#functions)
    - [Println Macro](#println-macro)
    - [Control flow using `if`](#control-flow-using-if)
      - [Example `if..else`](#example-ifelse)
      - [Example Nested `if..else`](#example-nested-ifelse)
      - [Example `else..if`](#example-elseif)
    - [Repetition using loops](#repetition-using-loops)

## Fundamentals

### Data Types
- Boolean
- Integer
- Double & float
- Character
- String

### Variables
* Assign data to a temporary memory location
    * Allows Programmer to easily work with memory
* Immutable by default, but can be mutable

```rust
let j = 'j';
let two = 2;
let my_half = 0.5;
let mut my_name = "Angel";
let quit_program = false;
let your_half = my_half;
```

### Functions
* A way to encapsulate program functionality
* Optionally accept data
* Optionally return data
* Utilized for code organization
    * Also makes code easier to read

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

let x = add(1, 1);
let y = add(3, 0);
let z = add(x, 1);
```

### Println Macro
* "Prints" (displays) information to the terminal
* Macros use an exclamation point to call
* Generate additional Rust code
* Useful for debugging

```rust
let life = 42;
println!("hello");
println!("{:?}", life);
println!("{:?} {:?}", life, life);
```

### Control flow using `if`
* Code executed line-by-line
* Actions are performed & control flow may change
    * Specific conditions can change control flow
        * `if`
        * `else`
        * `else if`

#### Example `if..else`
```rust
let a = 99;
if a > 99 {
    println!("Big number");
} else {
    println!("Small number");
}

```

#### Example Nested `if..else`

```rust
let a = 99;
if a > 99 {
    if a > 200 {
        println!("Huge number");
    } else {
        println!("Big number");
    }
} else {
    println!("Small number");
}
```

#### Example `else..if`

```rust
let a = 99;
if a > 200 {
    println!("Huge number");
} else if a > 99 {
    println!("Big number");
} else {
    println!("Small number");
}
```

### Repetition using loops
* Called "lopping" or "iteration"
* Multiple types of loops
    * `loop` - infinite loop
    * `while` - conditional loop

**Loop:**
```rust
let mut a = 0;
loop {
    if a == 5 {
        break;
    }
    println!("{:?}", a);
    a = a + 1;
}
```
**While loop:**
```rust
let mut a = 0;
while a != 5 {
    println!("{:?}", a);
    a = a + 1;
}
```
