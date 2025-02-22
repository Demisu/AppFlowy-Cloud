use std::time::SystemTime;

use client_api::ws::{ConnectState, WSClient, WSClientConfig};
use client_api_test_util::generate_unique_registered_user_client;

#[tokio::test]
async fn realtime_connect_test() {
  let (c, _user) = generate_unique_registered_user_client().await;
  let ws_client = WSClient::new(WSClientConfig::default(), c.clone());
  let mut state = ws_client.subscribe_connect_state();
  let device_id = "fake_device_id";
  loop {
    tokio::select! {
        _ = ws_client.connect(c.ws_url(device_id).await.unwrap(), device_id) => {},
       value = state.recv() => {
        let new_state = value.unwrap();
        if new_state == ConnectState::Connected {
          break;
        }
      },
    }
  }
}

#[tokio::test]
async fn realtime_connect_after_token_exp_test() {
  let (c, _user) = generate_unique_registered_user_client().await;

  // Set the token to be expired
  c.token().write().as_mut().unwrap().expires_at = SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .unwrap()
    .as_secs() as i64;

  let ws_client = WSClient::new(WSClientConfig::default(), c.clone());
  let mut state = ws_client.subscribe_connect_state();
  let device_id = "fake_device_id";
  loop {
    tokio::select! {
        _ = ws_client.connect(c.ws_url(device_id).await.unwrap(), device_id) => {},
       value = state.recv() => {
        let new_state = value.unwrap();
        if new_state == ConnectState::Connected {
          break;
        }
      },
    }
  }
}

#[tokio::test]
async fn realtime_disconnect_test() {
  let (c, _user) = generate_unique_registered_user_client().await;
  let ws_client = WSClient::new(WSClientConfig::default(), c.clone());
  let device_id = "fake_device_id";
  ws_client
    .connect(c.ws_url(device_id).await.unwrap(), device_id)
    .await
    .unwrap();

  let mut state = ws_client.subscribe_connect_state();
  loop {
    tokio::select! {
        _ = ws_client.disconnect() => {},
       value = state.recv() => {
        let new_state = value.unwrap();
        if new_state == ConnectState::Closed {
          break;
        }
      },
    }
  }
}
