// Derived based on data from https://github.com/bbcmicrobit/micropython/blob/master/source/microbit/microbitconstimage.cpp
// Which require this licence.

/*
 * This file is part of the MicroPython project, http://micropython.org/
 *
 * The MIT License (MIT)
 *
 * Copyright (c) 2015 Damien P. George
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 */

#![no_std]

pub mod img {
    #![allow(non_upper_case_globals, unused)]

    pub fn image_to_preformated_vector(image: u32) -> [[u8; 9]; 3] {
        let image = !image;

        [
            [0, 1, 2, 3, 4, 5, 6, 7, 8],
            [9, 10, 11, 12, 13, 14, 15, 16, 17],
            [18, 19, 20, 21, 22, 23, 24, 25, 26],
        ]
        .map(|v| v.map(|i| 1_u8 & (image >> i) as u8))
    }



    pub fn image_to_5x5(image:u32) -> [[u8;5];5] {
    const LED_LAYOUT: [[(usize, usize); 5]; 5] = [
        [(0, 0), (1, 3), (0, 1), (1, 4), (0, 2)],
        [(2, 3), (2, 4), (2, 5), (2, 6), (2, 7)],
        [(1, 1), (0, 8), (1, 2), (2, 8), (1, 0)],
        [(0, 7), (0, 6), (0, 5), (0, 4), (0, 3)],
        [(2, 2), (1, 6), (2, 0), (1, 5), (2, 1)],
    ]   ;

        let ans=LED_LAYOUT.map(|v|v.map(|(row,col)| (row*9+col) as u8));
        ans
    }

