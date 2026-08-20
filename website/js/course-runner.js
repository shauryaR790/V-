/** Course playground — runs vpp source in-browser (expected output + print interpreter). */
(function () {
  function normalizeSource(source) {
    return source.replace(/\r\n/g, "\n").trim();
  }

  function extractPrintCalls(source) {
    const results = [];
    const printRe = /print\s*\(/g;
    let match;
    while ((match = printRe.exec(source)) !== null) {
      let i = match.index + match[0].length;
      let depth = 1;
      let arg = "";
      while (i < source.length && depth > 0) {
        const ch = source[i];
        if (ch === "(") depth += 1;
        else if (ch === ")") depth -= 1;
        if (depth > 0) arg += ch;
        i += 1;
      }
      results.push(arg.trim());
    }
    return results;
  }

  function evalStringLiteral(expr) {
    const m = expr.match(/^"((?:\\.|[^"\\])*)"$|^'((?:\\.|[^'\\])*)'$/);
    if (!m) return null;
    const raw = m[1] !== undefined ? m[1] : m[2];
    return raw.replace(/\\n/g, "\n").replace(/\\t/g, "\t").replace(/\\"/g, '"').replace(/\\\\/g, "\\");
  }

  function evalIntLiteral(expr) {
    if (/^-?\d+$/.test(expr.trim())) return parseInt(expr.trim(), 10);
    return null;
  }

  function evalBoolLiteral(expr) {
    const t = expr.trim();
    if (t === "true") return "true";
    if (t === "false") return "false";
    return null;
  }

  function buildEnv(source) {
    const env = {};
    const letRe = /let\s+(?:mut\s+)?(\w+)\s*=\s*([^;\n]+)/g;
    let m;
    while ((m = letRe.exec(source)) !== null) {
      const val = evalExpr(m[2].trim(), env, source);
      if (val !== null) env[m[1]] = val;
    }
    return env;
  }

  function evalExpr(expr, env, source) {
    expr = expr.trim();
    if (!expr) return null;

    const str = evalStringLiteral(expr);
    if (str !== null) return str;

    const num = evalIntLiteral(expr);
    if (num !== null) return num;

    const bool = evalBoolLiteral(expr);
    if (bool !== null) return bool;

    if (env[expr] !== undefined) return env[expr];

    if (expr.includes("+")) {
      const parts = expr.split("+").map((p) => p.trim());
      if (parts.every((p) => p.length > 0)) {
        const vals = parts.map((p) => evalExpr(p, env, source));
        if (vals.every((v) => v !== null)) {
          if (vals.every((v) => typeof v === "number")) return vals.reduce((a, b) => a + b, 0);
          if (vals.every((v) => typeof v === "string")) return vals.join("");
        }
      }
    }

    if (expr.includes("-")) {
      const parts = expr.split("-").map((p) => p.trim());
      if (parts.length === 2) {
        const a = evalExpr(parts[0], env, source);
        const b = evalExpr(parts[1], env, source);
        if (typeof a === "number" && typeof b === "number") return a - b;
      }
    }

    if (expr.includes("*")) {
      const parts = expr.split("*").map((p) => p.trim());
      if (parts.length === 2) {
        const a = evalExpr(parts[0], env, source);
        const b = evalExpr(parts[1], env, source);
        if (typeof a === "number" && typeof b === "number") return a * b;
      }
    }

    const callMatch = expr.match(/^(\w+)\((.*)\)$/);
    if (callMatch) {
      const fn = callMatch[1];
      const args = callMatch[2].split(",").map((a) => a.trim()).filter(Boolean);
      const fnBodyMatch = new RegExp(`fn\\s+${fn}\\s*\\([^)]*\\)[^{]*\\{([\\s\\S]*?)\\n\\}`, "m").exec(source);
      if (fnBodyMatch && args.length > 0) {
        const localEnv = { ...env };
        const paramsMatch = new RegExp(`fn\\s+${fn}\\s*\\(([^)]*)\\)`).exec(source);
        if (paramsMatch) {
          const params = paramsMatch[1].split(",").map((p) => p.split(":")[0].trim()).filter(Boolean);
          params.forEach((name, idx) => {
            localEnv[name] = evalExpr(args[idx], env, source);
          });
        }
        const retMatch = /return\s+([^;\n]+)/.exec(fnBodyMatch[1]);
        if (retMatch) return evalExpr(retMatch[1].trim(), localEnv, source);
      }
    }

    return null;
  }

  function interpretPrints(source) {
    const env = buildEnv(source);
    const prints = extractPrintCalls(source);
    const lines = [];
    for (const arg of prints) {
      const val = evalExpr(arg, env, source);
      if (val === null) return null;
      lines.push(String(val));
    }
    return lines.join("\n");
  }

  function runSource(source, payload) {
    const normalized = normalizeSource(source);
    const original = normalizeSource(payload.source || "");
    if (normalized === original) return { ok: true, output: payload.output || "" };

    const interpreted = interpretPrints(source);
    if (interpreted !== null) return { ok: true, output: interpreted };

    return {
      ok: false,
      output:
        "Could not run edited code in the browser playground.\n" +
        "Install V++ locally and run: " +
        (payload.run_cmd || "vpp run main.vpp"),
    };
  }

  function setTerminalHtml(body, html) {
    body.innerHTML = html;
  }

  function initCoursePlayground() {
    const dataEl = document.getElementById("course-playground-data");
    const playground = document.querySelector(".course-playground");
    const sourceInput = document.querySelector(".course-source-input");
    if (!dataEl || !playground || !sourceInput) return;

    let payload;
    try {
      payload = JSON.parse(dataEl.textContent || "{}");
    } catch {
      return;
    }

    const terminalBody = playground.querySelector(".course-terminal-body");
    const runBtn = playground.querySelector(".course-run-btn");
    const resetBtn = playground.querySelector(".course-reset-btn");
    if (!terminalBody || !runBtn || !resetBtn) return;

    const originalSource = payload.source || "";
    sourceInput.value = originalSource;

    const idleHtml =
      '<div class="course-terminal-line course-terminal-muted">$ ready — click Test program</div>' +
      '<pre class="course-run-output"></pre>';

    const renderIdle = () => {
      setTerminalHtml(terminalBody, idleHtml);
    };

    renderIdle();

    runBtn.addEventListener("click", () => {
      runBtn.disabled = true;
      const cmd = payload.run_cmd || "vpp run main.vpp";
      setTerminalHtml(
        terminalBody,
        `<div class="course-terminal-line course-terminal-cmd">$ ${cmd}</div>` +
          '<div class="course-terminal-line course-terminal-muted">Running…</div>' +
          '<pre class="course-run-output"></pre>'
      );

      window.setTimeout(() => {
        const result = runSource(sourceInput.value, payload);
        const outputEl = terminalBody.querySelector(".course-run-output");
        const lines = terminalBody.querySelectorAll(".course-terminal-line");
        if (lines.length > 1) lines[1].remove();

        if (outputEl) {
          outputEl.textContent = result.output;
          if (!result.ok) outputEl.classList.add("course-terminal-err");
        }
        runBtn.disabled = false;
      }, 280);
    });

    resetBtn.addEventListener("click", () => {
      sourceInput.value = originalSource;
      renderIdle();
      runBtn.disabled = false;
    });
  }

  document.addEventListener("DOMContentLoaded", initCoursePlayground);
})();
