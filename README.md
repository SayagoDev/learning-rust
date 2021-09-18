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
    - [Match](#match)
      - [`match` vs `else..if`](#match-vs-elseif)
      - [Match recap](#match-recap)
  - [Working With Data](#working-with-data)
    - [Enumeration `enum`](#enumeration-enum)
      - [Enums recap](#enums-recap)
    - [Structure `struct`](#structure-struct)
      - [Structs recap](#structs-recap)
    - [Tuples](#tuples)
      - [Tuples recap](#tuples-recap)
    - [Expressions](#expressions)
      - [Expressions recap](#expressions-recap)
    - [Intermediate Memory](#intermediate-memory)
      - [Addresses](#addresses)
      - [Offsets](#offsets)
      - [Intermediate Memory recap](#intermediate-memory-recap)
    - [Ownership](#ownership)
      - [Ownership recap](#ownership-recap)

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

### Match
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

#### `match` vs `else..if`
* `match` will be checked by the compiler
    * If a new possibility is added, you will be notified when this occurs
* `else..if` is <ins>not</ins> checked by the compiler
    * If a new possibility is added, your code may contain a bug

#### Match recap
* Prefer `match` over `else..if` when working with a single variable
* `match` considers all possibilities
    * More robust code
* Use underscore (_) to match "anything else"

## Working With Data

### Enumeration `enum`
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

#### Enums recap
* Enums can only be one variant at a time
* More robust programs when paired with `match`
* Make program code easier to read

### Structure `struct`
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

#### Structs recap
* Structs deal with multiple pieces of data
* All fields must be present to create a `struct`
* Fields can be accessed using a dot (.)

### Tuples
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

#### Tuples recap
* Allow for anonymous data access
* Useful when destructuring
* Can contain any number of fields
    * Use `struct` when more than 2 or 3 fields

### Expressions
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

#### Expressions recap
* Expressions allow nested logic
* `if` and `match` expressions can be nested
    * Best to not use more than two or three levels

### Intermediate Memory
**Basic memory refresh:**
* Memory is stored using binary
    * Bits: 0 or 1
* Computer optimized for bytes
    * 1 byte == 8 contiguous bits
* Fully contiguous

#### Addresses
* All data in memory has an "address"
    * Used to locate data
    * Always the same - only data changes
* Usually don't utilize addresses directly
    * Variables handle most of the work

#### Offsets
* Items can be located at and address using an "offset"
* Offsets begin at 0
* Represent the number of bytes away from the original address
    * Normally deal with indexes instead

![Addresses & Offsets](./images/address&offsets.png)

#### Intermediate Memory recap
* Memory uses addresses & offsets
* Addresses are permanent, data differs
* Offsets can be used to "index" into some data

### Ownership
**Managing memory:**
* Programs must track memory
    * If they fail to do so, a "leak" occurs
* Rust utilizes an "ownership" model to manage memory
    * The "owner" of memory is responsible f9or cleaning up the memory
* Memory can either be ¨"moved" or "borrowed"

**Example - Move (❌):**
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

**Example - Borrow:**
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

#### Ownership recap
* Memory must be managed in some way to present leaks
* Rust uses "ownership" to accomplish memory management
    * The "owner" of data must clean up the memory
    * This occurs automatically at the end of the scope
* Default behavior is to "move" memory to a new owner
    * Use and ampersand (&) to allow code to "borrow" memory