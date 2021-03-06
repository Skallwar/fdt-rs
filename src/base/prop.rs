use crate::base::iters::DevTreeIter;
use crate::base::{DevTree, DevTreeNode};
use crate::prelude::*;

use unsafe_unwrap::UnsafeUnwrap;

/// A handle to a [`DevTreeNode`]'s Device Tree Property
#[derive(Clone)]
pub struct DevTreeProp<'a, 'dt: 'a> {
    parent_iter: DevTreeIter<'a, 'dt>,
    propbuf: &'dt [u8],
    nameoff: usize,
}

impl<'a, 'dt: 'a> PartialEq for DevTreeProp<'a, 'dt> {
    fn eq(&self, other: &Self) -> bool {
        self.propbuf as *const [u8] == other.propbuf as *const [u8]
            && self.parent_iter == other.parent_iter
            && self.nameoff == other.nameoff
    }
}

impl<'r, 'dt: 'r> PropReader<'dt> for DevTreeProp<'r, 'dt> {
    type NodeType = DevTreeNode<'r, 'dt>;

    #[inline]
    fn propbuf(&self) -> &'dt [u8] {
        self.propbuf
    }

    #[inline]
    fn nameoff(&self) -> usize {
        self.nameoff
    }

    #[inline]
    fn fdt(&self) -> &DevTree<'dt> {
        self.parent_iter.fdt
    }

    /// Returns the node which this property is attached to
    #[must_use]
    fn node(&self) -> DevTreeNode<'r, 'dt> {
        unsafe {
            // Unsafe unwrap okay.
            // We're look back in the tree - our parent node is behind us.
            self.parent_iter.clone().last_node().unsafe_unwrap()
        }
    }
}

impl<'a, 'dt: 'a> DevTreeProp<'a, 'dt> {
    pub(super) fn new(
        parent_iter: DevTreeIter<'a, 'dt>,
        propbuf: &'dt [u8],
        nameoff: usize,
    ) -> Self {
        Self {
            parent_iter,
            propbuf,
            nameoff,
        }
    }
}
