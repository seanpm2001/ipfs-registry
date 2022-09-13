use std::path::PathBuf;

use clap::{Parser, Subcommand};
use mime::Mime;
use serde_json::json;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use url::Url;

use ipfs_registry::Result;
use ipfs_registry_core::{Namespace, PackageKey};

/// Signed package registry server.
#[derive(Parser, Debug)]
#[clap(name = "ipkg", author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Generate a signing key.
    Keygen {
        /// Write the keystore file to directory.
        #[clap(parse(from_os_str))]
        dir: PathBuf,
    },
    /// Signup the public key for publishing.
    Signup {
        /// Server URL.
        #[clap(short, long, default_value = "http://127.0.0.1:9060")]
        server: Url,

        /// Keystore for the signing key.
        #[clap(short, long, parse(from_os_str))]
        key: PathBuf,
    },
    /// Register a namespace.
    Register {
        /// Server URL.
        #[clap(short, long, default_value = "http://127.0.0.1:9060")]
        server: Url,

        /// Keystore for the signing key.
        #[clap(short, long, parse(from_os_str))]
        key: PathBuf,

        /// Namespace to register.
        namespace: Namespace,
    },
    /// Publish a package.
    Publish {
        /// Server URL.
        #[clap(short, long, default_value = "http://127.0.0.1:9060")]
        server: Url,

        /// Namespace for packages.
        #[clap(short, long)]
        namespace: Namespace,

        /// Media type for the file.
        #[clap(short, long, default_value = "application/gzip")]
        mime: Mime,

        /// Keystore for the signing key.
        #[clap(short, long, parse(from_os_str))]
        key: PathBuf,

        /// File to publish.
        #[clap(parse(from_os_str))]
        file: PathBuf,
    },
    /// Download a package.
    Fetch {
        /// Server URL.
        #[clap(short, long, default_value = "http://127.0.0.1:9060")]
        server: Url,

        /// Package identifier.
        id: PackageKey,

        /// Write package to file.
        #[clap(parse(from_os_str))]
        file: PathBuf,
    },
    /// Yank a package.
    Yank {
        /// Server URL.
        #[clap(short, long, default_value = "http://127.0.0.1:9060")]
        server: Url,

        /// Keystore for the signing key.
        #[clap(short, long, parse(from_os_str))]
        key: PathBuf,

        /// Package identifier.
        id: PackageKey,

        /// Reason for yanking the version.
        message: Option<String>,
    },
    /// Get information about a specific package version.
    Get {
        /// Server URL.
        #[clap(short, long, default_value = "http://127.0.0.1:9060")]
        server: Url,

        /// Package identifier.
        id: PackageKey,
    },
    /// Start a server.
    Server {
        /// Bind to host:port.
        #[clap(short, long, default_value = "127.0.0.1:9060")]
        bind: String,

        /// Config file to load.
        #[clap(short, long, parse(from_os_str))]
        config: PathBuf,
    },
}

async fn run() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Command::Keygen { dir } => {
            let address = ipfs_registry_client::keygen(dir).await?;
            serde_json::to_writer_pretty(std::io::stdout(), &address)?;
        }
        Command::Signup { server, key } => {
            let doc = ipfs_registry_client::signup(server, key).await?;
            serde_json::to_writer_pretty(std::io::stdout(), &doc)?;
        }
        Command::Register {
            server,
            key,
            namespace,
        } => {
            let doc = ipfs_registry_client::register(server, key, namespace)
                .await?;
            serde_json::to_writer_pretty(std::io::stdout(), &doc)?;
        }
        Command::Publish {
            server,
            namespace,
            mime,
            key,
            file,
        } => {
            let doc = ipfs_registry_client::publish(
                server, namespace, mime, key, file,
            )
            .await?;
            serde_json::to_writer_pretty(std::io::stdout(), &doc)?;
        }
        Command::Fetch { server, id, file } => {
            let file = ipfs_registry_client::fetch(server, id, file).await?;
            let size = file.metadata()?.len();
            tracing::info!(file = ?file, size = ?size);
        }
        Command::Yank {
            server,
            key,
            id,
            message,
        } => {
            let message = message.unwrap_or(String::new());
            ipfs_registry_client::yank(server, key, id, message).await?;
            serde_json::to_writer_pretty(
                std::io::stdout(),
                &json!({"ok": true}),
            )?;
        }
        Command::Get { server, id } => {
            let doc = ipfs_registry_client::get(server, id).await?;
            serde_json::to_writer_pretty(std::io::stdout(), &doc)?;
        }
        Command::Server { bind, config } => {
            ipfs_registry_server::start(bind, config).await?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "info,sqlx::query=warn".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    match run().await {
        Ok(_) => {}
        Err(e) => {
            tracing::error!("{}", e);
        }
    }
    Ok(())
}
