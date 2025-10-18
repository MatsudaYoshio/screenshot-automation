mod error;
mod config;
mod screenshot;
mod key_sender;
mod interrupt_monitor;
mod automation_engine;

use config::Config;
use automation_engine::AutomationEngine;

fn main() {
    println!("Screenshot Automation App");
    println!();
    
    // Config::from_user_input()で設定を収集
    let config = match Config::from_user_input() {
        Ok(cfg) => cfg,
        Err(e) => {
            // クリティカルエラー時のエラーメッセージ表示と終了
            eprintln!("設定エラー: {}", e);
            eprintln!("アプリケーションを終了します");
            std::process::exit(1);
        }
    };
    
    println!();
    println!("設定が完了しました:");
    println!("  対象キー: {:?}", config.target_key);
    println!("  出力ディレクトリ: {}", config.output_directory.display());
    println!("  ファイルプレフィックス: {}", config.file_prefix);
    println!("  最大反復回数: {}", config.max_iterations);
    println!();
    
    // AutomationEngine::new()でエンジンを初期化
    let engine = AutomationEngine::new(config);
    
    // AutomationEngine::run()で自動化を実行
    if let Err(e) = engine.run() {
        // クリティカルエラー時のエラーメッセージ表示と終了
        eprintln!("実行エラー: {}", e);
        eprintln!("アプリケーションを終了します");
        std::process::exit(1);
    }
    
    println!();
    println!("正常に終了しました");
}
