//! Tests for COLLECTIONS module

#[cfg(test)]
mod tests {
    use crate::stdlib::collections::*;
    use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

    #[test]
    fn test_array_new() {
        let vec: Vec<i32> = COLLECTIONS::array_new();
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn test_array_with_capacity() {
        let vec: Vec<i32> = COLLECTIONS::array_with_capacity(10);
        assert_eq!(vec.capacity(), 10);
    }

    #[test]
    fn test_array_from_slice() {
        let slice = &[1, 2, 3];
        let vec = COLLECTIONS::array_from_slice(slice);
        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn test_array_push() {
        let mut vec = vec![1, 2];
        COLLECTIONS::array_push(&mut vec, 3);
        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn test_array_pop() {
        let mut vec = vec![1, 2, 3];
        let result = COLLECTIONS::array_pop(&mut vec);
        assert_eq!(result, Some(3));
        assert_eq!(vec, vec![1, 2]);
    }

    #[test]
    fn test_array_insert() {
        let mut vec = vec![1, 3];
        let result = COLLECTIONS::array_insert(&mut vec, 1, 2);
        assert!(result.is_ok());
        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn test_array_remove() {
        let mut vec = vec![1, 2, 3];
        let result = COLLECTIONS::array_remove(&mut vec, 1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);
        assert_eq!(vec, vec![1, 3]);
    }

    #[test]
    fn test_array_len() {
        let vec = vec![1, 2, 3];
        let result = COLLECTIONS::array_len(&vec);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_array_capacity() {
        let vec: Vec<i32> = Vec::with_capacity(10);
        let result = COLLECTIONS::array_capacity(&vec);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_array_is_empty() {
        let vec: Vec<i32> = vec![];
        let result = COLLECTIONS::array_is_empty(&vec);
        assert!(result);
    }

    #[test]
    fn test_array_get() {
        let vec = vec![1, 2, 3];
        let result = COLLECTIONS::array_get(&vec, 1);
        assert_eq!(result, Some(&2));
    }

    #[test]
    fn test_array_get_mut() {
        let mut vec = vec![1, 2, 3];
        let result = COLLECTIONS::array_get_mut(&mut vec, 1);
        assert_eq!(result, Some(&mut 2));
    }

    #[test]
    fn test_array_set() {
        let mut vec = vec![1, 2, 3];
        let result = COLLECTIONS::array_set(&mut vec, 1, 4);
        assert!(result.is_ok());
        assert_eq!(vec, vec![1, 4, 3]);
    }

    #[test]
    fn test_array_slice() {
        let vec = vec![1, 2, 3, 4, 5];
        let result = COLLECTIONS::array_slice(&vec, 1, 4);
        assert_eq!(result, vec![2, 3, 4]);
    }

    #[test]
    fn test_array_append() {
        let mut vec1 = vec![1, 2];
        let mut vec2 = vec![3, 4];
        COLLECTIONS::array_append(&mut vec1, &mut vec2);
        assert_eq!(vec1, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_array_extend() {
        let mut vec1 = vec![1, 2];
        let vec2 = vec![3, 4];
        COLLECTIONS::array_extend(&mut vec1, &vec2);
        assert_eq!(vec1, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_array_reverse() {
        let mut vec = vec![1, 2, 3];
        COLLECTIONS::array_reverse(&mut vec);
        assert_eq!(vec, vec![3, 2, 1]);
    }

    #[test]
    fn test_array_sort() {
        let mut vec = vec![3, 1, 4, 1, 5];
        COLLECTIONS::array_sort(&mut vec);
        assert_eq!(vec, vec![1, 1, 3, 4, 5]);
    }

    #[test]
    fn test_array_sort_by() {
        let mut vec = vec![3, 1, 4, 1, 5];
        COLLECTIONS::array_sort_by(&mut vec, |a, b| b.cmp(a));
        assert_eq!(vec, vec![5, 4, 3, 1, 1]);
    }

    #[test]
    fn test_array_shuffle() {
        let mut vec = vec![1, 2, 3, 4, 5];
        let original = vec.clone();
        COLLECTIONS::array_shuffle(&mut vec);
        // The shuffled array should have the same elements but potentially different order
        assert_eq!(vec.len(), original.len());
    }

    #[test]
    fn test_array_unique() {
        let mut vec = vec![1, 2, 2, 3, 3, 3];
        COLLECTIONS::array_unique(&mut vec);
        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn test_array_filter() {
        let vec = vec![1, 2, 3, 4, 5];
        let result = COLLECTIONS::array_filter(&vec, |x| x % 2 == 0);
        assert_eq!(result, vec![2, 4]);
    }

    #[test]
    fn test_array_map() {
        let vec = vec![1, 2, 3];
        let result = COLLECTIONS::array_map(&vec, |x| x * 2);
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[test]
    fn test_array_reduce() {
        let vec = vec![1, 2, 3, 4];
        let result = COLLECTIONS::array_reduce(&vec, 0, |acc, x| acc + x);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_array_any() {
        let vec = vec![1, 2, 3, 4];
        let result = COLLECTIONS::array_any(&vec, |x| *x > 3);
        assert!(result);
    }

    #[test]
    fn test_array_all() {
        let vec = vec![1, 2, 3, 4];
        let result = COLLECTIONS::array_all(&vec, |x| *x > 0);
        assert!(result);
    }

    #[test]
    fn test_array_find() {
        let vec = vec![1, 2, 3, 4];
        let result = COLLECTIONS::array_find(&vec, |x| **x > 2);
        assert_eq!(result, Some(&3));
    }

    #[test]
    fn test_array_find_index() {
        let vec = vec![1, 2, 3, 4];
        let result = COLLECTIONS::array_find_index(&vec, |x| *x > 2);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_array_contains() {
        let vec = vec![1, 2, 3, 4];
        let result = COLLECTIONS::array_contains(&vec, &3);
        assert!(result);
    }

    #[test]
    fn test_array_index_of() {
        let vec = vec![1, 2, 3, 4];
        let result = COLLECTIONS::array_index_of(&vec, &3);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_array_last_index_of() {
        let vec = vec![1, 2, 3, 2, 4];
        let result = COLLECTIONS::array_last_index_of(&vec, &2);
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_map_new() {
        let map: HashMap<String, i32> = COLLECTIONS::map_new();
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_map_with_capacity() {
        let map: HashMap<String, i32> = COLLECTIONS::map_with_capacity(10);
        assert!(map.capacity() >= 10);
    }

    #[test]
    fn test_map_insert() {
        let mut map = HashMap::new();
        COLLECTIONS::map_insert(&mut map, "key".to_string(), 42);
        assert_eq!(map.get("key"), Some(&42));
    }

    #[test]
    fn test_map_get() {
        let mut map = HashMap::new();
        map.insert("key".to_string(), 42);
        let result = COLLECTIONS::map_get(&map, &"key".to_string());
        assert_eq!(result, Some(&42));
    }

    #[test]
    fn test_map_get_mut() {
        let mut map = HashMap::new();
        map.insert("key".to_string(), 42);
        let result = COLLECTIONS::map_get_mut(&mut map, &"key".to_string());
        assert_eq!(result, Some(&mut 42));
    }

    #[test]
    fn test_map_remove() {
        let mut map = HashMap::new();
        map.insert("key".to_string(), 42);
        let result = COLLECTIONS::map_remove(&mut map, &"key".to_string());
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_map_contains_key() {
        let mut map = HashMap::new();
        map.insert("key".to_string(), 42);
        let result = COLLECTIONS::map_contains_key(&map, &"key".to_string());
        assert!(result);
    }

    #[test]
    fn test_map_len() {
        let mut map = HashMap::new();
        map.insert("key".to_string(), 42);
        let result = COLLECTIONS::map_len(&map);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_map_is_empty() {
        let map: HashMap<String, i32> = HashMap::new();
        let result = COLLECTIONS::map_is_empty(&map);
        assert!(result);
    }

    #[test]
    fn test_map_keys() {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), 42);
        map.insert("key2".to_string(), 43);
        let result = COLLECTIONS::map_keys(&map);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_map_values() {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), 42);
        map.insert("key2".to_string(), 43);
        let result = COLLECTIONS::map_values(&map);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_map_entries() {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), 42);
        map.insert("key2".to_string(), 43);
        let result = COLLECTIONS::map_entries(&map);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_set_new() {
        let set: HashSet<i32> = COLLECTIONS::set_new();
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn test_set_with_capacity() {
        let set: HashSet<i32> = COLLECTIONS::set_with_capacity(10);
        assert!(set.capacity() >= 10);
    }

    #[test]
    fn test_set_from_vec() {
        let vec = vec![1, 2, 3];
        let set = COLLECTIONS::set_from_vec(vec);
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn test_set_insert() {
        let mut set = HashSet::new();
        COLLECTIONS::set_insert(&mut set, 42);
        assert!(set.contains(&42));
    }

    #[test]
    fn test_set_remove() {
        let mut set = HashSet::new();
        set.insert(42);
        let result = COLLECTIONS::set_remove(&mut set, &42);
        assert!(result);
    }

    #[test]
    fn test_set_contains() {
        let mut set = HashSet::new();
        set.insert(42);
        let result = COLLECTIONS::set_contains(&set, &42);
        assert!(result);
    }

    #[test]
    fn test_set_union() {
        let mut set1 = HashSet::new();
        set1.insert(1);
        set1.insert(2);
        let mut set2 = HashSet::new();
        set2.insert(2);
        set2.insert(3);
        let result = COLLECTIONS::set_union(&set1, &set2);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_set_intersection() {
        let mut set1 = HashSet::new();
        set1.insert(1);
        set1.insert(2);
        let mut set2 = HashSet::new();
        set2.insert(2);
        set2.insert(3);
        let result = COLLECTIONS::set_intersection(&set1, &set2);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_set_difference() {
        let mut set1 = HashSet::new();
        set1.insert(1);
        set1.insert(2);
        let mut set2 = HashSet::new();
        set2.insert(2);
        set2.insert(3);
        let result = COLLECTIONS::set_difference(&set1, &set2);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_set_symmetric_difference() {
        let mut set1 = HashSet::new();
        set1.insert(1);
        set1.insert(2);
        let mut set2 = HashSet::new();
        set2.insert(2);
        set2.insert(3);
        let result = COLLECTIONS::set_symmetric_difference(&set1, &set2);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_set_is_subset() {
        let mut set1 = HashSet::new();
        set1.insert(1);
        set1.insert(2);
        let mut set2 = HashSet::new();
        set2.insert(1);
        set2.insert(2);
        set2.insert(3);
        let result = COLLECTIONS::set_is_subset(&set1, &set2);
        assert!(result);
    }

    #[test]
    fn test_set_is_superset() {
        let mut set1 = HashSet::new();
        set1.insert(1);
        set1.insert(2);
        set1.insert(3);
        let mut set2 = HashSet::new();
        set2.insert(1);
        set2.insert(2);
        let result = COLLECTIONS::set_is_superset(&set1, &set2);
        assert!(result);
    }

    #[test]
    fn test_set_is_disjoint() {
        let mut set1 = HashSet::new();
        set1.insert(1);
        set1.insert(2);
        let mut set2 = HashSet::new();
        set2.insert(3);
        set2.insert(4);
        let result = COLLECTIONS::set_is_disjoint(&set1, &set2);
        assert!(result);
    }

    #[test]
    fn test_queue_new() {
        let queue: VecDeque<i32> = COLLECTIONS::queue_new();
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn test_queue_with_capacity() {
        let queue: VecDeque<i32> = COLLECTIONS::queue_with_capacity(10);
        assert_eq!(queue.capacity(), 10);
    }

    #[test]
    fn test_queue_push_back() {
        let mut queue = VecDeque::new();
        queue.push_back(1);
        queue.push_back(2);
        COLLECTIONS::queue_push_back(&mut queue, 3);
        assert_eq!(queue.len(), 3);
    }

    #[test]
    fn test_queue_push_front() {
        let mut queue = VecDeque::new();
        queue.push_back(2);
        queue.push_back(3);
        COLLECTIONS::queue_push_front(&mut queue, 1);
        assert_eq!(queue.len(), 3);
    }

    #[test]
    fn test_queue_pop_back() {
        let mut queue = VecDeque::new();
        queue.push_back(1);
        queue.push_back(2);
        queue.push_back(3);
        let result = COLLECTIONS::queue_pop_back(&mut queue);
        assert_eq!(result, Some(3));
        assert_eq!(queue.len(), 2);
    }

    #[test]
    fn test_queue_pop_front() {
        let mut queue = VecDeque::new();
        queue.push_back(1);
        queue.push_back(2);
        queue.push_back(3);
        let result = COLLECTIONS::queue_pop_front(&mut queue);
        assert_eq!(result, Some(1));
        assert_eq!(queue.len(), 2);
    }

    #[test]
    fn test_priority_queue_new() {
        let queue: BinaryHeap<i32> = COLLECTIONS::priority_queue_new();
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn test_priority_queue_with_capacity() {
        let queue: BinaryHeap<i32> = COLLECTIONS::priority_queue_with_capacity(10);
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn test_priority_queue_push() {
        let mut queue = BinaryHeap::new();
        queue.push(3);
        queue.push(1);
        queue.push(4);
        COLLECTIONS::priority_queue_push(&mut queue, 2);
        assert_eq!(queue.len(), 4);
    }

    #[test]
    fn test_priority_queue_pop() {
        let mut queue = BinaryHeap::new();
        queue.push(4);
        queue.push(3);
        queue.push(1);
        queue.push(2);
        let result = COLLECTIONS::priority_queue_pop(&mut queue);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_priority_queue_peek() {
        let mut queue = BinaryHeap::new();
        queue.push(4);
        queue.push(3);
        queue.push(1);
        queue.push(2);
        let result = COLLECTIONS::priority_queue_peek(&queue);
        assert_eq!(result, Some(&4));
    }

    #[test]
    fn test_binary_search() {
        let vec = vec![1, 2, 3, 4, 5];
        let result = COLLECTIONS::binary_search(&vec, &3);
        assert_eq!(result, Ok(2));
    }

    #[test]
    fn test_linear_search() {
        let vec = vec![1, 2, 3, 4, 5];
        let result = COLLECTIONS::linear_search(&vec, &3);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_sort() {
        let mut vec = vec![3, 1, 4, 1, 5];
        COLLECTIONS::sort(&mut vec);
        assert_eq!(vec, vec![1, 1, 3, 4, 5]);
    }

    #[test]
    fn test_sort_by() {
        let mut vec = vec![3, 1, 4, 1, 5];
        COLLECTIONS::sort_by(&mut vec, |a, b| b.cmp(a));
        assert_eq!(vec, vec![5, 4, 3, 1, 1]);
    }

    #[test]
    fn test_sort_by_key() {
        let mut vec = vec![3, 1, 4, 1, 5];
        COLLECTIONS::sort_by_key(&mut vec, |x| -x);
        assert_eq!(vec, vec![5, 4, 3, 1, 1]);
    }

    #[test]
    fn test_reverse() {
        let mut vec = vec![1, 2, 3];
        COLLECTIONS::reverse(&mut vec);
        assert_eq!(vec, vec![3, 2, 1]);
    }

    #[test]
    fn test_shuffle() {
        let mut vec = vec![1, 2, 3, 4, 5];
        let original = vec.clone();
        COLLECTIONS::shuffle(&mut vec);
        // The shuffled array should have the same elements but potentially different order
        assert_eq!(vec.len(), original.len());
    }

    #[test]
    fn test_rotate_left() {
        let mut vec = vec![1, 2, 3, 4, 5];
        COLLECTIONS::rotate_left(&mut vec, 2);
        assert_eq!(vec, vec![3, 4, 5, 1, 2]);
    }

    #[test]
    fn test_rotate_right() {
        let mut vec = vec![1, 2, 3, 4, 5];
        COLLECTIONS::rotate_right(&mut vec, 2);
        assert_eq!(vec, vec![4, 5, 1, 2, 3]);
    }

    #[test]
    fn test_partition() {
        let mut vec = vec![1, 2, 3, 4, 5];
        let count = COLLECTIONS::partition(&mut vec, |x| x % 2 == 0);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_dedup() {
        let mut vec = vec![1, 2, 2, 3, 3, 3];
        COLLECTIONS::dedup(&mut vec);
        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn test_dedup_by() {
        let mut vec = vec![1, 2, 2, 3, 3, 3];
        COLLECTIONS::dedup_by(&mut vec, |a, b| a == b);
        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn test_dedup_by_key() {
        let mut vec = vec![1, 2, 2, 3, 3, 3];
        COLLECTIONS::dedup_by_key(&mut vec, |x| *x);
        assert_eq!(vec, vec![1, 2, 3]);
    }
}
