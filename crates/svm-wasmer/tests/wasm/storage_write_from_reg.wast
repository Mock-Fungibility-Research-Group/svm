(module
  ;; import `svm` vmcalls
  (func $storage_write_from_reg (import "svm" "storage_write_from_reg") (param i32 i32 i32 i32 i32))

  (memory 1)  ;; memory `0` (default) is initialized with a `1 page`

  ;; exported function to be called
  (func (export "do_write_from_reg") (param i32 i32 i32 i32 i32)
        get_local 0 ;; src_reg
        get_local 1 ;; len
        get_local 2 ;; dst_page
        get_local 3 ;; dst_slice
        get_local 4 ;; dst_offset
        call $storage_write_from_reg))
