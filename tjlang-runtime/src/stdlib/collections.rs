//! COLLECTIONS Module - Collection operations
//!
//! Provides comprehensive collection functionality including:
//! - Array/Vector operations
//! - Map/Dictionary operations
//! - Set operations
//! - Iterator utilities
//! - Collection algorithms
//! - Data structure implementations

use crate::values::Value;
use std::collections::{HashMap, HashSet, VecDeque, BinaryHeap, BTreeMap, BTreeSet};

/// COLLECTIONS module for collection operations
pub struct COLLECTIONS;

impl COLLECTIONS {
    // Array/Vector operations
    pub fn array_new<T>() -> Vec<T> { Vec::new() }
    pub fn array_with_capacity<T>(capacity: usize) -> Vec<T> { Vec::with_capacity(capacity) }
    pub fn array_from_slice<T: Clone>(slice: &[T]) -> Vec<T> { slice.to_vec() }
    
    pub fn array_push<T>(vec: &mut Vec<T>, item: T) { vec.push(item); }
    pub fn array_pop<T>(vec: &mut Vec<T>) -> Option<T> { vec.pop() }
    pub fn array_insert<T>(vec: &mut Vec<T>, index: usize, item: T) -> Result<(), String> {
        if index <= vec.len() {
            vec.insert(index, item);
            Ok(())
        } else {
            Err("Index out of bounds".to_string())
        }
    }
    pub fn array_remove<T>(vec: &mut Vec<T>, index: usize) -> Result<T, String> {
        if index < vec.len() {
            Ok(vec.remove(index))
        } else {
            Err("Index out of bounds".to_string())
        }
    }
    
    pub fn array_len<T>(vec: &Vec<T>) -> usize { vec.len() }
    pub fn array_capacity<T>(vec: &Vec<T>) -> usize { vec.capacity() }
    pub fn array_is_empty<T>(vec: &Vec<T>) -> bool { vec.is_empty() }
    
    pub fn array_get<T>(vec: &Vec<T>, index: usize) -> Option<&T> { vec.get(index) }
    pub fn array_get_mut<T>(vec: &mut Vec<T>, index: usize) -> Option<&mut T> { vec.get_mut(index) }
    pub fn array_set<T>(vec: &mut Vec<T>, index: usize, item: T) -> Result<(), String> {
        if index < vec.len() {
            vec[index] = item;
            Ok(())
        } else {
            Err("Index out of bounds".to_string())
        }
    }
    
    pub fn array_slice<T: Clone>(vec: &Vec<T>, start: usize, end: usize) -> Vec<T> {
        if start >= vec.len() || end > vec.len() || start >= end {
            return Vec::new();
        }
        vec[start..end].to_vec()
    }
    
    pub fn array_append<T>(vec: &mut Vec<T>, other: &mut Vec<T>) { vec.append(other); }
    pub fn array_extend<T: Clone>(vec: &mut Vec<T>, other: &[T]) { vec.extend_from_slice(other); }
    
    pub fn array_reverse<T>(vec: &mut Vec<T>) { vec.reverse(); }
    pub fn array_sort<T: Ord>(vec: &mut Vec<T>) { vec.sort(); }
    pub fn array_sort_by<F, T>(vec: &mut Vec<T>, compare: F) 
    where F: FnMut(&T, &T) -> std::cmp::Ordering {
        vec.sort_by(compare);
    }
    
    pub fn array_shuffle<T>(vec: &mut Vec<T>) {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        vec.shuffle(&mut rng);
    }
    
    pub fn array_unique<T: Clone + Eq + std::hash::Hash>(vec: &mut Vec<T>) {
        let mut seen = HashSet::new();
        vec.retain(|item| seen.insert(item.clone()));
    }
    
    pub fn array_filter<F, T>(vec: &Vec<T>, predicate: F) -> Vec<T> 
    where F: Fn(&T) -> bool, T: Clone {
        vec.iter().filter(|&x| predicate(x)).cloned().collect()
    }
    
    pub fn array_map<F, T, U>(vec: &Vec<T>, mapper: F) -> Vec<U> 
    where F: Fn(&T) -> U {
        vec.iter().map(mapper).collect()
    }
    
