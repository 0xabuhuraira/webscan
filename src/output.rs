//! Output formatting and writing

use crate::error::{Result, WebScanError};
use crate::minecraft_status::MinecraftStatus;
use serde_json::{json, Value};
use std::fs::File;
use std::io::Write;
use std::net::SocketAddr;
use std::path::Path;

#[derive(Debug, Clone)]
pub enum OutputFormat {
    Text,
    Json,
    Csv,
    Ndjson,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "json" => OutputFormat::Json,
            "csv" => OutputFormat::Csv,
            "ndjson" => OutputFormat::Ndjson,
            _ => OutputFormat::Text,
        }
    }
}

pub struct OutputWriter {
    format: OutputFormat,
    file: Option<File>,
}

impl OutputWriter {
    pub fn new(format: OutputFormat, file_path: Option<&Path>) -> Result<Self> {
        let file = if let Some(path) = file_path {
            Some(
                File::create(path)
                    .map_err(|e| WebScanError::Io(e))?
            )
        } else {
            None
        };

        Ok(OutputWriter { format, file })
    }

    pub fn write_header(&mut self) -> Result<()> {
        match self.format {
            OutputFormat::Csv => {
                let header = "ip,port,protocol,version,protocol_version,players_online,players_max,description,latency_ms\n";
                self.write_output(header)?;
            }
            OutputFormat::Json => {
                self.write_output("[\n")?;
            }
            _ => {}
        }
        Ok(())
    }

    pub fn write_footer(&mut self) -> Result<()> {
        match self.format {
            OutputFormat::Json => {
                self.write_output("\n]\n")?;
            }
            _ => {}
        }
        Ok(())
    }

    pub fn write_result(
        &mut self,
        addr: SocketAddr,
        status: &MinecraftStatus,
    ) -> Result<()> {
        match self.format {
            OutputFormat::Text => self.write_text(addr, status),
            OutputFormat::Json => self.write_json(addr, status),
            OutputFormat::Csv => self.write_csv(addr, status),
            OutputFormat::Ndjson => self.write_ndjson(addr, status),
        }
    }

    pub fn write_open_port(&mut self, addr: SocketAddr) -> Result<()> {
        match self.format {
            OutputFormat::Text => {
                let line = format!("{}:{}  TCP OPEN\n", addr.ip(), addr.port());
                self.write_output(&line)
            }
            OutputFormat::Json => {
                let obj = json!({
                    "ip": addr.ip().to_string(),
                    "port": addr.port(),
                    "protocol": "tcp",
                    "state": "open"
                });
                self.write_output(&format!("{},\n", obj))
            }
            OutputFormat::Csv => {
                let line = format!("{},,tcp,open\n", addr);
                self.write_output(&line)
            }
            OutputFormat::Ndjson => {
                let obj = json!({
                    "ip": addr.ip().to_string(),
                    "port": addr.port(),
                    "protocol": "tcp",
                    "state": "open"
                });
                self.write_output(&format!("{}\n", obj))
            }
        }
    }

    fn write_text(&mut self, addr: SocketAddr, status: &MinecraftStatus) -> Result<()> {
        let version = status.version.as_ref().map(|s| s.as_str()).unwrap_or("unknown");
        let players = format!(
            "{}/{}",
            status.players_online.unwrap_or(0),
            status.players_max.unwrap_or(0)
        );
        let line = format!(
            "{}:{}  MINECRAFT  "{}"  {}  latency={}ms\n",
            addr.ip(),
            addr.port(),
            version,
            players,
            status.latency_ms
        );
        self.write_output(&line)
    }

    fn write_json(&mut self, addr: SocketAddr, status: &MinecraftStatus) -> Result<()> {
        let obj = json!({
            "ip": addr.ip().to_string(),
            "port": addr.port(),
            "protocol": "minecraft",
            "version": status.version,
            "protocol_version": status.protocol_version,
            "players_online": status.players_online,
            "players_max": status.players_max,
            "description": status.description,
            "latency_ms": status.latency_ms
        });
        self.write_output(&format!("{},\n", obj))
    }

    fn write_csv(&mut self, addr: SocketAddr, status: &MinecraftStatus) -> Result<()> {
        let version = status.version.as_ref().map(|s| escape_csv(s)).unwrap_or_default();
        let desc = status.description.as_ref().map(|s| escape_csv(s)).unwrap_or_default();
        let line = format!(
            "{},{},minecraft,\"{}\",{},{},{},\"{}\",{}\n",
            addr.ip(),
            addr.port(),
            version,
            status.protocol_version.unwrap_or(0),
            status.players_online.unwrap_or(0),
            status.players_max.unwrap_or(0),
            desc,
            status.latency_ms
        );
        self.write_output(&line)
    }

    fn write_ndjson(&mut self, addr: SocketAddr, status: &MinecraftStatus) -> Result<()> {
        let obj = json!({
            "ip": addr.ip().to_string(),
            "port": addr.port(),
            "protocol": "minecraft",
            "version": status.version,
            "protocol_version": status.protocol_version,
            "players_online": status.players_online,
            "players_max": status.players_max,
            "description": status.description,
            "latency_ms": status.latency_ms
        });
        self.write_output(&format!("{}\n", obj))
    }

    fn write_output(&mut self, content: &str) -> Result<()> {
        if let Some(ref mut file) = self.file {
            file.write_all(content.as_bytes())
                .map_err(|e| WebScanError::Io(e))?;
            file.flush().map_err(|e| WebScanError::Io(e))?;
        } else {
            print!("{}", content);
        }
        Ok(())
    }
}

fn escape_csv(s: &str) -> String {
    if s.contains('"') || s.contains(',') || s.contains('\n') {
        format!("\"{}\"" , s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}
