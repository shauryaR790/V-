/** Wavy perspective grid for home hero — grid lines + pixelated side fills. */
(function () {
  const canvas = document.getElementById("home-grid-canvas");
  if (!canvas) return;

  const ctx = canvas.getContext("2d");
  if (!ctx) return;

  const GRID = 40;
  const GRID_LINE = "rgba(251, 219, 90, 0.28)";
  const PIXEL = 22;
  const SIDE_FILL = [
    "rgba(251, 219, 90, 0.07)",
    "rgba(251, 219, 90, 0.11)",
    "rgba(251, 219, 90, 0.15)",
    "rgba(251, 219, 90, 0.09)",
  ];
  const PIXEL_EDGE = "rgba(251, 219, 90, 0.18)";

  function seededRandom(seed) {
    let s = seed >>> 0;
    return () => {
      s = (s * 1664525 + 1013904223) >>> 0;
      return s / 4294967296;
    };
  }

  function pickFill(rand) {
    return SIDE_FILL[Math.floor(rand() * SIDE_FILL.length)];
  }

  /** Non-uniform pixel blocks on left/right edges only. */
  function drawSidePixels(width, height) {
    const zoneW = Math.min(width * 0.2, 280);
    drawPixelZone(0, zoneW, height, 7919);
    drawPixelZone(width - zoneW, zoneW, height, 12007);
  }

  function drawPixelZone(zoneX, zoneW, height, seed) {
    const rand = seededRandom(seed);
    const cols = Math.ceil(zoneW / PIXEL);
    const rows = Math.ceil(height / PIXEL);
    const used = Array.from({ length: rows }, () => Array(cols).fill(false));

    for (let r = 0; r < rows; r++) {
      for (let c = 0; c < cols; c++) {
        if (used[r][c]) continue;
        if (rand() > 0.48) continue;

        const maxW = Math.min(rand() > 0.72 ? 4 : rand() > 0.4 ? 3 : 2, cols - c);
        const maxH = Math.min(rand() > 0.68 ? 4 : rand() > 0.38 ? 3 : 2, rows - r);
        const wUnits = 1 + Math.floor(rand() * maxW);
        const hUnits = 1 + Math.floor(rand() * maxH);

        let free = true;
        for (let dr = 0; dr < hUnits && free; dr++) {
          for (let dc = 0; dc < wUnits; dc++) {
            if (used[r + dr][c + dc]) free = false;
          }
        }
        if (!free) continue;

        for (let dr = 0; dr < hUnits; dr++) {
          for (let dc = 0; dc < wUnits; dc++) {
            used[r + dr][c + dc] = true;
          }
        }

        const gap = rand() > 0.65 ? 3 : 2;
        const x = zoneX + c * PIXEL + 1;
        const y = r * PIXEL + 1;
        const w = wUnits * PIXEL - gap;
        const h = hUnits * PIXEL - gap;

        const edgeDist = Math.min(c / cols, 1 - c / cols);
        const fade = 0.55 + edgeDist * 0.45;

        ctx.save();
        ctx.globalAlpha = fade;
        ctx.fillStyle = pickFill(rand);
        ctx.fillRect(x, y, w, h);
        ctx.strokeStyle = PIXEL_EDGE;
        ctx.lineWidth = 1;
        ctx.strokeRect(x + 0.5, y + 0.5, w - 1, h - 1);
        ctx.restore();
      }
    }
  }

  function drawGridLines(width, height) {
    ctx.strokeStyle = GRID_LINE;
    ctx.lineWidth = 1;

    const centerX = width / 2;
    const centerY = height / 2;

    for (let x = -GRID; x < width + GRID; x += GRID) {
      ctx.beginPath();
      for (let y = 0; y <= height; y += 2) {
        const dist = Math.hypot(x - centerX, y - centerY);
        const wave = Math.sin(dist * 0.02) * 20;
        const perspective = 1 - dist / (width * 0.8);
        const adjustedX = x + wave * Math.max(0, perspective);
        if (y === 0) ctx.moveTo(adjustedX, y);
        else ctx.lineTo(adjustedX, y);
      }
      ctx.stroke();
    }

    for (let y = -GRID; y < height + GRID; y += GRID) {
      ctx.beginPath();
      for (let x = 0; x <= width; x += 2) {
        const dist = Math.hypot(x - centerX, y - centerY);
        const wave = Math.sin(dist * 0.02) * 20;
        const perspective = 1 - dist / (height * 0.8);
        const adjustedY = y + wave * Math.max(0, perspective);
        if (x === 0) ctx.moveTo(x, adjustedY);
        else ctx.lineTo(x, adjustedY);
      }
      ctx.stroke();
    }
  }

  function draw(width, height) {
    ctx.clearRect(0, 0, width, height);
    drawSidePixels(width, height);
    drawGridLines(width, height);
  }

  function resize() {
    const dpr = window.devicePixelRatio || 1;
    const w = window.innerWidth;
    const h = window.innerHeight;
    canvas.width = w * dpr;
    canvas.height = h * dpr;
    canvas.style.width = `${w}px`;
    canvas.style.height = `${h}px`;
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    draw(w, h);
  }

  resize();
  window.addEventListener("resize", resize);
})();
