use std::collections::HashMap;
use std::fs::{self, create_dir_all, File, write};
use std::io::BufReader;
use std::path::PathBuf;
use std::process::Command;
use std::fmt::Write as _;

use anyhow::{Context, Result};
use cwl::v11::CwlDocument;
use specifications::version::Version;
use specifications::package::{PackageInfo, PackageKind};
use specifications::common::{Function, Type};
use brane_cli::errors::BuildError;

/// Parses a CWL file and generates a Brane-compatible package directory & Docker image.
pub async fn handle(path: PathBuf) -> Result<()> {
    // Open and parse CWL
    let file = File::open(&path).context("❌ Failed to open CWL file")?;
    let reader = BufReader::new(file);
    let document = CwlDocument::from_reader(reader).context("❌ Failed to parse CWL document")?;

    match &document {
        CwlDocument::CommandLineTool(tool) => {
            println!("✅ Parsed CWL CommandLineTool");

            // Extract fields
            let name = tool.schema.name.clone().unwrap_or_else(|| "unknown".into());
            let version_str = tool.schema.version.clone().unwrap_or_else(|| "0.1.0".into());
            let description = tool.label.clone().unwrap_or_else(|| "No description provided".into());

            // Fallback hardcoded version
            let version = Version::new(1, 0, 0);

            // Prepare output
            let out_dir = PathBuf::from(format!("target/generated/{}", name));
            create_dir_all(&out_dir).context("❌ Failed to create output directory")?;

            // --- Package.toml ---
            let mut toml = String::new();
            writeln!(toml, "name = {:?}", name)?;
            writeln!(toml, "version = {:?}", version_str)?;
            writeln!(toml, "kind = \"cwl\"")?;
            writeln!(toml, "description = {:?}", description)?;
            write(out_dir.join("Package.toml"), toml).context("❌ Failed to write Package.toml")?;

            // --- entry.sh ---
            let entry = "#!/bin/bash\ncwltool hello_world.cwl\n";
            write(out_dir.join("entry.sh"), entry).context("❌ Failed to write entry.sh")?;

            // --- Dockerfile ---
            let dockerfile = r#"
FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y cwltool
COPY hello_world.cwl /app/hello_world.cwl
COPY entry.sh /app/entry.sh
WORKDIR /app
RUN chmod +x entry.sh
CMD ["./entry.sh"]
"#;
            write(out_dir.join("Dockerfile"), dockerfile).context("❌ Failed to write Dockerfile")?;

            // --- Copy CWL ---
            fs::copy(&path, out_dir.join("hello_world.cwl")).context("❌ Failed to copy CWL file")?;

            // --- Docker build ---
            println!("🐳 Building Docker image...");
            let image_name = format!("brane-cwl-{}:latest", name);
            let status = Command::new("docker")
                .arg("build")
                .arg("--load")
                .arg("-t")
                .arg(&image_name)
                .arg(&out_dir)
                .status()
                .context("❌ Failed to invoke docker build")?;
            if !status.success() {
                anyhow::bail!("❌ Docker build failed");
            }

            println!("✅ Docker image built: {image_name}");

            // --- Create PackageInfo ---
            let package_info = PackageInfo::new(
                name.clone(),
                version,
                PackageKind::Ecu,
                vec![],
                description.clone(),
                true,
                HashMap::new(),
                HashMap::new(),
            );

            // --- Write package.yml ---
            package_info.to_path(out_dir.join("package.yml")).context("❌ Failed to write package.yml")?;

            println!("📦 Brane CWL package available at: {}\\", out_dir.display());
        }
        _ => {
            println!("⚠️ Unsupported CWL class: {:?}", document);
        }
    }

    Ok(())
}

/// `brane package build` calls this entry point for CWL packages.
pub fn build(_workdir: PathBuf, file: PathBuf) -> Result<(), BuildError> {
    println!("🛠️  Building Brane CWL package...");
    futures::executor::block_on(handle(file))
        .map_err(|e| BuildError::PackageInfoFromOpenAPIError { source: e })
}

