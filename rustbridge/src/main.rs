extern crate pencil;


use pencil::Pencil;
use pencil::method::Get;

mod routes;


fn main() {
    let mut app = Pencil::new("/web/hello");

    app.route("/", &[Get], "motivation", routes::motivation);
    app.route("/hello/<name:string>", &[Get], "hello_name", routes::hello_name);

    let host = "0.0.0.0";
    let port = "7878";

    let address = format!("{}:{}", host, port);

    println!("* Running on http://{}/", address);
    app.run(address);
}
