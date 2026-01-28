use core::fmt;

const INTERRUPT_BIT: u64 = 0x8000_0000_0000_0000_u64;

#[repr(u64)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TrapType {
    InstructionAddressMisaligned(u64),
    InstructionAccessFault(u64),
    IllegalInstruction(u64),
    Breakpoint(u64),
    LoadAddressMisaligned(u64),
    LoadAccessFault(u64),
    StoreAddressMisaligned(u64),
    StoreAccessFault(u64),
    EnvironmentCallFromUMode = 8,
    EnvironmentCallFromSMode = 9,
    EnvironmentCallFromMMode = 11,
    InstructionPageFault(u64),
    LoadPageFault(u64),
    StorePageFault(u64),
    UserSoftwareInterrupt,
    SupervisorSoftwareInterrupt,
    MachineSoftwareInterrupt,
    UserTimerInterrupt,
    SupervisorTimerInterrupt,
    MachineTimerInterrupt,
    UserExternalInterrupt,
    SupervisorExternalInterrupt,
    MachineExternalInterrupt,
}

impl fmt::Display for TrapType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InstructionAddressMisaligned(_) => write!(f, "InstructionAddressMisaligned"),
            Self::InstructionAccessFault(_) => write!(f, "InstructionAccessFault"),
            Self::IllegalInstruction(_) => write!(f, "IllegalInstruction"),
            Self::Breakpoint(_) => write!(f, "Breakpoint"),
            Self::LoadAddressMisaligned(_) => write!(f, "LoadAddressMisaligned"),
            Self::LoadAccessFault(_) => write!(f, "LoadAccessFault"),
            Self::StoreAddressMisaligned(_) => write!(f, "StoreAddressMisaligned"),
            Self::StoreAccessFault(_) => write!(f, "StoreAccessFault"),
            Self::EnvironmentCallFromUMode => write!(f, "EnvironmentCallFromUMode"),
            Self::EnvironmentCallFromSMode => write!(f, "EnvironmentCallFromSMode"),
            Self::EnvironmentCallFromMMode => write!(f, "EnvironmentCallFromMMode"),
            Self::InstructionPageFault(_) => write!(f, "InstructionPageFault"),
            Self::LoadPageFault(_) => write!(f, "LoadPageFault"),
            Self::StorePageFault(_) => write!(f, "StorePageFault"),
            Self::UserSoftwareInterrupt => write!(f, "UserSoftwareInterrupt"),
            Self::SupervisorSoftwareInterrupt => write!(f, "SupervisorSoftwareInterrupt"),
            Self::MachineSoftwareInterrupt => write!(f, "MachineSoftwareInterrupt"),
            Self::UserTimerInterrupt => write!(f, "UserTimerInterrupt"),
            Self::SupervisorTimerInterrupt => write!(f, "SupervisorTimerInterrupt"),
            Self::MachineTimerInterrupt => write!(f, "MachineTimerInterrupt"),
            Self::UserExternalInterrupt => write!(f, "UserExternalInterrupt"),
            Self::SupervisorExternalInterrupt => write!(f, "SupervisorExternalInterrupt"),
            Self::MachineExternalInterrupt => write!(f, "MachineExternalInterrupt"),
        }
    }
}

impl TrapType {
    #[must_use]
    pub const fn idx(&self) -> u64 {
        match self {
            Self::InstructionAddressMisaligned(_) => 0,
            Self::InstructionAccessFault(_) => 1,
            Self::IllegalInstruction(_) => 2,
            Self::Breakpoint(_) => 3,
            Self::LoadAddressMisaligned(_) => 4,
            Self::LoadAccessFault(_) => 5,
            Self::StoreAddressMisaligned(_) => 6,
            Self::StoreAccessFault(_) => 7,
            Self::EnvironmentCallFromUMode => 8,
            Self::EnvironmentCallFromSMode => 9,
            Self::EnvironmentCallFromMMode => 11,
            Self::InstructionPageFault(_) => 12,
            Self::LoadPageFault(_) => 13,
            Self::StorePageFault(_) => 15,
            Self::UserSoftwareInterrupt => INTERRUPT_BIT,
            Self::SupervisorSoftwareInterrupt => INTERRUPT_BIT + 1,
            Self::MachineSoftwareInterrupt => INTERRUPT_BIT + 3,
            Self::UserTimerInterrupt => INTERRUPT_BIT + 4,
            Self::SupervisorTimerInterrupt => INTERRUPT_BIT + 5,
            Self::MachineTimerInterrupt => INTERRUPT_BIT + 7,
            Self::UserExternalInterrupt => INTERRUPT_BIT + 8,
            Self::SupervisorExternalInterrupt => INTERRUPT_BIT + 9,
            Self::MachineExternalInterrupt => INTERRUPT_BIT + 11,
        }
    }

    #[must_use]
    pub const fn is_interupt(&self) -> bool {
        ((self.idx()) & INTERRUPT_BIT) != 0
    }
    #[must_use]
    pub fn get_irq_num(&self) -> u64 {
        assert!(self.is_interupt());
        (self.idx()) & (!INTERRUPT_BIT)
    }

    #[must_use]
    pub fn get_exception_num(&self) -> u64 {
        assert!(!self.is_interupt());
        self.idx()
    }

    #[must_use]
    pub const fn get_tval(&self) -> u64 {
        // The TVAL register is only used for some types of traps.
        // See the RISC-V privilege specification for details.
        match self {
            Self::LoadPageFault(val)
            | Self::StorePageFault(val)
            | Self::StoreAccessFault(val)
            | Self::LoadAccessFault(val)
            | Self::LoadAddressMisaligned(val)
            | Self::StoreAddressMisaligned(val)
            | Self::InstructionAccessFault(val)
            | Self::InstructionPageFault(val)
            | Self::InstructionAddressMisaligned(val)
            | Self::Breakpoint(val)
            | Self::IllegalInstruction(val) => *val,
            _ => 0,
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum DebugCause {
    NoDebug = 0,
    Ebreak = 1,
    Trigger = 2,
    HaltReq = 3,
    Step = 4,
    ResetHaltReq = 5,
    Group = 6,
}

impl DebugCause {
    #[must_use]
    pub fn from_usize(val: usize) -> Self {
        match val {
            0 => Self::NoDebug,
            1 => Self::Ebreak,
            2 => Self::Trigger,
            3 => Self::HaltReq,
            4 => Self::Step,
            5 => Self::ResetHaltReq,
            6 => Self::Group,
            _ => panic!("Invalid DebugCause"),
        }
    }
}
