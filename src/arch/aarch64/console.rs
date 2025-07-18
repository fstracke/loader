use core::ptr::NonNull;

use fdt::Fdt;

pub struct Console {
	stdout: NonNull<u8>,
}

fn stdout() -> u32 {
	/// Physical address of UART0 at Qemu's virt emulation
	const SERIAL_PORT_ADDRESS: u32 = 0x09000000;

	let dtb = unsafe {
		Fdt::from_ptr(sptr::from_exposed_addr(super::DEVICE_TREE as usize))
			.expect(".dtb file has invalid header")
	};

	let property = dtb.chosen().stdout();
	if let Some(stdout) = property {
		let stdout = stdout.name.trim_matches(char::from(0));
		if let Some(pos) = stdout.find('@') {
			let len = stdout.len();
			u32::from_str_radix(&stdout[pos + 1..len], 16).unwrap_or(SERIAL_PORT_ADDRESS)
		} else {
			SERIAL_PORT_ADDRESS
		}
	} else {
		SERIAL_PORT_ADDRESS
	}
}

impl Console {
	pub fn write_bytes(&mut self, bytes: &[u8]) {
		for byte in bytes.iter().copied() {
			unsafe {
				self.stdout.as_ptr().write_volatile(byte);
			}
		}
	}

	pub(super) fn get_stdout(&self) -> NonNull<u8> {
		self.stdout
	}

	pub(super) fn set_stdout(&mut self, stdout: NonNull<u8>) {
		self.stdout = stdout;
	}
}

impl Default for Console {
	fn default() -> Self {
		let stdout = NonNull::new(stdout() as *mut u8).unwrap();
		Self { stdout }
	}
}

unsafe impl Send for Console {}
