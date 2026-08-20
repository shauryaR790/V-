/** Course hub filter tabs and card grid. */
(function () {
  function initCourseFilters() {
    const hub = document.querySelector(".courses-hub-inner");
    if (!hub) return;

    const buttons = [...hub.querySelectorAll(".courses-filter-btn")];
    const cards = [...hub.querySelectorAll(".course-card")];

    buttons.forEach((btn) => {
      btn.addEventListener("click", () => {
        buttons.forEach((b) => b.classList.remove("active"));
        btn.classList.add("active");
        const filter = btn.dataset.filter || "all";
        cards.forEach((card) => {
          const level = card.dataset.level || "";
          const show = filter === "all" || level === filter;
          card.hidden = !show;
        });
      });
    });
  }

  document.addEventListener("DOMContentLoaded", initCourseFilters);
})();
