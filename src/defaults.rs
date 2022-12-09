pub fn get_def_cfg_path(is_unix: bool) -> &'static str {
    if is_unix {
        "/etc/yggdrasil.conf"
    } else {
        r"C:\ProgramData\Yggdrasil\yggdrasil.conf"
    }
}

pub fn get_def_socket_addr(is_unix: bool) -> &'static str {
    if is_unix {
        "/var/run/yggdrasil.sock"
    } else {
        "localhost:9001"
    }
}
