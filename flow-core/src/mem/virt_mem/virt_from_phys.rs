use super::{VirtualReadIterator, VirtualWriteIterator};
use crate::architecture::Architecture;
use crate::error::{Error, Result};
use crate::mem::{
    virt_translate::{translate_arch, TranslateArch, VirtualTranslate},
    PhysicalMemory, VirtualMemory,
};
use crate::process::OsProcessInfo;
use crate::types::{Address, Page};

pub struct VirtualFromPhysical<T: PhysicalMemory, V: VirtualTranslate> {
    phys_mem: T,
    sys_arch: Architecture,
    vat: V,
    proc_arch: Architecture,
    dtb: Address,
}

impl<T: PhysicalMemory> VirtualFromPhysical<T, TranslateArch> {
    pub fn new(phys_mem: T, sys_arch: Architecture, proc_arch: Architecture, dtb: Address) -> Self {
        Self {
            phys_mem,
            sys_arch,
            vat: TranslateArch::new(sys_arch),
            proc_arch,
            dtb,
        }
    }

    pub fn with_process_info<U: OsProcessInfo>(phys_mem: T, process_info: U) -> Self {
        Self {
            phys_mem,
            sys_arch: process_info.sys_arch(),
            vat: TranslateArch::new(process_info.sys_arch()),
            proc_arch: process_info.proc_arch(),
            dtb: process_info.dtb(),
        }
    }
}

impl<T: PhysicalMemory, V: VirtualTranslate> VirtualFromPhysical<T, V> {
    pub fn with_vat(
        phys_mem: T,
        sys_arch: Architecture,
        proc_arch: Architecture,
        dtb: Address,
        vat: V,
    ) -> Self {
        Self {
            phys_mem,
            sys_arch,
            vat,
            proc_arch,
            dtb,
        }
    }

    pub fn sys_arch(&self) -> Architecture {
        self.sys_arch
    }

    pub fn vat(&mut self) -> &mut V {
        &mut self.vat
    }

    pub fn proc_arch(&self) -> Architecture {
        self.proc_arch
    }

    pub fn dtb(&self) -> Address {
        self.dtb
    }

    pub fn virt_read_addr(&mut self, addr: Address) -> Result<Address> {
        match self.proc_arch.bits() {
            64 => self.virt_read_addr64(addr),
            32 => self.virt_read_addr32(addr),
            _ => Err(Error::new("invalid instruction set address size")),
        }
    }
}

impl<T: PhysicalMemory, V: VirtualTranslate> VirtualMemory for VirtualFromPhysical<T, V> {
    fn virt_read_raw_iter<'a, VI: VirtualReadIterator<'a>>(&mut self, iter: VI) -> Result<()> {
        translate_arch::virt_read_raw_iter(
            &mut self.phys_mem,
            &mut self.vat,
            self.sys_arch,
            self.dtb,
            iter,
        )
    }

    fn virt_write_raw_iter<'a, VI: VirtualWriteIterator<'a>>(&mut self, iter: VI) -> Result<()> {
        translate_arch::virt_write_raw_iter(
            &mut self.phys_mem,
            &mut self.vat,
            self.sys_arch,
            self.dtb,
            iter,
        )
    }

    fn virt_page_info(&mut self, addr: Address) -> Result<Page> {
        translate_arch::virt_page_info(
            &mut self.phys_mem,
            &mut self.vat,
            self.sys_arch,
            self.dtb,
            addr,
        )
    }
}
