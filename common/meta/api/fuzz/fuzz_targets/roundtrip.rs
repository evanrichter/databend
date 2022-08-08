#![no_main]
use common_meta_api::deserialize_struct;
use common_meta_api::serialize_struct;
use common_meta_types as mt;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let x: Result<mt::UserInfo, _> = deserialize_struct(data);
    if let Ok(t) = x {
        let mut ser = serialize_struct(&t).expect("a deserialized type should serialize");
        #[cfg(feature = "debug")]
        dbg!(&ser);

        let des: mt::UserInfo =
            deserialize_struct(&mut ser).expect("a serialized type should deserialize");
        #[cfg(feature = "debug")]
        dbg!(&des);

        assert_eq!(t, des, "roundtripped object changed");
    }
});
