mod r#type;
pub use r#type::Size;
#[cfg(test)]
mod tests {
    use super::Size;
    #[test]
    fn it_works() {
        let mut a = Size::from_byte(1024usize);
        let mut b = Size::from_byte(512usize);
        println!("{:?} + {:?} = {:?}", a, b, a + b);
        println!("{:?} - {:?} = {:?}", a, b, a - b);
        a += b;
        println!("After {:?} += {:?} a: {:?}", a, b, a);
        a -= b;
        println!("After {:?} -= {:?} a: {:?}", a, b, a);
    }
}
