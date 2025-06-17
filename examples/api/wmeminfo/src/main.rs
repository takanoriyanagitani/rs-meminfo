use std::io;
use std::process::ExitCode;

use axum::{
    Router,
    http::StatusCode,
    response::{self, IntoResponse},
    routing,
};

use rs_meminfo::MemoryInfo;

use wmeminfo::MemoryInfoService;
use wmeminfo::mem_info_svc_new;

async fn get_memory_info<S>(state: S) -> impl IntoResponse
where
    S: MemoryInfoService,
{
    let rminfo: Result<MemoryInfo, _> = state.get_mem_info().await;
    match rminfo {
        Ok(minfo) => (StatusCode::OK, response::Json(Some(minfo))),
        Err(e) => {
            eprintln!("{e}");
            (StatusCode::INTERNAL_SERVER_ERROR, response::Json(None))
        }
    }
}

async fn sub() -> Result<(), io::Error> {
    // MemoryInfoService + Clone
    let minf_svc = mem_info_svc_new().await;

    let app = Router::new().route(
        "/api/v1/meminfo",
        routing::get({
            let misvc = minf_svc.clone();
            move || get_memory_info(misvc)
        }),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:61781").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> ExitCode {
    sub().await.map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
