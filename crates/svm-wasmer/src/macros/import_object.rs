/// Creates an instance of `SvmCtx` to be injected into `wasmer` context `data` field.
/// `svm vmcalls` will access that `SvmCtx` while runninng smart contracts
#[macro_export]
macro_rules! create_svm_ctx {
    ($node_data: expr, $pages_storage_gen: expr, $PC: ident, $max_pages: expr, $max_pages_slices: expr) => {{
        use svm_storage::PageSliceCache;
        use $crate::ctx::SvmCtx;

        // pages storage
        let pages = $pages_storage_gen();
        let boxed_pages = Box::new(pages);
        let leaked_pages: &mut _ = Box::leak(boxed_pages);

        // page cache
        let page_cache = $PC::new(leaked_pages, $max_pages);
        let boxed_page_cache = Box::new(page_cache);
        let page_cache: &mut _ = Box::leak(boxed_page_cache);

        // page-slice cache
        let storage = PageSliceCache::new(page_cache, $max_pages_slices);
        let boxed_storage = Box::new(storage);
        let storage: &mut _ = Box::leak(boxed_storage);

        let ctx = SvmCtx::new($node_data, storage);
        let boxed_ctx = Box::new(ctx);

        let ctx_ptr = Box::leak(boxed_ctx);
        let ctx = &mut *ctx_ptr;

        ctx
    }};
}

/// Builds a `svm wasmer` import object to be used when creating a `wasmer` instance.
#[macro_export]
macro_rules! create_svm_state_gen {
    ($node_data: expr, $pages_storage_gen: expr, $PC: ident, $max_pages: expr, $max_pages_slices: expr) => {{
        use std::ffi::c_void;
        use $crate::ctx::SvmCtx;

        let ctx = create_svm_ctx!(
            $node_data,
            $pages_storage_gen,
            $PC,
            $max_pages,
            $max_pages_slices
        );

        let data = ctx as *mut _ as *mut c_void;
        let dtor: fn(*mut c_void) = |ctx_data| {
            let ctx_ptr = ctx_data as *mut SvmCtx<$PC>;
            let ctx: Box<SvmCtx<$PC>> = unsafe { Box::from_raw(ctx_ptr) };
            drop(ctx);
        };

        (data, dtor)
    }};
}

/// Returns a closure that when invoked (without args) calls `create_svm_state_gen`
#[macro_export]
macro_rules! lazy_create_svm_state_gen {
    ($node_data: expr, $pages_storage_gen: expr, $PC: ident, $max_pages: expr, $max_pages_slices: expr) => {{
        move || {
            create_svm_state_gen!(
                $node_data,
                $pages_storage_gen,
                $PC,
                $max_pages,
                $max_pages_slices
            )
        }
    }};
}