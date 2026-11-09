//! Windows only: embed version info (VERSIONINFO) into `pdfcraft-cli.exe`, so Explorer's
//! Details tab, `(Get-Item pdfcraft-cli.exe).VersionInfo` and inventory tools can read the version
//! and publisher, as they can for the app (`apps/pdfcraft/build.rs`).
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `PDFCRAFT_REQUIRE_WINRES=1` turns it
//! into an error (for release builds).

#[path = "../pdfcraft/src/windows_version.rs"]
mod windows_version;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../pdfcraft/src/windows_version.rs");
    println!("cargo:rerun-if-env-changed=PDFCRAFT_REQUIRE_WINRES");
    println!("cargo:rerun-if-env-changed=WATHIQA_VERSION");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let version = windows_version::version(env!("CARGO_PKG_VERSION"));
    let number = windows_version::version_number(&version);
    let mut res = winresource::WindowsResource::new();
    res.set("ProductName", windows_version::PRODUCT_NAME)
        .set("FileDescription", "وثيقة command-line tool and MCP server")
        .set("CompanyName", windows_version::COMPANY_NAME)
        .set("LegalCopyright", windows_version::COPYRIGHT)
        .set("OriginalFilename", "wathiqa-cli.exe")
        .set("InternalName", "wathiqa-cli")
        .set("FileVersion", &version)
        .set("ProductVersion", &version)
        .set_version_info(winresource::VersionInfo::FILEVERSION, number)
        .set_version_info(winresource::VersionInfo::PRODUCTVERSION, number);
    if let Err(e) = res.compile() {
        if std::env::var_os("PDFCRAFT_REQUIRE_WINRES").is_some() {
            println!("cargo::error=embedding Windows resources failed: {e}");
            return;
        }
        println!("cargo:warning=pdfcraft-cli.exe built without version resources: {e}");
    }
}
