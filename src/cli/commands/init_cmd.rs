// llmdoc/src/cli/commands/init_cmd.rs

use crate::app_config::Config;
use crate::core::errors::Result; // Import the errors module and its Result type
use std::path::PathBuf;
use colored::*;
use dialoguer::Confirm;

pub async fn init(config: &Config, db_path: &PathBuf, force: bool) -> Result<()> {
    let llmdocs_dir = db_path.parent().unwrap();
    
    // Check if already initialized
    if llmdocs_dir.exists() && !force {
        if !Confirm::new()
            .with_prompt("LLMDocs directory already exists. Overwrite?")
            .default(false)
            .interact()? 
        {
            println!("Initialization cancelled");
            return Ok(());
        }
    }
    
    // Create directory structure
    let schemas_dir = llmdocs_dir.join("schemas");
    let backups_dir = llmdocs_dir.join("backups");
    let logs_dir = llmdocs_dir.join("logs");
    let templates_dir = llmdocs_dir.join("templates");

    let dirs = vec![
        llmdocs_dir,
        &schemas_dir,
        &backups_dir,
        &logs_dir,
        &templates_dir,
    ];
    
    for dir in dirs {
        std::fs::create_dir_all(dir)?;
    }
    
    // Copy schema files
    let schemas = vec![
        ("task.json", include_str!("../../../schemas/task.json")),
        ("sprint.json", include_str!("../../../schemas/sprint.json")),
        ("component.json", include_str!("../../../schemas/component.json")),
        ("adr.json", include_str!("../../../schemas/adr.json")),
        ("user_story.json", include_str!("../../../schemas/user_story.json")),
    ];
    
    for (name, content) in schemas {
        std::fs::write(llmdocs_dir.join("schemas").join(name), content)?;
    }
    
    // Create default config
    config.save(&llmdocs_dir.join("config.toml"))?;
    
    // Initialize database
    
    // Create LLMDOCS_GUIDE.md
    std::fs::write(
        llmdocs_dir.join("LLMDOCS_GUIDE.md"),
        include_str!("../../../templates/LLMDOCS_GUIDE.md")
    )?;
    
    println!("{}", "✅ LLMDocs project initialized successfully!".green());
    println!("\nNext steps:");
    println!("  1. Start the embedding server (if using HTTP provider):");
    println!("     {}", "python embedding_server.py".cyan());
    println!("  2. Add your first task:");
    println!("     {}", "llmdocs task add".cyan());
    println!("  3. View the guide:");
    println!("     {}", "cat LLMDOCS_GUIDE.md".cyan());
    
    Ok(())
}