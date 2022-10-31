use nifti::{NiftiObject, ReaderOptions, NiftiError, IntoNdArray};
use ndarray::Axis;
use image::{RgbImage, Rgb, ImageFormat::Png};

fn main() -> Result<(), NiftiError> {
    let obj = ReaderOptions::new().read_file("avg152T1_LR_nifti.nii.gz")?;
    
    let dim: &[u16] = obj.header().dim()?;
    let dimensionality: usize = dim.len();
    assert!(dimensionality == 3);
    
    let volume = obj.volume().into_ndarray::<u8>()?;
    let matrix_it = volume.axis_iter(Axis(dimensionality - 1));
    for (i, m) in matrix_it.enumerate() {
        let mut rgb = RgbImage::new(dim[0] as u32,dim[1] as u32);
        for (x, y, pixel) in rgb.enumerate_pixels_mut() {
            let val = m[[x as usize, y as usize]];
            *pixel = Rgb([val, val, val]);
        }
        rgb.save_with_format(format!("{i}.png"), Png).unwrap();
    }
    Ok(())
}
