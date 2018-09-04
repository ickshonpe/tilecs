
use anymap::AnyMap;
use std::any::TypeId;
use std::collections::HashSet;

pub struct Tiles {
    size: usize,
    registered: HashSet<TypeId>,
    components: AnyMap
}

impl Tiles {
    pub fn new(size: usize) -> Tiles {
        Tiles {
            size,
            registered: HashSet::new(),
            components: AnyMap::new()
        }
    }

    pub fn get_components<T: 'static>(&self) -> &Vec<Option<T>> {
        self.components.get::<Vec<Option<T>>>().unwrap()
    }

    pub fn get_components_mut<T: 'static>(&mut self) -> &mut Vec<Option<T>> {
        self.components.get_mut::<Vec<Option<T>>>().unwrap()
    }

    pub fn register<T>(&mut self) where T: 'static {
        self.registered.insert(TypeId::of::<T>());
        self.components.insert({
            let mut v = Vec::<Option<T>>::with_capacity(self.size);
            for _ in 0..self.size {
                v.push(None);
            }
            v
        });        
    }

    pub fn get<T>(&self, tile: usize) -> Option<&T>
        where T: 'static {
        self.get_components::<T>()[tile].as_ref()
    }   

    pub fn insert<T>(&mut self, tile: usize, component: T) 
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

    pub fn clear(&mut self, tile: usize) {
        for t in self.registered.iter() {
            let mut b = self.components.as_mut().get_mut(k);
            let x: u32 = b;
        }
    }
}



#[cfg(test)]
mod tests {
    use Tiles;

    #[test]
    fn insert_component_and_get() {
        let item = "Hello";
        let mut tile_map = Tiles::new(100);
        tile_map.register::<String>();        
        tile_map.insert(11, String::from(item));
        let value = tile_map.get::<String>(11).unwrap();
        assert_eq!(value, item);
    }

    #[test]
    fn check_has_after_insert_and_remove() {
        let i = 7;
        let c = 10;
        let mut tile_map = Tiles::new(97);
        tile_map.register::<i32>();                
        assert!(!tile_map.has::<i32>(i));
        tile_map.insert(i, c);
        assert!(tile_map.has::<i32>(i));
        tile_map.remove::<i32>(i);
        assert!(!tile_map.has::<i32>(i));
    }
}
