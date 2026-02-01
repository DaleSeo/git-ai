const fs = require("fs");
const path = require("path");
const https = require("https");
const { execSync } = require("child_process");
const { getPlatformInfo, BINARY_NAME } = require("./index.js");

const REPO_OWNER = "DaleSeo";
const REPO_NAME = "git-ai";

async function downloadFile(url, dest) {
  return new Promise((resolve, reject) => {
    const follow = (url) => {
      https
        .get(url, (response) => {
          if (response.statusCode === 302 || response.statusCode === 301) {
            follow(response.headers.location);
            return;
          }
          if (response.statusCode !== 200) {
            reject(new Error(`Failed to download: ${response.statusCode}`));
            return;
          }
          const file = fs.createWriteStream(dest);
          response.pipe(file);
          file.on("finish", () => {
            file.close();
            resolve();
          });
        })
        .on("error", reject);
    };
    follow(url);
  });
}

async function install() {
  const { platform, arch } = getPlatformInfo();
  const version = require("./package.json").version;

  const ext = platform === "windows" ? ".zip" : ".tar.gz";
  const assetName = `${BINARY_NAME}-${platform}-${arch}${ext}`;
  const downloadUrl = `https://github.com/${REPO_OWNER}/${REPO_NAME}/releases/download/v${version}/${assetName}`;

  const binDir = path.join(__dirname, "bin");
  const archivePath = path.join(__dirname, assetName);

  // Create bin directory
  if (!fs.existsSync(binDir)) {
    fs.mkdirSync(binDir, { recursive: true });
  }

  console.log(`Downloading ${BINARY_NAME} for ${platform}-${arch}...`);

  try {
    await downloadFile(downloadUrl, archivePath);

    // Extract archive
    if (platform === "windows") {
      execSync(`tar -xf "${archivePath}" -C "${binDir}"`, { stdio: "inherit" });
    } else {
      execSync(`tar -xzf "${archivePath}" -C "${binDir}"`, {
        stdio: "inherit",
      });
      // Make binary executable
      const binaryPath = path.join(binDir, BINARY_NAME);
      fs.chmodSync(binaryPath, 0o755);
    }

    // Clean up archive
    fs.unlinkSync(archivePath);

    console.log(`${BINARY_NAME} installed successfully!`);
  } catch (error) {
    console.error(`Failed to install ${BINARY_NAME}:`, error.message);
    console.error(
      `\nYou can manually download from: https://github.com/${REPO_OWNER}/${REPO_NAME}/releases`,
    );
    process.exit(1);
  }
}

install();
