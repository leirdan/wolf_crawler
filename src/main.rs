use std::collections::HashSet;

use anyhow::Result;
use html_parser::{Dom, Node};
use log::{debug, info};

struct Wolfie {
    visited_nodes: HashSet<String>,
}

impl Wolfie {
    pub fn new() -> Self {
        Wolfie {
            visited_nodes: HashSet::new(),
        }
    }

    /// Crawls into a url, parses the HTML and retrieve all the links on it
    pub async fn howl_to_url(&mut self, url: &str) -> Result<()> {
        let howl = reqwest::get(url).await?.text().await?;
        let dom = Dom::parse(&howl)?;
        let _ = self.howl_nodes(dom.children);

        Ok(())
    }

    /// Recursively explores each node of the page and stores the links
    fn howl_nodes(&mut self, nodes: Vec<Node>) -> Result<()> {
        for node in nodes {
            match node {
                Node::Text(_) => {
                    debug!("found text");
                }
                Node::Element(el) => {
                    debug!("found element: {} ", el.name);

                    if el.name == "a" {
                        match el.attributes.get("href") {
                            Some(link) => {
                                if let Some(text) = link {
                                    debug!("found link!: {}", text);
                                    self.visited_nodes.insert(text.to_string());
                                    }
                                }
                            }
                            None => debug!("this anchor doesn't has a link"),
                        }
                    }

                    self.howl_nodes(el.children)?;
                }
                Node::Comment(c) => {
                    debug!("ignoring comment: {}", c);
                }
            }
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    debug!("Making request...");

    let mut wolfie = Wolfie::new();
    let _ = wolfie
        .howl_to_url("https://en.uesp.net/wiki/Skyrim:Skyrim")
        .await?;

    info!(
        "These are all the retrieved links: {:?}",
        wolfie.visited_nodes
    );

    Ok(())
}