    pub fn array_reduce<F, T>(vec: &Vec<T>, initial: T, reducer: F) -> T 
    where F: Fn(T, &T) -> T {
        vec.iter().fold(initial, reducer)
    }
    
    pub fn array_fold<F, T, U>(vec: &Vec<T>, initial: U, folder: F) -> U 
    where F: Fn(U, &T) -> U {
        vec.iter().fold(initial, folder)
    }
    
    pub fn array_any<F, T>(vec: &Vec<T>, predicate: F) -> bool 
    where F: Fn(&T) -> bool {
        vec.iter().any(predicate)
    }
    
    pub fn array_all<F, T>(vec: &Vec<T>, predicate: F) -> bool 
    where F: Fn(&T) -> bool {
        vec.iter().all(predicate)
    }
    
    pub fn array_find<F, T>(vec: &Vec<T>, predicate: F) -> Option<&T> 
    where F: Fn(&&T) -> bool {
        vec.iter().find(predicate)
    }
    
    pub fn array_find_index<F, T>(vec: &Vec<T>, predicate: F) -> Option<usize> 
    where F: Fn(&T) -> bool {
        vec.iter().position(predicate)
    }
    
    pub fn array_contains<T: PartialEq>(vec: &Vec<T>, item: &T) -> bool {
        vec.contains(item)
    }
    
    pub fn array_index_of<T: PartialEq>(vec: &Vec<T>, item: &T) -> Option<usize> {
        vec.iter().position(|x| x == item)
    }
    
    pub fn array_last_index_of<T: PartialEq>(vec: &Vec<T>, item: &T) -> Option<usize> {
        vec.iter().rposition(|x| x == item)
    }
    
    // Map/Dictionary operations
    pub fn map_new<K, V>() -> HashMap<K, V> { HashMap::new() }
    pub fn map_with_capacity<K, V>(capacity: usize) -> HashMap<K, V> { HashMap::with_capacity(capacity) }
    
    pub fn map_insert<K, V>(map: &mut HashMap<K, V>, key: K, value: V) -> Option<V> 
    where K: std::hash::Hash + Eq {
        map.insert(key, value)
    }
    
