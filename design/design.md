# Sable Design

## Mission
The main mission of Sable is to be a really fast and simple compiled coding language. Therefore, the syntax tries to embody this.

## Basic Hello World Program
Here is a basic hello world program:
```
fun main() {
    print("Hello, world!")
}
```

Nothing much to note here, besides the fact that there aren't semicolons. This language doesn't require them. Also important to note is that function **definitions** use the `fun` keyword, while function **types** use the `function` identifier.

## Variables
Variables can only be defined inside functions. For any variables here, assume them to be in a main function.

To define an immutable variable, you use the `let` keyword.
```
let x = 10
```

But what if you want it to be mutable? It's not hard, just use the `var` keyword instead of `let`.
```
var x = 10
```

You may also notice there isn't any type decleration in these variables. This is because the type is easily inferred from this, however that isn't always the case. To specify the type, you use the following syntax:
```
let x (int) = 10
```

The type of the variable is placed right after the name.

## Types
### Primitives
- `int`: A 32-bit integer.
- `float`: A 32-bit float.
- `uint`: A 32-bit unsigned integer.
- `char`: An 8-bit character.
- `type`: A type.
- `bool`: A boolean value
- `function`: A function **type**

### Type Operators
You can use the operator "or" with types to say something is one of a few types.

## Functions
A function without any paremeters or return type is what you are pretty used to. You'll mainly use this for the main function.
```
fun main() {
    print("I ran!")
}
```

To specify the return type of a functiom, you use the `returns` keyword. Note that the type is within parentheses.
```
fun return_ten() returns (int) {
    return 10
}
```

To add a paremeter, you place it inside the parentheses, with its type right afterwards.
```
fun add_one( number (int) ) returns (int) {
    return number + 1
}
```

To have many paremeters, you **don't** seperate them with commas.
```
fun add(number1 (int) 
        number2 (int))
    returns (int) {
    return number1 + number2
}
```

## Structs and Templates
Structures in Sable are just like other languages. They store data with names.
```
struct Person {
    pub name (string)
    pub age (int)
}
```

To create an instance of this Person, you use curly braces and assign the name and age.
```
let my_person (Person) = Person {
    name = "Jeff"
    age = 26
}
```

You may want some of these fields to not be required. You have only one option for this: set a default value, like this.
```
struct Person {
    pub name (string)
    pub age (int) = 99
}
let my_person (Person) = Person {
    name = "John"
}
```

Notice the keyword `pub`? This marks this function as public, meaning anything outside this struct can call this function. Without this, only this struct can call this function. You can also use this keyword on fields of the struct.

Now, what if you want shared fields between two structs? This is where templates come in. You can define a template just like a struct, but use the keyword `template` instead. We'll stick with the theme of people and make a Person template. Here, we use the `mut` keyword to say these are mutuable.
```
template Person {
    mut name (string)
    mut age (int)
}
```

Now, to use this template, you use the `from` keyword! Let's make a student.
```
struct Student from Person {
    major (string)
}
```

All together, here is our basic program for a student!
```
// Create the person template
template Person {
    mut name (string)
    mut age (int)
}
// Create the struct from the person
struct Student from Person {
    major (string)
}

// Create a student
let student (Student) = Student {
    name = "John"
    age = 22
    major = "CS"
}
```

Now, having these inherited fields wouldn't be very useful without a way to tell if they did have them, right? Well, you can! There is a special kind of type for structs that are from some template.
```
let student (from Person) = Student {
    ...
}
```
Since `Student` is `from Person`, this type is completely valid.

Finally, what if you want a sort of "parent struct" that also acts as a template? Well, you can define something as a `template struct` like this to do exactly that! This is just syntax sugar for defining a struct and a template with the same name, which is also valid.
```
template struct Person {
    name (string)
    age (int)
}

let p = Person {...}

struct Student from Person {
    major (string)
}

let s = Student {...}
```

## Enums
Enums are very simple.
```
enum Operator {
    Plus
    Minus
    Multiply
    Divide
}
```

Enums can also have a value inside of them.
```
enum Token {
    Number (float)
    String (string)
    Operator (Operator)
    Semicolon
}
```

## Exporting & Multiple Files
Having all your code in one file is a pain. You can split your code off into many files and import them with the `imports` keyword, using a `.` to mark folders instead of a `/`. Everything inside of a file is private by default, unless you use the `exports` keyword.

Assume the project structure of `src/main.sbl` and `src/add.sbl`:
```
// src/add.sbl
exports fun add(number1 (int)
                number2 (int))
            returns (int) {
    return number1 + number2
}

// src/main.sbl
imports add

fun main() {
    print(add(10 20))
}
```

There is also a `protect` keyword. This is like `export` but it only passes it out to files in the same directory or a subdirectory.

Assume the project structure of
```
src/
 - main.sbl
 - lib/
    - add.sbl
    - test.sbl
```

```
// src/lib/add.sbl
protect fun add(a (int)
                b (int))
            returns (int) {
    return a + b
}

// src/lib/test.sbl
imports add
exports fun add_one(a (int))
            returns (int) {
    return add.add(a 1)
}

// src/main.sbl
imports lib.test
imports lib.add as add // Use the `as` keyword to use a custom name

fun main() {
    print(lib.test.add_one(2)) // Success
    print(add.add(2 1)) // Error: function "add" not found in "add" (protected)
}
```

Finally, if you wish to import a file that isn't up some amount of directories from where you are, you can use `imports` from the `main.sbl` directory by prefixing what you are importing with a `@` symbol.

Assume this file structure:
```
src/
 - main.sbl
 - test.sbl
 - lib/
    - test2.sbl
```
```
// src/lib/test.sbl
exports fun test() {
    print("Test!")
}

// src/lib/test2.sbl
imports @test

exports fun test2() {
    print("Testing: ")
    test.test()
}

// src/main.sbl
imports test
imports @lib.test2 as test2 // Using @ here just to show off; it does nothing in the main file.

fun main() {
    test.test() // "Test!"
    test2.test2() // "Testing: " "Test!"
}
```