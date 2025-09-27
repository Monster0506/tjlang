//! TJLang Module System
//! 
//! Advanced module loading, importing, and dependency management.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use crate::values::Value;
use tjlang_ast::{Program, Declaration, FunctionDecl, Type, SourceSpan};

/// Module system for managing imports and exports
pub struct ModuleSystem {
    /// Loaded modules
    modules: HashMap<String, Module>,
    
    /// Module search paths
    search_paths: Vec<PathBuf>,
    
    /// Module cache
    cache: HashMap<String, Arc<Module>>,
    
    /// Import graph for dependency resolution
    import_graph: HashMap<String, Vec<String>>,
}

/// A loaded module
#[derive(Debug, Clone)]
pub struct Module {
    /// Module name
    pub name: String,
    
    /// Module path
    pub path: PathBuf,
    
    /// Module declarations
    pub declarations: Vec<Declaration>,
    
    /// Exported items
    pub exports: HashMap<String, ExportItem>,
    
    /// Dependencies
    pub dependencies: Vec<String>,
    
    /// Module metadata
    pub metadata: ModuleMetadata,
}

/// Exported item from a module
#[derive(Debug, Clone)]
pub enum ExportItem {
    /// Exported function
    Function(FunctionDecl),
    
    /// Exported type
    Type(Type),
    
    /// Exported value
    Value(Value),
    
    /// Re-exported item
    ReExport(String, String), // module, item
}

/// Module metadata
#[derive(Debug, Clone)]
pub struct ModuleMetadata {
    /// Module version
    pub version: String,
    
    /// Module author
    pub author: String,
    
    /// Module description
    pub description: String,
    
    /// Module dependencies
    pub dependencies: HashMap<String, String>,
    
    /// Module features
    pub features: Vec<String>,
}

