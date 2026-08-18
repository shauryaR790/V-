# std.fs

File system access (native codegen supported).

```vpp
import std.fs

let text = fs.read_file("data.txt")
fs.write_file("out.txt", "hello")
let exists = fs.file_exists("data.txt")
```

Use `vpp run` (interpreter) or `vpp build` depending on deployment needs.
