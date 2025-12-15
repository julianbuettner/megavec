use std::path::Path;

use megavec::{Lz4, Megavec};
use rand_chacha::{
    rand_core::{RngCore, SeedableRng as _},
    ChaCha12Rng,
};

#[test]
fn hello() {
    let mut mega = Megavec::new(Lz4, 128);
    let mut classic = Vec::new();
    mega.push(123);
    classic.push(123);

    let mut rng = ChaCha12Rng::from_seed([42; 32]);
    for i in 0..100_000 {
        let v = rng.next_u64();
        mega.push(v);
        classic.push(v);
        assert_eq!(mega.len(), classic.len());
        if i % 13 == 0 {
            assert_eq!(mega.iter().collect::<Vec<_>>(), classic);
        }
    }
    assert_eq!(mega.iter().collect::<Vec<_>>(), classic);
    for i in 0.. {
        let mega_pop = mega.pop();
        let classic_pop = classic.pop();
        assert_eq!(mega_pop, classic_pop);
        assert_eq!(mega.len(), classic.len());
        if i % 13 == 0 {
            assert_eq!(mega.iter().collect::<Vec<_>>(), classic);
        }
        if mega.is_empty() {
            break;
        }
    }
    assert_eq!(mega.iter().collect::<Vec<_>>(), classic);
}
