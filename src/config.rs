use crate::error::ConfigError;
use std::path::PathBuf;
use std::io::{self, Write};
use std::fs;

pub struct Config {
    pub target_key: ArrowKey,
    pub output_directory: PathBuf,
    pub file_prefix: String,
    pub max_iterations: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum ArrowKey {
    Left,
    Right,
}

impl Config {
    pub fn from_user_input() -> Result<Self, ConfigError> {
        // Prompt for arrow key selection
        let target_key = loop {
            print!("矢印キーを選択してください (left/right): ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            
            match ArrowKey::from_str(input) {
                Ok(key) => break key,
                Err(_) => {
                    println!("エラー: 'left' または 'right' を入力してください");
                    continue;
                }
            }
        };
        
        // Prompt for output directory
        print!("出力ディレクトリのパスを入力してください: ");
        io::stdout().flush().unwrap();
        
        let mut output_directory = String::new();
        io::stdin().read_line(&mut output_directory).unwrap();
        let output_directory = PathBuf::from(output_directory.trim());
        
        // Create directory if it doesn't exist
        Self::ensure_directory_exists(&output_directory)?;
        
        // Prompt for file prefix
        print!("ファイル名のプレフィックスを入力してください: ");
        io::stdout().flush().unwrap();
        
        let mut file_prefix = String::new();
        io::stdin().read_line(&mut file_prefix).unwrap();
        let file_prefix = file_prefix.trim().to_string();
        
        // Prompt for max iterations
        let max_iterations = loop {
            print!("最大反復回数を入力してください (正の整数): ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            
            match input.parse::<u32>() {
                Ok(n) if n > 0 => break n,
                _ => {
                    println!("エラー: 正の整数を入力してください");
                    continue;
                }
            }
        };
        
        Ok(Config {
            target_key,
            output_directory,
            file_prefix,
            max_iterations,
        })
    }
    
    fn ensure_directory_exists(path: &PathBuf) -> Result<(), ConfigError> {
        if !path.exists() {
            fs::create_dir_all(path)
                .map_err(|e| ConfigError::DirectoryCreationFailed(e))?;
            println!("ディレクトリを作成しました: {}", path.display());
        }
        Ok(())
    }
}

impl ArrowKey {
    pub fn from_str(s: &str) -> Result<Self, ConfigError> {
        match s.to_lowercase().as_str() {
            "left" => Ok(ArrowKey::Left),
            "right" => Ok(ArrowKey::Right),
            _ => Err(ConfigError::InvalidKey),
        }
    }

    pub fn to_vk_code(&self) -> u16 {
        match self {
            ArrowKey::Left => 0x25,  // VK_LEFT
            ArrowKey::Right => 0x27, // VK_RIGHT
        }
    }
}
