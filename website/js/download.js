/** Download hub — version/OS picker and one-click release links. */
(function () {
  const REPO = "shauryaR790/V-";
  const SOURCE_URL = "/V-/contribute.html";

  const PLATFORMS = {
    windows: {
      label: "Windows",
      formats: [
        {
          id: "installer",
          label: "Installer (.exe)",
          primary: true,
          file: (v) => `vpp-${v}-setup.exe`,
          lang: "powershell",
          filename: "terminal",
          install: (v) =>
            `# Run the downloaded vpp-${v}-setup.exe installer
# If SmartScreen appears: More info → Run anyway
# Then open a new terminal:
vpp run examples\\hello.vpp
vpp check examples\\hello.vpp
vpp --version
# Verify your install
vpp doctor`,
          info: "The Windows installer adds <code>vpp</code> to PATH automatically. Bundled LLVM is included for native builds.",
          pathNote: 'If <code>vpp</code> is not found after install, add the install folder to PATH:',
          showPath: true,
        },
        {
          id: "zip",
          label: "Portable (.zip)",
          file: (v) => `vpp-v${v}-windows-x64.zip`,
          lang: "powershell",
          filename: "terminal",
          install: (v) =>
            `# Extract vpp-v${v}-windows-x64.zip, then from that folder:
.\\GO.bat
vpp run examples\\hello.vpp
vpp check examples\\hello.vpp
vpp --version
# Verify your install
vpp doctor`,
          info: "Portable zip — extract anywhere. Run <code>GO.bat</code> or add the folder to PATH manually.",
        },
      ],
    },
    linux: {
      label: "Linux",
      formats: [
        {
          id: "tarball",
          label: "Linux x64 (.tar.gz)",
          primary: true,
          file: (v) => `vpp-v${v}-linux-x64.tar.gz`,
          lang: "bash",
          filename: "terminal",
          install: (v) =>
            `# Extract the downloaded tarball
tar -xzf vpp-v${v}-linux-x64.tar.gz
cd vpp-v${v}-linux-x64
./run.sh examples/hello.vpp
vpp check examples/hello.vpp
vpp --version
# Verify your install
vpp doctor`,
          info: "Linux x64 bundle — extract and run <code>./run.sh</code>. Add the folder to PATH for global use.",
        },
        {
          id: "source",
          label: "Build from source",
          source: true,
          lang: "bash",
          filename: "terminal",
          install: () =>
            `# Clone and build (requires Rust + LLVM 22)
git clone https://github.com/shauryaR790/V-.git
cd V-
cargo build --release --features codegen,lsp
./target/release/vpp --version
# Verify your install
./target/release/vpp doctor`,
          info: "Build from source when prebuilt bundles are unavailable for your distro.",
        },
      ],
    },
    macos: {
      label: "macOS",
      formats: [
        {
          id: "tarball",
          label: "Apple Silicon (.tar.gz)",
          primary: true,
          file: (v) => `vpp-v${v}-macos-arm64.tar.gz`,
          lang: "bash",
          filename: "terminal",
          install: (v) =>
            `# Extract the downloaded tarball
tar -xzf vpp-v${v}-macos-arm64.tar.gz
cd vpp-v${v}-macos-arm64
./run.sh examples/hello.vpp
vpp check examples/hello.vpp
vpp --version
# Verify your install
vpp doctor`,
          info: "macOS Apple Silicon bundle — extract and run <code>./run.sh</code>. Intel Macs: build from source.",
        },
        {
          id: "source",
          label: "Build from source",
          source: true,
          lang: "bash",
          filename: "terminal",
          install: () =>
            `# Clone and build (requires Rust + LLVM 22)
git clone https://github.com/shauryaR790/V-.git
cd V-
cargo build --release --features codegen,lsp
./target/release/vpp --version
# Verify your install
./target/release/vpp doctor`,
          info: "Build from source for Intel Macs or when you need a custom build.",
        },
      ],
    },
  };

  function detectOS() {
    const ua = navigator.userAgent.toLowerCase();
    const platform = (navigator.platform || "").toLowerCase();
    if (ua.includes("win") || platform.includes("win")) return "windows";
    if (ua.includes("mac") || platform.includes("mac")) return "macos";
    return "linux";
  }

  function releaseUrl(tag, filename) {
    return `https://github.com/${REPO}/releases/download/${tag}/${filename}`;
  }

  function releaseTag(version) {
    return version.startsWith("v") ? version : `v${version}`;
  }

  function $(id) {
    return document.getElementById(id);
  }

  function getFormat(os, formatId) {
    return PLATFORMS[os].formats.find((f) => f.id === formatId);
  }

  function populateFormats(osKey, preferredId) {
    const sel = $("dl-format");
    if (!sel) return null;
    sel.innerHTML = "";
    const formats = PLATFORMS[osKey].formats;
    formats.forEach((fmt) => {
      const opt = document.createElement("option");
      opt.value = fmt.id;
      opt.textContent = fmt.label;
      sel.appendChild(opt);
    });
    const pick = preferredId && formats.some((f) => f.id === preferredId)
      ? preferredId
      : (formats.find((f) => f.primary) || formats[0]).id;
    sel.value = pick;
    return pick;
  }

  function updateCodeBlock(wrapId, raw, lang, filename) {
    const wrap = $(wrapId);
    if (!wrap) return;
    const pre = wrap.querySelector("pre");
    const code = pre?.querySelector("code");
    if (!pre || !code) return;

    pre.className = `language-${lang}`;
    code.className = `language-${lang}`;
    code.textContent = raw;

    const fnEl = wrap.querySelector(".code-block-filename");
    if (fnEl) fnEl.textContent = filename;

    const finalized = typeof finalizeCodeText === "function" ? finalizeCodeText(raw, lang) : raw;
    if (typeof renderLineBasedCode === "function") {
      renderLineBasedCode(pre, finalized, lang, []);
    }

    const header = wrap.querySelector(".code-block-header");
    if (header && typeof attachCopyButton === "function") {
      const existing = header.querySelector(".code-copy-btn");
      if (existing) existing.remove();
      attachCopyButton(header, () => finalized);
    }
  }

  function updateUI() {
    const version = $("dl-version")?.value || "0.5.0";
    const osKey = $("dl-os")?.value || "windows";
    const formatId = $("dl-format")?.value;
    const tag = releaseTag(version);
    const platform = PLATFORMS[osKey];
    const fmt = getFormat(osKey, formatId);
    if (!fmt) return;

    const infoEl = $("dl-info");
    if (infoEl) infoEl.innerHTML = `<strong>Info</strong> ${fmt.info || ""}`;

    updateCodeBlock("dl-code-wrap", fmt.install(version), fmt.lang, fmt.filename);

    const pathWrap = $("dl-path-wrap");
    const codeNote = $("dl-code-note");
    if (fmt.showPath && pathWrap) {
      pathWrap.hidden = false;
      if (codeNote) {
        codeNote.hidden = false;
        codeNote.innerHTML = fmt.pathNote || "";
      }
      updateCodeBlock(
        "dl-path-wrap",
        `$dir = "$env:LOCALAPPDATA\\Programs\\vpp"\n[Environment]::SetEnvironmentVariable("Path", $env:Path + ";$dir;$dir\\llvm\\bin", "User")\n# Restart terminal after updating PATH\nvpp --version\nvpp doctor`,
        "powershell",
        "terminal",
      );
    } else {
      if (pathWrap) pathWrap.hidden = true;
      if (codeNote) codeNote.hidden = true;
    }

    const primary = $("dl-primary");
    const primaryLabel = $("dl-primary-label");
    const secondary = $("dl-secondary");
    const secondaryLabel = $("dl-secondary-label");

    if (fmt.source) {
      if (primary) {
        primary.href = SOURCE_URL;
        primary.removeAttribute("download");
      }
      if (primaryLabel) primaryLabel.textContent = "Build from source";
      if (secondary) secondary.hidden = true;
    } else {
      const filename = fmt.file(version);
      const url = releaseUrl(tag, filename);
      if (primary) {
        primary.href = url;
        primary.setAttribute("download", filename);
      }
      if (primaryLabel) primaryLabel.textContent = `Download ${filename}`;

      const altFmt = platform.formats.find((f) => f.id !== fmt.id && !f.source);
      if (altFmt && secondary && secondaryLabel) {
        const altName = altFmt.file(version);
        secondary.href = releaseUrl(tag, altName);
        secondary.hidden = false;
        secondary.setAttribute("download", altName);
        secondaryLabel.textContent = altFmt.label;
      } else if (secondary) {
        secondary.hidden = true;
      }
    }

    const detectEl = $("dl-detect");
    const detected = detectOS();
    if (detectEl) {
      if (osKey === detected) {
        detectEl.textContent = `Recommended for your system (${platform.label}).`;
      } else {
        detectEl.innerHTML =
          `Detected ${PLATFORMS[detected].label}. `
          + `<button type="button" class="dl-detect-link" data-os="${detected}">Switch to ${PLATFORMS[detected].label}</button>`;
      }
    }
  }

  function initDownloadHub() {
    const hub = $("download-hub");
    if (!hub) return;

    const detected = detectOS();
    const osSel = $("dl-os");
    if (osSel) osSel.value = detected;

    populateFormats(osSel?.value || "windows");
    updateUI();

    ["dl-version", "dl-os", "dl-format"].forEach((id) => {
      $(id)?.addEventListener("change", () => {
        if (id === "dl-os") populateFormats($("dl-os").value);
        updateUI();
      });
    });

    hub.addEventListener("click", (e) => {
      const btn = e.target.closest(".dl-detect-link");
      if (!btn) return;
      const os = btn.dataset.os;
      if (osSel && os) {
        osSel.value = os;
        populateFormats(os);
        updateUI();
      }
    });
  }

  document.addEventListener("DOMContentLoaded", initDownloadHub);
})();
