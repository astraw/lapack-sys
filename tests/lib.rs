extern crate lapack_sys as ffi;

#[test]
fn link() {
    let jobz = b'V' as i8;
    let uplo = b'U' as i8;
    let n = 1;
    let mut a = vec![0.0];
    let lda = 1;
    let mut w = vec![0.0];
    let mut work = vec![0.0, 0.0];
    let lwork = 2;
    let mut flag = 0;

    unsafe {
        ffi::dsyev_(&jobz, &uplo, &n, a.as_mut_ptr(), &lda, w.as_mut_ptr(), work.as_mut_ptr(),
                    &lwork, &mut flag);
    }
}
