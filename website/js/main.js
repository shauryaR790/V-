const CODE_SAMPLES = {
  hello: `import std.io

fn greet(name: string) -> string {
    return "Hello, " + name + "!"
}

fn main() -> int {
    print(greet("world"))
    return 0
}`,
  build: `# Native compile to .exe
vpp build app.vpp -o app.exe

# Run with interpreter (no build step)
vpp run app.vpp`,
  test: `test "addition works" {
    assert_eq(add(2, 3), 5)
}

fn add(a: int, b: int) -> int {
    return a + b
}`,
  project: `name = "my-app"
version = "0.1.0"
entry = "src/main.vpp"

[dependencies]
hello-lib = "0.1.0"`,
  native: `struct User {
    name: string
    age: int
}

fn main() -> int {
    let user = User {
        name: "Shaurya"
        age: 18
    }
    print(user.name)
    return 0
}`,
};

function highlightVpp(code) {
  return code
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/(#.*)$/gm, '<span class="cm">$1</span>')
    .replace(/\b(fn|let|mut|return|import|struct|enum|match|if|test|assert_eq)\b/g, '<span class="kw">$1</span>')
    .replace(/"([^"]*)"/g, '<span class="str">"$1"</span>');
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
    if (pre) pre.innerHTML = highlightVpp(activeCode);
    if (langLabel) langLabel.textContent = key === "project" ? "TOML" : key === "build" || key === "test" ? "Shell / v++" : "v++";
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
    if (a.getAttribute("href") === path) a.classList.add("active");
  });
});
