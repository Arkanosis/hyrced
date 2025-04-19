use actix_files::NamedFile;

use actix_web::{
    get,
    web::Data,
    App,
    HttpResponse,
    HttpServer,
    Responder,
    Result as WebResult,
};

use askama::Template;

pub fn version() -> &'static str {
    if env!("CARGO_PKG_VERSION").ends_with("-dev") {
        concat!(env!("CARGO_PKG_VERSION"), "+", env!("VERGEN_GIT_SHA"))
    } else {
        env!("CARGO_PKG_VERSION")
    }
}

struct AppState {

}

#[derive(Template)]
#[template(path = "index.htm")]
struct IndexTemplate<'a> {
    version: &'a str
}

#[get("/")]
async fn serve_index(_data: Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(IndexTemplate {
        version: version(),
    }.render().unwrap())
}

#[get("/logo.svg")]
async fn serve_logo(_data: Data<AppState>) -> WebResult<NamedFile> {
    Ok(NamedFile::open("static/logo.svg")?)
}


#[get("/favicon.ico")]
async fn serve_favicon(_data: Data<AppState>) -> WebResult<NamedFile> {
    Ok(NamedFile::open("static/favicon.ico")?)
}

#[get("/version")]
async fn serve_version(_data: Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(format!("Running hyrced v{}\n", version()))
}

#[actix_web::main]
pub async fn start(hostname: String, port: u16) -> std::io::Result<()> {
    let data = Data::new(AppState {

    });
    println!("Listening on {}:{}...", hostname, port);
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(serve_index)
            .service(serve_logo)
            .service(serve_favicon)
            .service(serve_version)
    })
        .bind((hostname, port))?
        .run()
        .await
}
