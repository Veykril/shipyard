use shipyard::*;

#[test]
fn get() {
    let world = World::new();
    world
        .try_run(|mut all_storages: AllStoragesViewMut| {
            let (owned, shared) = all_storages
                .try_run(|mut entities: EntitiesViewMut, mut u32s: ViewMut<u32>| {
                    let owned = entities.add_entity(&mut u32s, 0);
                    let shared = entities.add_entity((), ());
                    u32s.try_share(owned, shared).unwrap();

                    assert_eq!(u32s.get(owned), Ok(&0));
                    assert_eq!(u32s.get(shared), Ok(&0));
                    assert_eq!(u32s.try_as_window(..).unwrap().get(owned), Ok(&0));
                    assert_eq!(u32s.try_as_window(..).unwrap().get(shared), Ok(&0));

                    assert_eq!(u32s.try_remove(shared).unwrap(), Some(OldComponent::Shared));
                    assert_eq!(u32s.get(owned), Ok(&0));
                    assert!(u32s.get(shared).is_err());

                    u32s.try_share(owned, shared).unwrap();
                    u32s.try_unshare(shared).unwrap();
                    assert_eq!(u32s.get(owned), Ok(&0));
                    assert!(u32s.get(shared).is_err());

                    u32s.try_share(owned, shared).unwrap();
                    (owned, shared)
                })
                .unwrap();

            all_storages.delete(owned);

            all_storages
                .try_run(|u32s: View<u32>| {
                    assert!(u32s.get(shared).is_err());
                })
                .unwrap();
        })
        .unwrap()
}

#[test]
fn get_mut() {
    let world = World::new();
    world
        .try_run(|mut all_storages: AllStoragesViewMut| {
            let (owned, shared) = all_storages
                .try_run(|mut entities: EntitiesViewMut, mut u32s: ViewMut<u32>| {
                    let owned = entities.add_entity(&mut u32s, 0);
                    let shared = entities.add_entity((), ());
                    u32s.try_share(owned, shared).unwrap();

                    assert_eq!((&mut u32s).get(owned), Ok(&mut 0));
                    assert_eq!((&mut u32s).get(shared), Ok(&mut 0));
                    assert_eq!(
                        (&mut u32s).try_as_window_mut(..).unwrap().get(owned),
                        Ok(&0)
                    );
                    assert_eq!(
                        (&mut u32s).try_as_window_mut(..).unwrap().get(shared),
                        Ok(&0)
                    );

                    assert_eq!(u32s.try_remove(shared).unwrap(), Some(OldComponent::Shared));
                    assert_eq!((&mut u32s).get(owned), Ok(&mut 0));
                    assert!((&mut u32s).get(shared).is_err());

                    assert_eq!(u32s.try_unshare(shared).err(), Some(error::Unshare));
                    assert_eq!((&mut u32s).get(owned), Ok(&mut 0));
                    assert!((&mut u32s).get(shared).is_err());

                    u32s.try_share(owned, shared).unwrap();
                    (owned, shared)
                })
                .unwrap();

            all_storages.delete(owned);

            all_storages
                .try_run(|mut u32s: ViewMut<u32>| {
                    assert!((&mut u32s).get(shared).is_err());
                })
                .unwrap();
        })
        .unwrap()
}

#[test]
fn iter() {
    let world = World::new();
    world
        .try_run(|mut entities: EntitiesViewMut, mut u32s: ViewMut<u32>| {
            let owned = entities.add_entity(&mut u32s, 0);
            let shared = entities.add_entity((), ());
            u32s.try_share(owned, shared).unwrap();

            let mut iter = u32s.iter();
            assert_eq!(iter.next(), Some(&0));
            assert_eq!(iter.next(), None);
        })
        .unwrap();
}

#[test]
fn double_shared() {
    let world = World::new();
    world
        .try_run(|mut entities: EntitiesViewMut, mut u32s: ViewMut<u32>| {
            let owned1 = entities.add_entity(&mut u32s, 1);
            let owned2 = entities.add_entity(&mut u32s, 2);
            let shared1 = entities.add_entity((), ());
            let shared2 = entities.add_entity((), ());
            u32s.try_share(owned1, shared1).unwrap();
            u32s.try_share(shared1, shared2).unwrap();

            assert_eq!(u32s.get(owned1), Ok(&1));
            assert_eq!(u32s.get(owned2), Ok(&2));
            assert_eq!(u32s.get(shared1), Ok(&1));
            assert_eq!(u32s.get(shared2), Ok(&1));

            u32s.try_unshare(shared1).unwrap();
            assert_eq!(u32s.get(owned1), Ok(&1));
            assert_eq!(u32s.get(owned2), Ok(&2));
            assert!(u32s.get(shared1).is_err());
            assert!(u32s.get(shared2).is_err());

            u32s.try_share(owned2, shared1).unwrap();
            assert_eq!(u32s.get(owned1), Ok(&1));
            assert_eq!(u32s.get(owned2), Ok(&2));
            assert_eq!(u32s.get(shared1), Ok(&2));
            assert_eq!(u32s.get(shared2), Ok(&2));
        })
        .unwrap();
}

#[test]
fn shared_override() {
    let world = World::new();

    world
        .try_run(|mut entities: EntitiesViewMut, mut u32s: ViewMut<u32>| {
            let owned = entities.add_entity(&mut u32s, 0);
            let shared = entities.add_entity((), ());

            u32s.try_share(owned, shared).unwrap();

            u32s.try_remove(owned).unwrap();
            entities.try_add_component(&mut u32s, 1, shared).unwrap();

            u32s.get(shared).unwrap();
            u32s.try_remove(shared).unwrap();
            assert!(u32s.get(shared).is_err());
        })
        .unwrap();
}

#[test]
fn self_shared() {
    let world = World::new();

    world
        .try_run(|mut entities: EntitiesViewMut, mut u32s: ViewMut<u32>| {
            let shared = entities.add_entity((), ());

            u32s.try_share(shared, shared).unwrap();

            assert!(u32s.get(shared).is_err());
        })
        .unwrap();
}
