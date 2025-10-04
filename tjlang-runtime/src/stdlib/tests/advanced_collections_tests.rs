//! Advanced Collection Types Tests
//!
//! Tests for Python collections module equivalents:
//! - Counter
//! - DefaultDict
//! - ChainMap
//! - NamedTuple
//! - OrderedDict
//! - Deque
//! - Heap

#[cfg(test)]
mod advanced_collections_tests {
    use super::*;
    use crate::stdlib::collections::*;
    use std::collections::HashMap;

    // ============================================================================
    // Counter Tests
    // ============================================================================

    #[test]
    fn test_counter_new() {
        let counter: Counter<String> = COLLECTIONS::counter_new();
        assert!(counter.is_empty());
        assert_eq!(counter.len(), 0);
    }

    #[test]
    fn test_counter_from_iter() {
        let items = vec!["a", "b", "a", "c", "b", "a"];
        let counter = COLLECTIONS::counter_from_iter(items.into_iter());
        assert_eq!(counter.get(&"a"), 3);
        assert_eq!(counter.get(&"b"), 2);
        assert_eq!(counter.get(&"c"), 1);
        assert_eq!(counter.get(&"d"), 0);
    }

    #[test]
    fn test_counter_increment() {
        let mut counter = COLLECTIONS::counter_new::<String>();
        counter.increment("test".to_string());
        counter.increment("test".to_string());
        assert_eq!(counter.get(&"test".to_string()), 2);
    }

    #[test]
    fn test_counter_decrement() {
        let mut counter = COLLECTIONS::counter_new::<String>();
        counter.increment("test".to_string());
        counter.increment("test".to_string());
        counter.decrement("test".to_string());
        assert_eq!(counter.get(&"test".to_string()), 1);
        counter.decrement("test".to_string());
        assert_eq!(counter.get(&"test".to_string()), 0);
    }

    #[test]
    fn test_counter_most_common() {
        let items = vec!["a", "b", "a", "c", "b", "a", "d"];
        let counter = COLLECTIONS::counter_from_iter(items.into_iter());
        let most_common = COLLECTIONS::counter_most_common(&counter, Some(3));
        assert_eq!(most_common[0], ("a", 3));
        assert_eq!(most_common[1], ("b", 2));
        // The third item could be either "c" or "d" since they both have count 1
        assert_eq!(most_common[2].1, 1); // Check count is 1
        assert!(most_common[2].0 == "c" || most_common[2].0 == "d");
    }

    #[test]
    fn test_counter_total() {
        let items = vec!["a", "b", "a", "c", "b", "a"];
        let counter = COLLECTIONS::counter_from_iter(items.into_iter());
        assert_eq!(COLLECTIONS::counter_total(&counter), 6);
    }

    #[test]
    fn test_counter_subtract() {
        let mut counter1 = COLLECTIONS::counter_from_iter(vec!["a", "a", "b", "c"].into_iter());
        let counter2 = COLLECTIONS::counter_from_iter(vec!["a", "b"].into_iter());
        COLLECTIONS::counter_subtract(&mut counter1, &counter2);
        assert_eq!(counter1.get(&"a"), 1);
        assert_eq!(counter1.get(&"b"), 0);
        assert_eq!(counter1.get(&"c"), 1);
    }

    #[test]
    fn test_counter_update() {
        let mut counter = COLLECTIONS::counter_new::<String>();
        counter.increment("a".to_string());
        COLLECTIONS::counter_update(
            &mut counter,
            vec!["b", "a", "c"].into_iter().map(|s| s.to_string()),
        );
        assert_eq!(counter.get(&"a".to_string()), 2);
        assert_eq!(counter.get(&"b".to_string()), 1);
        assert_eq!(counter.get(&"c".to_string()), 1);
    }

    // ============================================================================
    // DefaultDict Tests
    // ============================================================================

    #[test]
    fn test_default_dict_new() {
        let mut dict = COLLECTIONS::default_dict_new::<_, String, Vec<i32>>(|| Vec::new());
        let value = COLLECTIONS::default_dict_get(&mut dict, "key".to_string());
        assert_eq!(value, Vec::<i32>::new());
    }

    #[test]
    fn test_default_dict_get_set() {
        let mut dict = COLLECTIONS::default_dict_new::<_, String, i32>(|| 0);
        COLLECTIONS::default_dict_set(&mut dict, "key".to_string(), 42);
        let value = COLLECTIONS::default_dict_get(&mut dict, "key".to_string());
        assert_eq!(value, 42);
    }

    #[test]
    fn test_default_dict_default_factory() {
        let mut dict = COLLECTIONS::default_dict_new::<_, String, i32>(|| 10);
        let value = COLLECTIONS::default_dict_get(&mut dict, "new_key".to_string());
        assert_eq!(value, 10);
    }

    // ============================================================================
    // ChainMap Tests
    // ============================================================================

    #[test]
    fn test_chain_map_new() {
        let chain = COLLECTIONS::chain_map_new::<String, i32>();
        assert!(chain.is_empty());
    }

