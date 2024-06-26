use alloc::vec::Vec;
use polkavm_common::program::RawReg;

#[derive(Copy, Clone)]
pub enum RegImm {
    Reg(RawReg),
    Imm(u32),
}

impl From<RawReg> for RegImm {
    #[inline]
    fn from(reg: RawReg) -> Self {
        RegImm::Reg(reg)
    }
}

impl From<u32> for RegImm {
    #[inline]
    fn from(value: u32) -> Self {
        RegImm::Imm(value)
    }
}

#[derive(Copy, Clone, Default)]
pub struct GuestInit<'a> {
    pub page_size: u32,
    pub ro_data: &'a [u8],
    pub rw_data: &'a [u8],
    pub ro_data_size: u32,
    pub rw_data_size: u32,
    pub stack_size: u32,
}

impl<'a> GuestInit<'a> {
    pub fn memory_map(&self) -> Result<polkavm_common::abi::MemoryMap, &'static str> {
        polkavm_common::abi::MemoryMap::new(self.page_size, self.ro_data_size, self.rw_data_size, self.stack_size)
    }
}

pub(crate) struct FlatMap<T> {
    inner: Vec<Option<T>>,
}

impl<T> FlatMap<T>
where
    T: Copy,
{
    #[inline]
    pub fn new(capacity: u32) -> Self {
        let mut inner = Vec::new();
        inner.resize_with(capacity as usize, || None);

        Self { inner }
    }

    #[inline]
    pub fn new_reusing_memory(mut memory: Self, capacity: u32) -> Self {
        memory.inner.clear();
        memory.inner.resize_with(capacity as usize, || None);
        memory
    }

    #[inline]
    pub fn get(&self, key: u32) -> Option<T> {
        self.inner.get(key as usize).and_then(|value| *value)
    }

    #[inline]
    pub fn len(&self) -> u32 {
        self.inner.len() as u32
    }

    #[inline]
    pub fn insert(&mut self, key: u32, value: T) {
        self.inner[key as usize] = Some(value);
    }

    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear();
    }
}
