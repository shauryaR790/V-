/** Course hub filter tabs and card grid. */
(function () {
  function applyFilter(buttons, cards, filter) {
    buttons.forEach((btn) => {
      btn.classList.toggle("active", (btn.dataset.filter || "all") === filter);
    });
    cards.forEach((card) => {
      const level = card.dataset.level || "";
      const show = filter === "all" || level === filter;
      card.hidden = !show;
    });
  }

  function initCourseFilters() {
    const hub = document.querySelector(".courses-hub-inner");
    if (!hub) return;

    const buttons = [...document.querySelectorAll(".courses-filter-btn")];
    const cards = [...document.querySelectorAll(".course-card")];
    if (!buttons.length || !cards.length) return;

    const filterFromHash = () => {
      const hash = location.hash.replace("#", "");
      if (hash === "courses-beginner") return "beginner";
      if (hash === "courses-intermediate") return "intermediate";
      if (hash === "courses-advanced") return "advanced";
      return null;
    };

    buttons.forEach((btn) => {
      btn.addEventListener("click", () => {
        applyFilter(buttons, cards, btn.dataset.filter || "all");
      });
    });

    const hashFilter = filterFromHash();
    if (hashFilter) {
      applyFilter(buttons, cards, hashFilter);
    }

    window.addEventListener("hashchange", () => {
      const next = filterFromHash();
      if (next) applyFilter(buttons, cards, next);
    });
  }

  document.addEventListener("DOMContentLoaded", initCourseFilters);
})();
