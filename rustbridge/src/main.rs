extern crate pencil;

use pencil::Pencil;
use pencil::{Request, PencilResult, Response};
use pencil::method::Get;


fn main() {
    let mut app = Pencil::new("/web/hello");
    app.run("0.0.0.0:5000");
}
