use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use cmd::run::RunArgs;
use std::thread;
use actix_cors::Cors;
use std::time::Duration;
mod cmd;


async fn run_cast_command(tx: web::Path<String>) -> Result<String, String> {
    let args = RunArgs::new(
        "https://api-archived.roninchain.com/rpc".to_string(),
        tx.as_str().to_string()
    );

    // Run on another thread
    let handle = thread::spawn(move || {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async { args.run().await })
    });

    let trace_str = handle.join().unwrap();

    match trace_str {
        Ok(trace_str) => Ok(trace_str),
        Err(e) => Err(format!("Transaction trace failed: {}", e)),
    }
}

#[get("/run_cast/{tx}")]
async fn cast_command_handler(tx: web::Path<String>) -> impl Responder {
    match run_cast_command(tx).await {
        Ok(output) => {
            HttpResponse::Ok().content_type("text/plain").body(output)
        },
        Err(e) => {
            println!("Error response: {}", e);
            HttpResponse::InternalServerError().body(format!("Error executing command: {}", e))
        },
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new().wrap(cors).service(cast_command_handler)
    })
    .keep_alive(Duration::from_secs(600))
    .bind("127.0.0.1:8080")?
    .workers(5)
    .run()
    .await
}