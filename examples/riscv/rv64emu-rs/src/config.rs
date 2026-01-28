use log::info;

use crate::rv64core::csr_regs_define::StapMode;

const IMPLMENTED_ISA: [u8; 4] = [b'i', b'm', b'a', b'c'];

#[derive(Debug)]
pub struct Config {
    icache_size: Option<usize>,
    dcache_size: Option<usize>,
    decode_cache_size: Option<usize>,
    tlb_size: Option<usize>,
    mmu_type: StapMode,
    s_mode: bool,
    u_mode: bool,
    isa_falgs: u32,
    disable_check_tohost: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            icache_size: Option::default(),
            dcache_size: Option::default(),
            decode_cache_size: Option::default(),
            tlb_size: Option::default(),
            mmu_type: StapMode::Bare,
            isa_falgs: 0,
            s_mode: false,
            u_mode: false,
            disable_check_tohost: false,
        }
    }
}

impl Config {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    pub const fn set_s_mode(&mut self) {
        self.s_mode = true;
        self.u_mode = true;
    }
    pub const fn set_u_mode(&mut self) {
        self.u_mode = true;
    }

    pub const fn set_icache_size(&mut self, size: usize) {
        self.icache_size = Some(size);
    }
    pub const fn set_dcache_size(&mut self, size: usize) {
        self.dcache_size = Some(size);
    }
    pub const fn set_decode_cache_size(&mut self, size: usize) {
        self.decode_cache_size = Some(size);
    }
    pub const fn set_tlb_size(&mut self, size: usize) {
        self.tlb_size = Some(size);
    }
    // sv39 sv48
    /// # Panics
    /// TODO
    pub fn set_mmu_type(&mut self, mmu_type: &str) {
        let mmu_type = mmu_type.to_lowercase();
        match mmu_type.as_str() {
            "bare" => self.mmu_type = StapMode::Bare,
            "sv39" => self.mmu_type = StapMode::Sv39,
            "sv48" => self.mmu_type = StapMode::Sv48,
            "sv57" => self.mmu_type = StapMode::Sv57,
            err => panic!("mmu type err:{err}"),
        }
    }
    // TODO: parse isa string
    /// # Panics
    /// TODO
    pub fn set_isa(&mut self, isa_str: &str) {
        let isa_str = isa_str.to_ascii_lowercase();
        info!("isa_str:{isa_str:?}");
        isa_str.strip_prefix("rv64").map_or_else(
            || panic!("isa err:{isa_str}"),
            |f| {
                for i in f.bytes() {
                    if IMPLMENTED_ISA.contains(&i) {
                        let idx = i - b'a';
                        self.isa_falgs |= 1 << idx;
                    }
                }
            },
        );
    }

    pub const fn set_disable_check_tohost(&mut self, disable: bool) {
        self.disable_check_tohost = disable;
    }

    #[must_use]
    pub const fn disable_check_tohost(&self) -> bool {
        self.disable_check_tohost
    }

    #[must_use]
    pub const fn is_enable_isa(&self, isa: u8) -> bool {
        let idx = isa - b'a';
        self.isa_falgs & (1 << idx) != 0
    }

    #[must_use]
    pub const fn get_mmu_type(&self) -> StapMode {
        self.mmu_type
    }

    #[must_use]
    pub const fn icache_size(&self) -> Option<usize> {
        self.icache_size
    }
    #[must_use]
    pub const fn dcache_size(&self) -> Option<usize> {
        self.dcache_size
    }
    #[must_use]
    pub const fn decode_cache_size(&self) -> Option<usize> {
        self.decode_cache_size
    }
    #[must_use]
    pub const fn tlb_size(&self) -> Option<usize> {
        self.tlb_size
    }

    #[must_use]
    pub const fn s_mode(&self) -> bool {
        self.s_mode
    }
    #[must_use]
    pub const fn u_mode(&self) -> bool {
        self.u_mode
    }
}

#[test]
fn config_isa_test() {
    simple_logger::SimpleLogger::new().init().unwrap();
    let mut config = Config::new();
    config.set_isa("RV64IMAC_zicsr");

    assert!(config.is_enable_isa(b'i'));
    assert!(config.is_enable_isa(b'm'));
    assert!(config.is_enable_isa(b'a'));
    assert!(config.is_enable_isa(b'c'));

    assert!(!config.is_enable_isa(b'f'));
    assert!(!config.is_enable_isa(b'd'));
}
