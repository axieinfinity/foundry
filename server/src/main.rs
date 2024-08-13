pub mod cmd;

use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use cmd::{chain::ChainId, run::RunArgs};
use std::{thread, time::Duration};

async fn run_cast_command(chain: ChainId, tx: String, quick: u64) -> Result<String, String> {
    let chain_info = chain.info();
    let quickly_option = match quick {
        1 => true,
        _ => false,
    };
    
    let args: RunArgs = RunArgs::new(chain_info.rpc_url.to_string(), tx, quickly_option);
    // Run on another thread
    let handle = thread::spawn(move || {
        tokio::runtime::Runtime::new().unwrap().block_on(async { args.run().await })
    });

    let trace_str = handle.join().unwrap();

    match trace_str {
        Ok(trace_str) => Ok(trace_str),
        Err(e) => Err(format!("Transaction trace failed: {}", e)),
    }
}

#[get("/run_cast/{chain_id}/{tx}")]
async fn cast_command_handler(path: web::Path<(u64, String, u64)>) -> impl Responder {
    let (chain_id, tx, quick) = path.into_inner();

    let chain = match ChainId::from_id(chain_id) {
        Some(chain) => chain,
        None => ChainId::RoninMainnet,
    };

    match run_cast_command(chain, tx, quick).await {
        Ok(output) => HttpResponse::Ok().content_type("text/plain").body(output),
        Err(e) => {
            println!("Error response: {}", e);
            HttpResponse::InternalServerError().body(format!("Error executing command: {}", e))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new().wrap(cors).service(cast_command_handler)
    })
    .keep_alive(Duration::from_secs(900))
    .bind("0.0.0.0:8080")?
    .workers(5)
    .run()
    .await
}
