const CODE_SAMPLES = {
  hello: `// hello.vpp — run: vpp run hello.vpp
import std.io

fn greet(name: string) -> string {
    return "Hello, " + name + "!"
}

fn add(a: int, b: int) -> int {
    return a + b
}

fn main() -> int {
    print(greet("world"))
    print(add(10, 20))

    let mut total = 0
    for i in 0..5 {
        total = total + i
    }
    print(total)

    let words = ["v++", "native", "fast"]
    for w in words {
        print(w)
    }
    return 0
}`,

  native: `struct User {
    name: string
    age: int
    active: bool
}

enum Role {
    Dev
    User
}

fn describe(user: User) -> string {
    return user.name
}

fn main() -> int {
    let user = User {
        name: "Shaurya"
        age: 18
        active: true
    }
    print(describe(user))
    print(user.age)
    return 0
}`,

  build: `# Compile to native .exe (requires LLVM/clang)
vpp build app.vpp -o app.exe

# Run the executable
./app.exe

# Or interpret without building
vpp run app.vpp

# Type-check only
vpp check app.vpp

# Format source
vpp fmt app.vpp`,

  test: `fn add(a: int, b: int) -> int {
    return a + b
}

test "addition works" {
    assert_eq(add(2, 3), 5)
}

test "zero identity" {
    assert_eq(add(0, 0), 0)
}

test "negative nums" {
    assert_eq(add(-1, 1), 0)
}`,

  project: `# vpp.toml — project manifest
name = "my-app"
version = "0.1.0"
entry = "src/main.vpp"

[dependencies]
hello-lib = "0.1.0"

# Commands:
#   vpp new my-app
#   vpp add hello-lib
#   vpp test
#   vpp run src/main.vpp`,
};

function highlightLine(line) {
  return line
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/(#.*)$/, '<span class="cm">$1</span>')
    .replace(/\b(fn|let|mut|return|import|struct|enum|match|if|for|test|assert_eq|true|false)\b/g, '<span class="kw">$1</span>')
    .replace(/"([^"]*)"/g, '<span class="str">"$1"</span>');
}

function renderCode(code) {
  return code
    .split("\n")
    .map((line, i) =>
      `<div class="code-line"><span class="ln">${i + 1}</span><span class="lc">${highlightLine(line) || " "}</span></div>`
    )
    .join("");
}

document.addEventListener("DOMContentLoaded", () => {
  const toggle = document.querySelector(".nav-toggle");
  const nav = document.querySelector(".top-nav");
  if (toggle && nav) {
    toggle.addEventListener("click", () => nav.classList.toggle("open"));
  }

  const tabs = document.querySelectorAll(".code-tab");
  const pre = document.querySelector("#code-display");
  const langLabel = document.querySelector("#code-lang");
  const copyBtn = document.querySelector(".copy-btn");

  let activeCode = CODE_SAMPLES.hello;

  function show(key) {
    activeCode = CODE_SAMPLES[key] || CODE_SAMPLES.hello;
    if (pre) {
      pre.innerHTML = `<div class="code-lines">${renderCode(activeCode)}</div>`;
    }
    if (langLabel) {
      langLabel.textContent =
        key === "project" ? "TOML" : key === "build" ? "Shell" : key === "test" ? "v++" : "v++";
    }
  }

  tabs.forEach((tab) => {
    tab.addEventListener("click", () => {
      tabs.forEach((t) => t.classList.remove("active"));
      tab.classList.add("active");
      show(tab.dataset.sample);
    });
  });

  if (pre) show("hello");

  if (copyBtn) {
    copyBtn.addEventListener("click", () => {
      navigator.clipboard.writeText(activeCode).then(() => {
        copyBtn.textContent = "Copied!";
        setTimeout(() => { copyBtn.textContent = "Copy to clipboard"; }, 1500);
      });
    });
  }

  const path = location.pathname.split("/").pop() || "index.html";
  document.querySelectorAll(".nav-link").forEach((a) => {
    const href = a.getAttribute("href") || "";
    if (href.endsWith(path) || (path === "" && href.endsWith("index.html"))) {
      a.classList.add("active");
    }
  });
});
