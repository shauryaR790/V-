"use client"

import React from "react"
import { CodeBlock } from "@/components/ui/code-block"

export function CodeBlockDemo() {
  const code = `// hello.vpp — run: vpp run hello.vpp
import std.io

fn greet(name: string) -> string {
    return "Welcome, " + name
}

fn main() -> int {
    print(greet("developer"))
    return 0
}`

  const shell = `vpp run examples/hello.vpp
vpp check examples/hello.vpp
vpp test
vpp build src/main.vpp -o app.exe
vpp doctor`

  return (
    <div className="max-w-3xl mx-auto w-full space-y-6 p-6 bg-[#0d1117]">
      <CodeBlock
        language="vpp"
        filename="hello.vpp"
        highlightLines={[5, 9, 10]}
        code={code}
      />
      <CodeBlock
        language="bash"
        filename="terminal"
        code={shell}
      />
    </div>
  )
}
