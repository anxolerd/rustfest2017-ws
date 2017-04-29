extern crate pencil;
use pencil::Pencil;


mod routes;


fn main() {
    let mut app = Pencil::new("./src");

    app.register_template("motivation.html");
    app.get("/", "motivation", routes::motivation);
    app.get("/hello/<name:string>", "hello_name", routes::hello_name);

    let host = "0.0.0.0";
    let port = "7878";

    let address = format!("{}:{}", host, port);

    println!("* Running on http://{}/", address);
    app.run(address);
}
