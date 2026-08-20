/** Simulated vpp run output for course playground. */
(function () {
  function typeOutput(el, text, onDone) {
    el.hidden = false;
    el.textContent = "";
    const lines = text.split("\n");
    let lineIdx = 0;
    let charIdx = 0;
    const tick = () => {
      if (lineIdx >= lines.length) {
        if (onDone) onDone();
        return;
      }
      const line = lines[lineIdx];
      if (charIdx <= line.length) {
        const chunk = lines.slice(0, lineIdx).join("\n");
        const prefix = chunk ? chunk + "\n" : "";
        el.textContent = prefix + line.slice(0, charIdx);
        charIdx += 1;
        requestAnimationFrame(tick);
      } else {
        lineIdx += 1;
        charIdx = 0;
        requestAnimationFrame(tick);
      }
    };
    requestAnimationFrame(tick);
  }

  function initCoursePlayground() {
    const dataEl = document.getElementById("course-playground-data");
    const playground = document.querySelector(".course-playground");
    if (!dataEl || !playground) return;

    let payload;
    try {
      payload = JSON.parse(dataEl.textContent || "{}");
    } catch {
      return;
    }

    const outputEl = playground.querySelector(".course-run-output");
    const runBtn = playground.querySelector(".course-run-btn");
    const resetBtn = playground.querySelector(".course-reset-btn");
    if (!outputEl || !runBtn) return;

    runBtn.addEventListener("click", () => {
      runBtn.disabled = true;
      runBtn.textContent = "Running…";
      if (resetBtn) resetBtn.hidden = true;
      typeOutput(outputEl, payload.output || "", () => {
        runBtn.disabled = false;
        runBtn.textContent = "Run program";
        if (resetBtn) resetBtn.hidden = false;
      });
    });

    if (resetBtn) {
      resetBtn.addEventListener("click", () => {
        outputEl.hidden = true;
        outputEl.textContent = "";
        resetBtn.hidden = true;
      });
    }
  }

  document.addEventListener("DOMContentLoaded", initCoursePlayground);
})();
