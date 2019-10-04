use super::m_non_packed_filter::*;
use super::m_non_packed_with_id::*;
use super::{AbstractMut, IntoAbstract};
use crate::entity::Key;
#[cfg(feature = "parallel")]
use rayon::iter::plumbing::{Folder, UnindexedProducer};

macro_rules! impl_iterators {
    (
        $number: literal
        $non_packed: ident
        $non_packed_filter: ident
        $non_packed_with_id: ident
        $(($type: ident, $index: tt))+
    ) => {
        #[doc = "Non packed iterator over"]
        #[doc = $number]
        #[doc = "components.\n"]
        pub struct $non_packed<$($type: IntoAbstract),+> {
            pub(super) data: ($($type::AbsView,)+),
            pub(super) indices: *const Key,
            pub(super) current: usize,
            pub(super) end: usize,
            pub(super) array: usize,
        }

        impl<$($type: IntoAbstract),+> $non_packed<$($type),+> {
            pub fn filtered<P: FnMut(&<Self as Iterator>::Item) -> bool>(self, pred: P) -> $non_packed_filter<$($type),+, P> {
                $non_packed_filter {
                    iter: self,
                    pred
                }
            }
            pub fn with_id(self) -> $non_packed_with_id<$($type),+> {
                $non_packed_with_id(self)
            }
        }

        unsafe impl<$($type: IntoAbstract),+> Send for $non_packed<$($type),+> {}

        impl<$($type: IntoAbstract),+> $non_packed<$($type),+> {
            #[cfg(feature = "parallel")]
            pub(super) fn clone(&self) -> Self {
                $non_packed {
                    data: self.data.clone(),
                    indices: self.indices,
                    current: self.current,
                    end: self.end,
                    array: self.array,
                }
            }
        }

        impl<$($type: IntoAbstract),+> Iterator for $non_packed<$($type,)+> {
            type Item = ($(<$type::AbsView as AbstractMut>::Out,)+);
            fn next(&mut self) -> Option<Self::Item> {
                while self.current < self.end {
                    // SAFE at this point there are no mutable reference to sparse or dense
                    // and self.indices can't access out of bounds
                    let index = unsafe { std::ptr::read(self.indices.add(self.current)) };
                    self.current += 1;
                    let data_indices = ($(
                        if $index == self.array {
                            self.current - 1
                        } else {
                            if let Some(index) = self.data.$index.index_of(index) {
                                index
                            } else {
                                continue
                            }
                        },
                    )+);
                    unsafe {
                        return Some(($(self.data.$index.get_data(data_indices.$index),)+))
                    }
                }
                None
            }
        }

        #[cfg(feature = "parallel")]
        impl<$($type: IntoAbstract),+> UnindexedProducer for $non_packed<$($type,)+> {
            type Item = ($(<$type::AbsView as AbstractMut>::Out,)+);
            fn split(mut self) -> (Self, Option<Self>) {
                let len = self.end - self.current;
                if len >= 2 {
                    let mut clone = self.clone();
                    clone.current += len / 2;
                    self.end = clone.current;
                    (self, Some(clone))
                } else {
                    (self, None)
                }
            }
            fn fold_with<Fold>(self, folder: Fold) -> Fold
            where Fold: Folder<Self::Item> {
                folder.consume_iter(self)
            }
        }
    }
}

macro_rules! iterators {
    (
        $($number: literal)*; $number1: literal $($queue_number: literal)+;
        $($non_packed: ident)*; $non_packed1: ident $($queue_non_packed: ident)+;
        $($non_packed_filter: ident)*; $non_packed_filter1: ident $($queue_non_packed_filter: ident)+;
        $($non_packed_with_id: ident)*; $non_packed_with_id1: ident $($queue_non_packed_with_id: ident)+;
        $(($type: ident, $index: tt))*;($type1: ident, $index1: tt) $(($queue_type: ident, $queue_index: tt))*
    ) => {
        impl_iterators![$number1 $non_packed1 $non_packed_filter1 $non_packed_with_id1 $(($type, $index))*];
        iterators![
            $($number)* $number1; $($queue_number)+;
            $($non_packed)* $non_packed1; $($queue_non_packed)+;
            $($non_packed_filter)* $non_packed_filter1; $($queue_non_packed_filter)+;
            $($non_packed_with_id)* $non_packed_with_id1; $($queue_non_packed_with_id)+;
            $(($type, $index))* ($type1, $index1); $(($queue_type, $queue_index))*
        ];
    };
    (
        $($number: literal)*; $number1: literal;
        $($non_packed: ident)*; $non_packed1: ident;
        $($non_packed_filter: ident)*; $non_packed_filter1: ident;
        $($non_packed_with_id: ident)*; $non_packed_with_id1: ident;
        $(($type: ident, $index: tt))*;
    ) => {
        impl_iterators![$number1 $non_packed1 $non_packed_filter1 $non_packed_with_id1 $(($type, $index))*];
    }
}

iterators![
    ;"2" "3" "4" "5" "6" "7" "8" "9" "10";
    ;NonPacked2 NonPacked3 NonPacked4 NonPacked5 NonPacked6 NonPacked7 NonPacked8 NonPacked9 NonPacked10;
    ;NonPackedFilter2 NonPackedFilter3 NonPackedFilter4 NonPackedFilter5 NonPackedFilter6 NonPackedFilter7 NonPackedFilter8 NonPackedFilter9 NonPackedFilter10;
    ;NonPackedWithId2 NonPackedWithId3 NonPackedWithId4 NonPackedWithId5 NonPackedWithId6 NonPackedWithId7 NonPackedWithId8 NonPackedWithId9 NonPackedWithId10;
    (A, 0) (B, 1); (C, 2) (D, 3) (E, 4) (F, 5) (G, 6) (H, 7) (I, 8) (J, 9)
];
