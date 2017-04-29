extern crate pencil;
extern crate pick_one;
extern crate motivations;

use pencil::{Request, PencilResult};
use std::collections::BTreeMap;


fn get_rand_motivation() -> String {
    return pick_one::pick_one_str(&motivations::MOTIVATIONS).to_string();
}


pub fn motivation(req: &mut Request) -> PencilResult {
    let mut context = BTreeMap::new();
    let motivation = get_rand_motivation();
    context.insert("motivation".to_string(), motivation);
    return req.app.render_template("motivation.html", &context);
}


pub fn hello_name(r: &mut Request) -> PencilResult {
    let name = r.view_args.get("name").unwrap();
    return Ok(format!("Hello, {}!", name).into())
}

