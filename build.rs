//! Build script: embeds the Windows icon resource into `blackjack.exe`.
//!
//! On every non-Windows target this is a no-op, so Linux/macOS builds (and
//! the CI runners) are unaffected.

use std::env;
use std::path::Path;

fn main() {
    // Guard on the *target* (CARGO_CFG_* reflects the target platform, even
    // when cross-compiling), not on the host.
    if env::var("CARGO_CFG_WINDOWS").is_err() {
        return;
    }

    println!("cargo:rerun-if-changed=icon.rc");
    println!("cargo:rerun-if-changed=assets/images/BlackJack.ico");

    let icon = Path::new("assets/images/BlackJack.ico");
    assert!(
        icon.exists(),
        "Windows icon not found at {} (build from the crate root)",
        icon.display()
    );

    // `icon.rc` sits at the crate root and references the icon relative to
    // it; embed_resource resolves its argument relative to the crate root.
    // NOTE: deliberately no /SUBSYSTEM:WINDOWS — this is a console app.
    embed_resource::compile("icon.rc", embed_resource::NONE);
}