    pub fn map_get<'a, K, V>(map: &'a HashMap<K, V>, key: &K) -> Option<&'a V> 
    where K: std::hash::Hash + Eq {
        map.get(key)
    }
    
    pub fn map_get_mut<'a, K, V>(map: &'a mut HashMap<K, V>, key: &K) -> Option<&'a mut V> 
    where K: std::hash::Hash + Eq {
        map.get_mut(key)
    }
    
    pub fn map_remove<K, V>(map: &mut HashMap<K, V>, key: &K) -> Option<V> 
    where K: std::hash::Hash + Eq {
        map.remove(key)
    }
    
    pub fn map_contains_key<K, V>(map: &HashMap<K, V>, key: &K) -> bool 
    where K: std::hash::Hash + Eq {
        map.contains_key(key)
    }
    
    pub fn map_len<K, V>(map: &HashMap<K, V>) -> usize { map.len() }
    pub fn map_is_empty<K, V>(map: &HashMap<K, V>) -> bool { map.is_empty() }
    
    pub fn map_keys<K, V>(map: &HashMap<K, V>) -> Vec<&K> { map.keys().collect() }
    pub fn map_values<K, V>(map: &HashMap<K, V>) -> Vec<&V> { map.values().collect() }
    pub fn map_entries<K, V>(map: &HashMap<K, V>) -> Vec<(&K, &V)> { map.iter().collect() }
    
    pub fn map_clear<K, V>(map: &mut HashMap<K, V>) { map.clear(); }
    
    // Set operations
    pub fn set_new<T>() -> HashSet<T> { HashSet::new() }
    pub fn set_with_capacity<T>(capacity: usize) -> HashSet<T> { HashSet::with_capacity(capacity) }
    pub fn set_from_vec<T: std::hash::Hash + Eq>(vec: Vec<T>) -> HashSet<T> { vec.into_iter().collect() }
    
    pub fn set_insert<T>(set: &mut HashSet<T>, item: T) -> bool 
    where T: std::hash::Hash + Eq {
        set.insert(item)
    }
    
    pub fn set_remove<T>(set: &mut HashSet<T>, item: &T) -> bool 
    where T: std::hash::Hash + Eq {
        set.remove(item)
    }
    
    pub fn set_contains<T>(set: &HashSet<T>, item: &T) -> bool 
    where T: std::hash::Hash + Eq {
        set.contains(item)
    }
    
    pub fn set_len<T>(set: &HashSet<T>) -> usize { set.len() }
    pub fn set_is_empty<T>(set: &HashSet<T>) -> bool { set.is_empty() }
    
    pub fn set_union<T: std::hash::Hash + Eq + Clone>(set1: &HashSet<T>, set2: &HashSet<T>) -> HashSet<T> {
        set1.union(set2).cloned().collect()
    }
    
    pub fn set_intersection<T: std::hash::Hash + Eq + Clone>(set1: &HashSet<T>, set2: &HashSet<T>) -> HashSet<T> {
        set1.intersection(set2).cloned().collect()
    }
    
    pub fn set_difference<T: std::hash::Hash + Eq + Clone>(set1: &HashSet<T>, set2: &HashSet<T>) -> HashSet<T> {
        set1.difference(set2).cloned().collect()
    }
    
    pub fn set_symmetric_difference<T: std::hash::Hash + Eq + Clone>(set1: &HashSet<T>, set2: &HashSet<T>) -> HashSet<T> {
        set1.symmetric_difference(set2).cloned().collect()
    }
    
    pub fn set_is_subset<T: std::hash::Hash + Eq>(set1: &HashSet<T>, set2: &HashSet<T>) -> bool {
        set1.is_subset(set2)
    }
    
    pub fn set_is_superset<T: std::hash::Hash + Eq>(set1: &HashSet<T>, set2: &HashSet<T>) -> bool {
        set1.is_superset(set2)
    }
    
    pub fn set_is_disjoint<T: std::hash::Hash + Eq>(set1: &HashSet<T>, set2: &HashSet<T>) -> bool {
        set1.is_disjoint(set2)
    }
    
    // Queue operations
    pub fn queue_new<T>() -> VecDeque<T> { VecDeque::new() }
    pub fn queue_with_capacity<T>(capacity: usize) -> VecDeque<T> { VecDeque::with_capacity(capacity) }
    
    pub fn queue_push_back<T>(queue: &mut VecDeque<T>, item: T) { queue.push_back(item); }
    pub fn queue_push_front<T>(queue: &mut VecDeque<T>, item: T) { queue.push_front(item); }
    pub fn queue_pop_back<T>(queue: &mut VecDeque<T>) -> Option<T> { queue.pop_back() }
    pub fn queue_pop_front<T>(queue: &mut VecDeque<T>) -> Option<T> { queue.pop_front() }
    
    pub fn queue_len<T>(queue: &VecDeque<T>) -> usize { queue.len() }
    pub fn queue_is_empty<T>(queue: &VecDeque<T>) -> bool { queue.is_empty() }
    
    // Priority queue operations
    pub fn priority_queue_new<T: Ord>() -> BinaryHeap<T> { BinaryHeap::new() }
    pub fn priority_queue_with_capacity<T: Ord>(capacity: usize) -> BinaryHeap<T> { BinaryHeap::with_capacity(capacity) }
    
    pub fn priority_queue_push<T: Ord>(queue: &mut BinaryHeap<T>, item: T) { queue.push(item); }
    pub fn priority_queue_pop<T: Ord>(queue: &mut BinaryHeap<T>) -> Option<T> { queue.pop() }
    pub fn priority_queue_peek<T: Ord>(queue: &BinaryHeap<T>) -> Option<&T> { queue.peek() }
    
    pub fn priority_queue_len<T: Ord>(queue: &BinaryHeap<T>) -> usize { queue.len() }
    pub fn priority_queue_is_empty<T: Ord>(queue: &BinaryHeap<T>) -> bool { queue.is_empty() }
    
    // BTree operations
    pub fn btree_map_new<K, V>() -> BTreeMap<K, V> { BTreeMap::new() }
    pub fn btree_set_new<T>() -> BTreeSet<T> { BTreeSet::new() }
    
    // Iterator utilities
    pub fn iterator_map<F, T, U>(iter: impl Iterator<Item = T>, mapper: F) -> impl Iterator<Item = U> 
    where F: Fn(T) -> U {
        iter.map(mapper)
    }
    
    pub fn iterator_filter<F, T>(iter: impl Iterator<Item = T>, predicate: F) -> impl Iterator<Item = T> 
    where F: Fn(&T) -> bool {
        iter.filter(predicate)
    }
    
    pub fn iterator_fold<F, T, U>(iter: impl Iterator<Item = T>, initial: U, folder: F) -> U 
    where F: Fn(U, T) -> U {
        iter.fold(initial, folder)
    }
    
    pub fn iterator_reduce<F, T>(iter: impl Iterator<Item = T>, reducer: F) -> Option<T> 
    where F: Fn(T, T) -> T {
        iter.reduce(reducer)
    }
    
    pub fn iterator_any<F, T>(mut iter: impl Iterator<Item = T>, predicate: F) -> bool 
    where F: Fn(T) -> bool {
        iter.any(predicate)
    }
    
    pub fn iterator_all<F, T>(mut iter: impl Iterator<Item = T>, predicate: F) -> bool 
    where F: Fn(T) -> bool {
        iter.all(predicate)
    }
    
    pub fn iterator_find<F, T>(mut iter: impl Iterator<Item = T>, predicate: F) -> Option<T> 
    where F: Fn(&T) -> bool {
        iter.find(predicate)
    }
    
    pub fn iterator_count<T>(iter: impl Iterator<Item = T>) -> usize { iter.count() }
    pub fn iterator_sum<T>(iter: impl Iterator<Item = T>) -> T 
    where T: std::iter::Sum {
        iter.sum()
    }
    
    pub fn iterator_product<T>(iter: impl Iterator<Item = T>) -> T 
    where T: std::iter::Product {
        iter.product()
    }
    
    pub fn iterator_min<T: Ord>(iter: impl Iterator<Item = T>) -> Option<T> { iter.min() }
    pub fn iterator_max<T: Ord>(iter: impl Iterator<Item = T>) -> Option<T> { iter.max() }
    
    pub fn iterator_zip<T, U>(iter1: impl Iterator<Item = T>, iter2: impl Iterator<Item = U>) -> impl Iterator<Item = (T, U)> {
        iter1.zip(iter2)
    }
    
    pub fn iterator_chain<T>(iter1: impl Iterator<Item = T>, iter2: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
        iter1.chain(iter2)
    }
    
    pub fn iterator_cycle<T: Clone>(iter: impl Iterator<Item = T> + Clone) -> impl Iterator<Item = T> {
        iter.cycle()
    }
    
    pub fn iterator_take<T>(iter: impl Iterator<Item = T>, n: usize) -> impl Iterator<Item = T> {
        iter.take(n)
    }
    
    pub fn iterator_skip<T>(iter: impl Iterator<Item = T>, n: usize) -> impl Iterator<Item = T> {
        iter.skip(n)
    }
    
    pub fn iterator_step_by<T>(iter: impl Iterator<Item = T>, step: usize) -> impl Iterator<Item = T> {
        iter.step_by(step)
    }
    
    pub fn iterator_enumerate<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = (usize, T)> {
        iter.enumerate()
    }
    
    pub fn iterator_rev<T>(iter: impl Iterator<Item = T> + DoubleEndedIterator) -> impl Iterator<Item = T> {
        iter.rev()
    }
    
    // Collection algorithms
    pub fn binary_search<T: Ord>(vec: &Vec<T>, target: &T) -> Result<usize, usize> {
        vec.binary_search(target)
    }
    
    pub fn linear_search<T: PartialEq>(vec: &Vec<T>, target: &T) -> Option<usize> {
        vec.iter().position(|x| x == target)
    }
    
    pub fn sort<T: Ord>(vec: &mut Vec<T>) { vec.sort(); }
    pub fn sort_by<F, T>(vec: &mut Vec<T>, compare: F) 
    where F: FnMut(&T, &T) -> std::cmp::Ordering {
        vec.sort_by(compare);
    }
    
    pub fn sort_by_key<F, K, T>(vec: &mut Vec<T>, key: F) 
    where F: FnMut(&T) -> K, K: Ord {
        vec.sort_by_key(key);
    }
    
    pub fn reverse<T>(vec: &mut Vec<T>) { vec.reverse(); }
    pub fn shuffle<T>(vec: &mut Vec<T>) {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        vec.shuffle(&mut rng);
    }
    
    pub fn rotate_left<T>(vec: &mut Vec<T>, mid: usize) { vec.rotate_left(mid); }
    pub fn rotate_right<T>(vec: &mut Vec<T>, k: usize) { vec.rotate_right(k); }
    
    pub fn partition<F, T>(vec: &mut Vec<T>, mut predicate: F) -> usize 
    where F: FnMut(&T) -> bool {
        let (left, right): (Vec<T>, Vec<T>) = vec.drain(..).partition(|x| predicate(x));
        left.len()
    }
    
    pub fn dedup<T: PartialEq>(vec: &mut Vec<T>) { vec.dedup(); }
    pub fn dedup_by<F, T>(vec: &mut Vec<T>, mut same_bucket: F) 
    where F: FnMut(&mut T, &mut T) -> bool {
        vec.dedup_by(same_bucket);
    }
    
    pub fn dedup_by_key<F, K, T>(vec: &mut Vec<T>, mut key: F) 
    where F: FnMut(&mut T) -> K, K: PartialEq {
        vec.dedup_by_key(key);
    }
    
    // ============================================================================
    // Advanced Collection Types (Python collections module equivalents)
    // ============================================================================
    
    // Counter - Count occurrences of elements
    pub fn counter_new<T: std::hash::Hash + Eq + Clone>() -> Counter<T> {
        Counter::new()
    }
    
    pub fn counter_from_iter<T: std::hash::Hash + Eq + Clone, I: Iterator<Item = T>>(iter: I) -> Counter<T> {
        Counter::from_iter(iter)
    }
    
    pub fn counter_most_common<T: std::hash::Hash + Eq + Clone>(counter: &Counter<T>, n: Option<usize>) -> Vec<(T, usize)> {
        counter.most_common(n)
    }
    
    pub fn counter_total<T: std::hash::Hash + Eq + Clone>(counter: &Counter<T>) -> usize {
        counter.total()
    }
    
    pub fn counter_subtract<T: std::hash::Hash + Eq + Clone>(counter: &mut Counter<T>, other: &Counter<T>) {
        counter.subtract(other);
    }
    
    pub fn counter_update<T: std::hash::Hash + Eq + Clone, I: Iterator<Item = T>>(counter: &mut Counter<T>, iter: I) {
        counter.update(iter);
    }
    
    // DefaultDict - Dictionary with default values
    pub fn default_dict_new<F: Fn() -> V, K: std::hash::Hash + Eq + Clone, V: Clone>(default_factory: F) -> DefaultDict<K, V, F> {
        DefaultDict::new(default_factory)
    }
    
    pub fn default_dict_get<F: Fn() -> V, K: std::hash::Hash + Eq + Clone, V: Clone>(dict: &mut DefaultDict<K, V, F>, key: K) -> V {
        dict.get(key)
    }
    
    pub fn default_dict_set<F: Fn() -> V, K: std::hash::Hash + Eq + Clone, V: Clone>(dict: &mut DefaultDict<K, V, F>, key: K, value: V) {
        dict.set(key, value);
    }
    
    // ChainMap - Chain multiple dictionaries
    pub fn chain_map_new<K: std::hash::Hash + Eq + Clone, V: Clone>() -> ChainMap<K, V> {
        ChainMap::new()
    }
    
    pub fn chain_map_add<K: std::hash::Hash + Eq + Clone, V: Clone>(chain: &mut ChainMap<K, V>, map: HashMap<K, V>) {
        chain.add(map);
    }
    
    pub fn chain_map_get<'a, K: std::hash::Hash + Eq + Clone, V: Clone>(chain: &'a ChainMap<K, V>, key: &'a K) -> Option<&'a V> {
        chain.get(key)
    }
    
    // NamedTuple - Tuple with named fields
    pub fn named_tuple_new<T: Clone>(fields: Vec<String>, values: Vec<T>) -> NamedTuple<T> {
        NamedTuple::new(fields, values)
    }
    
    pub fn named_tuple_get<'a, T: Clone>(nt: &'a NamedTuple<T>, field: &'a str) -> Option<&'a T> {
        nt.get(field)
    }
    
    pub fn named_tuple_set<T: Clone>(nt: &mut NamedTuple<T>, field: &str, value: T) -> Result<(), String> {
        nt.set(field, value)
    }
    
    // OrderedDict - Dictionary that remembers insertion order
    pub fn ordered_dict_new<K: std::hash::Hash + Eq + Clone, V: Clone>() -> OrderedDict<K, V> {
        OrderedDict::new()
    }
    
    pub fn ordered_dict_move_to_end<K: std::hash::Hash + Eq + Clone, V: Clone>(dict: &mut OrderedDict<K, V>, key: &K) {
        dict.move_to_end(key);
    }
    
    pub fn ordered_dict_popitem<K: std::hash::Hash + Eq + Clone, V: Clone>(dict: &mut OrderedDict<K, V>, last: bool) -> Option<(K, V)> {
        dict.popitem(last)
    }
    
    // Deque - Double-ended queue with additional features
    pub fn deque_new<T>() -> Deque<T> {
        Deque::new()
    }
    
    pub fn deque_rotate<T>(deque: &mut Deque<T>, n: isize) {
        deque.rotate(n);
    }
    
    pub fn deque_extend<T: Clone>(deque: &mut Deque<T>, iter: impl Iterator<Item = T>) {
        deque.extend(iter);
    }
    
    // Heap - Priority queue with additional operations
    pub fn heap_new<T: Ord + Clone>() -> Heap<T> {
        Heap::new()
    }
    
    pub fn heap_merge<T: Ord + Clone>(heap: &mut Heap<T>, other: BinaryHeap<T>) {
        heap.merge(other);
    }
    
    pub fn heap_nlargest<T: Ord + Clone>(heap: &Heap<T>, n: usize) -> Vec<T> {
        heap.nlargest(n)
    }
    
    pub fn heap_nsmallest<T: Ord + Clone>(heap: &Heap<T>, n: usize) -> Vec<T> {
        heap.nsmallest(n)
    }
}

