pub mod pdb_struct;
pub use pdb_struct::PdbStruct;

pub mod pdb_cache;

use std::path::Path;

use crate::error::{Error, Result};
use crate::kernel::ntos::Win32GUID;

#[derive(Debug, Clone)]
pub struct Win32Offsets {
    pub list_blink: usize,
    pub eproc_link: usize,

    pub kproc_dtb: usize,
    pub eproc_pid: usize,
    pub eproc_name: usize,
    pub eproc_peb: usize,
    pub eproc_wow64: usize,

    pub peb_ldr_x86: usize,
    pub peb_ldr_x64: usize,
    pub ldr_list_x86: usize,
    pub ldr_list_x64: usize,

    pub ldr_data_base_x86: usize,
    pub ldr_data_base_x64: usize,
    pub ldr_data_size_x86: usize,
    pub ldr_data_size_x64: usize,
    pub ldr_data_name_x86: usize,
    pub ldr_data_name_x64: usize,
}

// initialize from pdb -> open pdb by file / by guid
// initialize from guid
// initialize manually

impl Win32Offsets {
    pub fn try_with_pdb(pdb_path: &Path) -> Result<Self> {
        let list = PdbStruct::with(pdb_path, "_LIST_ENTRY")?;
        let kproc = PdbStruct::with(pdb_path, "_KPROCESS")?;
        let eproc = PdbStruct::with(pdb_path, "_EPROCESS")?;

        let list_blink = list
            .find_field("Blink")
            .ok_or_else(|| Error::new("_LIST_ENTRY::Blink not found"))?
            .offset;

        let eproc_link = eproc
            .find_field("ActiveProcessLinks")
            .ok_or_else(|| Error::new("_EPROCESS::ActiveProcessLinks not found"))?
            .offset;

        let kproc_dtb = kproc
            .find_field("DirectoryTableBase")
            .ok_or_else(|| Error::new("_KPROCESS::DirectoryTableBase not found"))?
            .offset;
        let eproc_pid = eproc
            .find_field("UniqueProcessId")
            .ok_or_else(|| Error::new("_EPROCESS::UniqueProcessId not found"))?
            .offset;
        let eproc_name = eproc
            .find_field("ImageFileName")
            .ok_or_else(|| Error::new("_EPROCESS::ImageFileName not found"))?
            .offset;
        let eproc_peb = eproc
            .find_field("Peb")
            .ok_or_else(|| Error::new("_EPROCESS::Peb not found"))?
            .offset;
        let eproc_wow64 = match eproc.find_field("WoW64Process") {
            Some(f) => f.offset,
            None => 0,
        };

        Ok(Self {
            list_blink,
            eproc_link,
            kproc_dtb,
            eproc_pid,
            eproc_name,
            eproc_peb,
            eproc_wow64,
            peb_ldr_x86: 0xC,        // _PEB::Ldr
            peb_ldr_x64: 0x18,       // _PEB::Ldr
            ldr_list_x86: 0xC,       // _PEB_LDR_DATA::InLoadOrderModuleList
            ldr_list_x64: 0x10,      // _PEB_LDR_DATA::InLoadOrderModuleList
            ldr_data_base_x86: 0x18, // _LDR_DATA_TABLE_ENTRY::DllBase
            ldr_data_base_x64: 0x30, // _LDR_DATA_TABLE_ENTRY::DllBase
            ldr_data_size_x86: 0x20, // _LDR_DATA_TABLE_ENTRY::SizeOfImage
            ldr_data_size_x64: 0x40, // _LDR_DATA_TABLE_ENTRY::SizeOfImage
            ldr_data_name_x86: 0x2C, // _LDR_DATA_TABLE_ENTRY::BaseDllName
            ldr_data_name_x64: 0x58, // _LDR_DATA_TABLE_ENTRY::BaseDllName
        })
    }

    pub fn try_with_guid(guid: &Win32GUID) -> Result<Self> {
        let pdb = pdb_cache::try_get_pdb(guid)?;
        Self::try_with_pdb(&pdb)
    }
}
