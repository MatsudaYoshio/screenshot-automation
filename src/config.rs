use crate::error::ConfigError;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

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
            fs::create_dir_all(path).map_err(ConfigError::DirectoryCreationFailed)?;
            println!("ディレクトリを作成しました: {}", path.display());
        }
        Ok(())
    }

    #[cfg(test)]
    pub fn new_for_test(
        target_key: ArrowKey,
        output_directory: PathBuf,
        file_prefix: String,
        max_iterations: u32,
    ) -> Self {
        Config {
            target_key,
            output_directory,
            file_prefix,
            max_iterations,
        }
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

    pub fn to_vk_code(self) -> u16 {
        match self {
            ArrowKey::Left => 0x25,  // VK_LEFT
            ArrowKey::Right => 0x27, // VK_RIGHT
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_arrow_key_from_str_valid_left() {
        let result = ArrowKey::from_str("left");
        assert!(result.is_ok());
        matches!(result.unwrap(), ArrowKey::Left);
    }

    #[test]
    fn test_arrow_key_from_str_valid_right() {
        let result = ArrowKey::from_str("right");
        assert!(result.is_ok());
        matches!(result.unwrap(), ArrowKey::Right);
    }

    #[test]
    fn test_arrow_key_from_str_case_insensitive() {
        assert!(matches!(
            ArrowKey::from_str("LEFT").unwrap(),
            ArrowKey::Left
        ));
        assert!(matches!(
            ArrowKey::from_str("Right").unwrap(),
            ArrowKey::Right
        ));
        assert!(matches!(
            ArrowKey::from_str("LeFt").unwrap(),
            ArrowKey::Left
        ));
    }

    #[test]
    fn test_arrow_key_from_str_invalid() {
        assert!(ArrowKey::from_str("up").is_err());
        assert!(ArrowKey::from_str("down").is_err());
        assert!(ArrowKey::from_str("invalid").is_err());
        assert!(ArrowKey::from_str("").is_err());
    }

    #[test]
    fn test_arrow_key_to_vk_code_left() {
        let key = ArrowKey::Left;
        assert_eq!(key.to_vk_code(), 0x25);
    }

    #[test]
    fn test_arrow_key_to_vk_code_right() {
        let key = ArrowKey::Right;
        assert_eq!(key.to_vk_code(), 0x27);
    }

    // Config validation tests
    #[test]
    fn test_config_valid_creation() {
        let config = Config::new_for_test(
            ArrowKey::Left,
            PathBuf::from("test_output"),
            "screenshot".to_string(),
            10,
        );

        assert!(matches!(config.target_key, ArrowKey::Left));
        assert_eq!(config.output_directory, PathBuf::from("test_output"));
        assert_eq!(config.file_prefix, "screenshot");
        assert_eq!(config.max_iterations, 10);
    }

    #[test]
    fn test_config_valid_with_right_key() {
        let config = Config::new_for_test(
            ArrowKey::Right,
            PathBuf::from("output"),
            "img".to_string(),
            5,
        );

        assert!(matches!(config.target_key, ArrowKey::Right));
        assert_eq!(config.max_iterations, 5);
    }

    #[test]
    fn test_config_valid_with_large_iteration_count() {
        let config = Config::new_for_test(
            ArrowKey::Left,
            PathBuf::from("output"),
            "test".to_string(),
            1000,
        );

        assert_eq!(config.max_iterations, 1000);
    }

    #[test]
    fn test_ensure_directory_exists_creates_new_directory() {
        let test_dir = PathBuf::from("test_temp_dir_12345");

        // Clean up if exists from previous test
        if test_dir.exists() {
            fs::remove_dir_all(&test_dir).ok();
        }

        // Test directory creation
        let result = Config::ensure_directory_exists(&test_dir);
        assert!(result.is_ok());
        assert!(test_dir.exists());

        // Clean up
        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_ensure_directory_exists_with_existing_directory() {
        let test_dir = PathBuf::from("test_existing_dir_67890");

        // Create directory first
        fs::create_dir_all(&test_dir).ok();

        // Test with existing directory
        let result = Config::ensure_directory_exists(&test_dir);
        assert!(result.is_ok());
        assert!(test_dir.exists());

        // Clean up
        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_ensure_directory_exists_with_nested_path() {
        let test_dir = PathBuf::from("test_nested/sub1/sub2");

        // Clean up if exists
        if PathBuf::from("test_nested").exists() {
            fs::remove_dir_all("test_nested").ok();
        }

        // Test nested directory creation
        let result = Config::ensure_directory_exists(&test_dir);
        assert!(result.is_ok());
        assert!(test_dir.exists());

        // Clean up
        fs::remove_dir_all("test_nested").ok();
    }

    #[test]
    fn test_arrow_key_invalid_returns_error() {
        let result = ArrowKey::from_str("invalid_key");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ConfigError::InvalidKey));
    }

    #[test]
    fn test_arrow_key_empty_string_returns_error() {
        let result = ArrowKey::from_str("");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ConfigError::InvalidKey));
    }
}