// ============================================================================
// Advanced Collection Type Implementations
// ============================================================================

/// Counter - Count occurrences of elements (like Python's Counter)
#[derive(Debug, Clone, PartialEq)]
pub struct Counter<T: std::hash::Hash + Eq + Clone> {
    counts: HashMap<T, usize>,
}

impl<T: std::hash::Hash + Eq + Clone> Counter<T> {
    pub fn new() -> Self {
        Counter {
            counts: HashMap::new(),
        }
    }
    
    pub fn from_iter<I: Iterator<Item = T>>(iter: I) -> Self {
        let mut counter = Counter::new();
        for item in iter {
            *counter.counts.entry(item).or_insert(0) += 1;
        }
        counter
    }
    
    pub fn get(&self, key: &T) -> usize {
        self.counts.get(key).copied().unwrap_or(0)
    }
    
    pub fn set(&mut self, key: T, count: usize) {
        if count == 0 {
            self.counts.remove(&key);
        } else {
            self.counts.insert(key, count);
        }
    }
    
    pub fn increment(&mut self, key: T) {
        *self.counts.entry(key).or_insert(0) += 1;
    }
    
    pub fn decrement(&mut self, key: T) {
        if let Some(count) = self.counts.get_mut(&key) {
            if *count > 1 {
                *count -= 1;
            } else {
                self.counts.remove(&key);
            }
        }
    }
    
