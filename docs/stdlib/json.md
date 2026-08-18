# std.json

JSON parse and stringify (native codegen supported).

```vpp
import std.json

let obj = json.parse("{\"name\":\"v++\"}")
let s = json.stringify(obj)
```

For production JSON workloads, validate outputs against your schema in tests.
