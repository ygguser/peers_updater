#[cfg(target_os = "windows")]
pub const DEF_CFG_PATH: &'static str = r"C:\ProgramData\Yggdrasil\yggdrasil.conf";
#[cfg(not(target_os = "windows"))]
pub const DEF_CFG_PATH: &'static str = "/etc/yggdrasil.conf";

#[cfg(target_os = "windows")]
pub const DEF_SOCKET_ADDR: &'static str = "localhost:9001";
#[cfg(not(target_os = "windows"))]
pub const DEF_SOCKET_ADDR: &'static str = "/var/run/yggdrasil.sock";
