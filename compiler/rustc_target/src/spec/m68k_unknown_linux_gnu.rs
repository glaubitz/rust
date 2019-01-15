use crate::spec::{LinkerFlavor, Target, TargetResult};

pub fn target() -> TargetResult {
    let mut base = super::linux_base::opts();
    base.max_atomic_width = Some(32);

    Ok(Target {
        llvm_target: "m68k-unknown-linux-gnu".to_string(),
        target_endian: "big".to_string(),
        target_pointer_width: "32".to_string(),
        target_c_int_width: "32".to_string(),
        data_layout: "E-m:e-p:32:32-i8:8:8-i16:16:16-i32:32:32-n8:16:32-a:0:32-S16".to_string(),
        arch: "m68k".to_string(),
        target_os: "linux".to_string(),
        target_env: "gnu".to_string(),
        target_vendor: "unknown".to_string(),
        linker_flavor: LinkerFlavor::Gcc,
        options: base,
    })
}
