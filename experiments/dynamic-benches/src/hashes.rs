use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use uuid::Uuid;

const DATA_SIZE: usize = 10000;

pub fn load_uuid_data() -> Vec<Uuid> {
    let mut v: Vec<Uuid> = Vec::with_capacity(DATA_SIZE);
    for _i in 0..DATA_SIZE {
        v.push(Uuid::new_v4());
    }
    v
}

pub fn load_addr_data() -> Vec<SocketAddr> {
    let mut v: Vec<SocketAddr> = Vec::with_capacity(DATA_SIZE);
    for i in 0..DATA_SIZE {
        let bytes: [u8; 4] = unsafe { std::mem::transmute(i as u32) };
        let port_bytes = [bytes[0], bytes[1]];
        let port: u16 = unsafe { std::mem::transmute(port_bytes) };
        v.push(SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, bytes[3])),
            port,
        ));
    }
    v
}

pub const VAL: &'static str = "Test me!";

#[cfg(test)]
mod tests {
    use fnv::FnvHashMap;
    use std::collections::BTreeMap;
    use std::collections::HashMap;
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_uuid_insert_sip(b: &mut Bencher) {
        let data = load_uuid_data();
        let mut map: HashMap<Uuid, &'static str> =
            HashMap::with_capacity_and_hasher(DATA_SIZE, Default::default());
        b.iter(|| {
            data.iter().for_each(|id| {
                let _ = map.insert(id.clone(), VAL);
            });
        });
        assert_eq!(map.len(), DATA_SIZE);
        map.drain().for_each(|v| {
            assert_eq!(v.1, VAL);
        });
    }

    #[bench]
    fn bench_uuid_insert_fnv(b: &mut Bencher) {
        let data = load_uuid_data();
        let mut map: FnvHashMap<Uuid, &'static str> =
            FnvHashMap::with_capacity_and_hasher(DATA_SIZE, Default::default());
        b.iter(|| {
            data.iter().for_each(|id| {
                let _ = map.insert(id.clone(), VAL);
            });
        });
        assert_eq!(map.len(), DATA_SIZE);
        map.drain().for_each(|v| {
            assert_eq!(v.1, VAL);
        });
    }

    #[bench]
    fn bench_uuid_insert_btree(b: &mut Bencher) {
        let data = load_uuid_data();
        let mut map: BTreeMap<Uuid, &'static str> = BTreeMap::new();
        b.iter(|| {
            data.iter().for_each(|id| {
                let _ = map.insert(id.clone(), VAL);
            });
        });
        assert_eq!(map.len(), DATA_SIZE);
        map.iter().for_each(|v| {
            assert_eq!(*v.1, VAL);
        });
        map.clear();
    }

    #[bench]
    fn bench_socket_insert_sip(b: &mut Bencher) {
        let data = load_addr_data();
        let mut map: HashMap<SocketAddr, &'static str> =
            HashMap::with_capacity_and_hasher(DATA_SIZE, Default::default());
        b.iter(|| {
            data.iter().for_each(|id| {
                let _ = map.insert(id.clone(), VAL);
            });
        });
        assert_eq!(map.len(), DATA_SIZE);
        map.drain().for_each(|v| {
            assert_eq!(v.1, VAL);
        });
    }

    #[bench]
    fn bench_socket_insert_fnv(b: &mut Bencher) {
        let data = load_addr_data();
        let mut map: FnvHashMap<SocketAddr, &'static str> =
            FnvHashMap::with_capacity_and_hasher(DATA_SIZE, Default::default());
        b.iter(|| {
            data.iter().for_each(|id| {
                let _ = map.insert(id.clone(), VAL);
            });
        });
        assert_eq!(map.len(), DATA_SIZE);
        map.drain().for_each(|v| {
            assert_eq!(v.1, VAL);
        });
    }

    // LOOKUPS

    #[bench]
    fn bench_uuid_lookup_sip(b: &mut Bencher) {
        let data = load_uuid_data();
        let mut map: HashMap<Uuid, &'static str> =
            HashMap::with_capacity_and_hasher(DATA_SIZE, Default::default());
        data.iter().for_each(|id| {
            let _ = map.insert(id.clone(), VAL);
        });
        assert_eq!(map.len(), DATA_SIZE);
        b.iter(|| {
            data.iter().for_each(|id| {
                let r = map.get(id);
                assert!(r.is_some());
            });
        });
        map.clear();
    }

    #[bench]
    fn bench_uuid_lookup_fnv(b: &mut Bencher) {
        let data = load_uuid_data();
        let mut map: FnvHashMap<Uuid, &'static str> =
            FnvHashMap::with_capacity_and_hasher(DATA_SIZE, Default::default());
        data.iter().for_each(|id| {
            let _ = map.insert(id.clone(), VAL);
        });
        assert_eq!(map.len(), DATA_SIZE);
        b.iter(|| {
            data.iter().for_each(|id| {
                let r = map.get(id);
                assert!(r.is_some());
            });
        });
        map.clear();
    }

    #[bench]
    fn bench_uuid_lookup_btree(b: &mut Bencher) {
        let data = load_uuid_data();
        let mut map: BTreeMap<Uuid, &'static str> = BTreeMap::new();
        data.iter().for_each(|id| {
            let _ = map.insert(id.clone(), VAL);
        });
        assert_eq!(map.len(), DATA_SIZE);
        b.iter(|| {
            data.iter().for_each(|id| {
                let r = map.get(id);
                assert!(r.is_some());
            });
        });
        map.clear();
    }

    #[bench]
    fn bench_socket_lookup_sip(b: &mut Bencher) {
        let data = load_addr_data();
        let mut map: HashMap<SocketAddr, &'static str> =
            HashMap::with_capacity_and_hasher(DATA_SIZE, Default::default());
        data.iter().for_each(|id| {
            let _ = map.insert(id.clone(), VAL);
        });
        assert_eq!(map.len(), DATA_SIZE);
        b.iter(|| {
            data.iter().for_each(|id| {
                let r = map.get(id);
                assert!(r.is_some());
            });
        });
        map.clear();
    }

    #[bench]
    fn bench_socket_lookup_fnv(b: &mut Bencher) {
        let data = load_addr_data();
        let mut map: FnvHashMap<SocketAddr, &'static str> =
            FnvHashMap::with_capacity_and_hasher(DATA_SIZE, Default::default());
        data.iter().for_each(|id| {
            let _ = map.insert(id.clone(), VAL);
        });
        assert_eq!(map.len(), DATA_SIZE);
        b.iter(|| {
            data.iter().for_each(|id| {
                let r = map.get(id);
                assert!(r.is_some());
            });
        });
        map.clear();
    }
}