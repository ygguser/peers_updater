#[cfg(target_os = "windows")]
#[cfg(any(feature = "updating_cfg", feature = "using_api"))]
pub const DEF_CFG_PATH: &str = r"C:\ProgramData\Yggdrasil\yggdrasil.conf";
#[cfg(not(target_os = "windows"))]
#[cfg(any(feature = "updating_cfg", feature = "using_api"))]
pub const DEF_CFG_PATH: &str = "/etc/yggdrasil/yggdrasil.conf";

#[cfg(target_os = "windows")]
#[cfg(feature = "using_api")]
pub const DEF_SOCKET_ADDR: &str = "localhost:9001";
#[cfg(not(target_os = "windows"))]
#[cfg(feature = "using_api")]
pub const DEF_SOCKET_ADDR: &str = "/var/run/yggdrasil/yggdrasil.sock";

#[cfg(target_os = "windows")]
#[cfg(feature = "self_updating")]
pub const EXE_NAME: &str = "peers_updater.exe";
#[cfg(not(target_os = "windows"))]
#[cfg(feature = "self_updating")]
pub const EXE_NAME: &str = "peers_updater";
