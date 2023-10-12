use crate::error::Error;
use crate::traits::ZeroCopy;

mod sealed {
    #[cfg(feature = "alloc")]
    use crate::traits::ZeroCopy;

    pub trait Sealed {}

    #[cfg(feature = "alloc")]
    impl<'a, T> Sealed for crate::buf::aligned_buf::AlignedBufStoreStruct<'a, T> where T: ZeroCopy {}
}

/// A writer as returned from [`BufMut::store_struct`].
///
/// [`BufMut::store_struct`]: crate::buf::BufMut::store_struct
pub trait StoreStruct<T: ZeroCopy>: self::sealed::Sealed {
    /// Pad around the given field with zeros.
    ///
    /// Note that this is necessary to do correctly in order to satisfy the
    /// safety requirements by [`finish()`].
    ///
    /// This is typically not called directly, but rather is implemented by the
    /// [`ZeroCopy`] derive.
    ///
    /// [`finish()`]: Self::finish
    /// [`ZeroCopy`]: derive@crate::ZeroCopy
    ///
    /// # Examples
    ///
    /// ```
    /// use musli_zerocopy::{AlignedBuf, ZeroCopy};
    /// use musli_zerocopy::buf::StoreStruct;
    ///
    /// #[derive(Debug, PartialEq, Eq, ZeroCopy)]
    /// #[repr(C)]
    /// struct ZeroPadded(u8, u16);
    ///
    /// let padded = ZeroPadded(0x01u8.to_be(), 0x0203u16.to_be());
    ///
    /// let mut buf = AlignedBuf::new();
    ///
    /// // SAFETY: We do not pad beyond known fields and are
    /// // making sure to initialize all of the buffer.
    /// unsafe {
    ///     let mut w = buf.store_struct(&padded);
    ///     w.pad::<u8>();
    ///     w.pad::<u16>();
    ///     w.finish();
    /// }
    ///
    /// // Note: The bytes are explicitly convert to big-endian encoding above.
    /// assert_eq!(buf.as_slice(), &[1, 0, 2, 3]);
    /// # Ok::<_, musli_zerocopy::Error>(())
    /// ```
    fn pad<F>(&mut self)
    where
        F: ZeroCopy;

    /// Finish writing the current buffer.
    ///
    /// This is typically not called directly, but rather is implemented by the
    /// [`ZeroCopy`] derive.
    ///
    /// [`ZeroCopy`]: derive@crate::ZeroCopy
    ///
    /// # Safety
    ///
    /// Before calling `finish()`, the caller must ensure that they've called
    /// [`pad::<F>()`] *in order* for every field in a struct being serialized
    /// where `F` is the type of the field. Otherwise we might not have written
    /// the necessary padding to ensure that all bytes related to the struct are
    /// initialized. Failure to do so would result in undefined behavior.
    ///
    /// Fields which are [`ZeroSized`] can be skipped.
    ///
    /// [`pad::<F>()`]: Self::pad
    /// [`ZeroSized`]: crate::traits::ZeroSized
    ///
    /// # Examples
    ///
    /// ```
    /// use musli_zerocopy::{AlignedBuf, ZeroCopy};
    /// use musli_zerocopy::buf::StoreStruct;
    /// use musli_zerocopy::pointer::Ref;
    ///
    /// #[derive(Debug, PartialEq, Eq, ZeroCopy)]
    /// #[repr(C)]
    /// struct ZeroPadded(u8, u16);
    ///
    /// let padded = ZeroPadded(0x01u8.to_be(), 0x0203u16.to_be());
    ///
    /// let mut buf = AlignedBuf::new();
    ///
    /// let reference = Ref::<ZeroPadded>::new(buf.next_offset::<ZeroPadded>());
    ///
    /// // SAFETY: We do not pad beyond known fields and are
    /// // making sure to initialize all of the buffer.
    /// unsafe {
    ///     let mut w = buf.store_struct(&padded);
    ///     w.pad::<u8>();
    ///     w.pad::<u16>();
    ///     w.finish();
    /// }
    ///
    /// // Note: The bytes are explicitly convert to big-endian encoding above.
    /// assert_eq!(buf.as_slice(), &[1, 0, 2, 3]);
    ///
    /// let buf = buf.as_aligned();
    ///
    /// assert_eq!(buf.load(reference)?, &padded);
    /// # Ok::<_, musli_zerocopy::Error>(())
    /// ```
    unsafe fn finish(self) -> Result<(), Error>;
}
