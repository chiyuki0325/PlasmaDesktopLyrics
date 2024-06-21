use std::time::Duration;
use tokio::sync::broadcast;
use tokio::time::sleep;
use crate::ChannelMessage;
use crate::config::SharedConfig;
use crate::player_stream::MyPlayerStream;
use async_std::stream::StreamExt;
use std::sync::{Arc, Mutex};

struct MprisInfo {
    url: String,
}

type MprisCache = Arc<Mutex<MprisInfo>>;

pub async fn mpris_loop(
    tx: broadcast::Sender<ChannelMessage>,
    config: SharedConfig,
) {
    let mut cache = Arc::new(Mutex::new((MprisInfo {
        url: String::new(),
    })));

    let mut player_stream = MyPlayerStream::new(500);
    while let Some(player) = player_stream.next().await {
        // 得到播放器，进入循环
        if config.read().unwrap().verbose {
            println!("New player connected: {:?}", player);
        }
        let player_name = String::from(player.bus_name());
        let player_name = player_name.strip_prefix("org.mpris.MediaPlayer2.").unwrap();
        if config.read().unwrap().disabled_players.contains(&player_name.to_string()) {
            if config.read().unwrap().verbose {
                println!("Player {} detected, but disabled in the config.", player_name);
            }
            continue;
        }

        loop {
            // 主循环，此时 player 已被移动到此大括号中
            match player.get_metadata() {
                Ok(metadata) => {
                    // 判断歌曲是否更改
                    let url = metadata.url().unwrap_or_default();
                    let mut cache = cache.lock().unwrap();

                    if cache.url != url {
                        // 歌曲更改
                        if config.read().unwrap().verbose {
                            println!("New song detected: {}", url);
                        }
                        cache.url = url.to_string();

                        tx.send(ChannelMessage::UpdateMusicInfo(
                            metadata.title().unwrap_or_default().to_string(),
                            metadata.artists().unwrap_or_default()[0].to_string()
                        )).unwrap();
                    }
                }
                Err(e) => {
                    // 播放器已经关闭。
                    if config.read().unwrap().verbose {
                        println!("Player closed, exiting loop...");
                    }
                    tx.send(ChannelMessage::UpdateMusicInfo("".to_string(), "".to_string())).unwrap();
                    // 跳出 loop 块，继续等待下一个播放器
                    break;
                }
            }
            sleep(Duration::from_millis(50)).await;
        }
    }

    todo!("在退出播放器后重启无法检测到")
}
