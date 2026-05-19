use rs_backend::helpers::t_pagination::TPaginationCore;

#[test]
fn pagination_test_pages_vec() {
    let pagination = TPaginationCore::new(10, 5);
    assert_eq!(pagination.next_generate(), vec![6, 7, 8, 9]);
    assert_eq!(pagination.previus_generate(), vec![1, 2, 3, 4]);
    assert_eq!(pagination.pages_to_vec(), vec![1, 2, 3, 4, 6, 7, 8, 9]);
    let pagination = TPaginationCore::new(10, 2);
    assert_eq!(pagination.next_generate(), vec![3, 4, 5, 6]);
    assert_eq!(pagination.previus_generate(), vec![1]);
    assert_eq!(pagination.pages_to_vec(), vec![1, 3, 4, 5, 6]);
    let pagination = TPaginationCore::new(10, 1);
    assert_eq!(pagination.next_generate(), vec![2, 3, 4, 5]);
    assert!(pagination.previus_generate().is_empty());
    let pagination = TPaginationCore::new(10, 7);
    assert_eq!(pagination.next_generate(), vec![8, 9, 10]);
    assert_eq!(pagination.previus_generate(), vec![3, 4, 5, 6]);
    assert_eq!(pagination.pages_to_vec(), vec![3, 4, 5, 6, 8, 9, 10]);
}
