const CODE_LINE_COUNT = 27;

const CODE_SAMPLES = {
  hello: {
    lang: "vpp",
    label: "hello.vpp",
    code: `// hello.vpp: run vpp run hello.vpp
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

const COMMENT_LINES = {
  bash: [
    "# Verify install: vpp doctor",
    "# Restart the terminal after PATH changes",
    "# Type-check: vpp check main.vpp",
    "# Format source: vpp fmt main.vpp",
    "# Native build: vpp build src/main.vpp -o app.exe",
  ],
  vpp: [
    "// Run: vpp run main.vpp",
    "// Type-check only: vpp check main.vpp",
    "// Format: vpp fmt main.vpp",
    "// Tests: vpp test",
    "// vpp calls main() when defined",
  ],
  toml: [
    "# Package manifest for vpp run / vpp test / vpp build",
    "# Add dependencies with: vpp add <name> --version <ver>",
  ],
};

const MIN_DOC_CODE_LINES = 5;

function finalizeCodeText(text, lang) {
  const pool = COMMENT_LINES[lang] || COMMENT_LINES.bash;
  let lines = text.split("\n").filter((l) => l.trim());
  let ci = 0;
  while (lines.length < MIN_DOC_CODE_LINES) {
    lines.push(pool[ci % pool.length]);
    ci += 1;
  }
  return lines.join("\n");
}

function padToLineCount(code, target, lang = "vpp") {
  const pool = COMMENT_LINES[lang] || COMMENT_LINES.vpp;
  const lines = code.split("\n");
  if (lines.length > 1 && lines[lines.length - 1] === "") lines.pop();
  let ci = 0;
  while (lines.length < target) {
    lines.push(pool[ci % pool.length]);
    ci += 1;
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

  const article = document.querySelector(".docs-article");
  const pairs = links
    .map((link) => {
      const id = link.getAttribute("href")?.slice(1);
      if (!id) return null;
      const heading = document.getElementById(id);
      return heading ? { link, heading } : null;
    })
    .filter(Boolean);

  if (!pairs.length) return;

  const setActive = (activeHeading) => {
    pairs.forEach(({ link, heading }) => {
      link.classList.toggle("toc-current", heading === activeHeading);
    });
  };

  const observer = new IntersectionObserver(
    (entries) => {
      const visible = entries
        .filter((e) => e.isIntersecting)
        .sort((a, b) => a.boundingClientRect.top - b.boundingClientRect.top);
      if (visible.length) setActive(visible[0].target);
    },
    { rootMargin: "-12% 0px -68% 0px", threshold: [0, 0.25, 0.5, 1] },
  );
  pairs.forEach(({ heading }) => observer.observe(heading));

  const syncHash = () => {
    const id = location.hash.slice(1);
    if (!id) return;
    const match = pairs.find(({ heading }) => heading.id === id);
    if (match) setActive(match.heading);
  };
  window.addEventListener("hashchange", syncHash);
  syncHash();
}

window.addEventListener("hashchange", initSidebarHighlight);

function escapeHtml(text) {
  return text
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");
}

function parseHighlightLines(value) {
  if (!value) return [];
  return value.split(",").map((n) => parseInt(n.trim(), 10)).filter((n) => !Number.isNaN(n));
}

function getCodeLanguage(codeEl) {
  const match = [...codeEl.classList].find((c) => c.startsWith("language-"));
  return match ? match.slice(9) : "text";
}

function resolveCodeLang(lang) {
  const aliases = {
    powershell: "bash",
    shell: "bash",
    sh: "bash",
    text: "bash",
    terminal: "bash",
  };
  const resolved = aliases[lang] || lang;
  if (typeof Prism !== "undefined" && Prism.languages[resolved]) return resolved;
  if (typeof Prism !== "undefined" && Prism.languages.bash) return "bash";
  return lang;
}

function renderLineBasedCode(pre, rawText, lang, highlightLines = []) {
  if (typeof Prism === "undefined") return false;

  const resolvedLang = resolveCodeLang(lang);
  const grammar = Prism.languages[resolvedLang];
  const lines = rawText.split("\n");
  if (lines.length > 1 && lines[lines.length - 1] === "") lines.pop();

  const rows = document.createElement("div");
  rows.className = "code-line-rows";

  lines.forEach((line, idx) => {
    const lineNo = idx + 1;
    const row = document.createElement("div");
    row.className = "code-line-row";
    if (highlightLines.includes(lineNo)) row.classList.add("code-line-highlight");

    const ln = document.createElement("span");
    ln.className = "code-ln";
    ln.textContent = String(lineNo);

    const content = document.createElement("span");
    content.className = "code-line-content";
    if (grammar && line.trim()) {
      content.innerHTML = Prism.highlight(line, grammar, resolvedLang);
    } else if (line.trim()) {
      content.textContent = line;
    } else {
      const filler = (COMMENT_LINES[lang] || COMMENT_LINES.bash)[0];
      content.innerHTML = grammar
        ? Prism.highlight(filler, grammar, lang)
        : filler;
    }

    row.appendChild(ln);
    row.appendChild(content);
    rows.appendChild(row);
  });

  pre.innerHTML = "";
  pre.classList.add("code-block-pre");
  pre.classList.remove("has-line-numbers");
  pre.appendChild(rows);
  return true;
}

function highlightAllCode() {
  document.querySelectorAll("pre code[class*='language-']").forEach((el) => {
    const pre = el.closest("pre");
    if (!pre || pre.closest(".code-block-wrap") || pre.id === "code-display") return;
    const lang = getCodeLanguage(el);
    renderLineBasedCode(pre, el.textContent || "", lang, parseHighlightLines(pre.dataset.highlightLines));
  });
}

const COPY_ICON = (
  '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" '
  + 'stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">'
  + '<rect width="14" height="14" x="8" y="8" rx="2" ry="2"/>'
  + '<path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/>'
  + "</svg>"
);

const CHECK_ICON = (
  '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" '
  + 'stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">'
  + '<path d="M20 6 9 17l-5-5"/>'
  + "</svg>"
);

function attachCopyButton(header, getText) {
  if (!header || header.querySelector(".code-copy-btn")) return;
  const btn = document.createElement("button");
  btn.type = "button";
  btn.className = "code-copy-btn";
  btn.setAttribute("aria-label", "Copy code");
  btn.innerHTML = COPY_ICON;
  btn.addEventListener("click", () => {
    const raw = getText().replace(/\s+$/, "");
    navigator.clipboard.writeText(raw).then(() => {
      btn.classList.add("copied");
      btn.innerHTML = CHECK_ICON;
      setTimeout(() => {
        btn.classList.remove("copied");
        btn.innerHTML = COPY_ICON;
      }, 2000);
    });
  });
  header.appendChild(btn);
}

function initDocCodeBlocks() {
  document.querySelectorAll(".code-block-wrap").forEach((wrap) => {
    const pre = wrap.querySelector("pre");
    const code = pre?.querySelector("code");
    if (!pre || !code) return;

    const lang = getCodeLanguage(code);
    const raw = finalizeCodeText(code.textContent || "", lang);
    const highlights = parseHighlightLines(pre.dataset.highlightLines);
    renderLineBasedCode(pre, raw, lang, highlights);

    const header = wrap.querySelector(".code-block-header");
    attachCopyButton(header, () => raw);
  });
}

function mountHomeCode(key) {
  const sample = CODE_SAMPLES[key] || CODE_SAMPLES.hello;
  const pre = document.getElementById("code-display");
  const langLabel = document.getElementById("code-lang");
  if (!pre) return;

  const padded = padToLineCount(sample.code, CODE_LINE_COUNT, sample.lang);

  pre.className = `language-${sample.lang} home-code-pre code-block-pre`;
  renderLineBasedCode(pre, padded, sample.lang, []);

  if (langLabel) langLabel.textContent = sample.label;

  const filenameEl = document.getElementById("code-filename");
  if (filenameEl) filenameEl.textContent = sample.label;

  return padded;
}

function initFaqDeepLinks() {
  const openFromHash = () => {
    const id = location.hash.slice(1);
    if (!id) return;
    const item = document.getElementById(id);
    if (item instanceof HTMLDetailsElement && item.classList.contains("faq-item")) {
      item.open = true;
    }
  };
  window.addEventListener("hashchange", openFromHash);
  openFromHash();
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
  initFaqDeepLinks();
  highlightAllCode();
  initDocCodeBlocks();

  const tabs = document.querySelectorAll(".code-tab");
  const copyHeader = document.getElementById("code-copy-header");
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

  if (copyHeader) {
    copyHeader.addEventListener("click", () => {
      navigator.clipboard.writeText(activeCode).then(() => {
        copyHeader.classList.add("copied");
        const prev = copyHeader.innerHTML;
        copyHeader.innerHTML = CHECK_ICON;
        setTimeout(() => {
          copyHeader.classList.remove("copied");
          copyHeader.innerHTML = prev;
        }, 2000);
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
