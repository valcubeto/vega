# The Vega programming language.
I will document properly after adding functionality.

**PLEASE SEE OTHER BRANCHES**

# Quick docs
Types:
- Numbers: `Integer`, `Decimal`
- Fixed-size integers: `I8`, `I16`, `I32`, `I64`, `U8`, `U16`, `U32`, `U64`
- Pointer size numbers: `Size`, `Offset`
- Floats: `F32`, `F64`
- Text: `Char`, `String`
- Lists: `Slice` (fixed-size), `Array` (extensible), `Set`, tuples
- Collections: `Map`, `Dict`

Static variables:
```
const VERSION: String = "0.1.0"
state COUNTER: U8 = 0
```

Functions:
```
fun main() {
    println("Hello, world!")
}
```

Structs:
```
struct Point {
    x: Integer
    y: Integer

    fun Self::static_member() {
        println("Point::static_member")
    }

    fun otherside() -> Self {
        Point { x = -self.x, y = -self.y }
    }
}

let point = Point { x = 5, y = -11 }
```

Interfaces:
```
interface ToString {
    fun to_string() -> String
}

impl ToString for Point {
    fun to_string() {
        f"({self.x}, {self.y})"
    }
}
```
