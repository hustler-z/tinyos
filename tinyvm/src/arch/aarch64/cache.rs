// @Hustler
//
// Self-Education Only

use core::arch::global_asm;

/// Date Cache('DC') operations.
/// # Safety:
/// Data cache operations can't trigger any side effects on the safety of the system.
pub mod dc {
	macro_rules! define_dc {
		($mode:ident) => {
			pub fn $mode(va: usize) {
				unsafe {
					sysop!(dc, $mode, va as u64);
				}
			}
		};
	}

	define_dc!(ivac);
	define_dc!(isw);
	define_dc!(cvac);
	define_dc!(csw);
	define_dc!(cvau);
	define_dc!(civac);
	define_dc!(cisw);
	define_dc!(zva);
}

/// Instruction Cache('IC') operations.
/// #Safety:
/// Instruction cache operations can't trigger any side effects on the safety of the system.
pub mod ic {
	macro_rules! define_ic {
		($mode:ident) => {
			pub fn $mode(va: usize) {
				unsafe {
					sysop!(ic, $mode, va as u64);
				}
			}
		};
	}

	define_ic!(ialluis);
	define_ic!(iallu);
	define_ic!(ivau);
	define_ic!(ivac);
	define_ic!(isw);
}

// TODO: remove the FFI call and use trait for cache operation
global_asm!(include_str!("cache.S"));

extern "C" {
	/// Invalidate the data cache.
	pub fn cache_invalidate_d(start: usize, len: usize);
	/// Clean and invalidate the data cache.
	pub fn cache_clean_invalidate_d(start: usize, len: usize);
}
