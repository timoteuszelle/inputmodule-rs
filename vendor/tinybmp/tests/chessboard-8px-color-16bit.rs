use embedded_graphics::prelude::*;
use tinybmp::{Bpp, ChannelMasks, Header, RawBmp, RowOrder};

#[test]
fn chessboard_8px_color_16bit() {
    let bmp = RawBmp::from_slice(include_bytes!("./chessboard-8px-color-16bit.bmp"))
        .expect("Failed to parse");

    assert_eq!(
        bmp.header(),
        &Header {
            file_size: 266,
            image_data_start: 138,
            bpp: Bpp::Bits16,
            image_size: Size::new(8, 8),
            image_data_len: 128,
            channel_masks: Some(ChannelMasks::RGB565),
            row_order: RowOrder::BottomUp,
        }
    );

    assert!(
        bmp.color_table().is_none(),
        "there should be no color table for this image"
    );

    assert_eq!(bmp.image_data().len(), 266 - 138);
}

#[test]
fn chessboard_8px_color_16bit_iter() {
    let bmp = RawBmp::from_slice(include_bytes!("./chessboard-8px-color-16bit.bmp"))
        .expect("Failed to parse");

    let pixels: Vec<u32> = bmp.pixels().map(|pixel| pixel.color).collect();

    // 8px x 8px image. Check that iterator returns all pixels in it
    assert_eq!(pixels.len(), 8 * 8);

    let expected = vec![
        0xffff, 0xffff, 0x0000, 0x0000, 0xffff, 0xffff, 0x0000, 0x0000, //
        0xffff, 0xffff, 0x0000, 0x0000, 0xffff, 0xffff, 0x0000, 0x0000, //
        0x0000, 0x0000, 0xf800, 0xf800, 0x0000, 0x0000, 0x07e0, 0x07e0, //
        0x0000, 0x0000, 0xf800, 0xf800, 0x0000, 0x0000, 0x07e0, 0x07e0, //
        0xffff, 0xffff, 0x0000, 0x0000, 0x001f, 0x001f, 0x0000, 0x0000, //
        0xffff, 0xffff, 0x0000, 0x0000, 0x001f, 0x001f, 0x0000, 0x0000, //
        0x0000, 0x0000, 0xffff, 0xffff, 0x0000, 0x0000, 0xffff, 0xffff, //
        0x0000, 0x0000, 0xffff, 0xffff, 0x0000, 0x0000, 0xffff, 0xffff, //
    ];

    assert_eq!(pixels, expected);
}
