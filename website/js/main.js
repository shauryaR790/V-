const CODE_LINE_COUNT = 30;

const CODE_SAMPLES = {
  hello: {
    lang: "vpp",
    label: "hello.vpp",
    code: `// hello.vpp — run: vpp run hello.vpp
import std.io

fn greet(name: string) -> string {
    return "Welcome, " + name
}

fn add(a: int, b: int) -> int {
    return a + b
}

fn multiply(a: int, b: int) -> int {
    return a * b
}

fn sum_range(start: int, end: int) -> int {
    let mut total = 0
    for i in start..end {
        total = total + i
    }
    return total
}

fn main() -> int {
    print(greet("developer"))
    print(add(10, 20))
    print(multiply(3, 7))
    print(sum_range(0, 5))
    return 0
}`,
  },
  native: {
    lang: "vpp",
    label: "user.vpp",
    code: `import std.io

struct User {
    name: string
    age: int
    active: bool
}

enum Role {
    Dev
    Designer
    User
}

fn describe(user: User) -> string {
    return user.name + " (" + user.age + ")"
}

fn is_adult(user: User) -> bool {
    return user.age >= 18
}

fn main() -> int {
    let user = User {
        name: "Shaurya"
        age: 18
        active: true
    }
    print(describe(user))
    print(is_adult(user))
    return 0
}`,
  },
  build: {
    lang: "bash",
    label: "terminal",
    code: `# Compile to native .exe (requires LLVM/clang)
vpp build app.vpp -o app.exe
./app.exe

# Interpret without building
vpp run app.vpp
vpp check app.vpp
vpp fmt app.vpp

# Project workflow
vpp new myapp --path myapp
cd myapp
vpp run
vpp test
vpp build src/main.vpp -o bin/app.exe

# Doctor + toolchain
vpp doctor
vpp --version

# Release build with LLVM path (Windows)
$env:LLVM_SYS_221_PREFIX = "C:\\Program Files\\LLVM"
vpp build app.vpp -o app.exe`,
  },
  test: {
    lang: "vpp",
    label: "tests.vpp",
    code: `fn add(a: int, b: int) -> int {
    return a + b
}

fn subtract(a: int, b: int) -> int {
    return a - b
}

fn clamp(value: int, min: int, max: int) -> int {
    if value < min { return min }
    if value > max { return max }
    return value
}

test "addition works" {
    assert_eq(add(2, 3), 5)
}

test "subtraction works" {
    assert_eq(subtract(10, 4), 6)
}

test "clamp lower bound" {
    assert_eq(clamp(-5, 0, 10), 0)
}

test "clamp upper bound" {
    assert_eq(clamp(99, 0, 10), 10)
}`,
  },
  project: {
    lang: "toml",
    label: "vpp.toml",
    code: `name = "my-app"
version = "0.1.0"
entry = "src/main.vpp"
authors = ["Shaurya"]

[dependencies]
hello-lib = "0.1.0"
json-utils = "0.2.1"

[dev-dependencies]
test-helpers = "0.1.0"

[profile.release]
opt = true
debug = false

[profile.dev]
opt = false
debug = true

# Run: vpp run
# Test: vpp test
# Build: vpp build src/main.vpp -o my-app.exe`,
  },
};

function padToLineCount(code, target) {
  const lines = code.split("\n");
  if (lines.length > 1 && lines[lines.length - 1] === "") lines.pop();
  while (lines.length < target) {
    lines.push("");
  }
  return lines.slice(0, target).join("\n");
}

function countCodeLines(text) {
  if (!text) return 0;
  const parts = text.split("\n");
  if (parts.length > 1 && parts[parts.length - 1] === "") parts.pop();
  return parts.length;
}

function syncLineNumbers(pre, lineCount) {
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
  pre.dataset.lineCount = String(lineCount);

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

  sidebar.querySelectorAll(".tree-link").forEach((a) => a.classList.remove("sidebar-current"));

  const page = location.pathname.split("/").pop() || "index.html";
  const hash = location.hash;
  const links = [...sidebar.querySelectorAll(".tree-link")];

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

function initSidebarSearch() {
  const input = document.querySelector(".tree-search");
  const sidebar = document.querySelector(".sidebar-tree");
  if (!input || !sidebar) return;

  input.addEventListener("input", () => {
    const q = input.value.trim().toLowerCase();
    sidebar.querySelectorAll(".tree-folder").forEach((folder) => {
      let anyVisible = false;
      folder.querySelectorAll(".tree-list .tree-link").forEach((link) => {
        const li = link.closest("li");
        const match = !q || link.textContent.toLowerCase().includes(q);
        if (li) li.hidden = !match;
        if (match) anyVisible = true;
      });
      const label = folder.querySelector(".tree-folder-label");
      const labelMatch = label && label.textContent.toLowerCase().includes(q);
      folder.hidden = Boolean(q) && !anyVisible && !labelMatch;
      if (q && (anyVisible || labelMatch)) folder.open = true;
    });
  });
}

function initTocHighlight() {
  const toc = document.querySelector(".toc-tree");
  if (!toc) return;

  const links = [...toc.querySelectorAll(".tree-link")];
  if (!links.length) return;

  const setActive = (id) => {
    links.forEach((a) => {
      a.classList.toggle("toc-current", a.getAttribute("href") === `#${id}`);
    });
  };

  const headings = links
    .map((a) => {
      const id = a.getAttribute("href")?.slice(1);
      return id ? document.getElementById(id) : null;
    })
    .filter(Boolean);

  if (headings.length) {
    const observer = new IntersectionObserver(
      (entries) => {
        const visible = entries
          .filter((e) => e.isIntersecting)
          .sort((a, b) => b.intersectionRatio - a.intersectionRatio);
        if (visible.length) setActive(visible[0].target.id);
      },
      { rootMargin: "-15% 0px -65% 0px", threshold: [0, 0.1, 0.5] },
    );
    headings.forEach((h) => observer.observe(h));
  }

  const syncHash = () => {
    const id = location.hash.slice(1);
    if (id) setActive(id);
  };
  window.addEventListener("hashchange", syncHash);
  syncHash();
}

window.addEventListener("hashchange", initSidebarHighlight);

function highlightAllCode() {
  if (typeof Prism === "undefined") return;
  document.querySelectorAll("pre code[class*='language-']").forEach((el) => {
    Prism.highlightElement(el);
    const pre = el.closest("pre");
    syncLineNumbers(pre, countCodeLines(el.textContent));
  });
}

function mountHomeCode(key) {
  const sample = CODE_SAMPLES[key] || CODE_SAMPLES.hello;
  const pre = document.getElementById("code-display");
  const langLabel = document.getElementById("code-lang");
  if (!pre) return;

  const padded = padToLineCount(sample.code, CODE_LINE_COUNT);

  pre.className = `language-${sample.lang} home-code-pre`;
  pre.innerHTML = "";
  const code = document.createElement("code");
  code.className = `language-${sample.lang}`;
  code.textContent = padded;
  pre.appendChild(code);

  if (langLabel) langLabel.textContent = sample.label;

  if (typeof Prism !== "undefined") {
    Prism.highlightElement(code);
  }
  syncLineNumbers(pre, CODE_LINE_COUNT);

  return padded;
}

document.addEventListener("DOMContentLoaded", () => {
  const toggle = document.querySelector(".nav-toggle");
  const nav = document.querySelector(".top-nav");
  if (toggle && nav) {
    toggle.addEventListener("click", () => nav.classList.toggle("open"));
  }

  initSidebarHighlight();
  initSidebarSearch();
  initTocHighlight();
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
