use deku::{DekuContainerWrite, DekuContainerRead};
use teleport_server::{EncryptType, Message, MessageType};

#[test]
fn message_conversion() {
    let message = Message::new(
        "uid".to_string(),
        "token".to_string(),
        MessageType::Control,
        EncryptType::None,
        vec![],
    );

    let bytes = message.to_bytes().unwrap();
    dbg!(&bytes);
    let ((_, _), new_msg) = Message::from_bytes((&bytes, 0)).unwrap();
    dbg!(&new_msg);

    assert_eq!(message, new_msg)
    // assert_eq!(message.offsets, new_msg.offsets);
    // assert_eq!(message.uid, new_msg.uid);
    // assert_eq!(message.token, new_msg.token);
    // assert_eq!(message.message_type, new_msg.message_type);
    // assert_eq!(message.encryption, new_msg.encryption);
    // assert_eq!(message.content, new_msg.content);
}

#[test]
fn shift_left_with_overflow() {
    let x = 0u8;
    dbg!((x as u16) << 8);
}
