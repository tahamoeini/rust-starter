use actix_web::{web, App, HttpResponse, HttpServer};

// The main function needs to be asynchronous and should use the #[actix_web::main] macro
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index)) // Set up the route
    });

    println!("Serving on http://localhost:3000...");
    server
        .bind("127.0.0.1:3000")? // Use the ? operator for error handling
        .run()
        .await // Await the server to run
}

// The handler function should return an HttpResponse and can use the Responder trait
async fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
            <title>GCD Calculator</title>
            <form action="/gcd" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">Compute GCD</button>
            </form>
            "#,
        )
}