    #[test]
    fn test_chain_map_add_get() {
        let mut chain = COLLECTIONS::chain_map_new::<String, i32>();
        let mut map1 = HashMap::new();
        map1.insert("key1".to_string(), 1);
        map1.insert("key2".to_string(), 2);

        let mut map2 = HashMap::new();
        map2.insert("key2".to_string(), 20);
        map2.insert("key3".to_string(), 3);

        COLLECTIONS::chain_map_add(&mut chain, map1);
        COLLECTIONS::chain_map_add(&mut chain, map2);

        // Should get value from most recently added map
        assert_eq!(
            COLLECTIONS::chain_map_get(&chain, &"key2".to_string()),
            Some(&20)
        );
        assert_eq!(
            COLLECTIONS::chain_map_get(&chain, &"key1".to_string()),
            Some(&1)
        );
        assert_eq!(
            COLLECTIONS::chain_map_get(&chain, &"key3".to_string()),
            Some(&3)
        );
    }

    #[test]
    fn test_chain_map_set() {
        let mut chain = COLLECTIONS::chain_map_new::<String, i32>();
        chain.set("key".to_string(), 42);
        assert_eq!(
            COLLECTIONS::chain_map_get(&chain, &"key".to_string()),
            Some(&42)
        );
    }

    // ============================================================================
    // NamedTuple Tests
    // ============================================================================

    #[test]
    fn test_named_tuple_new() {
        let fields = vec!["x".to_string(), "y".to_string()];
        let values = vec![1, 2];
        let nt = COLLECTIONS::named_tuple_new(fields, values);
        assert_eq!(nt.len(), 2);
    }

    #[test]
    fn test_named_tuple_get() {
        let fields = vec!["x".to_string(), "y".to_string()];
        let values = vec![1, 2];
        let nt = COLLECTIONS::named_tuple_new(fields, values);
        assert_eq!(COLLECTIONS::named_tuple_get(&nt, "x"), Some(&1));
        assert_eq!(COLLECTIONS::named_tuple_get(&nt, "y"), Some(&2));
        assert_eq!(COLLECTIONS::named_tuple_get(&nt, "z"), None);
    }

    #[test]
    fn test_named_tuple_set() {
        let fields = vec!["x".to_string(), "y".to_string()];
        let values = vec![1, 2];
        let mut nt = COLLECTIONS::named_tuple_new(fields, values);

        let result = COLLECTIONS::named_tuple_set(&mut nt, "x", 10);
        assert!(result.is_ok());
        assert_eq!(COLLECTIONS::named_tuple_get(&nt, "x"), Some(&10));

        let result = COLLECTIONS::named_tuple_set(&mut nt, "z", 5);
        assert!(result.is_err());
    }

    // ============================================================================
    // OrderedDict Tests
    // ============================================================================

    #[test]
    fn test_ordered_dict_new() {
        let dict = COLLECTIONS::ordered_dict_new::<String, i32>();
        assert!(dict.is_empty());
    }

    #[test]
    fn test_ordered_dict_insert_get() {
        let mut dict = COLLECTIONS::ordered_dict_new::<String, i32>();
        dict.insert("first".to_string(), 1);
        dict.insert("second".to_string(), 2);
        dict.insert("third".to_string(), 3);

        assert_eq!(dict.get(&"first".to_string()), Some(&1));
        assert_eq!(dict.get(&"second".to_string()), Some(&2));
        assert_eq!(dict.get(&"third".to_string()), Some(&3));
    }

    #[test]
    fn test_ordered_dict_move_to_end() {
        let mut dict = COLLECTIONS::ordered_dict_new::<String, i32>();
        dict.insert("first".to_string(), 1);
        dict.insert("second".to_string(), 2);
        dict.insert("third".to_string(), 3);

        dict.move_to_end(&"first".to_string());
        let keys: Vec<&String> = dict.keys().collect();
        assert_eq!(
            keys,
            vec![
                &"second".to_string(),
                &"third".to_string(),
                &"first".to_string()
            ]
        );
    }

    #[test]
    fn test_ordered_dict_popitem() {
        let mut dict = COLLECTIONS::ordered_dict_new::<String, i32>();
        dict.insert("first".to_string(), 1);
        dict.insert("second".to_string(), 2);
        dict.insert("third".to_string(), 3);

        let (key, value) = COLLECTIONS::ordered_dict_popitem(&mut dict, true).unwrap();
        assert_eq!(key, "third");
        assert_eq!(value, 3);

        let (key, value) = COLLECTIONS::ordered_dict_popitem(&mut dict, false).unwrap();
        assert_eq!(key, "first");
        assert_eq!(value, 1);
    }

    // ============================================================================
    // Deque Tests
    // ============================================================================

    #[test]
    fn test_deque_new() {
        let deque = COLLECTIONS::deque_new::<i32>();
        assert!(deque.is_empty());
    }

