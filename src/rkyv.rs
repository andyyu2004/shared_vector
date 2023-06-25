use ::rkyv::{Archive, Deserialize, Serialize};
use allocator_api2::alloc::Allocator;
use rkyv::ser::{ScratchSpace, Serializer};
use rkyv::vec::{ArchivedVec, VecResolver};
use rkyv::{Archived, Fallible};

use crate::{DefaultRefCount, RefCount, RefCountedVector, SharedVector, Vector};

impl<T: Archive, R: RefCount, A: Allocator> Archive for RefCountedVector<T, R, A> {
    type Archived = ArchivedVec<Archived<T>>;
    type Resolver = VecResolver;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        ArchivedVec::resolve_from_slice(self.as_slice(), pos, resolver, out);
    }
}

impl<T: Serialize<S>, R: RefCount, A: Allocator, S: ScratchSpace + Serializer + ?Sized> Serialize<S>
    for RefCountedVector<T, R, A>
{
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        ArchivedVec::serialize_from_slice(self.as_slice(), serializer)
    }
}

impl<T, A, D> Deserialize<SharedVector<T, A>, D> for ArchivedVec<Archived<T>>
where
    Archived<T>: Deserialize<T, D>,
    T: Archive,
    A: Allocator + Clone + Default,
    D: Fallible + ?Sized,
{
    #[inline]
    fn deserialize(&self, deserializer: &mut D) -> Result<SharedVector<T, A>, D::Error> {
        let mut result = Vector::new_in(A::default());
        for item in self.as_slice() {
            result.push(item.deserialize(deserializer)?);
        }

        Ok(result.into_shared())
    }
}
