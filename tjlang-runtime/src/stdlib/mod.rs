//! TJLang Standard Library
//!
//! Comprehensive standard library with modules for:
//! - IO operations (print, input, formatting)
//! - File operations (read, write, copy, move, delete, rename)
//! - Math functions (trigonometry, logarithms, statistics, etc.)
//! - String operations (manipulation, regex, encoding)
//! - Collections (arrays, maps, sets, iterators)
//! - Time and date operations
//! - Network operations
//! - System operations
//! - Concurrency utilities
//! - Error handling
//! - Testing framework
//! - And much more!
// Re-export commonly used modules

pub mod collections;
pub mod error;
pub mod file;
pub mod io;
pub mod math;
pub mod string;
pub mod testing;
pub mod time;

/* To implement in the future

pub mod io;
pub mod file;
pub mod math;
pub mod string;
pub mod collections;
pub mod time;
pub mod network;
pub mod system;
pub mod concurrency;
pub mod error;
pub mod testing;


pub mod regex;
pub mod json;
pub mod xml;
pub mod crypto;
pub mod compression;
pub mod database;
pub mod graphics;
pub mod audio;
pub mod video;
pub mod machine_learning;
pub mod web;
pub mod cli;
pub mod logging;
pub mod profiling;
pub mod benchmarking;
pub mod serialization;
pub mod reflection;
pub mod macros;
pub mod async_utils;
pub mod functional;
pub mod algorithms;
pub mod data_structures;
pub mod memory;
pub mod threading;
pub mod process;
pub mod environment;
pub mod path;
pub mod url;
pub mod email;
pub mod http;
pub mod websocket;
pub mod grpc;
pub mod graphql;
pub mod rest;
pub mod soap;
pub mod rpc;
pub mod messaging;
pub mod queue;
pub mod cache;
pub mod session;
pub mod authentication;
pub mod authorization;
pub mod encryption;
pub mod hashing;
pub mod validation;
pub mod parsing;
pub mod formatting;
pub mod localization;
pub mod internationalization;
pub mod configuration;
pub mod monitoring;
pub mod metrics;
pub mod tracing;
pub mod debugging;
pub mod instrumentation;
pub mod observability;
pub mod telemetry;
pub mod analytics;
pub mod reporting;
pub mod visualization;
pub mod charts;
pub mod graphs;
pub mod plotting;
pub mod statistics;
pub mod probability;
pub mod optimization;
pub mod simulation;
pub mod modeling;
pub mod physics;
pub mod chemistry;
pub mod biology;
pub mod finance;
pub mod economics;
pub mod accounting;
pub mod banking;
pub mod trading;
pub mod insurance;
pub mod healthcare;
pub mod education;
pub mod gaming;
pub mod entertainment;
pub mod media;
pub mod social;
pub mod communication;
pub mod collaboration;
pub mod productivity;
pub mod automation;
pub mod robotics;
pub mod iot;
pub mod embedded;
pub mod real_time;
pub mod distributed;
pub mod microservices;
pub mod serverless;
pub mod cloud;
pub mod containers;
pub mod orchestration;
pub mod deployment;
pub mod infrastructure;
pub mod devops;
pub mod ci_cd;
*/

/* testing
pub mod testing_integration;
pub mod testing_unit;
pub mod testing_performance;
pub mod testing_security;
pub mod testing_accessibility;
pub mod testing_compatibility;
pub mod testing_usability;
pub mod testing_load;
pub mod testing_stress;
pub mod testing_chaos;
pub mod testing_mutation;
pub mod testing_property;
pub mod testing_contract;
pub mod testing_api;
pub mod testing_ui;
pub mod testing_mobile;
pub mod testing_web;
pub mod testing_desktop;
pub mod testing_cli;
pub mod testing_embedded;
pub mod testing_game;
pub mod testing_simulation;
pub mod testing_ai;
pub mod testing_ml;
pub mod testing_nlp;
pub mod testing_cv;
pub mod testing_robotics;
pub mod testing_iot;
pub mod testing_blockchain;
pub mod testing_crypto;
pub mod testing_finance;
pub mod testing_healthcare;
pub mod testing_education;
pub mod testing_gaming;
pub mod testing_entertainment;
pub mod testing_social;
pub mod testing_communication;
pub mod testing_collaboration;
pub mod testing_productivity;
pub mod testing_automation;
pub mod testing_analytics;
pub mod testing_reporting;
pub mod testing_visualization;
pub mod testing_monitoring;
pub mod testing_logging;
pub mod testing_metrics;
pub mod testing_tracing;
pub mod testing_debugging;
pub mod testing_profiling;
pub mod testing_benchmarking;
pub mod testing_optimization;
pub mod testing_simulation;
pub mod testing_modeling;
pub mod testing_physics;
pub mod testing_chemistry;
pub mod testing_biology;
pub mod testing_finance;
pub mod testing_economics;
pub mod testing_accounting;
pub mod testing_banking;
pub mod testing_trading;
pub mod testing_insurance;
pub mod testing_healthcare;
pub mod testing_education;
pub mod testing_gaming;
pub mod testing_entertainment;
pub mod testing_media;
pub mod testing_social;
pub mod testing_communication;
pub mod testing_collaboration;
pub mod testing_productivity;
pub mod testing_automation;
pub mod testing_robotics;
pub mod testing_iot;
pub mod testing_embedded;
pub mod testing_real_time;
pub mod testing_distributed;
pub mod testing_microservices;
pub mod testing_serverless;
pub mod testing_cloud;
pub mod testing_containers;
pub mod testing_orchestration;
pub mod testing_deployment;
pub mod testing_infrastructure;
pub mod testing_devops;
pub mod testing_ci_cd;
*/

// Re-export commonly used modules
pub use collections::*;
pub use error::*;
pub use file::*;
pub use io::*;
pub use math::*;
pub use string::*;
pub use testing::*;
pub use time::*;