    #[test]
    fn test_deque_push_pop() {
        let mut deque = COLLECTIONS::deque_new::<i32>();
        deque.push_front(1);
        deque.push_back(2);
        deque.push_front(0);

        assert_eq!(deque.pop_front(), Some(0));
        assert_eq!(deque.pop_back(), Some(2));
        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn test_deque_rotate() {
        let mut deque = COLLECTIONS::deque_new::<i32>();
        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3);
        deque.push_back(4);

        COLLECTIONS::deque_rotate(&mut deque, 1);
        assert_eq!(deque.pop_front(), Some(4));
        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), Some(2));
        assert_eq!(deque.pop_front(), Some(3));
    }

    #[test]
    fn test_deque_extend() {
        let mut deque = COLLECTIONS::deque_new::<i32>();
        deque.push_back(1);
        COLLECTIONS::deque_extend(&mut deque, vec![2, 3, 4].into_iter());

        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), Some(2));
        assert_eq!(deque.pop_front(), Some(3));
        assert_eq!(deque.pop_front(), Some(4));
    }

    #[test]
    fn test_deque_get() {
        let mut deque = COLLECTIONS::deque_new::<i32>();
        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3);

        assert_eq!(deque.get(0), Some(&1));
        assert_eq!(deque.get(1), Some(&2));
        assert_eq!(deque.get(2), Some(&3));
        assert_eq!(deque.get(-1), Some(&3));
        assert_eq!(deque.get(-2), Some(&2));
        assert_eq!(deque.get(-3), Some(&1));
    }

    // ============================================================================
    // Heap Tests
    // ============================================================================

    #[test]
    fn test_heap_new() {
        let heap = COLLECTIONS::heap_new::<i32>();
        assert!(heap.is_empty());
    }

    #[test]
    fn test_heap_push_pop() {
        let mut heap = COLLECTIONS::heap_new::<i32>();
        heap.push(3);
        heap.push(1);
        heap.push(4);
        heap.push(1);

        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_heap_peek() {
        let mut heap = COLLECTIONS::heap_new::<i32>();
        heap.push(3);
        heap.push(1);
        heap.push(4);

        assert_eq!(heap.peek(), Some(&4));
        heap.pop();
        assert_eq!(heap.peek(), Some(&3));
    }

    #[test]
    fn test_heap_merge() {
        let mut heap1 = COLLECTIONS::heap_new::<i32>();
        heap1.push(1);
        heap1.push(3);

        let mut heap2 = std::collections::BinaryHeap::new();
        heap2.push(2);
        heap2.push(4);

        COLLECTIONS::heap_merge(&mut heap1, heap2);
        assert_eq!(heap1.len(), 4);
    }

    #[test]
    fn test_heap_nlargest() {
        let mut heap = COLLECTIONS::heap_new::<i32>();
        heap.push(1);
        heap.push(3);
        heap.push(2);
        heap.push(5);
        heap.push(4);

        let largest = COLLECTIONS::heap_nlargest(&heap, 3);
        assert_eq!(largest, vec![5, 4, 3]);
    }

    #[test]
    fn test_heap_nsmallest() {
        let mut heap = COLLECTIONS::heap_new::<i32>();
        heap.push(3);
        heap.push(1);
        heap.push(4);
        heap.push(2);
        heap.push(5);

        let smallest = COLLECTIONS::heap_nsmallest(&heap, 3);
        assert_eq!(smallest, vec![1, 2, 3]);
    }

    // ============================================================================
    // Integration Tests
    // ============================================================================

    #[test]
    fn test_counter_elements() {
        let items = vec!["a", "b", "a", "c"];
        let counter = COLLECTIONS::counter_from_iter(items.into_iter());
        let elements: Vec<&str> = counter.elements().collect();
        assert_eq!(elements.len(), 4);
        assert!(elements.contains(&"a"));
        assert!(elements.contains(&"b"));
        assert!(elements.contains(&"c"));
    }

    #[test]
    fn test_ordered_dict_iteration_order() {
        let mut dict = COLLECTIONS::ordered_dict_new::<String, i32>();
        dict.insert("first".to_string(), 1);
        dict.insert("second".to_string(), 2);
        dict.insert("third".to_string(), 3);

        let keys: Vec<&String> = dict.keys().collect();
        let values: Vec<&i32> = dict.values().collect();

        assert_eq!(
            keys,
            vec![
                &"first".to_string(),
                &"second".to_string(),
                &"third".to_string()
            ]
        );
        assert_eq!(values, vec![&1, &2, &3]);
    }

    #[test]
    fn test_chain_map_priority() {
        let mut chain = COLLECTIONS::chain_map_new::<String, i32>();

        let mut map1 = HashMap::new();
        map1.insert("key".to_string(), 1);
        COLLECTIONS::chain_map_add(&mut chain, map1);

        let mut map2 = HashMap::new();
        map2.insert("key".to_string(), 2);
        COLLECTIONS::chain_map_add(&mut chain, map2);

        // Should get value from most recently added map (map2)
        assert_eq!(
            COLLECTIONS::chain_map_get(&chain, &"key".to_string()),
            Some(&2)
        );
    }

    #[test]
    fn test_named_tuple_fields_values() {
        let fields = vec!["x".to_string(), "y".to_string(), "z".to_string()];
        let values = vec![1, 2, 3];
        let nt = COLLECTIONS::named_tuple_new(fields.clone(), values.clone());

        assert_eq!(nt.fields(), &fields);
        assert_eq!(nt.values(), &values);
    }
}
