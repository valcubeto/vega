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
- Lists: `Slice` (fixed-size), `Array` (extensible), tuples
- Collections: `Map`, `Dict`

Static variables:
```crystal
const VERSION: String = "0.1.0"
state COUNTER: UInt8 = 0
```

Functions:
```crystal
// Optional args and return type
fun main(args: Args): Nothing {
    let closure = do x, y -> x + y
}
```

Interfaces:
```crystal
class Indent {
    fun indent(self, n: Size): Self
}

impl Indent for String {
    fun indent(self, n: Size = 1): Self {
        self.map_lines(do line -> "    ".repeat(n) + line)
            .collect()
    }
}
```
