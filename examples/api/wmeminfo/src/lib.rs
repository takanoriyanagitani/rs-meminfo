use std::io;

use rs_meminfo::sysinfo;

use sysinfo::System;

use rs_meminfo::MemoryInfo;

use tokio::sync::mpsc::{Receiver, Sender};

pub struct Request {
    pub reply: Sender<MemoryInfo>,
}

pub async fn handle_requests(mut reqs: Receiver<Request>) {
    let mut sys: System = System::new();
    loop {
        let oreq: Option<Request> = reqs.recv().await;
        match oreq {
            None => return,
            Some(req) => {
                let reply: Sender<MemoryInfo> = req.reply;
                let minfo: MemoryInfo = MemoryInfo::from_sys(&mut sys);
                let res: Result<_, _> = reply.send(minfo).await;
                match res {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("could not send the reply: {e}");
                        continue;
                    }
                }
            }
        }
    }
}

#[async_trait::async_trait]
pub trait MemoryInfoService {
    async fn get_mem_info(&self) -> Result<MemoryInfo, io::Error>;
}

#[derive(Clone)]
pub struct MemInfoSvc {
    sender: Sender<Request>,
}

impl MemInfoSvc {
    pub async fn new() -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel(1);
        tokio::spawn(async move {
            handle_requests(rx).await;
        });
        Self { sender: tx }
    }
}

#[async_trait::async_trait]
impl MemoryInfoService for MemInfoSvc {
    async fn get_mem_info(&self) -> Result<MemoryInfo, io::Error> {
        let (tx, mut rx) = tokio::sync::mpsc::channel(1);
        let req: Request = Request { reply: tx };
        self.sender.send(req).await.map_err(io::Error::other)?;
        let omem: Option<MemoryInfo> = rx.recv().await;
        let mem: MemoryInfo = omem.ok_or(io::Error::other("no meminfo got"))?;
        Ok(mem)
    }
}

pub async fn mem_info_svc_new() -> impl MemoryInfoService + Clone {
    MemInfoSvc::new().await
}
