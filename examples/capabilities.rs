//! Example demonstrating all the `is_*` functions from the `capabilities` module.
//!
//! This example shows how to check for various Windows Graphics Capture API
//! capabilities and configurations available on the current system.
//!
//! # Running this example
//! ```
//! cargo run --example capabilities
//! ```

use wgc::capabilities;

fn main() -> anyhow::Result<()> {
    println!("=== Windows Graphics Capture Capabilities Demo ===\n");
    println!("This example demonstrates all the `is_*` functions from `capabilities.rs`.");
    println!("These functions check what features are available on the current Windows version.\n");

    // Check if Windows Graphics Capture API is supported
    match capabilities::is_wgc_supported() {
        Ok(true) => println!("✅ WGC API is supported on this system"),
        Ok(false) => println!("❌ WGC API is NOT supported on this system"),
        Err(e) => println!("❓ Error checking WGC support: {}", e),
    }

    println!("\n--- Configurable Settings Availability ---");
    println!("The following properties can be configured on newer Windows versions:");

    // Check if border configuration is available
    // is_border_configurable(): Checks if IsBorderRequired property exists
    match capabilities::is_border_configurable() {
        Ok(true) => println!("✅ Border configuration is available (IsBorderRequired property)"),
        Ok(false) => println!("❌ Border configuration is NOT available"),
        Err(e) => println!("❓ Error checking border configuration: {}", e),
    }

    // Check if cursor capture configuration is available
    // is_cursor_configurable(): Checks if IsCursorCaptureEnabled property exists
    match capabilities::is_cursor_configurable() {
        Ok(true) => println!(
            "✅ Cursor capture configuration is available (IsCursorCaptureEnabled property)"
        ),
        Ok(false) => println!("❌ Cursor capture configuration is NOT available"),
        Err(e) => println!("❓ Error checking cursor configuration: {}", e),
    }

    // Check if dirty region mode configuration is available
    // is_dirty_region_mode_configurable(): Checks if DirtyRegionMode property exists
    match capabilities::is_dirty_region_mode_configurable() {
        Ok(true) => {
            println!("✅ Dirty region mode configuration is available (DirtyRegionMode property)")
        }
        Ok(false) => println!("❌ Dirty region mode configuration is NOT available"),
        Err(e) => println!("❓ Error checking dirty region mode configuration: {}", e),
    }

    // Check if include secondary windows configuration is available
    // is_include_secondary_windows_configurable(): Checks if IncludeSecondaryWindows property exists
    match capabilities::is_include_secondary_windows_configurable() {
        Ok(true) => println!(
            "✅ Include secondary windows configuration is available (IncludeSecondaryWindows property)"
        ),
        Ok(false) => println!("❌ Include secondary windows configuration is NOT available"),
        Err(e) => println!("❓ Error checking secondary windows configuration: {}", e),
    }

    // Check if minimum update interval configuration is available
    // is_min_update_interval_configurable(): Checks if MinUpdateInterval property exists
    match capabilities::is_min_update_interval_configurable() {
        Ok(true) => println!(
            "✅ Minimum update interval configuration is available (MinUpdateInterval property)"
        ),
        Ok(false) => println!("❌ Minimum update interval configuration is NOT available"),
        Err(e) => println!(
            "❓ Error checking minimum update interval configuration: {}",
            e
        ),
    }

    println!("\n=== Summary ===");
    println!("This system's Windows version determines which WGC features are available.");
    println!("Newer Windows versions support more configuration options.");
    println!("\nAvailable functions demonstrated:");
    println!("- is_wgc_supported(): Checks if Windows Graphics Capture API is available");
    println!("- is_border_configurable(): Checks if IsBorderRequired property exists");
    println!("- is_cursor_configurable(): Checks if IsCursorCaptureEnabled property exists");
    println!("- is_dirty_region_mode_configurable(): Checks if DirtyRegionMode property exists");
    println!(
        "- is_include_secondary_windows_configurable(): Checks if IncludeSecondaryWindows property exists"
    );
    println!(
        "- is_min_update_interval_configurable(): Checks if MinUpdateInterval property exists"
    );
    println!("\nNote: These functions check API availability, not whether features are enabled.");
    println!("Even if a property is configurable, it may require specific Windows builds");
    println!("or updates to be fully functional.");

    Ok(())
}