    #[test]
    fn test_array() {

        let x = image_to_5x5(0);
        assert_eq!(x, [[0;5];5]);
    }
    ///```text
    /// An image in the shape of:
    ///    ***
    ///   *   *
    ///     **
    ///        
    ///     *  
    ///
    ///```
    pub const question_mark: u32 = 0x01dbc7fd;
    ///```text
    /// An image in the shape of:
    ///   *   *
    ///    * *
    ///     *  
    ///    * *
    ///   *   *
    ///
    ///```
    pub const x_big: u32 = 0x06a7f7aa;
    ///```text
    /// An image in the shape of:
    ///    ***
    ///   *   *
    ///   *   *
    ///   *   *
    ///    ***
    ///
    ///```
    pub const circle: u32 = 0x05db0975;
    ///```text
    /// An image in the shape of:
    ///   *****
    ///        
    ///        
    ///        
    ///   *****
    ///
    ///```
    pub const hbars_top_botom: u32 = 0x07e30ff8;
    ///```text
    /// An image in the shape of:
    ///        
    ///   *****
    ///        
    ///   *****
    ///        
    ///
    ///```
    pub const hbars_center: u32 = 0x041fff07;
    ///```text
    /// An image in the shape of:
    ///   *****
    ///   *****
    ///   *****
    ///   *****
    ///   *****
    ///
    ///```
    pub const full_square: u32 = 0x00030000;
    ///```text
    /// An image in the shape of:
    ///        
    ///        
    ///     *  
    ///        
    ///        
    ///
    ///```
    pub const dot33: u32 = 0x07fff7ff;
    ///```text
    /// An image in the shape of:
    ///        
    ///        
    ///    ***
    ///        
    ///        
    ///
    ///```
    pub const minus: u32 = 0x03fff6ff;
    ///```text
    /// An image in the shape of:
    ///        
    ///        
    ///   *****
    ///        
    ///        
    ///
    ///```
    pub const longminus: u32 = 0x03fff0ff;
    ///```text
    /// An image in the shape of:
    ///    * *
    ///   *****
    ///   *****
    ///    ***
    ///     *  
    ///
    ///```
    pub const heart_image: u32 = 0x001bc08f;
    ///```text
    /// An image in the shape of:
    ///        
    ///    * *
    ///    ***
    ///     *  
    ///        
    ///
    ///```
    pub const heart_small_image: u32 = 0x02bff6df;
    ///```text
    /// An image in the shape of:
    ///        
    ///    * *
    ///        
    ///   *   *
    ///    ***
    ///
    ///```
    pub const happy_image: u32 = 0x06bb3f77;
    ///```text
    /// An image in the shape of:
    ///        
    ///        
    ///        
    ///   *   *
    ///    ***
    ///
    ///```
    pub const smile_image: u32 = 0x07fb3f77;
    ///```text
    /// An image in the shape of:
    ///        
    ///    * *
    ///        
    ///    ***
    ///   *   *
    ///
    ///```
    pub const sad_image: u32 = 0x06a7ff8f;
    ///```text
    /// An image in the shape of:
    ///        
    ///    * *
    ///        
    ///    * *
    ///   * * *
    ///
    ///```
    pub const confused_image: u32 = 0x06a3ffaf;
    ///```text
    /// An image in the shape of:
    ///   *   *
    ///    * *
    ///        
    ///   *****
    ///   * * *
    ///
    ///```
    pub const angry_image: u32 = 0x06a3ff02;
    ///```text
    /// An image in the shape of:
    ///        
    ///   ** **
    ///        
    ///    ***
    ///        
    ///
    ///```
    pub const asleep_image: u32 = 0x049fff8f;
    ///```text
    /// An image in the shape of:
    ///    * *
    ///        
    ///     *  
    ///    * *
    ///     *  
    ///
    ///```
    pub const surprised_image: u32 = 0x07fbc7af;
    ///```text
    /// An image in the shape of:
    ///   *   *
    ///        
    ///   *****
    ///     * *
    ///     ***
    ///
    ///```
    pub const silly_image: u32 = 0x03f3b0d2;
    ///```text
    /// An image in the shape of:
    ///   *****
    ///   ** **
    ///        
    ///    * *
    ///    ***
    ///
    ///```
    pub const fabulous_image: u32 = 0x049b0fa8;
    ///```text
    /// An image in the shape of:
    ///    * *
    ///        
    ///      *
    ///     *  
    ///    *   
    ///
    ///```
    pub const meh_image: u32 = 0x03ff4fdf;
    ///```text
    /// An image in the shape of:
    ///        
    ///       *
    ///      *
    ///   * *  
    ///    *   
    ///
    ///```
    pub const yes_image: u32 = 0x01ff7f5f;
    ///```text
    /// An image in the shape of:
    ///   *   *
    ///    * *
    ///     *  
    ///    * *
    ///   *   *
    ///
    ///```
    pub const no_image: u32 = 0x06a7f7aa;
    ///```text
    /// An image in the shape of:
    ///     *  
    ///     *  
    ///     *  
    ///        
    ///        
    ///
    ///```
    pub const clock12_image: u32 = 0x077ff7fd;
    ///```text
    /// An image in the shape of:
    ///      *
    ///      *
    ///     *  
    ///        
    ///        
    ///
    ///```
    pub const clock1_image: u32 = 0x06ffd7ff;
    ///```text
    /// An image in the shape of:
    ///        
    ///      **
    ///     *  
    ///        
    ///        
    ///
    ///```
    pub const clock2_image: u32 = 0x04fff7ff;
    ///```text
    /// An image in the shape of:
    ///        
    ///        
    ///     ***
    ///        
    ///        
    ///
    ///```
    pub const clock3_image: u32 = 0x03fff5ff;
    ///```text
    /// An image in the shape of:
    ///        
    ///        
    ///     *  
    ///      **
    ///        
    ///
    ///```
    pub const clock4_image: u32 = 0x07fff7e7;
    ///```text
    /// An image in the shape of:
    ///        
    ///        
    ///     *  
    ///      *
    ///      *
    ///
    ///```
    pub const clock5_image: u32 = 0x07ffb7ef;
    ///```text
    /// An image in the shape of:
    ///        
    ///        
    ///     *  
    ///     *  
    ///     *  
    ///
    ///```
    pub const clock6_image: u32 = 0x07fbf7df;
    ///```text
    /// An image in the shape of:
    ///        
    ///        
    ///     *  
    ///    *   
    ///    *   
    ///
    ///```
    pub const clock7_image: u32 = 0x07ff77bf;
    ///```text
    /// An image in the shape of:
    ///        
    ///        
    ///     *  
    ///   **   
    ///        
    ///
    ///```
    pub const clock8_image: u32 = 0x07fff73f;
    ///```text
    /// An image in the shape of:
    ///        
    ///        
    ///   ***  
    ///        
    ///        
    ///
    ///```
    pub const clock9_image: u32 = 0x07fff2ff;
    ///```text
    /// An image in the shape of:
    ///        
    ///   **   
    ///     *  
    ///        
    ///        
    ///
    ///```
    pub const clock10_image: u32 = 0x079ff7ff;
    ///```text
    /// An image in the shape of:
    ///    *   
    ///    *   
    ///     *  
    ///        
    ///        
    ///
    ///```
    pub const clock11_image: u32 = 0x07bfe7ff;
    ///```text
    /// An image in the shape of:
    ///     *  
    ///    ***
    ///   * * *
    ///     *  
    ///     *  
    ///
    ///```
    pub const arrow_n_image: u32 = 0x063bf1dd;
    ///```text
    /// An image in the shape of:
    ///     ***
    ///      **
    ///     * *
    ///    *   
    ///   *    
    ///
    ///```
    pub const arrow_ne_image: u32 = 0x04efd5b9;
    ///```text
    /// An image in the shape of:
    ///     *  
    ///      *
    ///   *****
    ///      *
    ///     *  
    ///
    ///```
    pub const arrow_e_image: u32 = 0x02fbf0ed;
    ///```text
    /// An image in the shape of:
    ///   *    
    ///    *   
    ///     * *
    ///      **
    ///     ***
    ///
    ///```
    pub const arrow_se_image: u32 = 0x07b3b5e6;
    ///```text
    /// An image in the shape of:
    ///     *  
    ///     *  
    ///   * * *
    ///    ***
    ///     *  
    ///
    ///```
    pub const arrow_s_image: u32 = 0x077bf18d;
    ///```text
    /// An image in the shape of:
    ///       *
    ///      *
    ///   * *  
    ///   **   
    ///   ***  
    ///
    ///```
    pub const arrow_sw_image: u32 = 0x06eb733b;
    ///```text
    /// An image in the shape of:
    ///     *  
    ///    *   
    ///   *****
    ///    *   
    ///     *  
    ///
    ///```
    pub const arrow_w_image: u32 = 0x03bbf0bd;
    ///```text
    /// An image in the shape of:
    ///   ***  
    ///   **   
    ///   * *  
    ///      *
    ///       *
    ///
    ///```
    pub const arrow_nw_image: u32 = 0x0797e3ec;
    ///```text
    /// An image in the shape of:
    ///        
    ///     *  
    ///    * *
    ///   *****
    ///        
    ///
    ///```
    pub const letriangle: u32 = 0x037ffe07;
    ///```text
    /// An image in the shape of:
    ///   *    
    ///   **   
    ///   * *  
    ///   *  *
    ///   *****
    ///
    ///```
    pub const letriangle_left: u32 = 0x0783336e;
    ///```text
    /// An image in the shape of:
    ///    * *
    ///   * * *
    ///    * *
    ///   * * *
    ///    * *
    ///
    ///```
    pub const chessboard_image: u32 = 0x015f0e57;
    ///```text
    /// An image in the shape of:
    ///     *  
    ///    * *
    ///   *   *
    ///    * *
    ///     *  
    ///
    ///```
    pub const diamond_image: u32 = 0x06bbf9ad;
    ///```text
    /// An image in the shape of:
    ///        
    ///     *  
    ///    * *
    ///     *  
    ///        
    ///
    ///```
    pub const diamond_small_image: u32 = 0x037ffedf;
    ///```text
    /// An image in the shape of:
    ///   *****
    ///   *   *
    ///   *   *
    ///   *   *
    ///   *****
    ///
    ///```
    pub const square_image: u32 = 0x05c30970;
    ///```text
    /// An image in the shape of:
    ///        
    ///    ***
    ///    * *
    ///    ***
    ///        
    ///
    ///```
    pub const square_small_image: u32 = 0x023ffe8f;
    ///```text
    /// An image in the shape of:
    ///   * *  
    ///   * *  
    ///   ****
    ///   ** *
    ///   ****
    ///
    ///```
    pub const image_rabbit: u32 = 0x034b322c;
    ///```text
    /// An image in the shape of:
    ///   *   *
    ///   *   *
    ///   *****
    ///    ***
    ///     *  
    ///
    ///```
    pub const image_cow: u32 = 0x01dbf08a;
    ///```text
    /// An image in the shape of:
    ///     *  
    ///     *  
    ///     *  
    ///   ***  
    ///   ***  
    ///
    ///```
    pub const music_crotchet_image: u32 = 0x076b771d;
    ///```text
    /// An image in the shape of:
    ///     *  
    ///     **
    ///     * *
    ///   ***  
    ///   ***  
    ///
    ///```
    pub const music_quaver_image: u32 = 0x066b751d;
    ///```text
    /// An image in the shape of:
    ///    ****
    ///    *  *
    ///    *  *
    ///   ** **
    ///   ** **
    ///
    ///```
    pub const music_quavers_image: u32 = 0x05a70c21;
    ///```text
    /// An image in the shape of:
    ///   * * *
    ///   * * *
    ///   *****
    ///     *  
    ///     *  
    ///
    ///```
    pub const pitchfork_image: u32 = 0x015bf0d8;
    ///```text
    /// An image in the shape of:
    ///     *  
    ///    ***
    ///     *  
    ///    ***
    ///   *****
    ///
    ///```
    pub const xmas_image: u32 = 0x0623378d;
    ///```text
    /// An image in the shape of:
    ///    ****
    ///   ** *
    ///   ***  
    ///   ****
    ///    ****
    ///
    ///```
    pub const pacman_image: u32 = 0x06930209;
    ///```text
    /// An image in the shape of:
    ///     *  
    ///    ***
    ///   ** **
    ///    ***
    ///     *  
    ///
    ///```
    pub const letarget: u32 = 0x023bf88d;
    ///```text
    /// An image in the shape of:
    ///   ** **
    ///   *****
    ///    ***
    ///    ***
    ///    ***
    ///
    ///```
    pub const letshirt: u32 = 0x001b068a;
    ///```text
    /// An image in the shape of:
    ///      **
    ///      **
    ///   *****
    ///   *****
    ///    * *
    ///
    ///```
    pub const rollerskate_image: u32 = 0x00ff1003;
    ///```text
    /// An image in the shape of:
    ///    **  
    ///   ***  
    ///    ****
    ///    ***
    ///        
    ///
    ///```
    pub const duck_image: u32 = 0x031fe48d;
    ///```text
    /// An image in the shape of:
    ///     *  
    ///    ***
    ///   *****
    ///    ***
    ///    * *
    ///
    ///```
    pub const house_image: u32 = 0x023f308d;
    ///```text
    /// An image in the shape of:
    ///        
    ///    ***
    ///   *****
    ///    * *
    ///        
    ///
    ///```
    pub const letortoise: u32 = 0x023ff0af;
    ///```text
    /// An image in the shape of:
    ///   ** **
    ///   *****
    ///     *  
    ///   *****
    ///   ** **
    ///
    ///```
    pub const butterfly_image: u32 = 0x04070702;
    ///```text
    /// An image in the shape of:
    ///     *  
    ///   *****
    ///     *  
    ///    * *
    ///   *   *
    ///
    ///```
    pub const stickfigure_image: u32 = 0x0407f7ad;
    ///```text
    /// An image in the shape of:
    ///   *****
    ///   * * *
    ///   *****
    ///   *****
    ///   * * *
    ///
    ///```
    pub const ghost_image: u32 = 0x0143c000;
    ///```text
    /// An image in the shape of:
    ///     *  
    ///     *  
    ///     *  
    ///    ***
    ///     *  
    ///
    ///```
    pub const sword_image: u32 = 0x077bf78d;
    ///```text
    /// An image in the shape of:
    ///   **   
    ///    *   
    ///    *   
    ///    ***
    ///    * *
    ///
    ///```
    pub const giraffe_image: u32 = 0x07bf2e8e;
    ///```text
    /// An image in the shape of:
    ///    ***
    ///   * * *
    ///   *****
    ///    ***
    ///    ***
    ///
    ///```
    pub const skull_image: u32 = 0x015b008d;
    ///```text
    /// An image in the shape of:
    ///    ***
    ///   *****
    ///     *  
    ///   * *  
    ///    **  
    ///
    ///```
    pub const umbrella_image: u32 = 0x041b475d;
    ///```text
    /// An image in the shape of:
    ///   **   
    ///   ** **
    ///    * *
    ///    ***
    ///        
    ///
    ///```
    pub const snake_image: u32 = 0x009fee8e;
}

