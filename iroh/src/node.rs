use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Result;
use futures::{future, StreamExt, TryStreamExt};
use iroh::bytes::store::flat;
use iroh::client::LiveEvent;
// use iroh::bytes::store::mem;
// use iroh::net::key::SecretKey;
use iroh::node;
use iroh::rpc_protocol::{BlobDownloadRequest, DocTicket, DownloadLocation, SetTagOption};
use iroh::sync::store::{fs, Query}; // memory,
use iroh::sync::ContentStatus;
use iroh::ticket::BlobTicket;
use iroh::util::path::IrohPaths;

// const DEF_RPC_PORT: u16 = 0x1337;

#[derive(Debug)]
pub struct Node {
    node: iroh::node::Node<flat::Store>,
    // node: iroh::node::Node<mem::Store>,
}

pub async fn start() -> Result<Node> {
    let repo = PathBuf::from("./data");
    let flat_dir = repo.join(IrohPaths::BaoFlatStoreDir);
    let docs_dir = repo.join(IrohPaths::DocsDatabase);

    let blob_store = flat::Store::load(&flat_dir).await?;
    let doc_store = fs::Store::new(docs_dir)?;
    // let blob_store = mem::Store::new();
    // let doc_store = memory::Store::default();

    let node = node::Node::builder(blob_store, doc_store).spawn().await?;

    Ok(Node { node })
}

impl Node {
    pub async fn join_doc(&self, tkt_str: &str) -> Result<()> {
        let iroh = self.node.client();

        let ticket = DocTicket::from_str(tkt_str)?;

        let Ok(doc) = iroh.docs.open(ticket.capability.id()).await else {
            return self.import_doc(ticket).await;
        };

        let Some(doc) = doc else {
            anyhow::bail!("Error opening doc");
        };

        let mut entries = doc.get_many(Query::single_latest_per_key()).await?;

        while let Some(entry) = entries.try_next().await? {
            let id = String::from_utf8(entry.key().to_owned())?;
            println!("Got todo: {}", id);

            // let Ok(entry) = iroh.blobs.read_to_bytes(entry.content_hash()).await else {
            //     anyhow::bail!("Error getting entry");
            // };
            // println!("Got entry: {:?}", entry);
        }

        Ok(())
    }

    async fn import_doc(&self, ticket: DocTicket) -> Result<()> {
        let iroh = self.node.client();

        let doc = iroh.docs.import(ticket).await?;

        let mut events = doc.subscribe().await?;
        let _ = tokio::spawn(async move {
            while let Some(Ok(event)) = events.next().await {
                match event {
                    LiveEvent::InsertRemote { content_status, .. } => {
                        // only update if we already have the content
                        if content_status == ContentStatus::Complete {
                            println!("insert remote");
                        }
                    }
                    LiveEvent::InsertLocal { .. } => {
                        println!("insert local");
                    }
                    LiveEvent::ContentReady { hash } => {
                        println!("content ready");
                        let bytes = iroh.blobs.read_to_bytes(hash).await.expect("should get bytes");
                        println!("Got bytes: {:?}", bytes.len());
                    }
                    _ => {}
                }
            }
        });

        Ok(())
    }

    pub async fn _download_blob(&self, ticket: &str) -> Result<()> {
        let iroh = self.node.client();

        let ticket = BlobTicket::from_str(ticket)?;
        let req = BlobDownloadRequest {
            hash: ticket.hash(),
            format: ticket.format(),
            peer: ticket.node_addr().clone(),
            tag: SetTagOption::Auto,
            out: DownloadLocation::Internal,
        };

        let stream = iroh.blobs.download(req).await?;
        let _ = stream
            .for_each(|item| {
                println!("Got item: {:?}", item);
                future::ready(())
            })
            .await;

        Ok(())
    }
}
