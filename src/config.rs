use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;
use sys_mount::FilesystemType;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "cargo-ramdisk",
    about = "Create target folder as a ramdisk for faster Rust compilation."
)]
pub struct CargoRamdiskConfig {
    /// The mount type to use. tmpfs, ramfs
    #[structopt(default_value = "tmpfs", long = "fs")]
    pub fs: MountType,

    /// The path to the target folder where compilation output is written
    #[structopt(default_value = "target/", short, long)]
    pub target: PathBuf,

    /// Save the mount info into fstab
    #[structopt(short = "s", long = "save-fstab")]
    pub save_fstab: bool,

    /// Path to the fstab file
    #[structopt(default_value = "/etc/fstab", long = "fstab-location")]
    pub fstab_location: PathBuf,

    #[structopt(subcommand)]
    pub subcommand: Option<Subcommands>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MountType {
    Tmpfs,
    Ramfs,
}

impl<'a> Into<FilesystemType<'a>> for MountType {
    fn into(self) -> FilesystemType<'a> {
        match self {
            Self::Tmpfs => FilesystemType::from("tmpfs"),
            Self::Ramfs => FilesystemType::from("ramfs"),
        }
    }
}

#[derive(Debug, StructOpt)]
pub struct MountConfig {
    /// The mount type to use. tmpfs, ramfs
    #[structopt(default_value = "tmpfs", long = "fs")]
    pub fs: MountType,

    /// The path to the target folder where compilation output is written
    #[structopt(default_value = "./target", short, long)]
    pub target: PathBuf,
}

impl From<CargoRamdiskConfig> for MountConfig {
    fn from(conf: CargoRamdiskConfig) -> Self {
        Self {
            fs: conf.fs,
            target: conf.target,
        }
    }
}

impl From<&RemountConfig> for MountConfig {
    fn from(config: &RemountConfig) -> Self {
        Self {
            fs: config.fs.clone(),
            target: config.target.clone(),
        }
    }
}

#[derive(Debug, StructOpt)]
pub struct RemountConfig {
    /// The mount type to use. tmpfs, ramfs
    #[structopt(default_value = "tmpfs", long = "fs")]
    pub fs: MountType,

    /// The path to the target folder where compilation output is written
    #[structopt(default_value = "target", short, long)]
    pub target: PathBuf,
}

#[derive(Debug, StructOpt)]
pub struct UnmountConfig {
    /// The path to the target folder where compilation output is written
    #[structopt(default_value = "target", short, long)]
    pub target: PathBuf,
}

impl From<&RemountConfig> for UnmountConfig {
    fn from(config: &RemountConfig) -> Self {
        Self {
            target: config.target.clone(),
        }
    }
}

#[derive(Debug, StructOpt)]
pub enum Subcommands {
    /// Mount a ramdisk, same as not specifying a subcommand
    Mount(MountConfig),

    /// Remount an existing ramdisk
    #[structopt(name = "remount")]
    Remount(RemountConfig),

    /// Unmount an existing ramdisk
    #[structopt(name = "unmount")]
    Unmount(UnmountConfig),
}

impl FromStr for MountType {
    type Err = String;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match &*str.to_lowercase() {
            "tmpfs" => Ok(Self::Tmpfs),
            "ramfs" => Ok(Self::Ramfs),
            _ => Err(format!(
                "Unknown fs type {}! Expected either `tmpfs` or `ramfs`",
                str
            )),
        }
    }
}