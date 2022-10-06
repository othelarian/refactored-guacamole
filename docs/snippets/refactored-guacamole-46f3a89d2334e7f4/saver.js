export function get_timestamp() { return new Date().toLocaleString(); }

export class GuacaConfig {
  constructor() {
    this.history = [];
    this.url = (try_storage())? ((validate_storage())? false : true) : true;
    if ("serviceWorker" in navigator) {
      navigator.serviceWorker.register("service-worker.js");
    }
  }

  // config's config interface

  islsa() { return try_storage(); }
  isurl() { return this.url; }

  toggle_ls(sel_ls) {
    if (sel_ls) {
      if (this.url) {
        if (try_storage()) {
          this.url = false;
          localStorage.setItem("guaca_cfgs", location.hash.substring(1));
          location.hash = "";
          return true;
        } else { return false; }
      } else { return true; }
    } else {
      if (this.url) { return true; }
      else {
        let cfgs = localStorage.getItem("guaca_cfgs");
        this.clear_config();
        this.url = true;
        location.hash = cfgs;
        return true;
      }
    }
  }

  copy_url() {
    let cfg = location.href + ((this.url)? "" : localStorage.getItem("guaca_cfgs"));
    navigator.clipboard.writeText(cfg);
    window.alert("Url de configuration copiée !");
  }

  clear_url() { location.hash = ""; }

  merge_url() {
    let url_cfgs = location.hash.substring(1);
    localStorage.setItem("guaca_cfgs", localStorage.getItem("guaca_cfgs") + "=" + url_cfgs);
    let new_names = Array(url_cfgs.split("=").length).fill("");
    new_names = JSON.stringify(JSON.parse(localStorage.getItem("guaca_names")).concat(new_names));
    localStorage.setItem("guaca_names", new_names);
    this.clear_url();
  }

  set_url() {
    localStorage.setItem("guaca_cfgs", location.hash.substring(1));
    localStorage.setItem("guaca_names", '[""]');
    this.clear_url();
  }

  // config interface

  clear_config() {
    if (this.url) { location.hash = ""; } else {
      localStorage.removeItem("guaca_cfgs");
      localStorage.removeItem("guaca_names");
      localStorage.removeItem("guaca_history");
    }
  }

  has_config() {
    if (this.url) {
      let cfgs = location.hash.substring(1).split("=");
      return {"has": cfgs[0] != "", "url": true, "cfgs": cfgs};
    } else {
      let cfgs = localStorage.getItem("guaca_cfgs").split("=");
      let names = localStorage.getItem("guaca_names");
      if (names == null) {
        this.clear_config();
        this.url = true;
        location.hash = cfgs;
        return {"has": cfgs[0] != "", "url": true, "cfgs": cfgs};
      } else {
        this.history = JSON.parse(localStorage.getItem("guaca_history"));
        return {
          "has": cfgs[0] != "", "url": false,
          "cfgs": cfgs, "names": JSON.parse(names)
        };
      }
    }
  }

  update_config(cfgs) {
    if (this.url) { location.hash = cfgs.join("="); } else {
      localStorage.setItem("guaca_cfgs", cfgs.join("="));
    }
  }

  update_names(names) {
    if (!this.url) { localStorage.setItem("guaca_names", JSON.stringify(names)); }
  }

  check_db_cfg() {
    if (try_storage()) {
      let url_cfgs = location.hash.substring(1).split("=")[0] != "";
      let ls_cfgs = localStorage.getItem("guaca_cfgs");
      ls_cfgs = (ls_cfgs == null)? false : ls_cfgs.split("=")[0] != "";
      return url_cfgs && ls_cfgs;
    } else { return false; }
  }

  // history interface

  add_history(new_res) { this.history.push(new_res); this.update_history(); }

  clear_history() { this.history = []; this.update_history(); }

  copy_history(history) { this.history = history; this.update_history(); }

  get_history() {
    if (!this.url) {
      this.history = JSON.parse(localStorage.getItem("guaca_history")); }
    return this.history;
  }

  has_history() {
    return (try_storage() && localStorage.hasOwnProperty("guaca_history"))? true : false;
  }

  update_history() {
    if (!this.url) {
      localStorage.setItem("guaca_history", JSON.stringify(this.history)); }
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

function validate_storage() {
  return (
    localStorage.hasOwnProperty("guaca_cfgs")
    && localStorage.hasOwnProperty("guaca_names")
    && localStorage.hasOwnProperty("guaca_history")
  );
}
