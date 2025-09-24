#![feature(const_trait_impl)]
#![feature(try_trait_v2)]
#![feature(trusted_random_access)]
#![feature(exact_size_is_empty)]
#![feature(trusted_len)]
#![feature(const_destruct)]
#![feature(const_ops)]
#![feature(const_cmp)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(maybe_uninit_slice)]

moddef::moddef!(
    flat(pub) mod {
        linspace,
        linspaced
    }
);

#[cfg(test)]
mod test
{
    use crate::{Linspace};

    #[test]
    fn test()
    {
        let t: [f32; 4] = (0.0..1.0).linspace_array();
        println!("{:?}", t);
        
        let t: Vec<f32> = (0.0..1.0).linspace(4).collect();
        println!("{:?}", t);
    }
}