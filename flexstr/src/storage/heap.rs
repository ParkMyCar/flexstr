use crate::custom::Size;
use crate::storage::StorageType;
use crate::string::Str;

#[derive(Clone)]
#[repr(C)]
pub(crate) struct HeapStr<SIZE, HEAP, STR>
where
    SIZE: Size<STR>,
    STR: Str + ?Sized,
{
    pub heap: HEAP,
    pad: SIZE::HeapPad,
    pub marker: StorageType,
}