    pub fn most_common(&self, n: Option<usize>) -> Vec<(T, usize)> {
        let mut items: Vec<(T, usize)> = self.counts.iter().map(|(k, v)| (k.clone(), *v)).collect();
        items.sort_by(|a, b| b.1.cmp(&a.1));
        if let Some(n) = n {
            items.truncate(n);
        }
        items
    }
    
    pub fn total(&self) -> usize {
        self.counts.values().sum()
    }
    
    pub fn subtract(&mut self, other: &Counter<T>) {
        for (key, count) in &other.counts {
            self.decrement(key.clone());
            for _ in 0..count.saturating_sub(1) {
                self.decrement(key.clone());
            }
        }
    }
    
    pub fn update<I: Iterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.increment(item);
        }
    }
    
    pub fn elements(&self) -> impl Iterator<Item = T> + '_ {
        self.counts.iter().flat_map(|(item, &count)| {
            std::iter::repeat(item.clone()).take(count)
        })
    }
    
    pub fn len(&self) -> usize {
        self.counts.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.counts.is_empty()
    }
}

/// DefaultDict - Dictionary with default values (like Python's defaultdict)
#[derive(Debug, Clone)]
pub struct DefaultDict<K: std::hash::Hash + Eq + Clone, V: Clone, F: Fn() -> V> {
    data: HashMap<K, V>,
    default_factory: F,
}

