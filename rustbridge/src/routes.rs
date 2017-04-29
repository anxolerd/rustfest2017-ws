extern crate pencil;
extern crate pick_one;
extern crate motivations;

use pencil::{Request, PencilResult, Response};


fn get_motivation() -> &'static str {
    return pick_one::pick_one_str(&motivations::MOTIVATIONS);
}


pub fn motivation(_: &mut Request) -> PencilResult {
    let motivation = get_motivation();
    return Ok(Response::from(motivation));
}


pub fn hello_name(r: &mut Request) -> PencilResult {
    let name = r.view_args.get("name").unwrap();
    return Ok(format!("Hello, {}!", name).into())
}

