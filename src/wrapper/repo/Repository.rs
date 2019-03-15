trait Repository {
    type Entity;
    type NewEntity;
    type Key;
    type RepoResult;

    /// get the entity by primary key.
    /// # arguments
    /// - id -- the entity key.
    /// # return
    /// - the entity.
    fn get_by_id(&mut self, id: Key) -> Option<Entity>;

    /// save the underlying entity.
    /// # arguments
    /// - item -- the new entity.
    /// # return
    /// - the created entity if success, or else None.
    fn save(&mut self, item: NewEntity) -> RepoResult;

    /// save the modified entity into the repository.
    /// # arguments
    /// - item -- the modified item.
    /// *note*: this could be implemented by many different ways.
    /// if you modify the id field, the behavior may be undefined.
    /// but who implement this trait should promise that
    /// if the entity is get by the `get_by_id` method, and the id field
    /// doesn't changed, every call of `get_by_id` with the same id
    /// after `save_modified` should return the modified entity.
    fn save_modified(&mut self, item: Entity);

    /// remove an entity by key.
    /// # arguments
    /// - id -- the key.
    fn remove_by_id(id: Key);
}