impl<K: std::hash::Hash + Eq + Clone, V: Clone, F: Fn() -> V> DefaultDict<K, V, F> {
    pub fn new(default_factory: F) -> Self {
        DefaultDict {
            data: HashMap::new(),
            default_factory,
        }
    }
    
    pub fn get(&mut self, key: K) -> V {
        self.data.entry(key).or_insert_with(&self.default_factory).clone()
    }
    
    pub fn set(&mut self, key: K, value: V) {
        self.data.insert(key, value);
    }
    
    pub fn contains_key(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }
    
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.data.remove(key)
    }
    
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

/// ChainMap - Chain multiple dictionaries (like Python's ChainMap)
#[derive(Debug, Clone)]
pub struct ChainMap<K: std::hash::Hash + Eq + Clone, V: Clone> {
    maps: Vec<HashMap<K, V>>,
}

impl<K: std::hash::Hash + Eq + Clone, V: Clone> ChainMap<K, V> {
    pub fn new() -> Self {
        ChainMap { maps: Vec::new() }
    }
    
    pub fn add(&mut self, map: HashMap<K, V>) {
        self.maps.push(map);
    }
    
    pub fn get(&self, key: &K) -> Option<&V> {
        for map in self.maps.iter().rev() {
            if let Some(value) = map.get(key) {
                return Some(value);
            }
        }
        None
    }
    
