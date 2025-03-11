# Sable Examples
## Simple text thing 
```
imports std.io as io
imports sdt.parse as parse

fun main() {
    print("What's your name?\n $  " false)
    let name = io.readline()

    print("Hello, " .. name .. "! I'm Sable. Anything you want me to say?")
    print(" $  " false)
    let to_say = io.readline()

    print("How many times do you want me to say this?")
    print(" $  " false)
    let times_str = io.readline()
    let times_num = parse.parse_int_or(times_str 10)

    for i in range(1 times_num) {
        print(i .. ": " .. to_say)
    }

    print("That was a lot!")
}

```
Single line (because this works)
```
imports std.io as io imports sdt.parse as parse fun main() { print("What's your name?\n $  " false) let name = io.readline() print("Hello, " .. name .. "! I'm Sable. Anything you want me to say?") print(" $  " false) let to_say = io.readline() print("How many times do you want me to say this?") print(" $  " false) let times_str = io.readline() let times_num = parse.parse_int_or(times_str 10) for i in range(1 times_num) { print(i .. ": " .. to_say) } print("That was a lot!") }
```

# Tokenize text into words and punctuation (simple)
```
imports std.io as io

enum Token {
    Word(string)
    Comma
    Period
}

fun tokenize(text (string))
    returns (list of Token) {
    var tokens (list of Token) = []
    var i = 0

    while i < string.len(text) {
        let character = string.char(text i)

        if string.is_alphanumeric(character) {
            var text (string) = character to (string)

            i += 1
            while i < string.len(text) and string.is_alphanumeric(string.char(text i)) {
                let character2 = string.char(text i)
                text ..= character2
                i += 1
            }

            list.append(tokens Token.Word(text))

        } elseif character == '.' {
            list.append(tokens Token.Period)
        } elseif character == ',' {
            list.append(tokens Token.Comma)
        } else {
            print("Weird character! " .. character)
        }

        i += 1
    }
}

fun main() {
    let text = io.readline()
    let tokens = tokenize(text)
    print(tokens)
}
```