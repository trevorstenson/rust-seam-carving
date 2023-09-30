mod utils;

use wasm_bindgen::prelude::*;
use js_sys::Uint8Array;
use image::{ImageBuffer, Rgba};

#[wasm_bindgen]
pub fn process_image(data: JsValue, width: usize, height: usize, iterations: usize) -> Uint8Array {
    let data_u8_array: Uint8Array = Uint8Array::from(data);
    let data_vec: Vec<u8> = data_u8_array.to_vec();
    let img = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_vec(width as u32, height as u32, data_vec).unwrap();
    let final_img = seam_carve(img, iterations);
    let final_data = final_img.into_raw();
    let final_data_u8_array = Uint8Array::from(final_data.as_slice());
    return final_data_u8_array;
}

#[derive(Clone)]
struct EnergyData {
    energy: i64,
}

fn compute_energies(energies: &mut Vec<Vec<EnergyData>>, img: &ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let (width, height) = img.dimensions();
    for y in 0..height as usize {
        for x in 0..width as usize {
            let left_x = if x == 0 { width - 1 } else { (x - 1) as u32 };
            let right_x = if x == width as usize - 1 { 0 } else { x + 1 } as u32;
            let top_y = if y == 0 { height - 1 } else { (y - 1) as u32 };
            let bottom_y = if y == height as usize - 1 { 0 } else { y + 1 } as u32;
            
            let left_pixel = img.get_pixel(left_x, y as u32).0;
            let right_pixel = img.get_pixel(right_x, y as u32).0;
            let top_pixel = img.get_pixel(x as u32, top_y).0;
            let bottom_pixel = img.get_pixel(x as u32, bottom_y).0;
            
            let dx = [
                (left_pixel[0] as i32 - right_pixel[0] as i32).abs(),
                (left_pixel[1] as i32 - right_pixel[1] as i32).abs(),
                (left_pixel[2] as i32 - right_pixel[2] as i32).abs()
            ];
            
            let dy = [
                (top_pixel[0] as i32 - bottom_pixel[0] as i32).abs(),
                (top_pixel[1] as i32 - bottom_pixel[1] as i32).abs(),
                (top_pixel[2] as i32 - bottom_pixel[2] as i32).abs()
            ];
            
            let energy: i64 = dx.iter().chain(dy.iter()).map(|&d| (d as i64) * (d as i64)).sum();
            energies[y][x].energy = energy;
        }
    }
}

fn find_seam(energies: &Vec<Vec<EnergyData>>) -> Vec<usize> {
    let height = energies.len();
    let width = energies[0].len();
    let mut seam_energies = vec![vec![0; width]; height];
    let mut choices = vec![vec![0; width]; height];

    for x in 0..width {
        seam_energies[0][x] = energies[0][x].energy;
    }

    for y in 1..height {
        for x in 0..width {
            let left_x = if x == 0 { width - 1 } else { x - 1 };
            let right_x = if x == width - 1 { 0 } else { x + 1 };
            let neighbors = [left_x, x, right_x];
            let (chosen, min_energy) = neighbors.iter().map(|&x| seam_energies[y - 1][x]).enumerate().min_by_key(|&(_, e)| e).unwrap();
            seam_energies[y][x] = energies[y][x].energy + min_energy;
            choices[y][x] = chosen;
        }
    }

    let mut seam = Vec::with_capacity(height);
    let mut curr_x = seam_energies[height - 1].iter().enumerate().min_by_key(|&(_, e)| e).unwrap().0;
    seam.push(curr_x);

    for y in (1..height).rev() {
        curr_x = choices[y][curr_x];
        seam.push(curr_x);
    }

    seam.reverse();
    seam
}

fn remove_seam(img: &ImageBuffer<Rgba<u8>, Vec<u8>>, seam: &Vec<usize>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let mut new_img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width - 1, height);
    for y in 0..height {
        let mut curr_x = 0;
        for x in 0..width {
            if x != seam[y as usize] as u32 {
                new_img.put_pixel(curr_x, y, *img.get_pixel(x, y));
                curr_x += 1;
            }
        }
    }
    new_img
}

fn seam_carve(mut img: ImageBuffer<Rgba<u8>, Vec<u8>>, iterations: usize) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    for _ in 0..iterations {
        let (width, height) = img.dimensions();
        let mut energies = vec![vec![EnergyData { energy: 0 }; width as usize]; height as usize];
        compute_energies(&mut energies, &img);
        let seam = find_seam(&energies);
        img = remove_seam(&img, &seam);
    }
    img
}