    pub fn set(&mut self, key: K, value: V) {
        if self.maps.is_empty() {
            self.maps.push(HashMap::new());
        }
        self.maps[0].insert(key, value);
    }
    
    pub fn len(&self) -> usize {
        self.maps.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.maps.is_empty()
    }
}

/// NamedTuple - Tuple with named fields (like Python's namedtuple)
#[derive(Debug, Clone, PartialEq)]
pub struct NamedTuple<T: Clone> {
    fields: Vec<String>,
    values: Vec<T>,
}

impl<T: Clone> NamedTuple<T> {
    pub fn new(fields: Vec<String>, values: Vec<T>) -> Self {
        NamedTuple { fields, values }
    }
    
    pub fn get(&self, field: &str) -> Option<&T> {
        if let Some(index) = self.fields.iter().position(|f| f == field) {
            self.values.get(index)
        } else {
            None
        }
    }
    
    pub fn set(&mut self, field: &str, value: T) -> Result<(), String> {
        if let Some(index) = self.fields.iter().position(|f| f == field) {
            self.values[index] = value;
            Ok(())
        } else {
            Err(format!("Field '{}' not found", field))
        }
    }
    
    pub fn fields(&self) -> &[String] {
        &self.fields
    }
    
    pub fn values(&self) -> &[T] {
        &self.values
    }
    
    pub fn len(&self) -> usize {
        self.fields.len()
    }
}

/// OrderedDict - Dictionary that remembers insertion order (like Python's OrderedDict)
#[derive(Debug, Clone)]
pub struct OrderedDict<K: std::hash::Hash + Eq + Clone, V: Clone> {
    data: HashMap<K, V>,
    order: Vec<K>,
}

