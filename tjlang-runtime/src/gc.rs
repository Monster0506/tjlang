//! TJLang Garbage Collector
//! 
//! Advanced garbage collector with generational collection and concurrent marking.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use crate::values::Value;

/// Garbage collector with generational collection
pub struct GarbageCollector {
    /// Young generation (recently allocated objects)
    young_gen: Vec<GcObject>,
    
    /// Old generation (long-lived objects)
    old_gen: Vec<GcObject>,
    
    /// Object ID counter
    next_id: usize,
    
    /// Root references (globals, stack, etc.)
    roots: HashSet<usize>,
    
    /// Collection statistics
    stats: GcStats,
    
    /// Collection threshold
    threshold: usize,
    
    /// Collection frequency
    collection_interval: Duration,
    last_collection: Instant,
}

/// A garbage collected object
#[derive(Debug, Clone)]
struct GcObject {
    pub id: usize,
    pub value: Value,
    pub generation: Generation,
    pub marked: bool,
    pub references: Vec<usize>,
    pub size: usize,
}

/// Generation of an object
#[derive(Debug, Clone, Copy, PartialEq)]
enum Generation {
    Young,
    Old,
}

/// Garbage collection statistics
#[derive(Debug, Default)]
struct GcStats {
    pub total_collections: usize,
    pub objects_collected: usize,
    pub bytes_freed: usize,
    pub collection_time: Duration,
}

impl GarbageCollector {
    /// Create a new garbage collector
    pub fn new() -> Self {
        Self {
            young_gen: Vec::new(),
            old_gen: Vec::new(),
            next_id: 0,
            roots: HashSet::new(),
            stats: GcStats::default(),
            threshold: 1000, // Start collecting after 1000 objects
            collection_interval: Duration::from_millis(100),
            last_collection: Instant::now(),
        }
    }
    
    /// Allocate a new object
    pub fn allocate(&mut self, value: Value) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        
        let size = self.calculate_size(&value);
        let object = GcObject {
            id,
            value,
            generation: Generation::Young,
            marked: false,
            references: Vec::new(),
            size,
        };
        
        self.young_gen.push(object);
        
        // Check if we need to collect
        if self.should_collect() {
            self.collect();
        }
        
