#[cfg(test)]
mod tests
{
    use es2::MyCycle;

    #[test]
    fn zero_item()
    {
        let array = Vec::<usize>::new();
        let mc = MyCycle::new(array.iter(), 5);
        assert_eq!(mc.count(), 0);
    }

    #[test]
    fn iter_from_iter()
    {
        let array = vec![1, 2, 3];
        let mc1 = MyCycle::new(array.iter(), 5);
        let mc2 = MyCycle::new(mc1.clone(), 4);
        assert_eq!(60, mc2.count());
    }

    #[test]
    fn iter_chain()
    {
        let array = vec![1, 2, 3];
        let array1 = vec![1, 2];
        let mc1 = MyCycle::new(array.iter(), 4);
        let mc2 = MyCycle::new(array1.iter(), 5);
        assert_eq!(12+10, mc1.chain(mc2).count());
    }

    #[test]
    fn zip_iter()
    {
        let array = vec![1, 2, 3];
        let array1 = vec![1, 2];
        let mc1 = MyCycle::new(array.iter(), 1);
        let mc2 = MyCycle::new(array1.iter(), 2);
        let mut zipped = mc1.zip(mc2);
        
        assert_eq!(zipped.next().unwrap(), (&1, &1));
    }
}