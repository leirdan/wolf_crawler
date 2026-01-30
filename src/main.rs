use std::collections::HashSet;

use anyhow::Result;
use html_parser::{Dom, Node};
use log::{debug, info};
use url::Url;

struct Wolfie {
    visited_nodes: HashSet<String>,
    // links: HashMap<String, HashSet<String>>, TODO: later change to a hashmap to better storage
}

impl Wolfie {
    pub fn new() -> Self {
        Wolfie {
            visited_nodes: HashSet::new(),
        }
    }

    pub fn print(&self) {
        for node in self.visited_nodes.clone() {
            info!("- {}\n", node);
        }
    }

    /// Crawls into a url, parses the HTML and retrieve all the links on it
    pub async fn howl_to_url(&mut self, url: &str) -> Result<()> {
        let howl = reqwest::get(url).await?.text().await?;
        let dom = Dom::parse(&howl)?;

        let root_url = Url::parse(url).expect("invalid url :(");
        let root = format!(
            "{}://{}",
            root_url.scheme(),
            root_url.host_str().unwrap_or("")
        );

        self.howl_nodes(dom.children, &root)?;

        Ok(())
    }

    fn extract_url(&self, url: &str, root: &str) -> String {
        if url.starts_with("https://") || url.starts_with("http://") {
            return url.into();
        }
        format!(
            "{}/{}",
            root.strip_suffix('/').unwrap_or(root),
            url.strip_prefix('/').unwrap_or(url)
        )
    }

    /// Recursively explores each node of the page and stores the links
    fn howl_nodes(&mut self, nodes: Vec<Node>, root: &str) -> Result<()> {
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

                                    // We don't want references to the same page
                                    if !text.starts_with("#") {
                                        self.visited_nodes.insert(self.extract_url(text, root));
                                    }
                                }
                            }
                            None => debug!("this anchor doesn't has a link"),
                        }
                    }

                    self.howl_nodes(el.children, root)?;
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

    info!("These are all the retrieved links:\n");
    wolfie.print();

    Ok(())
}
