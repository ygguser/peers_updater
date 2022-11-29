pub fn get_def_cfg_path(is_unix: bool) -> &'static str {
    if is_unix {
        "/etc/yggdrasil.conf"
    } else {
        r"C:\ProgramData\Yggdrasil\yggdrasil.conf"
    }
}

pub fn get_yggctl_path(is_unix: bool) -> &'static str {
    if is_unix {
        "yggdrasilctl"
    } else {
        r"C:\Program Files\Yggdrasil\yggdrasilctl.exe"
    }
}
