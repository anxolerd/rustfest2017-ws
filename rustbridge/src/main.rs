extern crate pencil;

use pencil::Pencil;
use pencil::{Request, PencilResult, Response};
use pencil::method::Get;


fn hello(_: &mut Request) -> PencilResult {
    Ok(Response::from("Hello RustBridge!"))
}


fn main() {
    let mut app = Pencil::new("/web/hello");
    app.route("/", &[Get], "hello", hello);
    app.run("0.0.0.0:5000");
}
