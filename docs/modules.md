# Modules

Modules are the building blocks of Iron programs. Any file with the appropriate
extension `.fe` is considered a module, with the name of the file being the name
of the module.

Modules help you organize and expose various different kinds of objects,
including:

- functions
- types
- protocols
- extensions
- enums

By default, all contents of an Iron module are *private*, meaning they can only
be used by objects within the same module. If you want to be able to use them
across different modules, they must be marked as `public`.

For example, we have two files below; the first defines a `Person` type, and the
second imports that type and shows how it is used in a function.

``` iron
// ~/types/Person.fe
public type Person {
    name: String,
    age: U8
}
```

``` iron
// ~/testPerson.fe
import { Person } from "./types/Person"

function hello(_ person: Person) -> String {
    return "Hello, my name is {person.name}!"
}

function main {
    let me: Person = {
        name: "Sam",
        age: 25
    }

    let greeting = "Hello, my name is Sam!"
    assert(hello(me), isEqualTo: greeting)
}
```