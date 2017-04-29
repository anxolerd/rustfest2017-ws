extern crate pencil;

use pencil::Pencil;
use pencil::{Request, PencilResult, Response};
use pencil::method::Get;


fn hello(_: &mut Request) -> PencilResult {
    Ok(Response::from("Hello RustBridge!"))
}


fn hello_name(r: &mut Request) -> PencilResult {
    let name = r.view_args.get("name").unwrap();
    Ok(format!("Hello, {}!", name).into())
}


fn main() {
    let mut app = Pencil::new("/web/hello");
    app.route("/", &[Get], "hello", hello);
    app.route("/hello/<name:string>", &[Get], "hello_name", hello_name);
    app.run("0.0.0.0:5000");
}