impl<K: std::hash::Hash + Eq + Clone, V: Clone> OrderedDict<K, V> {
    pub fn new() -> Self {
        OrderedDict {
            data: HashMap::new(),
            order: Vec::new(),
        }
    }
    
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let old_value = self.data.insert(key.clone(), value);
        if old_value.is_none() {
            self.order.push(key);
        }
        old_value
    }
    
    pub fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }
    
    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(value) = self.data.remove(key) {
            self.order.retain(|k| k != key);
            Some(value)
        } else {
            None
        }
    }
    
    pub fn move_to_end(&mut self, key: &K) {
        if self.data.contains_key(key) {
            self.order.retain(|k| k != key);
            self.order.push(key.clone());
        }
    }
    
    pub fn popitem(&mut self, last: bool) -> Option<(K, V)> {
        if let Some(key) = if last {
            self.order.pop()
        } else {
            if !self.order.is_empty() {
                Some(self.order.remove(0))
            } else {
                None
            }
        } {
            if let Some(value) = self.data.remove(&key) {
                Some((key, value))
            } else {
                None
            }
        } else {
            None
        }
    }
    
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.order.iter()
    }
    
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.order.iter().filter_map(|k| self.data.get(k))
    }
    
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

/// Deque - Double-ended queue with additional features (like Python's deque)
#[derive(Debug, Clone)]
pub struct Deque<T> {
    data: VecDeque<T>,
}

impl<T> Deque<T> {
    pub fn new() -> Self {
        Deque {
            data: VecDeque::new(),
        }
    }
    
    pub fn with_capacity(capacity: usize) -> Self {
        Deque {
            data: VecDeque::with_capacity(capacity),
        }
    }
    
    pub fn push_front(&mut self, item: T) {
        self.data.push_front(item);
    }
    
    pub fn push_back(&mut self, item: T) {
        self.data.push_back(item);
    }
    
    pub fn pop_front(&mut self) -> Option<T> {
        self.data.pop_front()
    }
    
    pub fn pop_back(&mut self) -> Option<T> {
        self.data.pop_back()
    }
    
    pub fn rotate(&mut self, n: isize) {
        if self.data.is_empty() {
            return;
        }
        
        let len = self.data.len() as isize;
        let n = n % len;
        
        if n > 0 {
            for _ in 0..n {
                if let Some(item) = self.data.pop_back() {
                    self.data.push_front(item);
                }
            }
        } else if n < 0 {
            for _ in 0..(-n) {
                if let Some(item) = self.data.pop_front() {
                    self.data.push_back(item);
                }
            }
        }
    }
    
    pub fn extend<I: Iterator<Item = T>>(&mut self, iter: I) {
        self.data.extend(iter);
    }
    
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    
    pub fn get(&self, index: isize) -> Option<&T> {
        if index >= 0 {
            self.data.get(index as usize)
        } else {
            let len = self.data.len() as isize;
            let adjusted_index = len + index;
            if adjusted_index >= 0 {
                self.data.get(adjusted_index as usize)
            } else {
                None
            }
        }
    }
}

/// Heap - Priority queue with additional operations (like Python's heapq)
#[derive(Debug, Clone)]
pub struct Heap<T: Ord> {
    data: BinaryHeap<T>,
}

impl<T: Ord + Clone> Heap<T> {
    pub fn new() -> Self {
        Heap {
            data: BinaryHeap::new(),
        }
    }
    
    pub fn push(&mut self, item: T) {
        self.data.push(item);
    }
    
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
    
    pub fn peek(&self) -> Option<&T> {
        self.data.peek()
    }
    
    pub fn merge(&mut self, other: BinaryHeap<T>) {
        self.data.extend(other);
    }
    
    pub fn nlargest(&self, n: usize) -> Vec<T> {
        let mut items: Vec<T> = self.data.iter().map(|x| x.clone()).collect();
        items.sort_by(|a, b| b.cmp(a));
        items.truncate(n);
        items
    }
    
    pub fn nsmallest(&self, n: usize) -> Vec<T> {
        let mut items: Vec<T> = self.data.iter().map(|x| x.clone()).collect();
        items.sort();
        items.truncate(n);
        items
    }
    
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
