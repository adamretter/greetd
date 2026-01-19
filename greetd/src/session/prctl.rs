use nix::{errno::Errno, Result};

#[cfg(target_os = "linux")]
pub const PRCTL_SET_PDEATHSIG: i32 = 1;

#[allow(non_camel_case_types)]
pub enum PrctlOption {
    SET_PDEATHSIG(i32),
}

#[cfg(target_os = "linux")]
pub fn prctl(option: PrctlOption) -> Result<()> {
    Errno::result(match option {
        PrctlOption::SET_PDEATHSIG(sig) => unsafe {
            libc::prctl(PRCTL_SET_PDEATHSIG, sig, 0, 0, 0)
        },
    })
    .map(drop)
}

#[cfg(target_os = "freebsd")]
pub fn prctl(option: PrctlOption) -> Result<()> {
    match option {
        PrctlOption::SET_PDEATHSIG(sig) => {
            let sig = sig as libc::c_int;
            let ret = unsafe {
                libc::procctl(libc::P_PID, 0, libc::PROC_PDEATHSIG_CTL, &sig as *const _ as *mut _)
            };
            Errno::result(ret).map(drop)
        }
    }
}
