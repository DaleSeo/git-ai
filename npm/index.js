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

  // Try to find binary in platform-specific package
  const ext = platform === "windows" ? ".exe" : "";
  const packageName = `@daleseo/git-ai-${platform}-${arch}`;

  try {
    // Look for the platform package in node_modules
    const platformPackagePath = require.resolve(`${packageName}/package.json`);
    const platformPackageDir = path.dirname(platformPackagePath);
    const binaryPath = path.join(platformPackageDir, "bin", `${BINARY_NAME}${ext}`);

    if (fs.existsSync(binaryPath)) {
      return binaryPath;
    }
  } catch (error) {
    // Platform package not found, continue to error message
  }

  throw new Error(
    `Platform-specific package not found: ${packageName}\n` +
    `This usually means the package was not installed correctly.\n` +
    `Please try reinstalling: npm install -g @daleseo/git-ai`
  );
}

module.exports = { getBinaryPath, getPlatformInfo, BINARY_NAME };
