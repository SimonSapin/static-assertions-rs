#![no_std]

#[macro_use]
extern crate static_assertions;

use core::ops::Range;

trait Tri<A: ?Sized, B: ?Sized, C: ?Sized> {}

impl<T, A: ?Sized, B: ?Sized, C: ?Sized> Tri<A, B, C> for T {}

assert_impl!(tri; u64, Tri<[&'static u8], Tri<Send, Sync, str>, (u16, u16)>);
assert_impl!(byte; u8, Send, Sync);
assert_impl!(iter; &[u8], IntoIterator /* TODO: <Item=&u8> */);
assert_impl!(range; Range<u8>, Iterator<Item=u8>);
assert_impl!(slice; [u8], Send, Sync, AsRef<[u8]>);

#[cfg(feature = "failure")]
assert_impl!(ptr; *const u8, Send, Sync);

#[test]
fn str_impl() {
    assert_impl!(str, Send, Sync, AsRef<[u8]>);
}
