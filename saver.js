export function get_timestamp() { return new Date().toLocaleString(); }

export class GuacaConfig {
  constructor() {
    this.url = false;
    this.history = [];
    if (try_storage() && localStorage.length == 0) { this.url = true; }
  }

  // config's config interface

  islsa() { return try_storage(); }
  isurl() { return this.url; }

  toggle_ls(sel_ls) {
    if (sel_ls) {
      if (this.url) {
        if (try_storage()) {
          this.url = false;
          localStorage.setItem("cfgs", location.hash.substring(1));
          location.hash = "";
          return true;
        } else { return false; }
      } else { return true; }
    } else {
      if (this.url) { return true; }
      else {
        this.url = true;
        let cfgs = localStorage.getItem("cfgs");
        location.hash = cfgs;
        localStorage.clear();
        return true;
      }
    }
  }

  // config interface

  clear_config() {
    if (this.url) { location.hash = ""; } else { localStorage.clear(); }
  }

  has_config() {
    if (this.url) {
      let cfgs = location.hash.substring(1).split("=");
      return {"has": cfgs[0] != "", "url": true, "cfgs": cfgs};
    } else {
      let cfgs = localStorage.getItem("cfgs").split("=");
      let names = localStorage.getItem("names");
      if (names == null) {
        this.url = true;
        location.hash = cfgs;
        localStorage.clear();
        return {"has": cfgs[0] != "", "url": true, "cfgs": cfgs};
      } else {
        this.history = JSON.parse(localStorage.getItem("history"));
        return {
          "has": cfgs[0] != "", "url": false,
          "cfgs": cfgs, "names": JSON.parse(names)
        };
      }
    }
  }

  update_config(cfgs) {
    if (this.url) { location.hash = cfgs.join("="); } else {
      localStorage.setItem("cfgs", cfgs.join("="));
    }
  }

  update_names(names) {
    if (!this.url) { localStorage.setItem("names", JSON.stringify(names)); }
  }

  // history interface

  add_history(new_res) { this.history.push(new_res); this.update_history(); }

  clear_history() { this.history = []; this.update_history(); }

  copy_history(history) { this.history = history; this.update_history(); }

  get_history() {
    if (!this.url) {
      this.history = JSON.parse(localStorage.getItem("history")); }
    return this.history;
  }

  update_history() {
    if (!this.url) {
      localStorage.setItem("history", JSON.stringify(this.history)); }
  }

  remove_history(id) { this.history.splice(id, 1); this.update_history(); }
}

function try_storage() {
  let tryit = "try_lstest";
  try {
    localStorage.setItem(tryit, tryit);
    localStorage.getItem(tryit);
    localStorage.removeItem(tryit);
    return true;
  } catch(e) { return false; }
}
