const path = require("path");
const fs = require("fs");

const BINARY_NAME = "git-ai";

function getPlatformInfo() {
  const platform = process.platform;
  const arch = process.arch;

  const platformMap = {
    darwin: "darwin",
    linux: "linux",
    win32: "windows",
  };

  const archMap = {
    x64: "x64",
    arm64: "arm64",
  };

  const mappedPlatform = platformMap[platform];
  const mappedArch = archMap[arch];

  if (!mappedPlatform || !mappedArch) {
    throw new Error(`Unsupported platform: ${platform}-${arch}`);
  }

  return { platform: mappedPlatform, arch: mappedArch };
}

function getBinaryPath() {
  const { platform, arch } = getPlatformInfo();
  const binaryDir = path.join(__dirname, "bin");
  const ext = platform === "windows" ? ".exe" : "";
  const binaryPath = path.join(binaryDir, `${BINARY_NAME}${ext}`);

  if (!fs.existsSync(binaryPath)) {
    throw new Error(
      `Binary not found at ${binaryPath}. ` +
        `Please run 'npm install' to download the binary.`,
    );
  }

  return binaryPath;
}

module.exports = { getBinaryPath, getPlatformInfo, BINARY_NAME };
