#![allow(dead_code)]
#![allow(unused_macros)]

extern crate anymap;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Entity {
    handle: usize,
    generation: u32
}

macro_rules! tiles {
    ($name:ident, [$($c:ty),+]) => {
        use Entity;

        pub struct $name {
            size: usize,
            components: ::anymap::AnyMap,
            entities: Vec<Entity>,
            entity_lookup: Vec<usize>
        }

        impl $name { 
            pub fn new(size: usize) -> $name {
                let entities = 
                    (0..size)
                    .map(|i| Entity { handle: i, generation: 0 })
                    .collect::<Vec<Entity>>();
                let entity_lookup = entities.iter().map(|e| e.handle).collect();
                $name {
                    size,
                    components: Self::create_component_storage(size),
                    entities,
                    entity_lookup
                }
            }        

            fn create_component_storage(size: usize) -> ::anymap::AnyMap {
                let mut components = ::anymap::AnyMap::new();
                $(components.insert({                    
                    let mut v = Vec::<Option<$c>>::with_capacity(size);
                    for _ in 0..size {
                        v.push(None);
                    }   
                    v
                });)*
                components
            }        

            pub fn get_components<T>(&self) -> &Vec<Option<T>>
                where T: 'static {
                self.components.get::<Vec<Option<T>>>().unwrap()
            }

            pub fn get_components_mut<T>(&mut self) -> &mut Vec<Option<T>>
                where T: 'static {
                self.components.get_mut::<Vec<Option<T>>>().unwrap()
            }

            pub fn add<T>(&mut self, tile: usize, component: T)
                where T: 'static {
                self.get_components_mut::<T>()[tile] = Some(component);
            }

            pub fn remove<T>(&mut self, tile: usize)
                where T: 'static {
                self.get_components_mut::<T>()[tile] = None;
            }

            pub fn has<T>(&self, tile: usize) -> bool
                where T: 'static {
                self.get_components::<T>()[tile].is_some()
            }

            pub fn delete(&mut self, tile: usize) {
                $(self.remove::<$c>(tile);)*
            }

            pub fn clear_all(&mut self) {
                $(for component in self.get_components_mut::<$c>() {
                    *component = None;
                }
                )*
            }

            pub fn clear<T>(&mut self) 
            where T: 'static {
                for component in self.get_components_mut::<T>() {
                    *component = None;
                }
            }

            pub fn get<T>(&self, tile: usize) -> Option<&T>
            where T: 'static {
                self.get_components::<T>()[tile].as_ref()
            }

            pub fn get_mut<T>(&mut self, tile: usize) -> Option<&mut T>
            where T: 'static {
                self.get_components_mut::<T>()[tile].as_mut()
            }

            pub fn len(&self) -> usize {
                self.size
            }

            pub fn move_tile(&mut self, source: usize, target: usize) {
                $({
                   self.transfer::<$c>(source, target);
                })*                
            }

            pub fn transfer<T>(&mut self, source: usize, target: usize)
            where T: 'static {
                let cs = self.get_components_mut::<T>();
                cs[target] = cs[source].take();
            }

            pub fn take<T>(&mut self, tile: usize) -> T
            where T: 'static {
                self.get_components_mut::<T>()[tile].take().unwrap()
            }
        }
    };
}








#[cfg(test)]
mod tests {
    tiles! (Tiles, [f64, String, i32]);

    #[test]
    fn insert_component_and_get() {
        let item = "Hello";
        let mut tile_map = Tiles::new(100);
        tile_map.add(11, String::from(item));
        let value = tile_map.get::<String>(11).unwrap();
        assert_eq!(value, item);
    }

    #[test]
    fn check_has_after_insert_and_remove() {
        let i = 7;
        let c = 10;
        let mut tile_map = Tiles::new(97);
        assert!(!tile_map.has::<i32>(i));
        tile_map.add(i, c);
        assert!(tile_map.has::<i32>(i));
        tile_map.remove::<i32>(i);
        assert!(!tile_map.has::<i32>(i));
    }

    #[test]
    fn does_not_have_component() {
        let mut tile_map = Tiles::new(1);
        assert!(!tile_map.has::<f64>(0));
        assert!(!tile_map.has::<i32>(0));
        assert!(!tile_map.has::<String>(0));
    }
}