impl ModuleSystem {
    /// Create a new module system
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            search_paths: Vec::new(),
            cache: HashMap::new(),
            import_graph: HashMap::new(),
        }
    }
    
    /// Add a search path for modules
    pub fn add_search_path(&mut self, path: PathBuf) {
        self.search_paths.push(path);
    }
    
    /// Load a module by name
    pub fn load_module(&mut self, name: &str) -> Result<Arc<Module>, String> {
        // Check cache first
        if let Some(module) = self.cache.get(name) {
            return Ok(module.clone());
        }
        
        // Find module file
        let module_path = self.find_module(name)?;
        
        // Load module content
        let module = self.load_module_from_path(&module_path)?;
        
        // Cache the module
        let module_arc = Arc::new(module);
        self.cache.insert(name.to_string(), module_arc.clone());
        
        Ok(module_arc)
    }
    
    /// Find a module file
    fn find_module(&self, name: &str) -> Result<PathBuf, String> {
        for search_path in &self.search_paths {
            let module_path = search_path.join(format!("{}.tj", name));
            if module_path.exists() {
                return Ok(module_path);
            }
            
            // Check for module directory
            let module_dir = search_path.join(name);
            let module_file = module_dir.join("mod.tj");
            if module_file.exists() {
                return Ok(module_file);
            }
        }
        
        Err(format!("Module '{}' not found", name))
    }
    
    /// Load module from file path
    fn load_module_from_path(&self, path: &Path) -> Result<Module, String> {
        // TODO: Implement actual file loading and parsing
        // For now, return a dummy module
        
        let name = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();
        
        Ok(Module {
            name,
            path: path.to_path_buf(),
            declarations: Vec::new(),
            exports: HashMap::new(),
            dependencies: Vec::new(),
            metadata: ModuleMetadata {
                version: "1.0.0".to_string(),
                author: "Unknown".to_string(),
                description: "".to_string(),
                dependencies: HashMap::new(),
                features: Vec::new(),
            },
        })
    }
    
    /// Import items from a module
    pub fn import(&mut self, module_name: &str, items: &[String]) -> Result<HashMap<String, ExportItem>, String> {
        let module = self.load_module(module_name)?;
        let mut imported_items = HashMap::new();
        
        for item_name in items {
            if let Some(export_item) = module.exports.get(item_name) {
                imported_items.insert(item_name.clone(), export_item.clone());
            } else {
                return Err(format!("Item '{}' not found in module '{}'", item_name, module_name));
            }
        }
        
        Ok(imported_items)
    }
    
    /// Import all items from a module
    pub fn import_all(&mut self, module_name: &str) -> Result<HashMap<String, ExportItem>, String> {
        let module = self.load_module(module_name)?;
        Ok(module.exports.clone())
    }
    
    /// Export an item from the current module
    pub fn export(&mut self, module_name: &str, name: String, item: ExportItem) {
        if let Some(module) = self.modules.get_mut(module_name) {
            module.exports.insert(name, item);
        }
    }
    
    /// Get module dependencies
    pub fn get_dependencies(&self, module_name: &str) -> Result<Vec<String>, String> {
        if let Some(module) = self.modules.get(module_name) {
            Ok(module.dependencies.clone())
        } else {
            Err(format!("Module '{}' not found", module_name))
        }
    }
    
    /// Resolve module dependencies
    pub fn resolve_dependencies(&mut self, module_name: &str) -> Result<Vec<String>, String> {
        let mut resolved = Vec::new();
        let mut visited = std::collections::HashSet::new();
        
        self.resolve_dependencies_recursive(module_name, &mut resolved, &mut visited)?;
        
        Ok(resolved)
    }
    
    /// Recursive dependency resolution
    fn resolve_dependencies_recursive(
        &mut self,
        module_name: &str,
        resolved: &mut Vec<String>,
        visited: &mut std::collections::HashSet<String>,
    ) -> Result<(), String> {
        if visited.contains(module_name) {
            return Err(format!("Circular dependency detected: {}", module_name));
        }
        
        visited.insert(module_name.to_string());
        
        let module = self.load_module(module_name)?;
        
        for dependency in &module.dependencies {
            if !resolved.contains(dependency) {
                self.resolve_dependencies_recursive(dependency, resolved, visited)?;
            }
        }
        
        resolved.push(module_name.to_string());
        visited.remove(module_name);
        
        Ok(())
    }
    
    /// Get module metadata
    pub fn get_module_metadata(&self, module_name: &str) -> Result<&ModuleMetadata, String> {
        if let Some(module) = self.modules.get(module_name) {
            Ok(&module.metadata)
        } else {
            Err(format!("Module '{}' not found", module_name))
        }
    }
    
    /// List all loaded modules
    pub fn list_modules(&self) -> Vec<String> {
        self.modules.keys().cloned().collect()
    }
    
    /// Clear module cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
    
    /// Get module search paths
    pub fn get_search_paths(&self) -> &[PathBuf] {
        &self.search_paths
    }
    
    /// Set module search paths
    pub fn set_search_paths(&mut self, paths: Vec<PathBuf>) {
        self.search_paths = paths;
    }
    
    /// Check if a module is loaded
    pub fn is_loaded(&self, module_name: &str) -> bool {
        self.modules.contains_key(module_name) || self.cache.contains_key(module_name)
    }
    
    /// Unload a module
    pub fn unload_module(&mut self, module_name: &str) {
        self.modules.remove(module_name);
        self.cache.remove(module_name);
    }
    
    /// Get module statistics
    pub fn get_stats(&self) -> ModuleStats {
        ModuleStats {
            loaded_modules: self.modules.len(),
            cached_modules: self.cache.len(),
            search_paths: self.search_paths.len(),
        }
    }
}

/// Module system statistics
#[derive(Debug, Clone)]
pub struct ModuleStats {
    pub loaded_modules: usize,
    pub cached_modules: usize,
    pub search_paths: usize,
}

impl Default for ModuleSystem {
    fn default() -> Self {
        Self::new()
    }
}


