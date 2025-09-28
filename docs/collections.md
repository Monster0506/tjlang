# TJLang Collections Module Documentation

This document describes the comprehensive collection functionality available in TJLang's `COLLECTIONS` module. This module provides powerful data structures and algorithms for working with arrays, maps, sets, queues, and more.

## Table of Contents

- [Array/Vector Operations](#arrayvector-operations) - Dynamic array functionality
- [Map/Dictionary Operations](#mapdictionary-operations) - Key-value storage
- [Set Operations](#set-operations) - Unique element collections
- [Queue Operations](#queue-operations) - FIFO data structures
- [Priority Queue Operations](#priority-queue-operations) - Heap-based priority queues
- [BTree Operations](#btree-operations) - Ordered tree structures
- [Iterator Utilities](#iterator-utilities) - Collection iteration and transformation
- [Collection Algorithms](#collection-algorithms) - Sorting, searching, and filtering
- [Advanced Collection Types](#advanced-collection-types) - Counter, DefaultDict, ChainMap, NamedTuple, OrderedDict, Deque, Heap
- [Examples](#examples) - Usage examples

## Array/Vector Operations

Dynamic arrays that can grow and shrink as needed.

### Creation and Initialization

#### `array_new<T>() -> Vec<T>`
Creates a new empty array.

```tjlang
arr: Vec<int> = COLLECTIONS.array_new()
```

#### `array_with_capacity<T>(capacity: int) -> Vec<T>`
Creates a new array with the specified initial capacity.

```tjlang
arr: Vec<int> = COLLECTIONS.array_with_capacity(10)
```

#### `array_from_slice<T>(slice: &[T]) -> Vec<T>`
Creates an array from a slice.

```tjlang
slice: &[int] = [1, 2, 3]
arr: Vec<int> = COLLECTIONS.array_from_slice(slice)
```

### Element Manipulation

#### `array_push<T>(arr: &mut Vec<T>, item: T)`
Adds an element to the end of the array.

```tjlang
arr: Vec<int> = [1, 2, 3]
COLLECTIONS.array_push(&mut arr, 4)  # arr is now [1, 2, 3, 4]
```

#### `array_pop<T>(arr: &mut Vec<T>) -> Option<T>`
Removes and returns the last element from the array.

```tjlang
arr: Vec<int> = [1, 2, 3]
last: Option<int> = COLLECTIONS.array_pop(&mut arr)  # last is Some(3), arr is [1, 2]
```

#### `array_insert<T>(arr: &mut Vec<T>, index: int, item: T) -> Result<(), str>`
Inserts an element at the specified index.

```tjlang
arr: Vec<int> = [1, 2, 4]
result: Result<(), str> = COLLECTIONS.array_insert(&mut arr, 2, 3)  # arr is now [1, 2, 3, 4]
```

#### `array_remove<T>(arr: &mut Vec<T>, index: int) -> Result<T, str>`
Removes and returns the element at the specified index.

```tjlang
arr: Vec<int> = [1, 2, 3, 4]
removed: Result<int, str> = COLLECTIONS.array_remove(&mut arr, 1)  # removed is Ok(2), arr is [1, 3, 4]
```

### Access and Information

#### `array_len<T>(arr: &Vec<T>) -> int`
Returns the number of elements in the array.

```tjlang
arr: Vec<int> = [1, 2, 3]
length: int = COLLECTIONS.array_len(&arr)  # length is 3
```

#### `array_capacity<T>(arr: &Vec<T>) -> int`
Returns the current capacity of the array.

```tjlang
arr: Vec<int> = [1, 2, 3]
capacity: int = COLLECTIONS.array_capacity(&arr)
```

#### `array_is_empty<T>(arr: &Vec<T>) -> bool`
Checks if the array is empty.

```tjlang
arr: Vec<int> = []
empty: bool = COLLECTIONS.array_is_empty(&arr)  # empty is true
```

#### `array_get<T>(arr: &Vec<T>, index: int) -> Option<&T>`
Gets a reference to the element at the specified index.

```tjlang
arr: Vec<int> = [1, 2, 3]
element: Option<&int> = COLLECTIONS.array_get(&arr, 1)  # element is Some(&2)
```

#### `array_set<T>(arr: &mut Vec<T>, index: int, item: T) -> Result<(), str>`
Sets the element at the specified index.

```tjlang
arr: Vec<int> = [1, 2, 3]
result: Result<(), str> = COLLECTIONS.array_set(&mut arr, 1, 5)  # arr is now [1, 5, 3]
```

### Array Operations

#### `array_slice<T>(arr: &Vec<T>, start: int, end: int) -> Vec<T>`
Creates a new array containing elements from start to end (exclusive).

```tjlang
arr: Vec<int> = [1, 2, 3, 4, 5]
slice: Vec<int> = COLLECTIONS.array_slice(&arr, 1, 4)  # slice is [2, 3, 4]
```

#### `array_append<T>(arr: &mut Vec<T>, other: &mut Vec<T>)`
Appends all elements from other to arr, consuming other.

```tjlang
arr1: Vec<int> = [1, 2]
arr2: Vec<int> = [3, 4]
COLLECTIONS.array_append(&mut arr1, &mut arr2)  # arr1 is [1, 2, 3, 4], arr2 is empty
```

#### `array_extend<T>(arr: &mut Vec<T>, other: &[T])`
Extends arr with elements from other.

```tjlang
arr: Vec<int> = [1, 2]
other: &[int] = [3, 4]
COLLECTIONS.array_extend(&mut arr, other)  # arr is [1, 2, 3, 4]
```

### Sorting and Ordering

#### `array_reverse<T>(arr: &mut Vec<T>)`
Reverses the order of elements in the array.

```tjlang
arr: Vec<int> = [1, 2, 3, 4]
COLLECTIONS.array_reverse(&mut arr)  # arr is now [4, 3, 2, 1]
```

#### `array_sort<T>(arr: &mut Vec<T>)`
Sorts the array in ascending order.

```tjlang
arr: Vec<int> = [3, 1, 4, 1, 5]
COLLECTIONS.array_sort(&mut arr)  # arr is now [1, 1, 3, 4, 5]
```

#### `array_shuffle<T>(arr: &mut Vec<T>)`
Randomly shuffles the elements in the array.

```tjlang
arr: Vec<int> = [1, 2, 3, 4, 5]
COLLECTIONS.array_shuffle(&mut arr)  # arr is now in random order
```

#### `array_unique<T>(arr: &mut Vec<T>)`
Removes duplicate elements from the array.

```tjlang
arr: Vec<int> = [1, 2, 2, 3, 3, 3]
COLLECTIONS.array_unique(&mut arr)  # arr is now [1, 2, 3]
```

### Functional Operations

#### `array_filter<T>(arr: &Vec<T>, predicate: Fn(&T) -> bool) -> Vec<T>`
Creates a new array containing only elements that satisfy the predicate.

```tjlang
arr: Vec<int> = [1, 2, 3, 4, 5]
evens: Vec<int> = COLLECTIONS.array_filter(&arr, |x| x % 2 == 0)  # evens is [2, 4]
```

#### `array_map<T, U>(arr: &Vec<T>, mapper: Fn(&T) -> U) -> Vec<U>`
Creates a new array by applying the mapper function to each element.

```tjlang
arr: Vec<int> = [1, 2, 3, 4]
doubled: Vec<int> = COLLECTIONS.array_map(&arr, |x| x * 2)  # doubled is [2, 4, 6, 8]
```

#### `array_reduce<T>(arr: &Vec<T>, initial: T, reducer: Fn(T, &T) -> T) -> T`
Reduces the array to a single value using the reducer function.

```tjlang
arr: Vec<int> = [1, 2, 3, 4]
sum: int = COLLECTIONS.array_reduce(&arr, 0, |acc, x| acc + x)  # sum is 10
```

#### `array_fold<T, U>(arr: &Vec<T>, initial: U, folder: Fn(U, &T) -> U) -> U`
Folds the array to a single value using the folder function.

```tjlang
arr: Vec<int> = [1, 2, 3, 4]
product: int = COLLECTIONS.array_fold(&arr, 1, |acc, x| acc * x)  # product is 24
```

### Search Operations

#### `array_any<T>(arr: &Vec<T>, predicate: Fn(&T) -> bool) -> bool`
Returns true if any element satisfies the predicate.

```tjlang
arr: Vec<int> = [1, 2, 3, 4, 5]
has_even: bool = COLLECTIONS.array_any(&arr, |x| x % 2 == 0)  # has_even is true
```

#### `array_all<T>(arr: &Vec<T>, predicate: Fn(&T) -> bool) -> bool`
Returns true if all elements satisfy the predicate.

```tjlang
arr: Vec<int> = [2, 4, 6, 8]
all_even: bool = COLLECTIONS.array_all(&arr, |x| x % 2 == 0)  # all_even is true
```

#### `array_find<T>(arr: &Vec<T>, predicate: Fn(&T) -> bool) -> Option<&T>`
Returns the first element that satisfies the predicate.

```tjlang
arr: Vec<int> = [1, 2, 3, 4, 5]
found: Option<&int> = COLLECTIONS.array_find(&arr, |x| x > 3)  # found is Some(&4)
```

#### `array_contains<T>(arr: &Vec<T>, item: &T) -> bool`
Checks if the array contains the specified item.

```tjlang
arr: Vec<int> = [1, 2, 3, 4, 5]
contains: bool = COLLECTIONS.array_contains(&arr, &3)  # contains is true
```

#### `array_index_of<T>(arr: &Vec<T>, item: &T) -> Option<int>`
Returns the index of the first occurrence of the item.

```tjlang
arr: Vec<int> = [1, 2, 3, 2, 4]
index: Option<int> = COLLECTIONS.array_index_of(&arr, &2)  # index is Some(1)
```

#### `array_last_index_of<T>(arr: &Vec<T>, item: &T) -> Option<int>`
Returns the index of the last occurrence of the item.

```tjlang
arr: Vec<int> = [1, 2, 3, 2, 4]
last_index: Option<int> = COLLECTIONS.array_last_index_of(&arr, &2)  # last_index is Some(3)
```

#### `array_get_mut<T>(arr: &mut Vec<T>, index: int) -> Option<&mut T>`
Gets a mutable reference to the element at the specified index.

```tjlang
arr: Vec<int> = [1, 2, 3]
element: Option<&mut int> = COLLECTIONS.array_get_mut(&mut arr, 1)  # element is Some(&mut 2)
```

## Map/Dictionary Operations

Key-value storage with fast lookup.

### Creation and Initialization

#### `map_new<K, V>() -> HashMap<K, V>`
Creates a new empty map.

```tjlang
map: HashMap<str, int> = COLLECTIONS.map_new()
```

#### `map_with_capacity<K, V>(capacity: int) -> HashMap<K, V>`
Creates a new map with the specified initial capacity.

```tjlang
map: HashMap<str, int> = COLLECTIONS.map_with_capacity(10)
```

### Element Operations

#### `map_insert<K, V>(map: &mut HashMap<K, V>, key: K, value: V) -> Option<V>`
Inserts a key-value pair, returning the previous value if the key existed.

```tjlang
map: HashMap<str, int> = COLLECTIONS.map_new()
old_value: Option<int> = COLLECTIONS.map_insert(&mut map, "age", 25)  # old_value is None
```

#### `map_get<K, V>(map: &HashMap<K, V>, key: &K) -> Option<&V>`
Gets a reference to the value for the specified key.

```tjlang
map: HashMap<str, int> = [("age", 25), ("height", 180)]
value: Option<&int> = COLLECTIONS.map_get(&map, &"age")  # value is Some(&25)
```

#### `map_remove<K, V>(map: &mut HashMap<K, V>, key: &K) -> Option<V>`
Removes the key-value pair, returning the value if the key existed.

```tjlang
map: HashMap<str, int> = [("age", 25), ("height", 180)]
removed: Option<int> = COLLECTIONS.map_remove(&mut map, &"age")  # removed is Some(25)
```

#### `map_contains_key<K, V>(map: &HashMap<K, V>, key: &K) -> bool`
Checks if the map contains the specified key.

```tjlang
map: HashMap<str, int> = [("age", 25)]
has_key: bool = COLLECTIONS.map_contains_key(&map, &"age")  # has_key is true
```

### Map Information

#### `map_len<K, V>(map: &HashMap<K, V>) -> int`
Returns the number of key-value pairs in the map.

```tjlang
map: HashMap<str, int> = [("age", 25), ("height", 180)]
size: int = COLLECTIONS.map_len(&map)  # size is 2
```

#### `map_is_empty<K, V>(map: &HashMap<K, V>) -> bool`
Checks if the map is empty.

```tjlang
map: HashMap<str, int> = COLLECTIONS.map_new()
empty: bool = COLLECTIONS.map_is_empty(&map)  # empty is true
```

#### `map_keys<K, V>(map: &HashMap<K, V>) -> Vec<&K>`
Returns a vector of all keys in the map.

```tjlang
map: HashMap<str, int> = [("age", 25), ("height", 180)]
keys: Vec<&str> = COLLECTIONS.map_keys(&map)  # keys contains "age" and "height"
```

#### `map_values<K, V>(map: &HashMap<K, V>) -> Vec<&V>`
Returns a vector of all values in the map.

```tjlang
map: HashMap<str, int> = [("age", 25), ("height", 180)]
values: Vec<&int> = COLLECTIONS.map_values(&map)  # values contains 25 and 180
```

#### `map_clear<K, V>(map: &mut HashMap<K, V>)`
Removes all key-value pairs from the map.

```tjlang
map: HashMap<str, int> = [("age", 25), ("height", 180)]
COLLECTIONS.map_clear(&mut map)  # map is now empty
```

#### `map_get_mut<K, V>(map: &mut HashMap<K, V>, key: &K) -> Option<&mut V>`
Gets a mutable reference to the value for the specified key.

```tjlang
map: HashMap<str, int> = [("age", 25), ("height", 180)]
value: Option<&mut int> = COLLECTIONS.map_get_mut(&mut map, &"age")  # value is Some(&mut 25)
```

#### `map_entries<K, V>(map: &HashMap<K, V>) -> Vec<(&K, &V)>`
Returns a vector of all key-value pairs in the map.

```tjlang
map: HashMap<str, int> = [("age", 25), ("height", 180)]
entries: Vec<(&str, &int)> = COLLECTIONS.map_entries(&map)  # entries contains ("age", 25) and ("height", 180)
```

## Set Operations

Collections of unique elements.

### Creation and Initialization

#### `set_new<T>() -> HashSet<T>`
Creates a new empty set.

```tjlang
set: HashSet<int> = COLLECTIONS.set_new()
```

#### `set_from_vec<T>(vec: Vec<T>) -> HashSet<T>`
Creates a set from a vector, removing duplicates.

```tjlang
vec: Vec<int> = [1, 2, 2, 3, 3, 3]
set: HashSet<int> = COLLECTIONS.set_from_vec(vec)  # set contains {1, 2, 3}
```

### Element Operations

#### `set_insert<T>(set: &mut HashSet<T>, item: T) -> bool`
Adds an item to the set, returning true if the item was not already present.

```tjlang
set: HashSet<int> = COLLECTIONS.set_new()
inserted: bool = COLLECTIONS.set_insert(&mut set, 5)  # inserted is true
inserted2: bool = COLLECTIONS.set_insert(&mut set, 5)  # inserted2 is false
```

#### `set_remove<T>(set: &mut HashSet<T>, item: &T) -> bool`
Removes an item from the set, returning true if the item was present.

```tjlang
set: HashSet<int> = [1, 2, 3]
removed: bool = COLLECTIONS.set_remove(&mut set, &2)  # removed is true
```

#### `set_contains<T>(set: &HashSet<T>, item: &T) -> bool`
Checks if the set contains the specified item.

```tjlang
set: HashSet<int> = [1, 2, 3]
contains: bool = COLLECTIONS.set_contains(&set, &2)  # contains is true
```

### Set Information

#### `set_len<T>(set: &HashSet<T>) -> int`
Returns the number of elements in the set.

```tjlang
set: HashSet<int> = [1, 2, 3]
size: int = COLLECTIONS.set_len(&set)  # size is 3
```

#### `set_is_empty<T>(set: &HashSet<T>) -> bool`
Checks if the set is empty.

```tjlang
set: HashSet<int> = COLLECTIONS.set_new()
empty: bool = COLLECTIONS.set_is_empty(&set)  # empty is true
```

### Set Operations

#### `set_union<T>(set1: &HashSet<T>, set2: &HashSet<T>) -> HashSet<T>`
Returns the union of two sets.

```tjlang
set1: HashSet<int> = [1, 2, 3]
set2: HashSet<int> = [3, 4, 5]
union_set: HashSet<int> = COLLECTIONS.set_union(&set1, &set2)  # union_set is {1, 2, 3, 4, 5}
```

#### `set_intersection<T>(set1: &HashSet<T>, set2: &HashSet<T>) -> HashSet<T>`
Returns the intersection of two sets.

```tjlang
set1: HashSet<int> = [1, 2, 3]
set2: HashSet<int> = [2, 3, 4]
intersection: HashSet<int> = COLLECTIONS.set_intersection(&set1, &set2)  # intersection is {2, 3}
```

#### `set_difference<T>(set1: &HashSet<T>, set2: &HashSet<T>) -> HashSet<T>`
Returns the difference of two sets (elements in set1 but not in set2).

```tjlang
set1: HashSet<int> = [1, 2, 3]
set2: HashSet<int> = [2, 3, 4]
difference: HashSet<int> = COLLECTIONS.set_difference(&set1, &set2)  # difference is {1}
```

#### `set_is_subset<T>(set1: &HashSet<T>, set2: &HashSet<T>) -> bool`
Checks if set1 is a subset of set2.

```tjlang
set1: HashSet<int> = [1, 2]
set2: HashSet<int> = [1, 2, 3, 4]
is_subset: bool = COLLECTIONS.set_is_subset(&set1, &set2)  # is_subset is true
```

#### `set_is_superset<T>(set1: &HashSet<T>, set2: &HashSet<T>) -> bool`
Checks if set1 is a superset of set2.

```tjlang
set1: HashSet<int> = [1, 2, 3, 4]
set2: HashSet<int> = [1, 2]
is_superset: bool = COLLECTIONS.set_is_superset(&set1, &set2)  # is_superset is true
```

#### `set_is_disjoint<T>(set1: &HashSet<T>, set2: &HashSet<T>) -> bool`
Checks if set1 and set2 have no elements in common.

```tjlang
set1: HashSet<int> = [1, 2, 3]
set2: HashSet<int> = [4, 5, 6]
is_disjoint: bool = COLLECTIONS.set_is_disjoint(&set1, &set2)  # is_disjoint is true
```

#### `set_symmetric_difference<T>(set1: &HashSet<T>, set2: &HashSet<T>) -> HashSet<T>`
Returns the symmetric difference of two sets (elements in either set but not in both).

```tjlang
set1: HashSet<int> = [1, 2, 3]
set2: HashSet<int> = [2, 3, 4]
symmetric_diff: HashSet<int> = COLLECTIONS.set_symmetric_difference(&set1, &set2)  # symmetric_diff is {1, 4}
```

#### `set_with_capacity<T>(capacity: int) -> HashSet<T>`
Creates a new set with the specified initial capacity.

```tjlang
set: HashSet<int> = COLLECTIONS.set_with_capacity(10)
```

## Queue Operations

FIFO (First In, First Out) data structures.

### Creation and Initialization

#### `queue_new<T>() -> VecDeque<T>`
Creates a new empty queue.

```tjlang
queue: VecDeque<int> = COLLECTIONS.queue_new()
```

#### `queue_with_capacity<T>(capacity: int) -> VecDeque<T>`
Creates a new queue with the specified initial capacity.

```tjlang
queue: VecDeque<int> = COLLECTIONS.queue_with_capacity(10)
```

### Queue Operations

#### `queue_push_back<T>(queue: &mut VecDeque<T>, item: T)`
Adds an item to the back of the queue.

```tjlang
queue: VecDeque<int> = COLLECTIONS.queue_new()
COLLECTIONS.queue_push_back(&mut queue, 1)
COLLECTIONS.queue_push_back(&mut queue, 2)  # queue is now [1, 2]
```

#### `queue_push_front<T>(queue: &mut VecDeque<T>, item: T)`
Adds an item to the front of the queue.

```tjlang
queue: VecDeque<int> = [1, 2]
COLLECTIONS.queue_push_front(&mut queue, 0)  # queue is now [0, 1, 2]
```

#### `queue_pop_back<T>(queue: &mut VecDeque<T>) -> Option<T>`
Removes and returns the item from the back of the queue.

```tjlang
queue: VecDeque<int> = [1, 2, 3]
back: Option<int> = COLLECTIONS.queue_pop_back(&mut queue)  # back is Some(3), queue is [1, 2]
```

#### `queue_pop_front<T>(queue: &mut VecDeque<T>) -> Option<T>`
Removes and returns the item from the front of the queue.

```tjlang
queue: VecDeque<int> = [1, 2, 3]
front: Option<int> = COLLECTIONS.queue_pop_front(&mut queue)  # front is Some(1), queue is [2, 3]
```

### Queue Information

#### `queue_len<T>(queue: &VecDeque<T>) -> int`
Returns the number of items in the queue.

```tjlang
queue: VecDeque<int> = [1, 2, 3]
length: int = COLLECTIONS.queue_len(&queue)  # length is 3
```

#### `queue_is_empty<T>(queue: &VecDeque<T>) -> bool`
Checks if the queue is empty.

```tjlang
queue: VecDeque<int> = COLLECTIONS.queue_new()
empty: bool = COLLECTIONS.queue_is_empty(&queue)  # empty is true
```

## Priority Queue Operations

Heap-based priority queues where the highest priority element is always at the front.

### Creation and Initialization

#### `priority_queue_new<T>() -> BinaryHeap<T>`
Creates a new empty priority queue.

```tjlang
pq: BinaryHeap<int> = COLLECTIONS.priority_queue_new()
```

#### `priority_queue_with_capacity<T>(capacity: int) -> BinaryHeap<T>`
Creates a new priority queue with the specified initial capacity.

```tjlang
pq: BinaryHeap<int> = COLLECTIONS.priority_queue_with_capacity(10)
```

### Priority Queue Operations

#### `priority_queue_push<T>(queue: &mut BinaryHeap<T>, item: T)`
Adds an item to the priority queue.

```tjlang
pq: BinaryHeap<int> = COLLECTIONS.priority_queue_new()
COLLECTIONS.priority_queue_push(&mut pq, 3)
COLLECTIONS.priority_queue_push(&mut pq, 1)
COLLECTIONS.priority_queue_push(&mut pq, 2)  # pq now contains [3, 1, 2] (max-heap order)
```

#### `priority_queue_pop<T>(queue: &mut BinaryHeap<T>) -> Option<T>`
Removes and returns the highest priority item from the queue.

```tjlang
pq: BinaryHeap<int> = [3, 1, 2]
max: Option<int> = COLLECTIONS.priority_queue_pop(&mut pq)  # max is Some(3)
```

#### `priority_queue_peek<T>(queue: &BinaryHeap<T>) -> Option<&T>`
Returns a reference to the highest priority item without removing it.

```tjlang
pq: BinaryHeap<int> = [3, 1, 2]
max_ref: Option<&int> = COLLECTIONS.priority_queue_peek(&pq)  # max_ref is Some(&3)
```

### Priority Queue Information

#### `priority_queue_len<T>(queue: &BinaryHeap<T>) -> int`
Returns the number of items in the priority queue.

```tjlang
pq: BinaryHeap<int> = [3, 1, 2]
length: int = COLLECTIONS.priority_queue_len(&pq)  # length is 3
```

#### `priority_queue_is_empty<T>(queue: &BinaryHeap<T>) -> bool`
Checks if the priority queue is empty.

```tjlang
pq: BinaryHeap<int> = COLLECTIONS.priority_queue_new()
empty: bool = COLLECTIONS.priority_queue_is_empty(&pq)  # empty is true
```

## BTree Operations

Ordered tree structures for sorted data.

### Creation and Initialization

#### `btree_map_new<K, V>() -> BTreeMap<K, V>`
Creates a new empty BTree map.

```tjlang
map: BTreeMap<str, int> = COLLECTIONS.btree_map_new()
```

#### `btree_set_new<T>() -> BTreeSet<T>`
Creates a new empty BTree set.

```tjlang
set: BTreeSet<int> = COLLECTIONS.btree_set_new()
```

## Iterator Utilities

Collection iteration and transformation utilities.

### Iterator Creation and Transformation

#### `iterator_map<F, T, U>(iter: Iterator<T>, mapper: F) -> Iterator<U>`
Transforms each element using the mapper function.

```tjlang
numbers: Vec<int> = [1, 2, 3, 4, 5]
squared: Iterator<int> = COLLECTIONS.iterator_map(numbers.iter(), |x| x * x)
```

#### `iterator_filter<F, T>(iter: Iterator<T>, predicate: F) -> Iterator<T>`
Filters elements that satisfy the predicate.

```tjlang
numbers: Vec<int> = [1, 2, 3, 4, 5]
evens: Iterator<int> = COLLECTIONS.iterator_filter(numbers.iter(), |x| x % 2 == 0)
```

#### `iterator_fold<F, T, U>(iter: Iterator<T>, initial: U, folder: F) -> U`
Folds the iterator to a single value.

```tjlang
numbers: Vec<int> = [1, 2, 3, 4, 5]
sum: int = COLLECTIONS.iterator_fold(numbers.iter(), 0, |acc, x| acc + x)  # sum is 15
```

#### `iterator_reduce<F, T>(iter: Iterator<T>, reducer: F) -> Option<T>`
Reduces the iterator to a single value.

```tjlang
numbers: Vec<int> = [1, 2, 3, 4, 5]
max: Option<int> = COLLECTIONS.iterator_reduce(numbers.iter(), |a, b| if a > b { a } else { b })
```

### Iterator Predicates

#### `iterator_any<F, T>(iter: Iterator<T>, predicate: F) -> bool`
Returns true if any element satisfies the predicate.

```tjlang
numbers: Vec<int> = [1, 2, 3, 4, 5]
has_even: bool = COLLECTIONS.iterator_any(numbers.iter(), |x| x % 2 == 0)  # has_even is true
```

#### `iterator_all<F, T>(iter: Iterator<T>, predicate: F) -> bool`
Returns true if all elements satisfy the predicate.

```tjlang
numbers: Vec<int> = [2, 4, 6, 8]
all_even: bool = COLLECTIONS.iterator_all(numbers.iter(), |x| x % 2 == 0)  # all_even is true
```

#### `iterator_find<F, T>(iter: Iterator<T>, predicate: F) -> Option<T>`
Returns the first element that satisfies the predicate.

```tjlang
numbers: Vec<int> = [1, 2, 3, 4, 5]
found: Option<int> = COLLECTIONS.iterator_find(numbers.iter(), |x| x > 3)  # found is Some(4)
```

### Iterator Aggregation

#### `iterator_count<T>(iter: Iterator<T>) -> int`
Returns the number of elements in the iterator.

```tjlang
numbers: Vec<int> = [1, 2, 3, 4, 5]
count: int = COLLECTIONS.iterator_count(numbers.iter())  # count is 5
```

#### `iterator_sum<T>(iter: Iterator<T>) -> T`
Returns the sum of all elements.

```tjlang
numbers: Vec<int> = [1, 2, 3, 4, 5]
sum: int = COLLECTIONS.iterator_sum(numbers.iter())  # sum is 15
```

#### `iterator_product<T>(iter: Iterator<T>) -> T`
Returns the product of all elements.

```tjlang
numbers: Vec<int> = [1, 2, 3, 4]
product: int = COLLECTIONS.iterator_product(numbers.iter())  # product is 24
```

#### `iterator_min<T>(iter: Iterator<T>) -> Option<T>`
Returns the minimum element.

```tjlang
numbers: Vec<int> = [3, 1, 4, 1, 5]
min: Option<int> = COLLECTIONS.iterator_min(numbers.iter())  # min is Some(1)
```

#### `iterator_max<T>(iter: Iterator<T>) -> Option<T>`
Returns the maximum element.

```tjlang
numbers: Vec<int> = [3, 1, 4, 1, 5]
max: Option<int> = COLLECTIONS.iterator_max(numbers.iter())  # max is Some(5)
```

### Iterator Combination

#### `iterator_zip<T, U>(iter1: Iterator<T>, iter2: Iterator<U>) -> Iterator<(T, U)>`
Combines two iterators into pairs.

```tjlang
names: Vec<str> = ["Alice", "Bob", "Charlie"]
ages: Vec<int> = [25, 30, 35]
pairs: Iterator<(str, int)> = COLLECTIONS.iterator_zip(names.iter(), ages.iter())
```

#### `iterator_chain<T>(iter1: Iterator<T>, iter2: Iterator<T>) -> Iterator<T>`
Chains two iterators together.

```tjlang
first: Vec<int> = [1, 2, 3]
second: Vec<int> = [4, 5, 6]
combined: Iterator<int> = COLLECTIONS.iterator_chain(first.iter(), second.iter())
```

#### `iterator_cycle<T>(iter: Iterator<T>) -> Iterator<T>`
Repeats the iterator infinitely.

```tjlang
numbers: Vec<int> = [1, 2, 3]
cycled: Iterator<int> = COLLECTIONS.iterator_cycle(numbers.iter())
```

### Iterator Slicing

#### `iterator_take<T>(iter: Iterator<T>, n: int) -> Iterator<T>`
Takes the first n elements.

```tjlang
numbers: Vec<int> = [1, 2, 3, 4, 5]
first_three: Iterator<int> = COLLECTIONS.iterator_take(numbers.iter(), 3)
```

#### `iterator_skip<T>(iter: Iterator<T>, n: int) -> Iterator<T>`
Skips the first n elements.

```tjlang
numbers: Vec<int> = [1, 2, 3, 4, 5]
last_two: Iterator<int> = COLLECTIONS.iterator_skip(numbers.iter(), 3)
```

#### `iterator_step_by<T>(iter: Iterator<T>, step: int) -> Iterator<T>`
Takes every nth element.

```tjlang
numbers: Vec<int> = [1, 2, 3, 4, 5, 6]
every_other: Iterator<int> = COLLECTIONS.iterator_step_by(numbers.iter(), 2)
```

#### `iterator_enumerate<T>(iter: Iterator<T>) -> Iterator<(int, T)>`
Adds indices to each element.

```tjlang
names: Vec<str> = ["Alice", "Bob", "Charlie"]
indexed: Iterator<(int, str)> = COLLECTIONS.iterator_enumerate(names.iter())
```

#### `iterator_rev<T>(iter: Iterator<T>) -> Iterator<T>`
Reverses the iterator.

```tjlang
numbers: Vec<int> = [1, 2, 3, 4, 5]
reversed: Iterator<int> = COLLECTIONS.iterator_rev(numbers.iter())
```

## Collection Algorithms

Sorting, searching, and filtering algorithms.

### Search Algorithms

#### `binary_search<T>(arr: &Vec<T>, target: &T) -> Result<int, int>`
Performs binary search on a sorted array.

```tjlang
arr: Vec<int> = [1, 2, 3, 4, 5]
result: Result<int, int> = COLLECTIONS.binary_search(&arr, &3)  # result is Ok(2)
```

#### `linear_search<T>(arr: &Vec<T>, target: &T) -> Option<int>`
Performs linear search on an array.

```tjlang
arr: Vec<int> = [1, 2, 3, 4, 5]
index: Option<int> = COLLECTIONS.linear_search(&arr, &3)  # index is Some(2)
```

### Sorting Algorithms

#### `sort<T>(arr: &mut Vec<T>)`
Sorts the array in ascending order.

```tjlang
arr: Vec<int> = [3, 1, 4, 1, 5]
COLLECTIONS.sort(&mut arr)  # arr is now [1, 1, 3, 4, 5]
```

#### `sort_by<F, T>(arr: &mut Vec<T>, compare: F)`
Sorts the array using a custom comparison function.

```tjlang
arr: Vec<int> = [3, 1, 4, 1, 5]
COLLECTIONS.sort_by(&mut arr, |a, b| b.cmp(a))  # arr is now [5, 4, 3, 1, 1]
```

#### `sort_by_key<F, K, T>(arr: &mut Vec<T>, key: F)`
Sorts the array using a key extraction function.

```tjlang
words: Vec<str> = ["hello", "hi", "world"]
COLLECTIONS.sort_by_key(&mut words, |s| s.len())  # words is now ["hi", "hello", "world"]
```

### Array Manipulation

#### `reverse<T>(arr: &mut Vec<T>)`
Reverses the order of elements in the array.

```tjlang
arr: Vec<int> = [1, 2, 3, 4]
COLLECTIONS.reverse(&mut arr)  # arr is now [4, 3, 2, 1]
```

#### `shuffle<T>(arr: &mut Vec<T>)`
Randomly shuffles the elements in the array.

```tjlang
arr: Vec<int> = [1, 2, 3, 4, 5]
COLLECTIONS.shuffle(&mut arr)  # arr is now in random order
```

#### `rotate_left<T>(arr: &mut Vec<T>, mid: int)`
Rotates the array left by the specified amount.

```tjlang
arr: Vec<int> = [1, 2, 3, 4, 5]
COLLECTIONS.rotate_left(&mut arr, 2)  # arr is now [3, 4, 5, 1, 2]
```

#### `rotate_right<T>(arr: &mut Vec<T>, k: int)`
Rotates the array right by the specified amount.

```tjlang
arr: Vec<int> = [1, 2, 3, 4, 5]
COLLECTIONS.rotate_right(&mut arr, 2)  # arr is now [4, 5, 1, 2, 3]
```

#### `partition<F, T>(arr: &mut Vec<T>, predicate: F) -> int`
Partitions the array based on the predicate.

```tjlang
arr: Vec<int> = [1, 2, 3, 4, 5]
pivot: int = COLLECTIONS.partition(&mut arr, |x| x % 2 == 0)  # pivot is 2
```

### Deduplication

#### `dedup<T>(arr: &mut Vec<T>)`
Removes consecutive duplicate elements.

```tjlang
arr: Vec<int> = [1, 1, 2, 2, 3, 3]
COLLECTIONS.dedup(&mut arr)  # arr is now [1, 2, 3]
```

#### `dedup_by<F, T>(arr: &mut Vec<T>, same_bucket: F)`
Removes consecutive duplicate elements using a custom equality function.

```tjlang
arr: Vec<str> = ["hello", "world", "hello", "rust"]
COLLECTIONS.dedup_by(&mut arr, |a, b| a.len() == b.len())  # arr is now ["hello", "world", "rust"]
```

#### `dedup_by_key<F, K, T>(arr: &mut Vec<T>, key: F)`
Removes consecutive duplicate elements based on a key.

```tjlang
arr: Vec<str> = ["hello", "world", "hello", "rust"]
COLLECTIONS.dedup_by_key(&mut arr, |s| s.len())  # arr is now ["hello", "world", "rust"]
```

## Advanced Collection Types

Advanced collection types inspired by Python's collections module.

### Counter Operations

Count occurrences of elements.

#### `counter_new<T>() -> Counter<T>`
Creates a new empty counter.

```tjlang
counter: Counter<str> = COLLECTIONS.counter_new()
```

#### `counter_from_iter<T>(iter: Iterator<T>) -> Counter<T>`
Creates a counter from an iterator.

```tjlang
words: Vec<str> = ["hello", "world", "hello", "rust"]
counter: Counter<str> = COLLECTIONS.counter_from_iter(words.iter())
```

#### `counter_most_common<T>(counter: &Counter<T>, n: Option<int>) -> Vec<(T, int)>`
Returns the most common elements.

```tjlang
counter: Counter<str> = [("hello", 2), ("world", 1), ("rust", 1)]
most_common: Vec<(str, int)> = COLLECTIONS.counter_most_common(&counter, Some(2))
```

#### `counter_total<T>(counter: &Counter<T>) -> int`
Returns the total count of all elements.

```tjlang
counter: Counter<str> = [("hello", 2), ("world", 1)]
total: int = COLLECTIONS.counter_total(&counter)  # total is 3
```

#### `counter_subtract<T>(counter: &mut Counter<T>, other: &Counter<T>)`
Subtracts counts from another counter.

```tjlang
counter1: Counter<str> = [("hello", 3), ("world", 2)]
counter2: Counter<str> = [("hello", 1)]
COLLECTIONS.counter_subtract(&mut counter1, &counter2)  # counter1 is now [("hello", 2), ("world", 2)]
```

#### `counter_update<T>(counter: &mut Counter<T>, iter: Iterator<T>)`
Updates the counter with elements from an iterator.

```tjlang
counter: Counter<str> = COLLECTIONS.counter_new()
words: Vec<str> = ["hello", "world", "hello"]
COLLECTIONS.counter_update(&mut counter, words.iter())
```

### DefaultDict Operations

Dictionary with default values.

#### `default_dict_new<F, K, V>(default_factory: F) -> DefaultDict<K, V, F>`
Creates a new default dictionary.

```tjlang
dict: DefaultDict<str, int> = COLLECTIONS.default_dict_new(|| 0)
```

#### `default_dict_get<F, K, V>(dict: &mut DefaultDict<K, V, F>, key: K) -> V`
Gets a value, creating it with the default factory if it doesn't exist.

```tjlang
dict: DefaultDict<str, int> = COLLECTIONS.default_dict_new(|| 0)
value: int = COLLECTIONS.default_dict_get(&mut dict, "age")  # value is 0
```

#### `default_dict_set<F, K, V>(dict: &mut DefaultDict<K, V, F>, key: K, value: V)`
Sets a value in the dictionary.

```tjlang
dict: DefaultDict<str, int> = COLLECTIONS.default_dict_new(|| 0)
COLLECTIONS.default_dict_set(&mut dict, "age", 25)
```

### ChainMap Operations

Chain multiple dictionaries.

#### `chain_map_new<K, V>() -> ChainMap<K, V>`
Creates a new chain map.

```tjlang
chain: ChainMap<str, int> = COLLECTIONS.chain_map_new()
```

#### `chain_map_add<K, V>(chain: &mut ChainMap<K, V>, map: HashMap<K, V>)`
Adds a map to the chain.

```tjlang
chain: ChainMap<str, int> = COLLECTIONS.chain_map_new()
map: HashMap<str, int> = [("age", 25)]
COLLECTIONS.chain_map_add(&mut chain, map)
```

#### `chain_map_get<K, V>(chain: &ChainMap<K, V>, key: &K) -> Option<&V>`
Gets a value from the chain map.

```tjlang
chain: ChainMap<str, int> = [("age", 25)]
value: Option<&int> = COLLECTIONS.chain_map_get(&chain, &"age")  # value is Some(&25)
```

### NamedTuple Operations

Tuple with named fields.

#### `named_tuple_new<T>(fields: Vec<str>, values: Vec<T>) -> NamedTuple<T>`
Creates a new named tuple.

```tjlang
fields: Vec<str> = ["name", "age", "city"]
values: Vec<str> = ["Alice", "25", "New York"]
person: NamedTuple<str> = COLLECTIONS.named_tuple_new(fields, values)
```

#### `named_tuple_get<T>(nt: &NamedTuple<T>, field: &str) -> Option<&T>`
Gets a value by field name.

```tjlang
person: NamedTuple<str> = [("name", "Alice"), ("age", "25")]
name: Option<&str> = COLLECTIONS.named_tuple_get(&person, "name")  # name is Some("Alice")
```

#### `named_tuple_set<T>(nt: &mut NamedTuple<T>, field: &str, value: T) -> Result<(), str>`
Sets a value by field name.

```tjlang
person: NamedTuple<str> = [("name", "Alice"), ("age", "25")]
result: Result<(), str> = COLLECTIONS.named_tuple_set(&mut person, "age", "26")
```

### OrderedDict Operations

Dictionary that remembers insertion order.

#### `ordered_dict_new<K, V>() -> OrderedDict<K, V>`
Creates a new ordered dictionary.

```tjlang
dict: OrderedDict<str, int> = COLLECTIONS.ordered_dict_new()
```

#### `ordered_dict_move_to_end<K, V>(dict: &mut OrderedDict<K, V>, key: &K)`
Moves a key to the end of the dictionary.

```tjlang
dict: OrderedDict<str, int> = [("first", 1), ("second", 2)]
COLLECTIONS.ordered_dict_move_to_end(&mut dict, &"first")
```

#### `ordered_dict_popitem<K, V>(dict: &mut OrderedDict<K, V>, last: bool) -> Option<(K, V)>`
Removes and returns the last or first item.

```tjlang
dict: OrderedDict<str, int> = [("first", 1), ("second", 2)]
item: Option<(str, int)> = COLLECTIONS.ordered_dict_popitem(&mut dict, true)  # item is Some(("second", 2))
```

### Deque Operations

Double-ended queue with additional features.

#### `deque_new<T>() -> Deque<T>`
Creates a new deque.

```tjlang
deque: Deque<int> = COLLECTIONS.deque_new()
```

#### `deque_rotate<T>(deque: &mut Deque<T>, n: int)`
Rotates the deque by the specified amount.

```tjlang
deque: Deque<int> = [1, 2, 3, 4, 5]
COLLECTIONS.deque_rotate(&mut deque, 2)  # deque is now [3, 4, 5, 1, 2]
```

#### `deque_extend<T>(deque: &mut Deque<T>, iter: Iterator<T>)`
Extends the deque with elements from an iterator.

```tjlang
deque: Deque<int> = [1, 2, 3]
numbers: Vec<int> = [4, 5, 6]
COLLECTIONS.deque_extend(&mut deque, numbers.iter())
```

### Heap Operations

Priority queue with additional operations.

#### `heap_new<T>() -> Heap<T>`
Creates a new heap.

```tjlang
heap: Heap<int> = COLLECTIONS.heap_new()
```

#### `heap_merge<T>(heap: &mut Heap<T>, other: BinaryHeap<T>)`
Merges another heap into this one.

```tjlang
heap: Heap<int> = [3, 1, 4]
other: BinaryHeap<int> = [2, 5]
COLLECTIONS.heap_merge(&mut heap, other)
```

#### `heap_nlargest<T>(heap: &Heap<T>, n: int) -> Vec<T>`
Returns the n largest elements.

```tjlang
heap: Heap<int> = [3, 1, 4, 1, 5]
largest: Vec<int> = COLLECTIONS.heap_nlargest(&heap, 3)  # largest is [5, 4, 3]
```

#### `heap_nsmallest<T>(heap: &Heap<T>, n: int) -> Vec<T>`
Returns the n smallest elements.

```tjlang
heap: Heap<int> = [3, 1, 4, 1, 5]
smallest: Vec<int> = COLLECTIONS.heap_nsmallest(&heap, 3)  # smallest is [1, 1, 3]
```

## Examples

Here are comprehensive examples showing how to use the collections module:

### Array Operations Example

```tjlang
# Create and manipulate arrays
numbers: Vec<int> = COLLECTIONS.array_new()
COLLECTIONS.array_push(&mut numbers, 1)
COLLECTIONS.array_push(&mut numbers, 2)
COLLECTIONS.array_push(&mut numbers, 3)

IO.print("Array length: " + COLLECTIONS.array_len(&numbers).to_string())
IO.print("Array contains 2: " + COLLECTIONS.array_contains(&numbers, &2).to_string())

# Filter even numbers
evens: Vec<int> = COLLECTIONS.array_filter(&numbers, |x| x % 2 == 0)
IO.print("Even numbers: " + evens.to_string())

# Map to squares
squares: Vec<int> = COLLECTIONS.array_map(&numbers, |x| x * x)
IO.print("Squares: " + squares.to_string())
```

### Map Operations Example

```tjlang
# Create and populate a map
person: HashMap<str, str> = COLLECTIONS.map_new()
COLLECTIONS.map_insert(&mut person, "name", "Alice")
COLLECTIONS.map_insert(&mut person, "age", "25")
COLLECTIONS.map_insert(&mut person, "city", "New York")

IO.print("Person name: " + COLLECTIONS.map_get(&person, &"name").unwrap())
IO.print("Has age: " + COLLECTIONS.map_contains_key(&person, &"age").to_string())

# Get all keys
keys: Vec<&str> = COLLECTIONS.map_keys(&person)
IO.print("Keys: " + keys.to_string())
```

### Set Operations Example

```tjlang
# Create sets and perform operations
set1: HashSet<int> = COLLECTIONS.set_from_vec([1, 2, 3, 4])
set2: HashSet<int> = COLLECTIONS.set_from_vec([3, 4, 5, 6])

# Union
union: HashSet<int> = COLLECTIONS.set_union(&set1, &set2)
IO.print("Union: " + union.to_string())

# Intersection
intersection: HashSet<int> = COLLECTIONS.set_intersection(&set1, &set2)
IO.print("Intersection: " + intersection.to_string())

# Difference
difference: HashSet<int> = COLLECTIONS.set_difference(&set1, &set2)
IO.print("Difference: " + difference.to_string())
```

### Queue Operations Example

```tjlang
# Create and use a queue
queue: VecDeque<str> = COLLECTIONS.queue_new()
COLLECTIONS.queue_push_back(&mut queue, "first")
COLLECTIONS.queue_push_back(&mut queue, "second")
COLLECTIONS.queue_push_back(&mut queue, "third")

# Process queue
while not COLLECTIONS.queue_is_empty(&queue) {
    item: Option<str> = COLLECTIONS.queue_pop_front(&mut queue)
    IO.print("Processing: " + item.unwrap())
}
```

### Priority Queue Example

```tjlang
# Create and use a priority queue
pq: BinaryHeap<int> = COLLECTIONS.priority_queue_new()
COLLECTIONS.priority_queue_push(&mut pq, 5)
COLLECTIONS.priority_queue_push(&mut pq, 1)
COLLECTIONS.priority_queue_push(&mut pq, 9)
COLLECTIONS.priority_queue_push(&mut pq, 3)

# Process in priority order (highest first)
while not COLLECTIONS.priority_queue_is_empty(&pq) {
    max: Option<int> = COLLECTIONS.priority_queue_pop(&mut pq)
    IO.print("Processing priority: " + max.unwrap().to_string())
}
```

This comprehensive collections module provides powerful data structures and algorithms for efficient data manipulation in TJLang, enabling complex data processing tasks with clean, functional interfaces.
