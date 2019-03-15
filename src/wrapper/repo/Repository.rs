trait Repository<T, K> {
    fn getById(ID: K) -> T;
    fn save(item: T);
    fn removeById(ID: K);
}