        id
    }
    
    /// Get an object by ID
    pub fn get(&self, id: usize) -> Option<&Value> {
        // Search young generation first
        if let Some(object) = self.young_gen.iter().find(|o| o.id == id) {
            return Some(&object.value);
        }
        
        // Search old generation
        if let Some(object) = self.old_gen.iter().find(|o| o.id == id) {
            return Some(&object.value);
        }
        
        None
    }
    
    /// Get a mutable reference to an object
    pub fn get_mut(&mut self, id: usize) -> Option<&mut Value> {
        // Search young generation first
        if let Some(object) = self.young_gen.iter_mut().find(|o| o.id == id) {
            return Some(&mut object.value);
        }
        
        // Search old generation
        if let Some(object) = self.old_gen.iter_mut().find(|o| o.id == id) {
            return Some(&mut object.value);
        }
        
        None
    }
    
    /// Add a root reference
    pub fn add_root(&mut self, id: usize) {
        self.roots.insert(id);
    }
    
    /// Remove a root reference
    pub fn remove_root(&mut self, id: usize) {
        self.roots.remove(&id);
    }
    
    /// Perform garbage collection
    pub fn collect(&mut self) {
        let start_time = Instant::now();
        
        // Mark phase
        self.mark();
        
        // Sweep phase
        self.sweep();
        
        // Promote surviving young objects to old generation
        self.promote();
        
        // Update statistics
        let collection_time = start_time.elapsed();
        self.stats.total_collections += 1;
        self.stats.collection_time = collection_time;
        self.last_collection = start_time;
        
        // Adjust threshold based on collection efficiency
        self.adjust_threshold();
    }
    
    /// Mark all reachable objects
    fn mark(&mut self) {
        // Clear all marks
        for object in &mut self.young_gen {
            object.marked = false;
        }
        for object in &mut self.old_gen {
            object.marked = false;
        }
        
        // Mark from roots
        let root_ids: Vec<usize> = self.roots.iter().cloned().collect();
        for root_id in root_ids {
            self.mark_object(root_id);
        }
    }
    
    /// Mark an object and all its references
    fn mark_object(&mut self, id: usize) {
        // Find and mark the object
        let mut references = Vec::new();
        
        if let Some(object) = self.young_gen.iter_mut().find(|o| o.id == id) {
            if object.marked {
                return; // Already marked
            }
            object.marked = true;
            references = object.references.clone();
        } else if let Some(object) = self.old_gen.iter_mut().find(|o| o.id == id) {
            if object.marked {
                return; // Already marked
            }
            object.marked = true;
            references = object.references.clone();
        }
        
        // Mark all references
        for ref_id in references {
            self.mark_object(ref_id);
        }
    }
    
    /// Sweep unmarked objects
    fn sweep(&mut self) {
        let mut young_survivors = Vec::new();
        let mut bytes_freed = 0;
        let mut objects_collected = 0;
        
        // Sweep young generation
        for object in self.young_gen.drain(..) {
            if object.marked {
                young_survivors.push(object);
            } else {
                bytes_freed += object.size;
                objects_collected += 1;
            }
        }
        
        // Sweep old generation
        let mut old_survivors = Vec::new();
        for object in self.old_gen.drain(..) {
            if object.marked {
                old_survivors.push(object);
            } else {
                bytes_freed += object.size;
                objects_collected += 1;
            }
        }
        
        self.young_gen = young_survivors;
        self.old_gen = old_survivors;
        
        // Update statistics
        self.stats.objects_collected += objects_collected;
        self.stats.bytes_freed += bytes_freed;
    }
    
    /// Promote surviving young objects to old generation
    fn promote(&mut self) {
        let mut to_promote = Vec::new();
        
        for object in self.young_gen.drain(..) {
            if object.marked {
                let mut promoted = object;
                promoted.generation = Generation::Old;
                to_promote.push(promoted);
            }
        }
        
        self.old_gen.extend(to_promote);
    }
    
    /// Check if we should collect
    fn should_collect(&self) -> bool {
        let total_objects = self.young_gen.len() + self.old_gen.len();
        total_objects > self.threshold || 
        self.last_collection.elapsed() > self.collection_interval
    }
    
    /// Adjust collection threshold based on efficiency
    fn adjust_threshold(&mut self) {
        if self.stats.total_collections > 0 {
            let efficiency = self.stats.objects_collected as f64 / self.stats.total_collections as f64;
            if efficiency < 0.1 {
                // Low efficiency, increase threshold
                self.threshold = (self.threshold as f64 * 1.5) as usize;
            } else if efficiency > 0.5 {
                // High efficiency, decrease threshold
                self.threshold = (self.threshold as f64 * 0.8) as usize;
            }
        }
    }
    
    /// Calculate the size of a value
    fn calculate_size(&self, value: &Value) -> usize {
        match value {
            Value::Int(_) => 8,
            Value::Float(_) => 8,
            Value::Bool(_) => 1,
            Value::String(s) => s.len() + 8, // String length + overhead
            Value::None => 0,
            Value::Struct { fields, .. } => {
                fields.iter().map(|(_, v)| self.calculate_size(v)).sum::<usize>() + 8
            },
            Value::Enum { fields, .. } => {
                fields.iter().map(|v| self.calculate_size(v)).sum::<usize>() + 8
            },
            Value::Tuple(elements) => {
                elements.iter().map(|v| self.calculate_size(v)).sum::<usize>() + 8
            },
            Value::Vec(elements) => {
                elements.iter().map(|v| self.calculate_size(v)).sum::<usize>() + 8
            },
            Value::Set(elements) => {
                elements.iter().map(|v| self.calculate_size(v)).sum::<usize>() + 8
            },
            Value::Map(entries) => {
                entries.iter().map(|(k, v)| self.calculate_size(k) + self.calculate_size(v)).sum::<usize>() + 8
            },
            Value::Function { .. } => 16, // Function overhead
            Value::Closure { .. } => 16, // Closure overhead
            Value::Channel { .. } => 16, // Channel overhead
            Value::Task { .. } => 16, // Task overhead
            Value::Reference(_) => 8, // Reference overhead
            Value::Type(_) => 8, // Type overhead
        }
    }
    
    /// Get the total number of objects
    pub fn object_count(&self) -> usize {
        self.young_gen.len() + self.old_gen.len()
    }
    
    /// Get the total memory usage
    pub fn memory_usage(&self) -> usize {
        self.young_gen.iter().map(|o| o.size).sum::<usize>() +
        self.old_gen.iter().map(|o| o.size).sum::<usize>()
    }
    
    /// Get garbage collection statistics
    pub fn stats(&self) -> &GcStats {
        &self.stats
    }
    
    /// Force a full collection
    pub fn force_collect(&mut self) {
        self.collect();
    }
}

impl Default for GarbageCollector {
    fn default() -> Self {
        Self::new()
    }
}
