//! imagen — OpenAI gpt-image-2 CLI (8 노드 공통).
//! POST /v1/images/generations → base64 PNG → file.
use anyhow::{Context, Result, bail};
use base64::{Engine, engine::general_purpose::STANDARD};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const MODEL: &str = "gpt-image-2";
const ENDPOINT: &str = "https://api.openai.com/v1/images/generations";

#[derive(Parser, Debug)]
#[command(version, about = "Generate images via OpenAI gpt-image-2 (Native-First Rust)")]
struct Args {
    /// Prompt text
    prompt: String,
    /// Output path (default: ./imagen-<epoch>.png)
    #[arg(long, short = 'o')]
    out: Option<PathBuf>,
    /// Size: 1024x1024 | 1024x1536 | 1536x1024 | auto
    #[arg(long, default_value = "1024x1024")]
    size: String,
    /// Quality: low | medium | high | auto
    #[arg(long, default_value = "auto")]
    quality: String,
    /// Number of images
    #[arg(long, short = 'n', default_value = "1")]
    n: u32,
    /// Print only path (scripts)
    #[arg(long)]
    quiet: bool,
}

#[derive(Serialize)]
struct Req<'a> {
    model: &'a str,
    prompt: &'a str,
    size: &'a str,
    quality: &'a str,
    n: u32,
    output_format: &'a str,
}

#[derive(Deserialize)]
struct Resp {
    data: Vec<ImgData>,
    usage: Option<serde_json::Value>,
}

#[derive(Deserialize)]
struct ImgData {
    b64_json: String,
}

/// Node-aware default output directory.
/// Priority: IMAGEN_OUT_DIR env > termux sdcard > nmsnas vflat > Windows Pictures > $HOME/Pictures/imagen > CWD.
fn default_out_dir(is_termux: bool) -> PathBuf {
    if let Ok(d) = std::env::var("IMAGEN_OUT_DIR") {
        if !d.is_empty() { return PathBuf::from(d); }
    }
    if is_termux {
        return PathBuf::from("/sdcard/DCIM");
    }
    // nmsnas: shared vflat dir for HTTP access
    if std::path::Path::new("/share/vflat").is_dir() {
        return PathBuf::from("/share/vflat/imagen");
    }
    // Windows native
    if cfg!(windows) {
        if let Ok(p) = std::env::var("USERPROFILE") {
            return PathBuf::from(p).join("Pictures").join("imagen");
        }
    }
    // WSL → Windows Pictures via /mnt/c
    if let Ok(w) = std::env::var("WSL_DISTRO_NAME") {
        if !w.is_empty() {
            if let Ok(u) = std::env::var("USER") {
                let p = format!("/mnt/c/Users/{}/Pictures/imagen", u);
                if std::path::Path::new("/mnt/c/Users").is_dir() {
                    return PathBuf::from(p);
                }
            }
        }
    }
    // Linux generic
    if let Ok(h) = std::env::var("HOME") {
        return PathBuf::from(h).join("Pictures").join("imagen");
    }
    PathBuf::from(".")
}

fn api_key() -> Result<String> {
    if let Ok(k) = std::env::var("OPENAI_API_KEY") {
        if !k.is_empty() { return Ok(k); }
    }
    bail!("OPENAI_API_KEY env var not set");
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let key = api_key()?;
    if args.n == 0 || args.n > 10 { bail!("n must be 1..=10"); }

    let req = Req {
        model: MODEL,
        prompt: &args.prompt,
        size: &args.size,
        quality: &args.quality,
        n: args.n,
        output_format: "png",
    };

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(180))
        .build()?;

    if !args.quiet {
        eprintln!("[imagen] model={} size={} quality={} n={}", MODEL, args.size, args.quality, args.n);
    }

    let r = client.post(ENDPOINT)
        .bearer_auth(&key)
        .json(&req)
        .send().await
        .context("HTTP request failed")?;

    let status = r.status();
    let body = r.text().await?;
    if !status.is_success() {
        bail!("API error {}: {}", status, body);
    }

    let parsed: Resp = serde_json::from_str(&body)
        .with_context(|| format!("parse response: {}", body.chars().take(300).collect::<String>()))?;

    if parsed.data.is_empty() { bail!("empty data in response"); }

    let is_termux = std::env::var("PREFIX").map(|p| p.contains("com.termux")).unwrap_or(false);
    let base: PathBuf = args.out.unwrap_or_else(|| {
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        let dir = default_out_dir(is_termux);
        let _ = std::fs::create_dir_all(&dir);
        dir.join(format!("imagen-{}.png", ts))
    });

    for (i, d) in parsed.data.iter().enumerate() {
        let png = STANDARD.decode(&d.b64_json).context("base64 decode")?;
        let path = if parsed.data.len() == 1 {
            base.clone()
        } else {
            let stem = base.file_stem().and_then(|s| s.to_str()).unwrap_or("imagen");
            let ext = base.extension().and_then(|s| s.to_str()).unwrap_or("png");
            base.with_file_name(format!("{}-{}.{}", stem, i, ext))
        };
        std::fs::write(&path, &png).with_context(|| format!("write {:?}", path))?;
        // Android: make gallery-visible via MediaStore broadcast when path under /sdcard/ or /storage/
        let path_str = path.to_string_lossy().to_string();
        if is_termux && (path_str.starts_with("/sdcard/") || path_str.starts_with("/storage/")) {
            let _ = std::process::Command::new("su")
                .arg("-c")
                .arg(format!("chmod 644 '{}' && am broadcast -a android.intent.action.MEDIA_SCANNER_SCAN_FILE -d file://{}", path_str, path_str))
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status();
        }
        if args.quiet {
            println!("{}", path.display());
        } else {
            eprintln!("[imagen] saved {} ({} bytes)", path.display(), png.len());
        }
    }

    if !args.quiet {
        if let Some(u) = parsed.usage {
            eprintln!("[imagen] usage: {}", u);
        }
    }
    Ok(())
}
