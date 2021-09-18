# RUST <!-- omit in toc -->

## Index: <!-- omit in toc -->
- [1. Fundamentals](#1-fundamentals)
  - [1.1. Data Types](#11-data-types)
  - [1.2. Variables](#12-variables)
  - [1.3. Functions](#13-functions)
  - [1.4. Println Macro](#14-println-macro)
  - [1.5. Control flow using `if`](#15-control-flow-using-if)
  - [1.6. Repetition using loops](#16-repetition-using-loops)
  - [1.7. Match](#17-match)
    - [1.7.1. `match` vs `else..if`](#171-match-vs-elseif)
    - [1.7.2. Match recap](#172-match-recap)
  - [1.8. Working With Data](#18-working-with-data)
    - [1.8.1. Enumeration `enum`](#181-enumeration-enum)
      - [1.8.1.1. Enums recap](#1811-enums-recap)
    - [1.8.2. Structure `struct`](#182-structure-struct)
      - [1.8.2.1. Structs recap](#1821-structs-recap)
    - [1.8.3. Tuples](#183-tuples)
      - [1.8.3.1. Tuples recap](#1831-tuples-recap)
    - [1.8.4. Expressions](#184-expressions)
      - [1.8.4.1. Expressions recap](#1841-expressions-recap)
    - [1.8.5. Intermediate Memory](#185-intermediate-memory)
      - [1.8.5.1. Addresses](#1851-addresses)
      - [1.8.5.2. Offsets](#1852-offsets)
      - [1.8.5.3. Intermediate Memory recap](#1853-intermediate-memory-recap)
    - [1.8.6. Ownership](#186-ownership)
      - [1.8.6.1. Ownership recap](#1861-ownership-recap)

# 1. Fundamentals

## 1.1. Data Types
- Boolean
- Integer
- Double & float
- Character
- String

## 1.2. Variables
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

## 1.3. Functions
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

## 1.4. Println Macro
* _Prints_ (displays) information to the terminal
* Macros use an exclamation point to call
* Generate additional Rust code
* Useful for debugging

```rust
let life = 42;
println!("hello");
println!("{:?}", life);
println!("{:?} {:?}", life, life);
```

## 1.5. Control flow using `if`
* Code executed line-by-line
* Actions are performed & control flow may change
    * Specific conditions can change control flow
        * `if`
        * `else`
        * `else if`

**Example `if..else`:**
```rust
let a = 99;
if a > 99 {
    println!("Big number");
} else {
    println!("Small number");
}

```

**Example Nested `if..else`:**
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

**Example `else..if`:**
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

## 1.6. Repetition using loops
* Called _"lopping"_ or _"iteration"_
* Multiple types of loops
    * `loop` - infinite loop
    * `while` - conditional loop

**Loop**:
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

## 1.7. Match
* Add logic to program
* Similar to `if..else`
* Exhaustive
  * All options must be accounted for

**Example with boolean:**
```rust
fn main() {
    let some_bool = true;
    match some_bool {
        true => println!("its true"),
        false => println!("its false"),
    }
}
```

**Example with int:**
```rust
fn main() {
    let some_int = 3;
    match some_int {
        1 => println!("its 1"),
        2 => println!("its 2"),
        3 => println!("its 3"),
        _ => println!("its somethings else"),
    }
}
```

### 1.7.1. `match` vs `else..if`
* `match` will be checked by the compiler
    * If a new possibility is added, you will be notified when this occurs
* `else..if` is <ins>not</ins> checked by the compiler
    * If a new possibility is added, your code may contain a bug

### 1.7.2. Match recap
* Prefer `match` over `else..if` when working with a single variable
* `match` considers all possibilities
    * More robust code
* Use underscore (_) to match **anything else**

## 1.8. Working With Data

### 1.8.1. Enumeration `enum`
* Data that can be one of multiple different possibilities
    * Each possibility is called a _**variant**_
* Provides information about your program to the compiler
    * More robust programs

**Example:**
```rust
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn which_way(go: Direction) {
    match go {
        Direction::Up => "up",
        Direction::Down => "down",
        Direction::Left => "left",
        Direction::Right => "right",
    }
}
```

#### 1.8.1.1. Enums recap
* Enums can only be one variant at a time
* More robust programs when paired with `match`
* Make program code easier to read

### 1.8.2. Structure `struct`
* A type that contains multiple pieces of data
    * All or nothing - cannot have some pieces of data and not others
* Each piece of data is called a _**field**_
* Makes working with data easier
    * Similar data can be grouped together

**Example:**
```rust
struct ShippingBox {
    depth: i32,
    width: i32,
    height: i32,
}

fn main() {
    let my_box = ShippingBox {
        depth: 3,
        width: 2,
        height: 5,
    };
    let tall = my_box.height; // 5
    println!("the box is {:?} units tall", tall);
}
```

#### 1.8.2.1. Structs recap
* Structs deal with multiple pieces of data
* All fields must be present to create a `struct`
* Fields can be accessed using a dot (.)

### 1.8.3. Tuples
* A type of "record"
* Store data anonymously
    * No need to name fields
* Useful to return paris of data from functions
* Can be "destructured" easily into variables

**Example:**
```rust
enum Access {
    Full,
}

fn one_two_three() -> (i32, i32, i32) {
    (1, 2, 3)
}

fn main() {
    let numbers = one_two_three();
    let (x, y, z) = one_two_three();
    println!("{:?}, {:?}", x, numbers.0); // 1
    println!("{:?}, {:?}", y, numbers.1); // 2
    println!("{:?}, {:?}", z, numbers.2); // 3

    let (employee, access) = ("Jake", Access::Full);
}
```

#### 1.8.3.1. Tuples recap
* Allow for anonymous data access
* Useful when destructuring
* Can contain any number of fields
    * Use `struct` when more than 2 or 3 fields

### 1.8.4. Expressions
* Rust is an expression-based language
    * Most things are evaluated and return some value
* Expression values coalesce to a single point
    * Can be used for nesting logic

**Examples:**
```rust
let my_num = 3;
let is_lt_5 = if my_num < 5 {
    true
} else {
    false
};

// or

let is_lt_5 = my_num < 5;

```

```rust
let my_num = 3;
let message = match my_num {
    1 => "hello",
    _ => "goodbye"
}
```

```rust
enum Menu {
    Burger,
    Fries,
    Drink,
}

let paid = true;
let item = Menu::Drink;
let drink_type = "water";
let order_placed = match item {
    Menu::Drink => {
        if drink_type == "water" {
            true
        } else {
            false
        }
    }
    _ => true,
};
```

#### 1.8.4.1. Expressions recap
* Expressions allow nested logic
* `if` and `match` expressions can be nested
    * Best to not use more than two or three levels

### 1.8.5. Intermediate Memory
**Basic memory refresh:**
* Memory is stored using binary
    * Bits: _0_ or _1_
* Computer optimized for bytes
    * _1_ byte == _8_ _contiguous bits_
* Fully contiguous

#### 1.8.5.1. Addresses
* All data in memory has an _**address**_
    * Used to locate data
    * Always the same - only data changes
* Usually don't utilize addresses directly
    * Variables handle most of the work

#### 1.8.5.2. Offsets
* Items can be located at and address using an _**offset**_
* Offsets begin at 0
* Represent the number of bytes away from the original address
    * Normally deal with indexes instead

![Addresses & Offsets](./images/address&offsets.png)

#### 1.8.5.3. Intermediate Memory recap
* Memory uses addresses & offsets
* Addresses are permanent, data differs
* Offsets can be used to "index" into some data

### 1.8.6. Ownership
**Managing memory:**
* Programs must track memory
    * If they fail to do so, a _**leak**_ occurs
* Rust utilizes an _**ownership**_ model to manage memory
    * The _**owner**_ of memory is responsible for cleaning up the memory
* Memory can either be _**moved**_ or _**borrowed**_

**Example - Move ❌:**
```rust
enum Light {
    Bright,
    Dull,
}

fn display_light(light: Light) {
    match light {
        Light::Bright => println!("bright");
        Light::Dull => println!("dull");
    }
}

fn main() {
    let dull = Light:Dull;
    display_light(dull);
    display_light(dull);
}
```

**Example - Borrow ✅:**
```rust
enum Light {
    Bright,
    Dull,
}

fn display_light(light: &Light) {
    match light {
        Light::Bright => println!("bright");
        Light::Dull => println!("dull");
    }
}

fn main() {
    let dull = Light:Dull;
    display_light(&dull);
    display_light(&dull);
}
```

#### 1.8.6.1. Ownership recap
* Memory must be managed in some way to present leaks
* Rust uses _**ownership**_ to accomplish memory management
    * The _**owner**_ of data must clean up the memory
    * This occurs automatically at the end of the scope
* Default behavior is to _**move**_ memory to a new owner
    * Use and ampersand _**&**_ to allow code to _**borrow**_ memory