//! SigmaOS-browser_handoff — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_browser_handoff_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-browser_handoff: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-browser_handoff: ok");
}
