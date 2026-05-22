use rs_backend::helpers::t_pagination::TPaginationCore;

#[test]
fn pagination_test_pages_vec() {
    let pagination = TPaginationCore::new(10, 7);
    assert_eq!(pagination.pages_to_vec(), vec![1, 6, 7, 8, 10]);

    let pagination = TPaginationCore::new(10, 5);
    assert_eq!(pagination.pages_to_vec(), vec![1, 4, 5, 6, 10]);

    let pagination = TPaginationCore::new(3, 1);
    assert_eq!(pagination.pages_to_vec(), vec![1, 2, 3]);

    let pagination = TPaginationCore::new(1, 1);
    assert_eq!(pagination.pages_to_vec(), vec![1]);

    let pagination = TPaginationCore::new(6, 6);
    assert_eq!(pagination.pages_to_vec(), vec![1, 5, 6]);

    let pagination = TPaginationCore::new(6, 2);
    assert_eq!(pagination.pages_to_vec(), vec![1, 2, 3, 6]);
}
