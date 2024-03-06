use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Result;
use futures::{future, StreamExt, TryStreamExt};
use iroh::bytes::store::flat;
// use iroh::bytes::store::mem;
// use iroh::net::key::SecretKey;
use iroh::node;
use iroh::rpc_protocol::{BlobDownloadRequest, DocTicket, DownloadLocation, SetTagOption};
use iroh::sync::store::{fs, Query}; // memory,
use iroh::ticket::BlobTicket;
use iroh::util::path::IrohPaths;

// const DEF_RPC_PORT: u16 = 0x1337;

#[derive(Debug)]
pub struct Node {
    // inner: iroh::node::Node<mem::Store>,
    inner: iroh::node::Node<flat::Store>,
}

pub async fn start() -> Result<Node> {
    let repo = PathBuf::from("./data");
    let flat_dir = repo.join(IrohPaths::BaoFlatStoreDir);
    let docs_dir = repo.join(IrohPaths::DocsDatabase);

    let blob_store = flat::Store::load(&flat_dir).await?;
    let doc_store = fs::Store::new(docs_dir)?;

    // let blob_store = mem::Store::new();
    // let doc_store = memory::Store::default();

    let inner = node::Node::builder(blob_store, doc_store).spawn().await?;

    Ok(Node { inner })
}

impl Node {
    pub async fn join_doc(&self, tkt_str: &str) -> Result<()> {
        let client = self.inner.client();

        let ticket = DocTicket::from_str(tkt_str)?;

        let doc = client.docs.import(ticket.clone()).await?;
        // let doc = match client.docs.open(ticket.capability.id()).await {
        //     Ok(Some(doc)) => doc,
        //     Err(_) => client.docs.import(ticket.clone()).await?,
        //     _ => anyhow::bail!("Error opening doc"),
        // };

        let mut entries = doc.get_many(Query::single_latest_per_key()).await?;

        while let Some(entry) = entries.try_next().await? {
            let id = String::from_utf8(entry.key().to_owned())?;
            println!("Got todo: {}", id);

            // let Ok(entry) = client.blobs.read_to_bytes(entry.content_hash()).await else {
            //     anyhow::bail!("Error getting entry");
            // };
            // println!("Got entry: {:?}", entry);
        }

        Ok(())
    }

    pub async fn _download_blob(&self, ticket: &str) -> Result<()> {
        let client = self.inner.client();

        let ticket = BlobTicket::from_str(ticket)?;
        let req = BlobDownloadRequest {
            hash: ticket.hash(),
            format: ticket.format(),
            peer: ticket.node_addr().clone(),
            tag: SetTagOption::Auto,
            out: DownloadLocation::Internal,
        };

        let stream = client.blobs.download(req).await?;
        let _ = stream
            .for_each(|item| {
                println!("Got item: {:?}", item);
                future::ready(())
            })
            .await;

        Ok(())
    }
}
