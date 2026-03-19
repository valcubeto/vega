# The Vega programming language.
I will document properly after adding functionality.<br />
At the moment this program does nothing.

# Quick docs
Types:
- Numbers: `Integer`, `Decimal`
- Fixed-size integers: `Int8`, `Int16`, `Int32`, `Int64`
- Fixed-size unsigned integers: `UInt8`, `UInt16`, `UInt32`, `UInt64`
- System-defined size numbers: `Size`, `Offset`
- Floats: `Float32`, `Float64`
- Text: `Char`, `String`
- Lists: `Array` (fixed-size), `List` (extensible), tuples
- Collections: `Map`, `Dict`

Static variables:
```kotlin
const NAME: String = "Dan"
state AGE: UInt8 = 25
```

Functions:
```kotlin
// Optional args and return type
fun main(args: Args): Nothing {
    let lambda = { x, y => x + y }
}
```

Interfaces:
```kotlin
class Indent {
    fun indent(self, n: Size): Self
}

impl Indent for String {
    fun indent(self, n: Size = 1): Self {
        // map_lines doesn't drop line breaks.
        self.map_lines({ line => "    ".repeat(n) + line })
            .collect()
    }
}
```
