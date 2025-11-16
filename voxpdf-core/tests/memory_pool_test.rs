use voxpdf_core::memory_pool::{StringPool, VecPool};

#[test]
fn test_string_pool() {
    let pool = StringPool::new(10);

    let mut s1 = pool.acquire();
    s1.push_str("test");
    assert_eq!(s1, "test");

    pool.release(s1);

    let s2 = pool.acquire();
    assert_eq!(s2.capacity(), 32); // Should reuse pooled string
}

#[test]
fn test_vec_pool() {
    let pool: VecPool<i32> = VecPool::new(10);

    let mut v1 = pool.acquire();
    v1.push(1);
    v1.push(2);
    assert_eq!(v1.len(), 2);

    pool.release(v1);

    let v2 = pool.acquire();
    assert_eq!(v2.capacity(), 64); // Should reuse pooled vec
}
