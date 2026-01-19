use nix::{ioctl_none_bad, ioctl_read_bad, ioctl_write_int_bad, ioctl_write_ptr_bad};

// Kernel Display (KD) mode constants - same on Linux and FreeBSD
pub const KDTEXT: i32 = 0x00;
pub const KDGRAPHICS: i32 = 0x01;
pub const VT_AUTO: u8 = 0;

// Platform-specific ioctl values
// Linux uses different values than FreeBSD, but both platforms support these operations
#[cfg(target_os = "linux")]
pub const KDSETMODE: u32 = 0x4B3A;
#[cfg(target_os = "linux")]
pub const VT_OPENQRY: u32 = 0x5600;
#[cfg(target_os = "linux")]
pub const VT_SETMODE: u32 = 0x5602;
#[cfg(target_os = "linux")]
pub const VT_GETSTATE: u32 = 0x5603;
#[cfg(target_os = "linux")]
pub const VT_ACTIVATE: u32 = 0x5606;
#[cfg(target_os = "linux")]
pub const VT_WAITACTIVE: u32 = 0x5607;
#[cfg(target_os = "linux")]
pub const VT_SETACTIVATE: u32 = 0x560F;
#[cfg(target_os = "linux")]
pub const TIOCSCTTY: u32 = 0x540E;

#[cfg(target_os = "freebsd")]
pub const KDSETMODE: u32 = 0x20044B0A; // _IOWINT('K', 10)
#[cfg(target_os = "freebsd")]
pub const VT_OPENQRY: u32 = 0x40047601; // _IOR('v', 1, int)
#[cfg(target_os = "freebsd")]
pub const VT_SETMODE: u32 = 0x80087602; // _IOW('v', 2, vtmode_t)
#[cfg(target_os = "freebsd")]
pub const VT_GETACTIVE: u32 = 0x40047607; // _IOR('v', 7, int)
#[cfg(target_os = "freebsd")]
pub const VT_ACTIVATE: u32 = 0x20047605; // _IOWINT('v', 5)
#[cfg(target_os = "freebsd")]
pub const VT_WAITACTIVE: u32 = 0x20047606; // _IOWINT('v', 6)
#[cfg(target_os = "freebsd")]
pub const TIOCSCTTY: u32 = 0x20007461; // _IO('t', 97)
#[cfg(target_os = "freebsd")]
pub const TIOCNOTTY: u32 = 0x20007471; // _IO('t', 113)

ioctl_write_int_bad!(kd_setmode, KDSETMODE);
ioctl_write_int_bad!(vt_activate, VT_ACTIVATE);
ioctl_write_int_bad!(vt_waitactive, VT_WAITACTIVE);
ioctl_write_ptr_bad!(vt_setmode, VT_SETMODE, vt_mode);
// VT_SETACTIVATE only exists on Linux
#[cfg(target_os = "linux")]
ioctl_write_ptr_bad!(vt_setactivate, VT_SETACTIVATE, vt_setactivate);
ioctl_read_bad!(vt_openqry, VT_OPENQRY, i64);
// VT_GETSTATE only exists on Linux
#[cfg(target_os = "linux")]
ioctl_read_bad!(vt_getstate, VT_GETSTATE, vt_state);
// FreeBSD has VT_GETACTIVE which returns just an int
#[cfg(target_os = "freebsd")]
ioctl_read_bad!(vt_getactive, VT_GETACTIVE, i32);
// TIOCSCTTY differs between platforms:
// - Linux: takes an int argument (1 = steal terminal, 0 = don't steal)
// - FreeBSD: takes no argument (_IO not _IOW)
#[cfg(target_os = "linux")]
ioctl_write_int_bad!(term_tiocsctty, TIOCSCTTY);
#[cfg(target_os = "freebsd")]
ioctl_none_bad!(term_tiocsctty, TIOCSCTTY);
// TIOCNOTTY - give up controlling terminal (FreeBSD needs this to release before taking)
#[cfg(target_os = "freebsd")]
ioctl_none_bad!(term_tiocnotty, TIOCNOTTY);

#[allow(dead_code)]
#[repr(C)]
pub struct vt_mode {
    pub mode: u8,
    pub waitv: u8,
    pub relsig: u16,
    pub acqsig: u16,
    pub frsig: u16,
}

// VT_SETACTIVATE struct only exists on Linux
#[cfg(target_os = "linux")]
#[allow(dead_code)]
#[repr(C)]
pub struct vt_setactivate {
    pub console: u64,
    pub mode: vt_mode,
}

#[allow(dead_code)]
#[repr(C)]
pub struct vt_state {
    pub v_active: u16,
    pub v_signal: u16,
    pub v_state: u16,
}
