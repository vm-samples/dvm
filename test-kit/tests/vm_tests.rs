use compiled_protos::vm_grpc::VmExecuteResponse;
use libra::{prelude::*, lcs};
use dvm_test_kit::*;
use dvm_test_kit::compiled_protos::vm_grpc::{VmArgs, VmTypeTag};
use runtime::resources::*;

#[test]
fn test_sender_as_argument() {
    let test_kit = TestKit::new();
    test_kit.add_std_module(include_str!("resources/store.move"));

    let script = "
        script {
        use 0x1::Store;

        fun main(account: &signer, addr: address) {
            Store::store_address(account, addr);
        }
        }
    ";

    let account_address = AccountAddress::random();
    let args = vec![VmArgs {
        r#type: VmTypeTag::Address as i32,
        value: account_address.to_vec(),
    }];
    let res = test_kit.execute_script(script, gas_meta(), args, vec![], vec![account("0x110")]);
    test_kit.assert_success(&res);
    let value: AddressStore = lcs::from_bytes(&res.write_set[0].value).unwrap();
    assert_eq!(value.val, account_address);
}

#[test]
fn test_senders_as_argument() {
    let test_kit = TestKit::new();
    test_kit.add_std_module(include_str!("resources/store.move"));

    let script = "
        script {
        use 0x1::Store;

        fun main(account: &signer, account_2: &signer, addr: address) {
            Store::store_address(account, addr);
            Store::store_address(account_2, addr);
        }
        }
    ";

    let account_address = AccountAddress::random();
    let args = vec![VmArgs {
        r#type: VmTypeTag::Address as i32,
        value: account_address.to_vec(),
    }];
    let res = test_kit.execute_script(
        script,
        gas_meta(),
        args,
        vec![],
        vec![account("0x110"), account("0x111")],
    );
    test_kit.assert_success(&res);
    let value: AddressStore = lcs::from_bytes(&res.write_set[0].value).unwrap();
    assert_eq!(value.val, account_address);
    let value: AddressStore = lcs::from_bytes(&res.write_set[1].value).unwrap();
    assert_eq!(value.val, account_address);
}

#[test]
fn test_vector_as_argument() {
    let test_kit = TestKit::new();
    test_kit.add_std_module(include_str!("resources/store.move"));

    let script = "
        script {
        use 0x1::Store;

        fun main(account: &signer, vec: vector<u8>) {
            Store::store_vector_u8(account, vec);
        }
        }
    ";

    let vec = AccountAddress::random().to_vec();
    let args = vec![VmArgs {
        r#type: VmTypeTag::Vector as i32,
        value: vec.clone(),
    }];
    let res = test_kit.execute_script(script, gas_meta(), args, vec![], vec![account("0x110")]);
    test_kit.assert_success(&res);
    let value: VectorU8Store = lcs::from_bytes(&res.write_set[0].value).unwrap();
    assert_eq!(value.val, vec);
}

#[test]
fn test_update_std_module() {
    let test_kit = TestKit::new();

    let res = test_kit.publish_module(
        "module Foo{ public fun foo(): u64 {1}}",
        gas_meta(),
        CORE_CODE_ADDRESS,
    );
    test_kit.assert_success(&res);
    test_kit.merge_result(&res);

    test_kit.add_std_module(include_str!("resources/store.move"));

    let load_foo = "\
        script {
        use 0x1::Foo;
        use 0x1::Store;

        fun main(account: &signer) {
            Store::store_u64(account, Foo::foo());
        }
        }
    ";
    let res = test_kit.execute_script(
        load_foo,
        gas_meta(),
        vec![],
        vec![],
        vec![AccountAddress::random()],
    );
    test_kit.assert_success(&res);
    let value: U64Store = lcs::from_bytes(&res.write_set[0].value).unwrap();
    assert_eq!(value.val, 1);

    let res = test_kit.publish_module(
        "module Foo{ public fun foo(): u64 {2}}",
        gas_meta(),
        CORE_CODE_ADDRESS,
    );
    test_kit.assert_success(&res);
    test_kit.merge_result(&res);
    test_kit.add_std_module(include_str!("resources/store.move"));

    let res = test_kit.execute_script(
        load_foo,
        gas_meta(),
        vec![],
        vec![],
        vec![AccountAddress::random()],
    );
    test_kit.assert_success(&res);
    let value: U64Store = lcs::from_bytes(&res.write_set[0].value).unwrap();
    assert_eq!(value.val, 2);
}

