extern crate rustc_serialize;
#[macro_use]
extern crate nickel;

use std::fs;
use std::collections::HashMap;
use nickel::{Nickel, StaticFilesHandler};

#[derive(RustcEncodable)]
struct IndexModel {
  files: Vec<HashMap<&'static str, String>>,
}

fn main() {
  let mut server = Nickel::new();

  server.utilize(StaticFilesHandler::new("assets/"));

  server.utilize(router! {
    get "/" => |_req, mut _res| {
      let motion_dir = "/var/tmp/motion"; // TODO: fix hard cord
      let mut v = vec![];
      for entry in fs::read_dir(motion_dir).unwrap() {
        let file = entry.unwrap().file_name().to_str().unwrap().to_string();
        println!("{}", file);

        let mut map: HashMap<&str, String> = HashMap::new();
        map.insert(&"jpeg", file);
        v.push(map);
      }
      let data = IndexModel {
        files: v,
      };
      return _res.render("assets/index.tpl", &data);
    }
  });

  server.listen("127.0.0.1:6767");
}