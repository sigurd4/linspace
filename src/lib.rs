#![feature(const_trait_impl)]
#![feature(const_transmute_copy)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_refs_to_cell)]

moddef::pub_flat_mods!(linspace linspace_array);

#[cfg(test)]
mod test
{
    use crate::{LinspaceArray, Linspace};

    #[test]
    fn test()
    {
        const T: [f32; 4] = (0.0..1.0).linspace_array();
        let t: Vec<f32> = (0.0..1.0).linspace(4);
        println!("{:?}", T);
        println!("{:?}", t);
    }
}