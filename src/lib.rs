pub mod core;
pub mod content;

// The `engine` module depends on Bevy and its native system libraries.
// Provide two variants:
// - When the "bevy" feature is enabled, compile the real engine module.
// - Otherwise, provide a lightweight stub so the rest of the crate can compile
//   for testing/CLI purposes without requiring system graphics/audio libs.

#[cfg(feature = "bevy")]
pub mod engine;

#[cfg(not(feature = "bevy"))]
pub mod engine {
	/// Minimal stub used when Bevy is not available.
	/// This allows `cargo build` to succeed on CI or machines without graphics
	/// or system dependencies while keeping the real engine behind a feature flag.
	#[derive(Debug, Clone, Copy)]
	pub struct EnginePlugin;
}

// Re-export for consumers
pub use core::*;
pub use content::*;
pub use engine::*;