#[test]
fn test_update_std_module_1() {
    let test_kit = TestKit::new();

    let res = test_kit.publish_module(
        "module Foo{ public fun foo(): u64 {1}}",
        gas_meta(),
        CORE_CODE_ADDRESS,
    );
    test_kit.assert_success(&res);
    test_kit.merge_result(&res);

    let res = test_kit.publish_module(
        "module Foo{ public fun foo(): u64 {2}}",
        gas_meta(),
        CORE_CODE_ADDRESS,
    );
    test_kit.assert_success(&res);
    test_kit.merge_result(&res);
}

#[test]
fn test_publish_one_module() {
    let test_kit = TestKit::new();
    test_kit.add_std_module(include_str!("resources/store.move"));

    let sources = [
        "address 0x1 { module Foo { public fun foo(): u64 { 1 } } }",
        "module Bar { public fun bar(): u64 { 2 } }",
    ];
    sources.iter().for_each(|src| {
        let res = test_kit.publish_module(src, gas_meta(), CORE_CODE_ADDRESS);
        test_kit.assert_success(&res);
        test_kit.merge_result(&res);
    });

    let res = store_call_tx_without_args(&test_kit, "use 0x1::Foo;", "Foo::foo();");
    let value: U64Store = lcs::from_bytes(&res.write_set[0].value).unwrap();
    assert_eq!(value.val, 1);

    let res = store_call_tx_without_args(&test_kit, "use 0x1::Bar;", "Bar::bar();");
    let value: U64Store = lcs::from_bytes(&res.write_set[0].value).unwrap();
    assert_eq!(value.val, 2);
}

#[test]
fn test_publish_multi_module() {
    let test_kit = TestKit::new();
    test_kit.add_std_module(include_str!("resources/store.move"));

    let source = r#"
        address 0x1 {
            module Foo { public fun foo(): u64 { 1 } }
            module Bar { public fun bar(): u64 { 2 } }
            module Boo { public fun boo(): u64 { 10 } }
        }
        "#;

    let res = test_kit.publish_module(&source, gas_meta(), CORE_CODE_ADDRESS);
    test_kit.assert_success(&res);
    test_kit.merge_result(&res);

    let res = store_call_tx_without_args(&test_kit, "use 0x1::Boo;", "Boo::boo();");
    let value: U64Store = lcs::from_bytes(&res.write_set[0].value).unwrap();
    assert_eq!(value.val, 4);

    let res = store_call_tx_without_args(
        &test_kit,
        "use 0x1::Foo; use 0x1::Bar;",
        "Foo::foo() + Bar::bar();",
    );
    let value: U64Store = lcs::from_bytes(&res.write_set[0].value).unwrap();
    assert_eq!(value.val, 1 + 2 + 10);
}

#[inline]
fn store_call_tx_without_args<S: AsRef<str>>(
    kit: &TestKit,
    using: S,
    call: S,
) -> VmExecuteResponse {
    let source_tx = format!(
        r#"
        script {{
            {using}
            use 0x1::Store;
            fun main(account: &signer) {{
                let res = {call}
                Store::store_u64(account, res);
            }}
        }}
        "#,
        using = using.as_ref(),
        call = call.as_ref()
    );
    let res = kit.execute_script(
        &source_tx,
        gas_meta(),
        vec![],
        vec![],
        vec![AccountAddress::random()],
    );
    kit.assert_success(&res);
    res
}
