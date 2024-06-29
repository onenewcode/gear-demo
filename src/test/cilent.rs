use gclient::{EventProcessor, GearApi, Result};

const WASM_PATH: &str = "./target/wasm32-unknown-unknown/release/counter.opt.wasm";

#[tokio::test]
#[ignore]
async fn test_example() -> Result<()> {
    // Create API instance
    let api = GearApi::dev().await?;
    println!("API created");
    
    // Subscribe to events
    let mut listener = api.subscribe().await?;
    println!("Listener created");

    // Check that blocks are still running
    assert!(listener.blocks_running().await?);
    println!("Listener blocks are running");

    // Calculate gas amount needed for initialization
    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(WASM_PATH)?,
            vec![],
            0,
            true,
        )
        .await?;
    println!("Gas info calculated");

    // Upload and init the program
    let (message_id, program_id, _hash) = api
        .upload_program_bytes_by_path(
            WASM_PATH,
            gclient::now_micros().to_le_bytes(),
            vec![],
            gas_info.min_limit,
            0,
        )
        .await?;
    println!("Program uploaded");
    assert!(listener.message_processed(message_id).await?.succeed());

    let payload = b"inc".to_vec();

    // Calculate gas amount needed for handling the message
    let gas_info = api
        .calculate_handle_gas(None, program_id, payload.clone(), 0, true)
        .await?;
    println!("Gas info calculated2");

    // Send the inc message
    let (message_id, _hash) = api
        .send_message_bytes(program_id, payload, gas_info.min_limit, 0)
        .await?;
    println!("Message sent");
    assert!(listener.message_processed(message_id).await?.succeed());

    // Send the get message
    let get_payload = b"get".to_vec();
    let gas_info = api
        .calculate_handle_gas(None, program_id, get_payload.clone(), 0, true)
        .await?;
    let (message_id, _hash) = api
        .send_message_bytes(program_id, get_payload, gas_info.min_limit, 0)
        .await?;

    // Listen and verify the returned message
    if let (message_id, result, value) = listener.reply_bytes_on(message_id).await? {
        if let Ok(data) = result {
            println!("Data: {:?}", data);
            assert_eq!(data, b"1");
        } else if let Err(error) = result {
            println!("Error: {:?}", error);
        }
    }

    Ok(())
}