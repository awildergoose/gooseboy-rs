pub const MEM_BASE: u64 = 0x8000_0000;
pub const DEVICE_BASE: u64 = 0xa000_0000;
pub const SERIAL_PORT: u64 = DEVICE_BASE + 0x0000_03f8;
pub const RTC_ADDR: u64 = DEVICE_BASE + 0x0000_0048;
pub const KBD_ADDR: u64 = DEVICE_BASE + 0x0000_0060;
pub const MOUSE_ADDR: u64 = DEVICE_BASE + 0x0000_0070;
pub const FB_ADDR: u64 = DEVICE_BASE + 0x0100_0000;
pub const VGACTL_ADDR: u64 = DEVICE_BASE + 0x0000_0100;

pub trait DeviceBase {
    fn do_read(&mut self, addr: u64, len: usize) -> u64;
    fn do_write(&mut self, addr: u64, data: u64, len: usize) -> u64;
    // The default implementation is slow, but it works
    // You can override it if you want
    fn copy_from_slice(&mut self, addr: u64, slice: &[u8]) {
        slice.iter().enumerate().for_each(|(i, &x)| {
            self.do_write(addr + i as u64, u64::from(x), 1);
        });
    }
    // The default implementation is slow, but it works
    // You can override it if you want
    fn copy_to_slice(&mut self, addr: u64, slice: &mut [u8]) {
        slice.iter_mut().enumerate().for_each(|(i, x)| {
            *x = u8::try_from(self.do_read(addr + i as u64, 1)).unwrap();
        });
    }
    fn get_name(&self) -> &'static str;
    fn do_update(&mut self) {}

    fn reset(&mut self) {}
}
