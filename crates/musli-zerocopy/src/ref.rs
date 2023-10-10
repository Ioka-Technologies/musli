use core::marker::PhantomData;

use crate::buf::{Buf, BufMut};
use crate::error::Error;
use crate::ptr::Ptr;
use crate::zero_copy::ZeroCopy;

/// A sized reference.
///
/// This is used to type a pointer with a [`ZeroCopy`] parameter so that it can
/// be used in combination with [`Buf`] to load the value from a buffer.
///
/// Note that the constructor is safe, because alignment and validation checks
/// happens whenever a value is loaded from a bare buffer.
///
/// # Examples
///
/// ```
/// use core::mem::align_of;
/// use musli_zerocopy::{AlignedBuf, Ref, Ptr};
///
/// let mut buf = AlignedBuf::with_alignment(align_of::<u32>());
/// buf.extend_from_slice(&[1, 2, 3, 4]);
///
/// let buf = buf.as_buf()?;
///
/// let number = Ref::<u32>::new(Ptr::ZERO);
/// assert_eq!(*buf.load(number)?, u32::from_ne_bytes([1, 2, 3, 4]));
/// # Ok::<_, musli_zerocopy::Error>(())
/// ```
#[derive(Debug)]
#[repr(C)]
pub struct Ref<T> {
    ptr: Ptr,
    _marker: PhantomData<T>,
}

impl<T> Ref<T>
where
    T: ZeroCopy,
{
    /// Construct a typed reference to the first position in a buffer.
    pub const fn zero() -> Self {
        Self::new(Ptr::ZERO)
    }

    /// Construct a reference wrapping the given type at the specified address.
    pub const fn new(ptr: Ptr) -> Self {
        Self {
            ptr,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub(crate) fn ptr(&self) -> Ptr {
        self.ptr
    }
}

unsafe impl<T> ZeroCopy for Ref<T> {
    const ANY_BITS: bool = true;

    fn write_to<B: ?Sized>(&self, buf: &mut B) -> Result<(), Error>
    where
        B: BufMut,
    {
        buf.write(&self.ptr)?;
        Ok(())
    }

    fn coerce(buf: &Buf) -> Result<&Self, Error> {
        let mut v = buf.validate::<Self>()?;
        v.field::<Ptr>()?;
        v.end()?;
        Ok(unsafe { buf.cast() })
    }

    unsafe fn validate(buf: &Buf) -> Result<(), Error> {
        let mut v = buf.validate_unchecked::<Self>()?;
        v.field::<Ptr>()?;
        v.end()
    }
}

impl<T> Clone for Ref<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Ref<T> {}
