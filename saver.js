export class GuacaConfig {
  constructor() {
    this.lsa = try_storage();
    let lsd = localStorage.length;
    this.url = (this.lsa && lsd == 0)? true : false;
  }

  // config's config interface

  islsa() { return this.lsa; }
  isurl() { return this.url; }

  toggle_ls(sel_ls) {
    if (sel_ls) {
      if (this.url) {
        this.url = false;
        this.lsa = try_storage();
        if (this.lsa) {
          localStorage.setItem("cfgs", location.hash);
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
      return {"has": cfgs.length > 0, "url": true, "cfgs": cfgs};
    } else {
      let cfgs = localStorage.getItem("cfgs").split("=");
      let names = localStorage.getItem("names");
      if (names == null) {
        this.url = true;
        location.hash = cfgs;
        localStorage.clear();
        return {"has": cfgs > 0, "url": true, "cfgs": cfgs};
      } else {
        return {
          "has": cfgs.length > 0, "url": false,
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
