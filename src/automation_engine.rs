use crate::config::Config;
use crate::error::AppError;
use crate::screenshot::ScreenshotCapture;
use crate::key_sender::KeySender;
use crate::interrupt_monitor::InterruptMonitor;
use std::thread;
use std::time::Duration;

pub struct AutomationEngine {
    config: Config,
}

impl AutomationEngine {
    pub fn new(config: Config) -> Self {
        AutomationEngine { config }
    }

    pub fn run(&self) -> Result<(), AppError> {
        // 10秒の開始遅延（カウントダウンメッセージ表示）
        println!("10秒後に開始します...");
        for i in (1..=10).rev() {
            println!("{}秒...", i);
            thread::sleep(Duration::from_secs(1));
        }
        
        // [START]メッセージを表示
        println!("[START]");
        
        // 指定回数の反復ループを実装
        for iteration in 1..=self.config.max_iterations {
            // 各反復でexecute_iteration()を呼び出し
            match self.execute_iteration(iteration) {
                Ok(should_continue) => {
                    if !should_continue {
                        // ユーザー中断時は正常終了として扱う
                        println!("ユーザーによって中断されました");
                        break;
                    }
                }
                Err(e) => {
                    // 非クリティカルエラー時に続行
                    println!("エラーが発生しましたが続行します: {}", e);
                    // エラーメッセージを表示して次の反復に進む
                    continue;
                }
            }
        }
        
        // [END]メッセージを表示
        println!("[END]");
        
        Ok(())
    }

    fn execute_iteration(&self, iteration: u32) -> Result<bool, AppError> {
        println!("反復 {}/{} を実行中...", iteration, self.config.max_iterations);
        
        // スクリーンショットをキャプチャ
        let bitmap = ScreenshotCapture::capture()?;
        
        // スクリーンショットを保存
        let filename = format!("{}_{:05}.bmp", self.config.file_prefix, iteration);
        let filepath = self.config.output_directory.join(&filename);
        ScreenshotCapture::save(&bitmap, &filepath)?;
        
        // 1秒待機
        thread::sleep(Duration::from_secs(1));
        
        // 選択された矢印キーを送信
        KeySender::send_arrow_key(self.config.target_key)?;
        
        // 中断チェック（スペースまたはESC）
        if InterruptMonitor::check_interrupt() {
            // 中断された場合はfalseを返す
            return Ok(false);
        }
        
        // 3秒待機（次の反復前）
        thread::sleep(Duration::from_secs(3));
        
        // 正常完了の場合はtrueを返す
        Ok(true)
    }
}
