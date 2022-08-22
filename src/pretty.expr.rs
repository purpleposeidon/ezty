&[
    // This file is here so that you can easily patch it with cargo.
    ("alloc::boxed::", "Box"),
    ("alloc::collections::binary_heap::", "BinaryHeap"),
    ("alloc::collections::btree::map::", "BTreeMap"),
    ("alloc::collections::btree::set::", "BTreeSet"),
    ("alloc::collections::linked_list::", "LinkedList"),
    ("alloc::collections::vec_deque::", "VecDeque"),
    ("alloc::sync::", "Arc"),
    ("alloc::vec::", "Vec"),
    ("core::cell::", "Cell"),
    ("core::cell::", "RefCell"),
    ("core::option::", "Option"),
    ("core::result::", "Result"),
    ("std::collections::hash::map::", "HashMap"),
    ("std::collections::hash::set::", "HashSet"),
    ("std::sync::rwlock::", "RwLock"),
]
