#[cfg(target_os = "windows")]
pub const DEF_CFG_PATH: &str = r"C:\ProgramData\Yggdrasil\yggdrasil.conf";
#[cfg(not(target_os = "windows"))]
pub const DEF_CFG_PATH: &str = "/etc/yggdrasil.conf";

#[cfg(target_os = "windows")]
pub const DEF_SOCKET_ADDR: &str = "localhost:9001";
#[cfg(not(target_os = "windows"))]
pub const DEF_SOCKET_ADDR: &str = "/var/run/yggdrasil.sock";

#[cfg(target_os = "windows")]
pub const EXE_NAME: &str = "peers_updater.exe";
#[cfg(not(target_os = "windows"))]
pub const EXE_NAME: &str = "peers_updater";