#[cfg(test)]
pub mod tests {
    use core::assert_eq;
    use core::prelude::rust_2024::test;
    use crate::img::image_to_preformated_vector;
    use crate::img;

    // use crate::bitmaps::img::{self, image_to_preformated_vector};

    #[test]
    fn check_images() {
        assert_eq!(
            image_to_preformated_vector(img::full_square),
            [
                [1, 1, 1, 1, 1, 1, 1, 1, 1],
                [1, 1, 1, 1, 1, 1, 1, 0, 0],
                [1, 1, 1, 1, 1, 1, 1, 1, 1]
            ]
        );
        assert_eq!(
            image_to_preformated_vector(img::umbrella_image),
            [
                [0, 1, 0, 0, 0, 1, 0, 1, 0],
                [0, 0, 1, 1, 1, 0, 1, 0, 0],
                [1, 0, 0, 1, 1, 1, 1, 1, 0]
            ]
        );
        assert_eq!(
            image_to_preformated_vector(img::minus),
            [
                [0, 0, 0, 0, 0, 0, 0, 0, 1],
                [0, 0, 1, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 1]
            ]
        );
        assert_eq!(
            image_to_preformated_vector(img::duck_image),
            [
                [0, 1, 0, 0, 1, 1, 1, 0, 1],
                [1, 0, 1, 1, 0, 0, 0, 0, 0],
                [0, 0, 0, 1, 1, 1, 0, 0, 1]
            ]
        );
    }
}
