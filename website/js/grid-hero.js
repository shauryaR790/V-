/** Wavy perspective grid for home hero — fill grid cells on sides, then draw lines. */
(function () {
  const canvas = document.getElementById("home-grid-canvas");
  if (!canvas) return;

  const ctx = canvas.getContext("2d");
  if (!ctx) return;

  const GRID = 40;
  const GRID_LINE = "rgba(251, 219, 90, 0.28)";
  const SIDE_FILL = [
    "rgba(251, 219, 90, 0.08)",
    "rgba(251, 219, 90, 0.12)",
    "rgba(251, 219, 90, 0.16)",
    "rgba(251, 219, 90, 0.1)",
  ];

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

  /** Fill actual grid-aligned cells on left/right — same spacing as grid lines. */
  function drawGridCellFills(width, height) {
    const cols = Math.ceil(width / GRID) + 1;
    const rows = Math.ceil(height / GRID) + 1;
    const leftMaxCol = Math.ceil((width * 0.22) / GRID);
    const rightMinCol = Math.floor((width * 0.78) / GRID);

    const inSideZone = (col) => col < leftMaxCol || col >= rightMinCol;

    const zoneFade = (col) => {
      if (col < leftMaxCol) {
        return 0.45 + (col / Math.max(leftMaxCol, 1)) * 0.55;
      }
      if (col >= rightMinCol) {
        const span = Math.max(cols - rightMinCol, 1);
        return 0.45 + (1 - (col - rightMinCol) / span) * 0.55;
      }
      return 0;
    };

    const randLeft = seededRandom(7919);
    const randRight = seededRandom(12007);
    const used = Array.from({ length: rows }, () => Array(cols).fill(false));

    for (let r = 0; r < rows; r++) {
      for (let c = 0; c < cols; c++) {
        if (!inSideZone(c) || used[r][c]) continue;

        const rand = c < leftMaxCol ? randLeft : randRight;
        if (rand() > 0.5) continue;

        const maxW = Math.min(rand() > 0.55 ? 3 : 2, cols - c);
        const maxH = Math.min(rand() > 0.55 ? 3 : 2, rows - r);
        const wUnits = 1 + Math.floor(rand() * maxW);
        const hUnits = 1 + Math.floor(rand() * maxH);

        let ok = true;
        for (let dr = 0; dr < hUnits && ok; dr++) {
          for (let dc = 0; dc < wUnits; dc++) {
            const cc = c + dc;
            const rr = r + dr;
            if (!inSideZone(cc) || used[rr]?.[cc]) ok = false;
          }
        }
        if (!ok) continue;

        for (let dr = 0; dr < hUnits; dr++) {
          for (let dc = 0; dc < wUnits; dc++) {
            used[r + dr][c + dc] = true;
          }
        }

        const x = c * GRID;
        const y = r * GRID;
        const w = wUnits * GRID;
        const h = hUnits * GRID;

        ctx.save();
        ctx.globalAlpha = zoneFade(c);
        ctx.fillStyle = pickFill(rand);
        ctx.fillRect(x + 1, y + 1, w - 2, h - 2);
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
    drawGridCellFills(width, height);
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
