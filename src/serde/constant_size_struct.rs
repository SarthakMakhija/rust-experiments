use std::mem;
use rkyv::{AlignedVec, Archive, Deserialize, Serialize};
use rkyv::ser::Serializer;
use rkyv::ser::serializers::AllocSerializer;

const MAX_HEIGHT: usize = 20;

const NODE_SIZE: usize = mem::size_of::<Node>();

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(Debug))]
pub struct Node {
    key_size: u16,
    key_offset: u32,
    value: u64,
    height: u16,
    tower: [u32; MAX_HEIGHT],
}

impl Node {
    pub fn new() -> Self {
        return Node {
            key_size: 20,
            key_offset: 32,
            value: 500,
            height: 8,
            tower: [10; MAX_HEIGHT],
        };
    }

    /**
        Preallocate the number of bytes that is equal to the size of the node.
        Takes around 160 ns to serialize on: MacBook Pro (16-inch, 2019), 2.6 GHz 6-Core Intel Core i7, 16 GB 2667 MHz DDR4
    **/
    pub fn serialize(&self) -> AlignedVec {
        return rkyv::to_bytes::<_, NODE_SIZE>(self).unwrap();
    }

    /**
        Use AllocSerializer
        Takes around 90 ns to serialize on: MacBook Pro (16-inch, 2019), 2.6 GHz 6-Core Intel Core i7, 16 GB 2667 MHz DDR4
    **/
    pub fn custom_serialize(&self) -> AlignedVec {
        let mut serializer = AllocSerializer::<0>::default();
        serializer.serialize_value(self).unwrap();
        let bytes = serializer.into_serializer().into_inner();
        return bytes;
    }

    fn deserialize<'a>(&'a self, bytes: &'a [u8]) -> &'a ArchivedNode {
        let archived_node = unsafe { rkyv::archived_root::<Node>(bytes) };
        return archived_node;
    }
}

#[cfg(test)]
mod tests {
    use crate::serde::constant_size_struct::Node;

    #[test]
    fn serialize_deserialize() {
        let node = Node::new();
        let serialized = node.serialize();

        let archived_node = node.deserialize(serialized.as_slice());
        let expected = Node::new();

        assert_eq!(&expected, archived_node);
    }
}