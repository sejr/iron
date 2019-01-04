# Types

One of Iron's biggest goals is to develop a powerful type system. All types must
be derived from a set of standard "base types."

## Integers and Floats

Integers are whole numbers that can be signed or unsigned. Signed integers can
include negative values, whereas unsigned integers cannot. Floating point values
can be positive or negative, and can include a fractional component.

All of these types come in a variety of different sizes to suit your use case.
Specifically, they all have 8-bit, 16-bit, 32-bit, 64-bit, and 128-bit variants.
Integers, for example, can be defined as `I8`, `I16`, `I32`, `I64`, or `I128`.
For unsigned integers and floats, you switch the `I` with `U` or `F`,
respectively.

``` iron
function numbers {
    let a: I16 = 1
    let b: U32 = 2
    let c: F64 = 3

    try {
        let sum = a + b + c
    } catch {
        print("Not so fast -- the types don't match up!")
    }
}
```

## Characters and Strings

Characters are [Unicode scalar values](http://www.unicode.org/glossary/#unicode_scalar_value) and are all four bytes
in size. A String is simply an array of characters. They implement many of the
same protocols and work extremely well with each other.

``` iron
function charsAndStrings {
    let a: Char = "I"
    let b: Char = "❤️"
    let c: String = "Iron"
    
    assert(c.prepend(b.prepend(a)) == "I❤️Iron")
}
```

## Booleans

Booleans can only be one of two different values: `true` or `false`. This type
is required by expressions such as `if`, `while`, and `assert`.

``` iron
function booleans {
    let earthIsFlat = false

    try {
        assert(earthIsFlat)
    } catch {
        print("Have you ever been on a plane?")
    }

    if !earthIsFlat {
        print("The NOT operator switches the value of a boolean")
    }

    let pigsCanFly = false
    while !pigsCanFly {
        doSomethingForever()
    }

    let a = true
    let b = false
    assert((a equals !b) equals (b equals !a))
    assert(a and !b)
    assert(a xor b)
    assert(a or b)
}
```

## Null

TODO

## Self

TODO