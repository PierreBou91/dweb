use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(studies_service))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

fn studies_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/studies")
            .route("", web::get().to(HttpResponse::NotImplemented))
            .route("/{study}", web::get().to(HttpResponse::NotImplemented))
            .route(
                "/{study}/metadata",
                web::get().to(HttpResponse::NotImplemented),
            )
            .route(
                "/{study}/rendered",
                web::get().to(HttpResponse::NotImplemented),
            )
            .route(
                "/{study}/series",
                web::get().to(HttpResponse::NotImplemented),
            )
            .route(
                "/{study}/series/{serie}",
                web::get().to(HttpResponse::NotImplemented),
            )
            .route(
                "/{study}/series/{serie}/metadata",
                web::get().to(HttpResponse::NotImplemented),
            )
            .route(
                "/{study}/series/{serie}/rendered",
                web::get().to(HttpResponse::NotImplemented),
            )
            .route(
                "/{study}/series/{serie}/instances",
                web::get().to(HttpResponse::NotImplemented),
            )
            .route(
                "/{study}/series/{serie}/instances/{instance}",
                web::get().to(HttpResponse::NotImplemented),
            )
            .route(
                "/{study}/series/{serie}/instances/{instance}/metadata",
                web::get().to(HttpResponse::NotImplemented),
            )
            .route(
                "/{study}/series/{serie}/instances/{instance}/rendered",
                web::get().to(HttpResponse::NotImplemented),
            )
            .route(
                "/{study}/series/{serie}/instances/{instance}/frames",
                web::get().to(HttpResponse::NotImplemented),
            )
            .route(
                "/{study}/series/{serie}/instances/{instance}/frames/{frames}",
                web::get().to(HttpResponse::NotImplemented),
            )
            .route(
                "/{study}/series/{serie}/instances/{instance}/frames/{frames}/metadata",
                web::get().to(HttpResponse::NotImplemented),
            )
            .route(
                "/{study}/series/{serie}/instances/{instance}/frames/{frames}/rendered",
                web::get().to(HttpResponse::NotImplemented),
            ),
    );
}
