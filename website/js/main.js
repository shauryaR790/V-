const CODE_SAMPLES = {
  hello: {
    lang: "javascript",
    label: "hello.vpp",
    code: `// hello.vpp — run: vpp run hello.vpp
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
    return 0
}`,
  },
  native: {
    lang: "javascript",
    label: "user.vpp",
    code: `struct User {
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
    return 0
}`,
  },
  build: {
    lang: "bash",
    label: "terminal",
    code: `# Compile to native .exe (requires LLVM/clang)
vpp build app.vpp -o app.exe
./app.exe

# Or interpret without building
vpp run app.vpp
vpp check app.vpp
vpp fmt app.vpp`,
  },
  test: {
    lang: "javascript",
    label: "tests.vpp",
    code: `fn add(a: int, b: int) -> int {
    return a + b
}

test "addition works" {
    assert_eq(add(2, 3), 5)
}

test "zero identity" {
    assert_eq(add(0, 0), 0)
}`,
  },
  project: {
    lang: "toml",
    label: "vpp.toml",
    code: `name = "my-app"
version = "0.1.0"
entry = "src/main.vpp"

[dependencies]
hello-lib = "0.1.0"`,
  },
};

function countCodeLines(text) {
  if (!text) return 0;
  const parts = text.split("\n");
  if (parts.length > 1 && parts[parts.length - 1] === "") parts.pop();
  return parts.length;
}

function syncLineNumbers(pre) {
  if (!pre) return;

  const code = pre.querySelector("code");
  if (!code) return;

  pre.querySelector(".line-numbers-rows")?.remove();

  let gutter = pre.querySelector(".code-ln-gutter");
  if (!gutter) {
    gutter = document.createElement("div");
    gutter.className = "code-ln-gutter";
    gutter.setAttribute("aria-hidden", "true");
    pre.insertBefore(gutter, code);
  }

  pre.classList.add("has-line-numbers");

  const lineCount = countCodeLines(code.textContent);
  gutter.innerHTML = "";

  for (let i = 1; i <= lineCount; i += 1) {
    const line = document.createElement("span");
    line.className = "code-ln";
    line.textContent = String(i);
    gutter.appendChild(line);
  }
}

function initSidebarHighlight() {
  const sidebar = document.querySelector(".docs-sidebar");
  if (!sidebar) return;

  sidebar.querySelectorAll("a").forEach((a) => a.classList.remove("sidebar-current"));

  const page = location.pathname.split("/").pop() || "index.html";
  const hash = location.hash;
  const links = [...sidebar.querySelectorAll("a")];

  let current = null;
  if (hash) {
    current = links.find((a) => {
      try {
        const u = new URL(a.href);
        return u.pathname.split("/").pop() === page && u.hash === hash;
      } catch {
        return false;
      }
    });
  }
  if (!current) {
    current = links.find((a) => {
      try {
        const u = new URL(a.href);
        return u.pathname.split("/").pop() === page && !u.hash;
      } catch {
        return false;
      }
    });
  }
  if (current) current.classList.add("sidebar-current");
}

window.addEventListener("hashchange", initSidebarHighlight);

function highlightAllCode() {
  if (typeof Prism === "undefined") return;
  document.querySelectorAll("pre code[class*='language-']").forEach((el) => {
    Prism.highlightElement(el);
    syncLineNumbers(el.closest("pre"));
  });
}

function mountHomeCode(key) {
  const sample = CODE_SAMPLES[key] || CODE_SAMPLES.hello;
  const pre = document.getElementById("code-display");
  const langLabel = document.getElementById("code-lang");
  if (!pre) return;

  pre.className = `language-${sample.lang}`;
  pre.innerHTML = "";
  const code = document.createElement("code");
  code.className = `language-${sample.lang}`;
  code.textContent = sample.code;
  pre.appendChild(code);

  if (langLabel) langLabel.textContent = sample.label;

  if (typeof Prism !== "undefined") {
    Prism.highlightElement(code);
  }
  syncLineNumbers(pre);

  return sample.code;
}

document.addEventListener("DOMContentLoaded", () => {
  const toggle = document.querySelector(".nav-toggle");
  const nav = document.querySelector(".top-nav");
  if (toggle && nav) {
    toggle.addEventListener("click", () => nav.classList.toggle("open"));
  }

  initSidebarHighlight();
  highlightAllCode();

  const tabs = document.querySelectorAll(".code-tab");
  const copyBtn = document.querySelector(".copy-btn");
  let activeCode = CODE_SAMPLES.hello.code;

  if (document.getElementById("code-display")) {
    activeCode = mountHomeCode("hello") || activeCode;
  }

  tabs.forEach((tab) => {
    tab.addEventListener("click", () => {
      tabs.forEach((t) => t.classList.remove("active"));
      tab.classList.add("active");
      activeCode = mountHomeCode(tab.dataset.sample) || activeCode;
    });
  });

  if (copyBtn) {
    copyBtn.addEventListener("click", () => {
      navigator.clipboard.writeText(activeCode).then(() => {
        copyBtn.textContent = "Copied!";
        setTimeout(() => { copyBtn.textContent = "Copy to clipboard"; }, 1500);
      });
    });
  }

  const path = location.pathname.split("/").pop() || "index.html";
  document.querySelectorAll(".top-nav .nav-link").forEach((a) => {
    const href = a.getAttribute("href") || "";
    if (href.endsWith(path) || (path === "" && href.endsWith("index.html"))) {
      a.classList.add("active");
    }
  });
});
