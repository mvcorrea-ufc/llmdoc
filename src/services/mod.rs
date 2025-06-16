// llmdoc/src/services/mod.rs

// Define submodules for each service
pub mod sprint_service;
pub mod task_service;
pub mod component_service;
pub mod adr_service;
pub mod user_story_service;
pub mod search_service;
pub mod export_service;

pub use sprint_service::SprintService;
pub use task_service::TaskService;
pub use component_service::ComponentService;
pub use adr_service::AdrService;
pub use user_story_service::UserStoryService;
pub use search_service::SearchService;
pub use export_service::ExportService;

pub fn services_init_message() {
    tracing::debug!("Services module initialized (placeholder).");
}