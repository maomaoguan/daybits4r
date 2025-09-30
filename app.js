(function () {
  "use strict";

  const state = {
    query: "",
    filters: {
      type: new Set(),
      date: new Set(),
    },
    sort: "relevance",
    get payload() {
      return {
        q: this.query,
        type: [...this.filters.type],
        date: [...this.filters.date],
        sort: this.sort,
      };
    },
  };

  const els = {
    q: document.getElementById("q"),
    clearInput: document.querySelector(".clear-input"),
    filtersBtn: document.getElementById("btn-filters"),
    filtersPanel: document.getElementById("filters-panel"),
    chipButtons: Array.from(document.querySelectorAll(".chip")),
    sort: document.getElementById("sort"),
    btnSearch: document.getElementById("btn-search"),
    btnApply: document.getElementById("btn-apply"),
    btnClear: document.getElementById("btn-clear"),
    active: document.querySelector(".active-filters"),
    stateOut: document.getElementById("state"),
  };

  // Util: ripple effect
  function attachRipples() {
    const rippleTargets = document.querySelectorAll("[data-ripple]");
    rippleTargets.forEach((el) => {
      el.addEventListener("click", function (e) {
        const rect = el.getBoundingClientRect();
        const size = Math.max(rect.width, rect.height);
        const ripple = document.createElement("span");
        ripple.className = "ripple";
        ripple.style.width = ripple.style.height = size + "px";
        const x = e.clientX - rect.left - size / 2;
        const y = e.clientY - rect.top - size / 2;
        ripple.style.left = x + "px";
        ripple.style.top = y + "px";
        el.appendChild(ripple);
        setTimeout(() => ripple.remove(), 650);
      });
    });
  }

  function updateActiveChips() {
    els.active.innerHTML = "";
    const makeChip = (filterKey, value) => {
      const chip = document.createElement("span");
      chip.className = "active-chip";
      chip.textContent = `${filterKey}: ${value}`;
      const btn = document.createElement("button");
      btn.className = "remove";
      btn.setAttribute("aria-label", `Remove ${filterKey} ${value}`);
      btn.textContent = "✕";
      btn.addEventListener("click", () => {
        toggleFilter(filterKey, value, false);
      });
      chip.appendChild(btn);
      els.active.appendChild(chip);
    };

    state.filters.type.forEach((v) => makeChip("type", v));
    state.filters.date.forEach((v) => makeChip("date", v));
  }

  function reflectChipButtons() {
    els.chipButtons.forEach((btn) => {
      const filter = btn.getAttribute("data-filter");
      const value = btn.getAttribute("data-value");
      const set = state.filters[filter];
      const isOn = set.has(value);
      btn.classList.toggle("on", isOn);
      btn.setAttribute("aria-pressed", String(isOn));
      if (isOn && !btn.querySelector(".tick")) {
        const tick = document.createElement("span");
        tick.className = "tick";
        btn.prepend(tick);
      }
      if (!isOn) {
        const tick = btn.querySelector(".tick");
        if (tick) tick.remove();
      }
    });
  }

  function toggleFilter(filterKey, value, toggle = true) {
    const set = state.filters[filterKey];
    if (!set) return;
    if (toggle) {
      if (set.has(value)) set.delete(value); else set.add(value);
    } else {
      set.delete(value);
    }
    reflectChipButtons();
    updateActiveChips();
    renderState();
  }

  function clearFilters() {
    state.filters.type.clear();
    state.filters.date.clear();
    reflectChipButtons();
    updateActiveChips();
    renderState();
  }

  function renderState() {
    els.stateOut.textContent = JSON.stringify(state.payload, null, 2);
  }

  function openFilters(open) {
    if (open) {
      els.filtersPanel.hidden = false;
      els.filtersBtn.setAttribute("aria-expanded", "true");
    } else {
      els.filtersPanel.hidden = true;
      els.filtersBtn.setAttribute("aria-expanded", "false");
    }
  }

  function wireEvents() {
    els.q.addEventListener("input", () => {
      state.query = els.q.value;
      renderState();
    });

    els.clearInput.addEventListener("click", () => {
      els.q.value = "";
      state.query = "";
      els.q.focus();
      renderState();
    });

    els.filtersBtn.addEventListener("click", () => {
      const open = els.filtersBtn.getAttribute("aria-expanded") !== "true";
      openFilters(open);
    });

    els.chipButtons.forEach((btn) => {
      btn.addEventListener("click", () => {
        const filter = btn.getAttribute("data-filter");
        const value = btn.getAttribute("data-value");
        toggleFilter(filter, value, true);
      });
    });

    els.sort.addEventListener("change", () => {
      state.sort = els.sort.value;
      renderState();
    });

    els.btnClear.addEventListener("click", () => {
      clearFilters();
    });

    els.btnApply.addEventListener("click", () => {
      // Could close panel or trigger a fetch
      openFilters(false);
      console.log("Apply filters", state.payload);
    });

    els.btnSearch.addEventListener("click", () => {
      // Simulate search action
      console.log("Search", state.payload);
      els.stateOut.animate([
        { transform: "scale(1)", filter: "brightness(1)" },
        { transform: "scale(1.01)", filter: "brightness(1.2)" },
        { transform: "scale(1)", filter: "brightness(1)" },
      ], { duration: 250, easing: "ease" });
    });

    document.addEventListener("keydown", (e) => {
      if (e.key === "Escape") openFilters(false);
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "k") {
        e.preventDefault(); els.q.focus();
      }
      if (e.key === "Enter" && document.activeElement === els.q) {
        els.btnSearch.click();
      }
    });
  }

  // Init
  attachRipples();
  wireEvents();
  reflectChipButtons();
  updateActiveChips();
  renderState();
})();

