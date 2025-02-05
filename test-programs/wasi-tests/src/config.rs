pub struct TestConfig {
    errno_mode: ErrnoMode,
    no_dangling_filesystem: bool,
    no_rename_dir_to_empty_dir: bool,
    no_fdflags_sync_support: bool,
    no_rights_readback_support: bool,
}

enum ErrnoMode {
    Unix,
    MacOS,
    Windows,
    Permissive,
}

impl TestConfig {
    pub fn from_env() -> Self {
        let errno_mode = if std::env::var("ERRNO_MODE_UNIX").is_ok() {
            ErrnoMode::Unix
        } else if std::env::var("ERRNO_MODE_MACOS").is_ok() {
            ErrnoMode::MacOS
        } else if std::env::var("ERRNO_MODE_WINDOWS").is_ok() {
            ErrnoMode::Windows
        } else {
            ErrnoMode::Permissive
        };
        let no_dangling_filesystem = std::env::var("NO_DANGLING_FILESYSTEM").is_ok();
        let no_rename_dir_to_empty_dir = std::env::var("NO_RENAME_DIR_TO_EMPTY_DIR").is_ok();
        let no_fdflags_sync_support = std::env::var("NO_FDFLAGS_SYNC_SUPPORT").is_ok();
        // Current support for rights readback is buggy, lets ignore that in tests and get
        // everything working first:
        let no_rights_readback_support = std::env::var("NO_RIGHTS_READBACK_SUPPORT").is_ok();
        TestConfig {
            errno_mode,
            no_dangling_filesystem,
            no_rename_dir_to_empty_dir,
            no_fdflags_sync_support,
            no_rights_readback_support,
        }
    }
    pub fn errno_expect_unix(&self) -> bool {
        match self.errno_mode {
            ErrnoMode::Unix | ErrnoMode::MacOS => true,
            _ => false,
        }
    }
    pub fn errno_expect_macos(&self) -> bool {
        match self.errno_mode {
            ErrnoMode::MacOS => true,
            _ => false,
        }
    }
    pub fn errno_expect_windows(&self) -> bool {
        match self.errno_mode {
            ErrnoMode::Windows => true,
            _ => false,
        }
    }
    pub fn support_dangling_filesystem(&self) -> bool {
        !self.no_dangling_filesystem
    }
    pub fn support_rename_dir_to_empty_dir(&self) -> bool {
        !self.no_rename_dir_to_empty_dir
    }
    pub fn support_fdflags_sync(&self) -> bool {
        !self.no_fdflags_sync_support
    }
    // Current support for rights readback is buggy, lets ignore that in tests and get
    // everything working first:
    pub fn support_rights_readback(&self) -> bool {
        !self.no_rights_readback_support
    }
}
