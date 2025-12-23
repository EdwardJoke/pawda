use std::env;
use std::fs;
use std::path::Path;
use colored::Colorize;
use git2::Repository;

#[tokio::main]
async fn main() {
    match env::current_dir() {
        Ok(path) => {
            let dir = path.to_string_lossy().to_string();
            let size_bytes = calculate_folder_size(&path);
            let size = format_size(size_bytes);
            let git_branch = get_git_branch(&path);
            let project_type = detect_project_type(&path);
            
            println!("{} | {} | {} | {}", 
                dir.bright_green(),
                size.bright_cyan(),
                git_branch.bright_yellow(),
                project_type.bright_magenta()
            );
        },
        Err(e) => {
            eprintln!("Error getting current directory: {}", e.to_string().bright_red());
            std::process::exit(1);
        }
    }
}

fn calculate_folder_size(path: &Path) -> u64 {
    let mut total_size: u64 = 0;
    
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    // Skip hidden directories like .git to speed up calculation
                    if let Some(file_name) = entry_path.file_name() {
                        if let Some(name_str) = file_name.to_str() {
                            if name_str.starts_with('.') {
                                continue;
                            }
                        }
                    }
                    total_size += calculate_folder_size(&entry_path);
                } else if let Ok(metadata) = entry.metadata() {
                    total_size += metadata.len();
                }
            }
        }
    }
    
    total_size
}

fn format_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    
    if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} B", size)
    }
}

fn get_git_branch(path: &Path) -> String {
    match Repository::open(path) {
        Ok(repo) => {
            if let Ok(head) = repo.head() {
                if let Some(branch_name) = head.shorthand() {
                    return branch_name.to_string();
                }
            }
            "Detached HEAD".to_string()
        },
        Err(_) => "Not a git repo".to_string()
    }
}

fn detect_project_type(path: &Path) -> String {
    let files = [
        ("Cargo.toml", "Rust"),
        ("package.json", "Node.js"),
        ("requirements.txt", "Python"),
        ("go.mod", "Go"),
        ("pom.xml", "Maven"),
        ("build.gradle", "Gradle"),
        ("Makefile", "Make"),
        ("Dockerfile", "Docker"),
        ("tsconfig.json", "TypeScript"),
        ("pyproject.toml", "Python"),
    ];
    
    for (file, ptype) in &files {
        if path.join(file).exists() {
            return ptype.to_string();
        }
    }
    
    "Unknown".to_string()
}
