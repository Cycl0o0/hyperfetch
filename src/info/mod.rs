pub mod audio;
pub mod desktop;
pub mod hardware;
pub mod misc;
pub mod network;
pub mod packages;
pub mod power;
pub mod system;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SystemInfo {
    // System
    pub os: Option<String>,
    pub os_id: Option<String>,
    pub kernel: Option<String>,
    pub hostname: Option<String>,
    pub uptime: Option<String>,
    pub uptime_seconds: Option<u64>,
    pub load_average: Option<String>,
    pub processes: Option<u32>,
    pub logged_users: Option<String>,
    pub machine_type: Option<String>,
    pub init_system: Option<String>,
    pub boot_time: Option<String>,

    // Hardware
    pub cpu: Option<String>,
    pub cpu_arch: Option<String>,
    pub cpu_cores: Option<u32>,
    pub cpu_threads: Option<u32>,
    pub cpu_freq: Option<String>,
    pub cpu_cache: Option<String>,
    pub cpu_temp: Option<String>,
    pub cpu_governor: Option<String>,
    pub gpu: Vec<GpuInfo>,
    pub memory: Option<String>,
    pub memory_used: Option<u64>,
    pub memory_total: Option<u64>,
    pub swap: Option<String>,
    pub swap_used: Option<u64>,
    pub swap_total: Option<u64>,
    pub disks: Vec<DiskInfo>,
    pub motherboard: Option<String>,
    pub bios: Option<String>,

    // Desktop
    pub de: Option<String>,
    pub wm: Option<String>,
    pub wm_theme: Option<String>,
    pub theme: Option<String>,
    pub icons: Option<String>,
    pub cursor: Option<String>,
    pub terminal: Option<String>,
    pub terminal_font: Option<String>,
    pub shell: Option<String>,
    pub shell_version: Option<String>,
    pub display_server: Option<String>,
    pub resolution: Option<String>,

    // Network
    pub interfaces: Vec<NetworkInterface>,
    #[cfg(feature = "network")]
    pub public_ip: Option<PublicIpInfo>,

    // Power
    pub battery: Option<BatteryInfo>,
    pub brightness: Option<String>,

    // Audio
    pub audio_device: Option<String>,
    pub volume: Option<String>,

    // Packages
    pub packages: Option<String>,
    pub package_counts: Vec<PackageCount>,

    // Misc
    pub locale: Option<String>,
    pub timezone: Option<String>,
    pub virtualization: Option<String>,
    pub container: Option<String>,
    pub security: Option<String>,
    pub ssh_connection: Option<String>,
    pub bluetooth: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GpuInfo {
    pub name: String,
    pub driver: Option<String>,
    pub vram: Option<String>,
    pub temp: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiskInfo {
    pub mount: String,
    pub filesystem: String,
    pub size: String,
    pub used: String,
    pub available: String,
    pub percent: u8,
    pub disk_type: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct NetworkInterface {
    pub name: String,
    pub ipv4: Option<String>,
    pub ipv6: Option<String>,
    pub mac: Option<String>,
    pub speed: Option<String>,
    pub state: Option<String>,
}

#[cfg(feature = "network")]
#[derive(Debug, Clone, Serialize)]
pub struct PublicIpInfo {
    pub ip: String,
    pub country: Option<String>,
    pub region: Option<String>,
    pub city: Option<String>,
    pub zip: Option<String>,
    pub isp: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BatteryInfo {
    pub percent: u8,
    pub status: String,
    pub time_remaining: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PackageCount {
    pub manager: String,
    pub count: u32,
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self {
            os: None,
            os_id: None,
            kernel: None,
            hostname: None,
            uptime: None,
            uptime_seconds: None,
            load_average: None,
            processes: None,
            logged_users: None,
            machine_type: None,
            init_system: None,
            boot_time: None,
            cpu: None,
            cpu_arch: None,
            cpu_cores: None,
            cpu_threads: None,
            cpu_freq: None,
            cpu_cache: None,
            cpu_temp: None,
            cpu_governor: None,
            gpu: Vec::new(),
            memory: None,
            memory_used: None,
            memory_total: None,
            swap: None,
            swap_used: None,
            swap_total: None,
            disks: Vec::new(),
            motherboard: None,
            bios: None,
            de: None,
            wm: None,
            wm_theme: None,
            theme: None,
            icons: None,
            cursor: None,
            terminal: None,
            terminal_font: None,
            shell: None,
            shell_version: None,
            display_server: None,
            resolution: None,
            interfaces: Vec::new(),
            #[cfg(feature = "network")]
            public_ip: None,
            battery: None,
            brightness: None,
            audio_device: None,
            volume: None,
            packages: None,
            package_counts: Vec::new(),
            locale: None,
            timezone: None,
            virtualization: None,
            container: None,
            security: None,
            ssh_connection: None,
            bluetooth: None,
        }
    }
}

impl SystemInfo {
    pub fn gather(fetch_public_ip: bool) -> Self {
        let mut info = SystemInfo::default();

        // System info
        system::gather(&mut info);

        // Hardware info
        hardware::gather(&mut info);

        // Desktop info
        desktop::gather(&mut info);

        // Network info
        network::gather(&mut info, fetch_public_ip);

        // Power info
        power::gather(&mut info);

        // Audio info
        audio::gather(&mut info);

        // Package info
        packages::gather(&mut info);

        // Misc info
        misc::gather(&mut info);

        info
    }
}
