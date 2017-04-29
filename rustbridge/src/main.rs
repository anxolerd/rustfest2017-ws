extern crate pencil;
extern crate pick_one;
extern crate motivations;


use pencil::Pencil;
use pencil::{Request, PencilResult, Response};
use pencil::method::Get;


fn get_motivation() -> &'static str {
    return pick_one::pick_one_str(&motivations::MOTIVATIONS);
}


fn motivation(_: &mut Request) -> PencilResult {
    let motivation = get_motivation();
    return Ok(Response::from(motivation));
}


fn hello_name(r: &mut Request) -> PencilResult {
    let name = r.view_args.get("name").unwrap();
    return Ok(format!("Hello, {}!", name).into())
}


fn main() {
    let mut app = Pencil::new("/web/hello");
    app.route("/", &[Get], "motivation", motivation);
    app.route("/hello/<name:string>", &[Get], "hello_name", hello_name);
    app.run("0.0.0.0:7878");
}
